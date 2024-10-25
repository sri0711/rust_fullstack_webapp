use reqwest::{Client, Error};
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use serde::Serialize;

pub struct Parameters<T> {
    pub key: String,
    pub value: Option<T>,
}

pub enum Methods {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

pub struct NetworkCall<T> {
    pub url: String,
    pub query: Option<Vec<Parameters<T>>>,
    pub body: Option<Vec<Parameters<T>>>,
    pub headers: Option<Vec<Parameters<T>>>,
    pub method: Methods
}

impl NetworkCall<T>{
    pub async fn api_call(&self) -> Result<String, Error>
    where
        T: Serialize,
    {
        let mut headers_map = HeaderMap::new();
        if let Some(header_params) = &self.headers {
            for header in header_params {
                if let Some(value) = &header.value {
                    headers_map.insert(
                        header.key.parse().unwrap(),
                        HeaderValue::from_str(&value.to_string()).unwrap(),
                    );
                }
            }
        }
        let url = Self.url;
        let  client = Client::new();
        let request_builder = match self.method {
            Methods::Get => {
                let mut req = client.get(&self.url).headers(headers_map);
                if let Some(query_params) = &self.query {
                    let query: Vec<(String, String)> = query_params
                        .iter()
                        .map(|p| (p.key.clone(), p.value.as_ref().map_or("".to_string(), |v| v.to_string())))
                        .collect();
                    req = req.query(&query);
                }
                req
            }
            Methods::Post => {
                let mut req = client.post(&self.url).headers(headers_map);
                if let Some(body_params) = &self.body {
                    let body: HashMap<String, String> = body_params
                        .iter()
                        .map(|p| (p.key.clone(), p.value.as_ref().map_or("".to_string(), |v| v.to_string())))
                        .collect();
                    req = req.json(&body);
                }
                req
            }
            Methods::Patch => {
                let mut req = client.patch(&self.url).headers(headers_map);
                if let Some(body_params) = &self.body {
                    let body: HashMap<String, String> = body_params
                        .iter()
                        .map(|p| (p.key.clone(), p.value.as_ref().map_or("".to_string(), |v| v.to_string())))
                        .collect();
                    req = req.json(&body);
                }
                req
            }
            Methods::Put => {
                let mut req = client.put(&self.url).headers(headers_map);
                if let Some(body_params) = &self.body {
                    let body: HashMap<String, String> = body_params
                        .iter()
                        .map(|p| (p.key.clone(), p.value.as_ref().map_or("".to_string(), |v| v.to_string())))
                        .collect();
                    req = req.json(&body);
                }
                req
            }
            Methods::Delete => {
                let mut req = client.delete(&self.url).headers(headers_map);
                if let Some(query_params) = &self.query {
                    let query: Vec<(String, String)> = query_params
                        .iter()
                        .map(|p| (p.key.clone(), p.value.as_ref().map_or("".to_string(), |v| v.to_string())))
                        .collect();
                    req = req.query(&query);
                }
                req
            }
        };
        let response = request_builder.send().await?;
        let response_text = response.text().await?;


        Ok(response_text)

    }
}