use der_parser::der::Tag;
use der_parser::oid::Oid;
use nom::HexDisplay;
use std::cmp::min;
use std::convert::TryFrom;
use std::fmt::{self, Write};
use std::net::{Ipv4Addr, Ipv6Addr};
use x509_parser::prelude::*;
use x509_parser::public_key::PublicKey;
use x509_parser::signature_algorithm::SignatureAlgorithm;

#[tauri::command]
pub fn x509_parse(pem: &str) -> Result<String, String> {
    let mut buffer = String::new();
    for (n, pem) in Pem::iter_from_buffer(pem.as_bytes()).enumerate() {
        match pem {
            Ok(pem) => {
                let data = &pem.contents;
                writeln!(
                    &mut buffer,
                    "-----------------------------------------------------------------------\nCertificate [{}]",
                    n
                )
                .map_err(|e| format!("{}", e))?;
                handle_certificate(&mut buffer, data)?;
            }
            Err(e) => {
                return Err(format!("Error while decoding PEM entry {}: {}", n, e));
            }
        }
    }

    Ok(buffer)
}

fn print_hex_dump(buffer: &mut String, bytes: &[u8], max_len: usize) -> Result<(), fmt::Error> {
    let m = min(bytes.len(), max_len);
    write!(buffer, "{}", &bytes[..m].to_hex(16))?;
    if bytes.len() > max_len {
        writeln!(buffer, "... <continued>")?;
    }

    Ok(())
}

fn format_oid(oid: &Oid) -> String {
    match oid2sn(oid, oid_registry()) {
        Ok(s) => s.to_owned(),
        _ => format!("{}", oid),
    }
}

fn generalname_to_string(gn: &GeneralName) -> String {
    match gn {
        GeneralName::DNSName(name) => format!("DNSName:{}", name),
        GeneralName::DirectoryName(n) => format!("DirName:{}", n),
        GeneralName::EDIPartyName(obj) => format!("EDIPartyName:{:?}", obj),
        GeneralName::IPAddress(n) => format!("IPAddress:{:?}", n),
        GeneralName::OtherName(oid, n) => format!("OtherName:{}, {:?}", oid, n),
        GeneralName::RFC822Name(n) => format!("RFC822Name:{}", n),
        GeneralName::RegisteredID(oid) => format!("RegisteredID:{}", oid),
        GeneralName::URI(n) => format!("URI:{}", n),
        GeneralName::X400Address(obj) => format!("X400Address:{:?}", obj),
    }
}

