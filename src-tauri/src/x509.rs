use x509_parser::prelude::*;

fn x509_parse(certs: &str) -> Result<(), String> {
    for pem in Pem::iter_from_buffer(certs.as_bytes()) {
        let pem = pem.map_err(|e| format!("{e:?}"))?;
        let pem = pem.parse_x509().map_err(|e| format!("{e:?}"))?;


    }

    Ok(())
}
