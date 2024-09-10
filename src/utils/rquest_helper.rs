use std::{collections::HashMap, error::Error};

use rquest::{tls::Impersonate, Proxy};
use serde::Serialize;



#[derive(Debug, Clone, Serialize)]
pub enum BodyDataMap {
  String(String),
  Number(f64),
  Boolean(bool),
  Array(Vec<BodyDataMap>),
  Map(HashMap<String, BodyDataMap>),
}

impl ToString for BodyDataMap {
  fn to_string(&self) -> String {
      match self {
          BodyDataMap::String(s) => s.clone(),
          BodyDataMap::Number(n) => n.to_string(),
          BodyDataMap::Boolean(b) => b.to_string(),
          BodyDataMap::Array(a) => format!("{:?}", a),
          BodyDataMap::Map(m) => format!("{:?}", m),
      }
  }
}

pub struct RquestHttpHelper {
  rquest_client: rquest::Client,
}

impl RquestHttpHelper {
  pub fn new(browser: Impersonate, proxy_url: String) -> Result<Self, Box<dyn Error>> {
    let client = rquest::Client::builder()
    .impersonate(browser)
    .proxy(Proxy::all(proxy_url)?)
    .enable_ech_grease()
    .permute_extensions()
    // .cookie_store(true)
    .build()?;
    
    return Ok(Self { rquest_client: client });
  }

  pub async fn get(&self, url: String, headers: &HashMap<String, String>, cookies: &HashMap<String, String>) -> Result<rquest::Response, Box<dyn Error>> {
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
    request.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)
  }

  pub async fn post(&self, url: String, headers: &HashMap<String, String>, cookies: &HashMap<String, String>, body_data: &HashMap<String, BodyDataMap>) -> Result<rquest::Response, Box<dyn Error>> {
    let mut request = self.rquest_client.post(&url);
    
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
    
    // 检查Content-Type并构建请求体
    if headers.get("Content-Type").map(|s| s.as_str()) == Some("application/x-www-form-urlencoded") {
      // 使用urlencode编码数据
      let form_data: String = body_data.iter()
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(&v.to_string())))
        .collect::<Vec<String>>()
        .join("&");
      request = request.body(form_data);
    } else {
      // 使用json编码数据
      let json_body = serde_json::to_string(body_data)?;
      request = request.body(json_body);
    }
    
    // 发送请求并返回结果
    request.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)
  }

}