fn print_x509_extension(
    buffer: &mut String,
    oid: &Oid,
    ext: &X509Extension,
) -> Result<(), fmt::Error> {
    writeln!(
        buffer,
        "    [crit:{} l:{}] {}: ",
        ext.critical,
        ext.value.len(),
        format_oid(oid)
    )?;
    match ext.parsed_extension() {
        ParsedExtension::AuthorityKeyIdentifier(aki) => {
            writeln!(buffer, "      X509v3 Authority Key Identifier")?;
            if let Some(key_id) = &aki.key_identifier {
                writeln!(buffer, "        Key Identifier: {:x}", key_id)?;
            }
            if let Some(issuer) = &aki.authority_cert_issuer {
                for name in issuer {
                    writeln!(buffer, "        Cert Issuer: {}", name)?;
                }
            }
            if let Some(serial) = aki.authority_cert_serial {
                writeln!(buffer, "        Cert Serial: {}", format_serial(serial))?;
            }
        }
        ParsedExtension::BasicConstraints(bc) => {
            writeln!(buffer, "      X509v3 CA: {}", bc.ca)?;
        }
        ParsedExtension::CRLDistributionPoints(points) => {
            writeln!(buffer, "      X509v3 CRL Distribution Points:")?;
            for point in points.iter() {
                if let Some(name) = &point.distribution_point {
                    writeln!(buffer, "        Full Name: {:?}", name)?;
                }
                if let Some(reasons) = &point.reasons {
                    writeln!(buffer, "        Reasons: {}", reasons)?;
                }
                if let Some(crl_issuer) = &point.crl_issuer {
                    write!(buffer, "        CRL Issuer: ")?;
                    for gn in crl_issuer {
                        write!(buffer, "{} ", generalname_to_string(gn))?;
                    }
                    writeln!(buffer)?;
                }
                writeln!(buffer)?;
            }
        }
        ParsedExtension::KeyUsage(ku) => {
            writeln!(buffer, "      X509v3 Key Usage: {}", ku)?;
        }
        ParsedExtension::NSCertType(ty) => {
            writeln!(buffer, "      Netscape Cert Type: {}", ty)?;
        }
        ParsedExtension::SubjectAlternativeName(san) => {
            for name in &san.general_names {
                let s = match name {
                    GeneralName::DNSName(s) => {
                        format!("DNS:{}", s)
                    }
                    GeneralName::IPAddress(b) => {
                        let ip = match b.len() {
                            4 => {
                                let b = <[u8; 4]>::try_from(*b).unwrap();
                                let ip = Ipv4Addr::from(b);
                                format!("{}", ip)
                            }
                            16 => {
                                let b = <[u8; 16]>::try_from(*b).unwrap();
                                let ip = Ipv6Addr::from(b);
                                format!("{}", ip)
                            }
                            l => format!("invalid (len={})", l),
                        };
                        format!("IP Address:{}", ip)
                    }
                    _ => {
                        format!("{:?}", name)
                    }
                };
                writeln!(buffer, "      X509v3 SAN: {}", s)?;
            }
        }
        ParsedExtension::SubjectKeyIdentifier(id) => {
            writeln!(buffer, "      X509v3 Subject Key Identifier: {:x}", id)?;
        }
        x => {
            writeln!(buffer, "      {:?}", x)?;
        }
    }

    Ok(())
}

fn print_x509_digest_algorithm(
    buffer: &mut String,
    alg: &AlgorithmIdentifier,
    level: usize,
) -> Result<(), fmt::Error> {
    writeln!(
        buffer,
        "{:indent$}Oid: {}",
        "",
        format_oid(&alg.algorithm),
        indent = level
    )?;
    if let Some(parameter) = &alg.parameters {
        let s = match parameter.tag() {
            Tag::Oid => {
                let oid = parameter.as_oid().unwrap();
                format_oid(&oid)
            }
            _ => format!("{}", parameter.tag()),
        };
        writeln!(
            buffer,
            "{:indent$}Parameter: <PRESENT> {}",
            "",
            s,
            indent = level
        )?;
        let bytes = parameter.as_bytes();
        print_hex_dump(buffer, bytes, 32)?;
    } else {
        writeln!(buffer, "{:indent$}Parameter: <ABSENT>", "", indent = level)?;
    }

    Ok(())
}

