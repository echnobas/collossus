use reqwest::{
    blocking::{
        Client,
        RequestBuilder,
    },
    header::{
        HeaderMap,
        HeaderValue,
        ACCEPT,
        CONTENT_LENGTH,
        COOKIE,
    },
};
use serde_json::Value;

/// Internal function to create new request with content-length and x-csrf handled
#[allow(dead_code)]
pub(in crate) fn new_request(
    http_client: &Client,
    body: Option<&str>,
    endpoint: &str,
    method: &'static str,
) -> RequestBuilder {
    let json = serde_json::from_str::<Value>(body.unwrap_or("{}")).unwrap();
    let mut headers = HeaderMap::new();
    // <HeaderMap>.insert(T,V) is always safe to unwrap, None is returned if the key inserted is unique
    headers.insert(CONTENT_LENGTH, HeaderValue::from(json.to_string().len()));
    headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
    match method {
        "POST" => http_client.post(endpoint).json(&json).headers(headers),
        "GET" => http_client.get(endpoint).json(&json).headers(headers),
        "DELETE" => http_client.delete(endpoint).json(&json).headers(headers),
        "PATCH" => http_client.patch(endpoint).json(&json).headers(headers),
        unsupported => panic!("unspported request method `{}`", unsupported),
    }
}

/// Internal function to create new request with content-length and x-csrf handled
#[allow(dead_code)]
pub(in crate) fn new_authorized_request(
    http_client: &Client,
    body: Option<&str>,
    endpoint: &str,
    method: &'static str,
    cookie: &str,
) -> Result<RequestBuilder, crate::errors::RbxError> {
    let json = serde_json::from_str::<Value>(body.unwrap_or("{}")).unwrap();
    let mut headers = HeaderMap::new();
    // <HeaderMap>.insert(T,V) is always safe to unwrap, None is returned if the key inserted is unique
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&format!(".ROBLOSECURITY={};", cookie)).unwrap(),
    );
    headers.insert(
        "X-CSRF-TOKEN",
        HeaderValue::from_str(&x_csrf(&http_client, cookie)?).unwrap(),
    );
    headers
        .insert(CONTENT_LENGTH, HeaderValue::from(json.to_string().len()))
        .unwrap();
    headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
    match method {
        "POST" => Ok(http_client.post(endpoint).json(&json).headers(headers)),
        "GET" => Ok(http_client.get(endpoint).json(&json).headers(headers)),
        "DELETE" => Ok(http_client.delete(endpoint).json(&json).headers(headers)),
        "PATCH" => Ok(http_client.patch(endpoint).json(&json).headers(headers)),
        unsupported => panic!("unspported request method `{}`", unsupported),
    }
}

/// Internal function to retrieve x-csrf-token
fn x_csrf(http_client: &Client, cookie: &str) -> Result<String, crate::errors::RbxError> {
    const ENDPOINT: &str = "https://auth.roblox.com/v2/logout";
    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&format!(".ROBLOSECURITY={};", cookie)).unwrap(),
    );
    headers.insert(CONTENT_LENGTH, HeaderValue::from_str("0").unwrap());
    headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
    let resp = http_client.post(ENDPOINT).headers(headers).send()?;
    let token = resp.headers().get("x-csrf-token").unwrap_or_else(|| {
        panic!("did not recieve x-csrf-token");
    });
    Ok(token.to_str().unwrap().to_string())
}
