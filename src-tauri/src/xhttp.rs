use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
    redirect, Method,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use url::Url;

#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub use_proxy: bool,
    pub use_redirect: bool,
    pub use_resolve: String,
    pub use_certs_check: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub status: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub addr: String,
}

#[tauri::command]
pub fn http_request(
    url: String,
    method: String,
    headers: HashMap<String, String>,
    body: String,
    use_proxy: bool,
    use_redirect: bool,
    use_resolve: String,
    use_certs_check: bool,
) -> Result<(String, HashMap<String, String>, String, String), String> {
    let req = Request {
        url,
        method,
        headers,
        body,
        use_proxy,
        use_redirect,
        use_resolve,
        use_certs_check,
    };

    match do_request(&req) {
        Ok(resp) => Ok((resp.status, resp.headers, resp.body, resp.addr)),
        Err(e) => Err(e),
    }
}

fn do_request(req: &Request) -> Result<Response, String> {
    let mut client = Client::builder();
    //是否使用系统代理
    if !req.use_proxy {
        client = client.no_proxy();
    }

    //是否处理重定向
    if !req.use_redirect {
        client = client.redirect(redirect::Policy::none());
    }

    //是否校验证书
    if !req.use_certs_check {
        client = client.danger_accept_invalid_certs(true);
    }

    //是否指定解析
    if req.use_resolve != "" {
        let url = Url::parse(&req.url).map_err(|e| format!("{e:?}"))?;
        let domain = url.host_str().ok_or("no host")?;
        let addr = req.use_resolve.parse().map_err(|e| format!("{e:?}"))?;

        client = client.resolve(domain, addr);
    }
    let client = client.build().map_err(|e| format!("{e:?}"))?;

    let method = match req.method.to_ascii_lowercase().as_str() {
        "head" => Method::HEAD,
        "get" => Method::GET,
        "post" => Method::POST,
        "put" => Method::PUT,
        "patch" => Method::PATCH,
        "delete" => Method::DELETE,
        "options" => Method::OPTIONS,
        _ => return Err("unknown http method".to_string()),
    };

    let mut headers = HeaderMap::new();
    for (k, v) in &req.headers {
        let k = HeaderName::from_str(k).map_err(|e| format!("{e:?}"))?;
        let v = HeaderValue::from_str(v).map_err(|e| format!("{e:?}"))?;

        headers.insert(k, v);
    }

    let mut rb = client.request(method, &req.url).headers(headers);

    if req.body != "" {
        rb = rb.body(req.body.to_owned());
    }

    let resp = rb.send().map_err(|e| format!("{e:?}"))?;

    let addr = match resp.remote_addr() {
        Some(addr) => addr.to_string(),
        None => "no remote addr".to_string(),
    };

    let resp_status = format!("{}", resp.status());
    let resp_headers = resp
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                String::from_utf8_lossy(v.as_bytes()).to_string(),
            )
        })
        .collect::<HashMap<String, String>>();
    let resp_body = match resp.text() {
        Ok(body) => body,
        Err(e) => format!("{e:?}"),
    };

    Ok(Response {
        status: resp_status,
        headers: resp_headers,
        body: resp_body,
        addr,
    })
}