fn print_x509_info(buffer: &mut String, x509: &X509Certificate) -> Result<(), fmt::Error> {
    let version = x509.version();
    if version.0 < 3 {
        writeln!(buffer, "  Version: {}", version)?;
    } else {
        writeln!(buffer, "  Version: INVALID({})", version.0)?;
    }
    writeln!(
        buffer,
        "  Serial: {}",
        x509.tbs_certificate.raw_serial_as_string()
    )?;
    writeln!(buffer, "  Subject: {}", x509.subject())?;
    writeln!(buffer, "  Issuer: {}", x509.issuer())?;
    writeln!(buffer, "  Validity:")?;
    writeln!(buffer, "    NotBefore: {}", x509.validity().not_before)?;
    writeln!(buffer, "    NotAfter:  {}", x509.validity().not_after)?;
    writeln!(buffer, "    is_valid:  {}", x509.validity().is_valid())?;
    writeln!(buffer, "  Subject Public Key Info:")?;
    print_x509_ski(buffer, x509.public_key())?;
    print_x509_signature_algorithm(buffer, &x509.signature_algorithm, 4)?;

    writeln!(buffer, "  Signature Value:")?;
    for l in format_number_to_hex_with_colon(&x509.signature_value.data, 16) {
        writeln!(buffer, "      {}", l)?;
    }
    writeln!(buffer, "  Extensions:")?;
    for ext in x509.extensions() {
        print_x509_extension(buffer, &ext.oid, ext)?;
    }
    writeln!(buffer)?;
    write!(buffer, "Structure validation status: ")?;
    #[cfg(feature = "validate")]
    {
        let mut logger = VecLogger::default();
        // structure validation status
        let ok = X509StructureValidator
            .chain(X509CertificateValidator)
            .validate(x509, &mut logger);
        if ok {
            writeln!(buffer, "Ok")?;
        } else {
            writeln!(buffer, "FAIL")?;
        }
        for warning in logger.warnings() {
            writeln!(buffer, "  [W] {}", warning)?;
        }
        for error in logger.errors() {
            writeln!(buffer, "  [E] {}", error)?;
        }
        writeln!(buffer)?;
    }

    #[cfg(feature = "verify")]
    {
        write!(buffer, "Signature verification: ")?;
        if x509.subject() == x509.issuer() {
            if x509.verify_signature(None).is_ok() {
                writeln!(buffer, "OK")?;
                writeln!(buffer, "  [I] certificate is self-signed")?;
            } else if x509.subject() == x509.issuer() {
                writeln!(buffer, "FAIL")?;
                writeln!(
                    buffer,
                    "  [W] certificate looks self-signed, but signature verification failed"
                )?;
            }
        } else {
            // if subject is different from issuer, we cannot verify certificate without the public key of the issuer
            writeln!(buffer, "N/A")?;
        }
    }
    Ok(())
}

fn print_x509_signature_algorithm(
    buffer: &mut String,
    signature_algorithm: &AlgorithmIdentifier,
    indent: usize,
) -> Result<(), fmt::Error> {
    match SignatureAlgorithm::try_from(signature_algorithm) {
        Ok(sig_alg) => {
            write!(buffer, "  Signature Algorithm: ")?;
            match sig_alg {
                SignatureAlgorithm::DSA => {
                    writeln!(buffer, "DSA")?;
                }
                SignatureAlgorithm::ECDSA => {
                    writeln!(buffer, "ECDSA")?;
                }
                SignatureAlgorithm::ED25519 => {
                    writeln!(buffer, "ED25519")?;
                }
                SignatureAlgorithm::RSA => {
                    writeln!(buffer, "RSA")?;
                }
                SignatureAlgorithm::RSASSA_PSS(params) => {
                    writeln!(buffer, "RSASSA-PSS")?;
                    let indent_s = format!("{:indent$}", "", indent = indent + 2);
                    writeln!(
                        buffer,
                        "{}Hash Algorithm: {}",
                        indent_s,
                        format_oid(params.hash_algorithm_oid()),
                    )?;
                    write!(buffer, "{}Mask Generation Function: ", indent_s)?;
                    if let Ok(mask_gen) = params.mask_gen_algorithm() {
                        writeln!(
                            buffer,
                            "{}/{}",
                            format_oid(&mask_gen.mgf),
                            format_oid(&mask_gen.hash),
                        )?;
                    } else {
                        writeln!(buffer, "INVALID")?;
                    }
                    writeln!(buffer, "{}Salt Length: {}", indent_s, params.salt_length())?;
                }
                SignatureAlgorithm::RSAAES_OAEP(params) => {
                    writeln!(buffer, "RSAAES-OAEP")?;
                    let indent_s = format!("{:indent$}", "", indent = indent + 2);
                    writeln!(
                        buffer,
                        "{}Hash Algorithm: {}",
                        indent_s,
                        format_oid(params.hash_algorithm_oid()),
                    )?;
                    write!(buffer, "{}Mask Generation Function: ", indent_s)?;
                    if let Ok(mask_gen) = params.mask_gen_algorithm() {
                        writeln!(
                            buffer,
                            "{}/{}",
                            format_oid(&mask_gen.mgf),
                            format_oid(&mask_gen.hash),
                        )?;
                    } else {
                        writeln!(buffer, "INVALID")?;
                    }
                    writeln!(
                        buffer,
                        "{}pSourceFunc: {}",
                        indent_s,
                        format_oid(&params.p_source_alg().algorithm),
                    )?;
                }
            }
        }
        Err(e) => {
            eprintln!("Could not parse signature algorithm: {}", e);
            writeln!(buffer, "  Signature Algorithm:")?;
            print_x509_digest_algorithm(buffer, signature_algorithm, indent)?;
        }
    }

    Ok(())
}

