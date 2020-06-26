use http::Method;
use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::collections::HashMap;
use std::time::Duration;


#[derive(Serialize, Deserialize)]
struct Req {
    url: String,
    method: String,
    timeout: u8,
    headers: HashMap<String, String>,
}

fn main() {
    let r_str = r#"{
        "url":"https://douban.com",
        "method":"GET",
        "timeout": 5,
        "headers": {
            "user-agent":"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.106 Safari/537.36"
        }
    }"#;
    let v: Req = serde_json::from_str(r_str).unwrap();
    let rsp = request(v);
    println!("{:#?}", rsp);
}


fn request(params: Req) -> Result<String, reqwest::Error> {
    let uri = reqwest::Url::parse(params.url.as_str()).unwrap();
    let ts = Duration::new(params.timeout.into(), 0);
    let method = Method::from_bytes(params.method.as_bytes()).unwrap();

    // let http_proxy = reqwest::Proxy::http(&params.proxy)?;
    // let https_proxy = reqwest::Proxy::https(&params.proxy)?;

    let mut headers_map = HeaderMap::new();
    for (key, value) in &params.headers {
        headers_map.insert(
            HeaderName::from_lowercase(key.as_bytes()).unwrap(),
            HeaderValue::from_bytes(value.as_bytes()).unwrap(),
        );
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(ts)
        .build()?;
    let res = client.request(method, uri).headers(headers_map).send()?;

    let mut rsp_headers = HashMap::new();
    for (key, value) in res.headers().iter() {
        rsp_headers.insert(key.to_string(), value.to_str().unwrap().to_string());
    }
    let length = res.content_length().unwrap_or(0);

    let rsp = json!({
        "code": 0,
        "data": {
            "url": params.url,
            "status": res.status().as_u16(),
            "headers": rsp_headers,
            "content_length": length,
            "content": res.text()?
        }
    });
    Ok(rsp.to_string())
}
