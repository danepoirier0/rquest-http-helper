use std::collections::HashMap;

use rquest::{tls::Impersonate, Proxy};


pub struct RquestHttpHelper {
  rquest_client: rquest::Client,
}

impl RquestHttpHelper {
  pub fn new(browser: Impersonate, proxy_url: String) -> Result<Self, rquest::Error> {
    let client = rquest::Client::builder()
    .impersonate(browser)
    .proxy(Proxy::all(proxy_url)?)
    .enable_ech_grease()
    .permute_extensions()
    // .cookie_store(true)
    .build()?;
    
    return Ok(Self { rquest_client: client });
  }

  pub async fn get(&self, url: String, headers: &HashMap<String, String>, cookies: &HashMap<String, String>) -> Result<rquest::Response, rquest::Error> {
    let mut request = self.rquest_client.get(&url);
    
    // 添加请求头
    for (key, value) in headers {
      request = request.header(key, value);
    }
    
    // 添加 cookies
    let cookie_header = cookies.iter()
      .map(|(k, v)| format!("{}={}", k, v))
      .collect::<Vec<String>>()
      .join("; ");
    request = request.header("Cookie", cookie_header);
    
    // 发送请求并返回结果
    request.send().await
  }
}