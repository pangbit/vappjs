use hickory_resolver::config::*;
use hickory_resolver::proto::rr::RecordType;
use hickory_resolver::TokioAsyncResolver;

#[tauri::command]
pub async fn dns_lookup(host: String) -> Result<Vec<String>, String> {
    let mut result = vec![];

    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    let response = resolver.lookup_ip(host).await.map_err(|e| e.to_string())?;

    for record in response.as_lookup().record_iter() {
        match record.record_type() {
            RecordType::CNAME => {
                if let Some(data) = record.data() {
                    if let Some(data) = data.as_cname() {
                        let cname = format!("CNAME\t{}\t--> {}", record.name(), data.0.to_ascii());
                        result.push(cname);
                    }
                }
            }
            RecordType::A => {
                if let Some(data) = record.data() {
                    if let Some(data) = data.as_a() {
                        let a = format!("A\t{}\t--> {}", record.name(), data.0);
                        result.push(a);
                    }
                }
            }
            RecordType::AAAA => {
                if let Some(data) = record.data() {
                    if let Some(data) = data.as_aaaa() {
                        let aaaa = format!("AAAA\t{}\t--> {}", record.name(), data.0);
                        result.push(aaaa);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(result)
}
