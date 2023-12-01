use std::{collections::HashMap, os::unix::process};

#[derive(Debug, PartialEq)]
pub enum Method {
  Get,
  Post,
  Uninitialized,
}

impl From<&str> for Method {
  fn from(s: &str) -> Self {
    match s {
      "GET" => Method::Get,
      "POST" => Method::Post,
      _ => Method::Uninitialized,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Version {
  V1_1,
  V2_0,
  Uninitialized,
}

impl From<&str> for Version {
  fn from(s: &str) -> Self {
    match s {
      "HTTP/1.1" => Version::V1_1,
      _ => Version::Uninitialized,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
  Path(String)
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
  pub method: Method,
  pub version: Version,
  pub resource: Resource,
  pub headers: HashMap<String, String>,
  pub msg_body: String,
}

impl From<String> for HttpRequest {
  fn from(req: String) -> Self {
    let mut parsed_method = Method::Uninitialized;
    let mut parsed_version = Version::Uninitialized;
    let mut parsed_resource = Resource::Path(String::from(""));
    let mut parsed_headers = HashMap::new();
    let mut parsed_msg_body = "";

    
    for line in req.lines() {
      if line.contains("HTTP") {
        let (method, resource, version ) = process_req_line(line);
        parsed_method = method;
        parsed_resource = resource;
        parsed_version = version;
      } else if line.contains(":") {
        let (key, value) = process_header_line(line);
        parsed_headers.insert(key, value);
      } else if line.len() == 0 {

      } else {
        parsed_msg_body = line;
      }
    }

    HttpRequest {
      method: parsed_method,
      version: parsed_version,
      resource: parsed_resource,
      headers: parsed_headers,
      msg_body: String::from(parsed_msg_body),
    }
  }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
  let mut words = s.split_whitespace();
  let method = words.next().unwrap();
  let resource = words.next().unwrap();
  let version = words.next().unwrap();

  (
    method.into(),
    Resource::Path(String::from(resource)),
    version.into(),
  )
}

fn process_header_line(s: &str) -> (String, String) {
  let mut header_items = s.split(":");
  let mut key = String::from("");
  let mut value = String::from("");

  if let Some(k) = header_items.next() {
    key = k.to_string();
  }

  if let Some(v) = header_items.next() {
    value = v.to_string();
  }

  (key, value)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_method_into() {
    let m: Method = "GET".into();
    assert_eq!(m, Method::Get);
  }

  #[test]
  fn test_version_into() {
    let v: Version = "HTTP/1.1".into();
    assert_eq!(v, Version::V1_1);
  }

  #[test]
  fn test_process_req_line() {
    let s = String::from("GET /greeting HTTP/1.1\r\nHost: localhost\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
    let mut headers_expected = HashMap::new();
    headers_expected.insert(String::from("Host"), String::from(" localhost"));
    headers_expected.insert(String::from("Accept"), String::from(" */*"));
    headers_expected.insert(String::from("User-Agent"), String::from(" curl/7.64.1"));

    let req: HttpRequest = s.into();
    assert_eq!(req.method, Method::Get);
    assert_eq!(req.version, Version::V1_1);
    assert_eq!(req.resource, Resource::Path(String::from("/greeting")));
    assert_eq!(req.headers, headers_expected);
  }
}