fn print_x509_ski(
    buffer: &mut String,
    public_key: &SubjectPublicKeyInfo,
) -> Result<(), fmt::Error> {
    writeln!(buffer, "    Public Key Algorithm:")?;
    print_x509_digest_algorithm(buffer, &public_key.algorithm, 6)?;
    match public_key.parsed() {
        Ok(PublicKey::RSA(rsa)) => {
            writeln!(buffer, "    RSA Public Key: ({} bit)", rsa.key_size())?;
            // print_hex_dump(rsa.modulus, 1024);
            for l in format_number_to_hex_with_colon(rsa.modulus, 16) {
                writeln!(buffer, "        {}", l)?;
            }
            if let Ok(e) = rsa.try_exponent() {
                writeln!(buffer, "    exponent: 0x{:x} ({})", e, e)?;
            } else {
                writeln!(buffer, "    exponent: <INVALID>:")?;
                print_hex_dump(buffer, rsa.exponent, 32)?;
            }
        }
        Ok(PublicKey::EC(ec)) => {
            writeln!(buffer, "    EC Public Key: ({} bit)", ec.key_size())?;
            for l in format_number_to_hex_with_colon(ec.data(), 16) {
                writeln!(buffer, "        {}", l)?;
            }
        }
        Ok(PublicKey::DSA(y)) => {
            writeln!(buffer, "    DSA Public Key: ({} bit)", 8 * y.len())?;
            for l in format_number_to_hex_with_colon(y, 16) {
                writeln!(buffer, "        {}", l)?;
            }
        }
        Ok(PublicKey::GostR3410(y)) => {
            writeln!(
                buffer,
                "    GOST R 34.10-94 Public Key: ({} bit)",
                8 * y.len()
            )?;
            for l in format_number_to_hex_with_colon(y, 16) {
                writeln!(buffer, "        {}", l)?;
            }
        }
        Ok(PublicKey::GostR3410_2012(y)) => {
            writeln!(
                buffer,
                "    GOST R 34.10-2012 Public Key: ({} bit)",
                8 * y.len()
            )?;
            for l in format_number_to_hex_with_colon(y, 16) {
                writeln!(buffer, "        {}", l)?;
            }
        }
        Ok(PublicKey::Unknown(b)) => {
            writeln!(buffer, "    Unknown key type")?;
            print_hex_dump(buffer, b, 256)?;
            if let Ok((rem, res)) = der_parser::parse_der(b) {
                writeln!(buffer, "rem: {} bytes", rem.len())?;
                writeln!(buffer, "{:?}", res)?;
            } else {
                writeln!(buffer, "      <Could not parse key as DER>")?;
            }
        }
        Err(_) => {
            writeln!(buffer, "    INVALID PUBLIC KEY")?;
        }
    }

    Ok(())
}

fn format_number_to_hex_with_colon(b: &[u8], row_size: usize) -> Vec<String> {
    let mut v = Vec::with_capacity(1 + b.len() / row_size);
    for r in b.chunks(row_size) {
        let s = r.iter().fold(String::with_capacity(3 * r.len()), |a, b| {
            a + &format!("{:02x}:", b)
        });
        v.push(s)
    }
    v
}

fn handle_certificate(buffer: &mut String, data: &[u8]) -> Result<(), String> {
    match parse_x509_certificate(data) {
        Ok((_, x509)) => print_x509_info(buffer, &x509).map_err(|e| format!("{}", e)),
        Err(e) => Err(format!("{}", e)),
    }
}
