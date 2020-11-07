use crate::{
    errors::RbxError,
    rbx_client::RequestMethod::Post,
};
use http::{
    header::{
        ACCEPT,
        CONTENT_LENGTH,
        COOKIE,
    },
    HeaderMap,
    HeaderValue,
    StatusCode,
};
use reqwest::blocking::RequestBuilder;
use serde_json::Value;

pub struct RbxClient {
    http_client: reqwest::blocking::Client,
    cookie: Option<String>,
}
#[allow(dead_code)]
pub(in crate) enum RequestMethod {
    Post,
    Get,
    Delete,
    Patch,
}

impl RbxClient {
    pub fn new<T>(cookie: T) -> Self
    where
        T: Into<String>,
    {
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        Self {
            http_client: client,
            cookie: Some(cookie.into()),
        }
    }

    pub fn new_without_cookie() -> Self {
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();
        Self {
            http_client: client,
            cookie: None,
        }
    }

    /// Internal function to create new request with content-length and x-csrf handled
    #[allow(dead_code)]
    pub(in crate) fn new_request(
        &self,
        body: Option<&str>,
        endpoint: &str,
        method: RequestMethod,
    ) -> RequestBuilder {
        let json = serde_json::from_str::<Value>(body.unwrap_or("{}")).unwrap();
        let mut headers = HeaderMap::new();
        // <HeaderMap>.insert(T,V) is always safe to unwrap, None is returned if the key inserted is unique
        headers.insert(CONTENT_LENGTH, HeaderValue::from(json.to_string().len()));
        headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
        match method {
            RequestMethod::Post => self.http_client.post(endpoint).json(&json).headers(headers),
            RequestMethod::Get => self.http_client.get(endpoint).json(&json).headers(headers),
            RequestMethod::Delete => self
                .http_client
                .delete(endpoint)
                .json(&json)
                .headers(headers),
            RequestMethod::Patch => self
                .http_client
                .patch(endpoint)
                .json(&json)
                .headers(headers),
        }
    }

    /// Internal function to create new request with content-length and x-csrf handled
    pub(in crate) fn new_authorized_request(
        &self,
        body: Option<&str>,
        endpoint: &str,
        method: RequestMethod,
    ) -> Result<RequestBuilder, crate::errors::RbxError> {
        if self.cookie.is_none() {
            return Err(RbxError::Unauthorized(
                "cannot create authorized request with unauthorized client",
            ));
        }
        let json = serde_json::from_str::<Value>(body.unwrap_or("{}")).unwrap();
        let mut headers = HeaderMap::new();
        // <HeaderMap>.insert(T,V) is always safe to unwrap, None is returned if the key inserted is unique
        headers.insert("x-csrf-token", (&self.x_csrf().unwrap()).parse().unwrap());
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&format!(
                ".ROBLOSECURITY={};",
                self.cookie.as_ref().unwrap()
            ))
            .unwrap(),
        );
        headers.insert(CONTENT_LENGTH, HeaderValue::from(json.to_string().len()));
        headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
        match method {
            RequestMethod::Post => Ok(self.http_client.post(endpoint).json(&json).headers(headers)),
            RequestMethod::Get => Ok(self.http_client.get(endpoint).json(&json).headers(headers)),
            RequestMethod::Delete => Ok(self
                .http_client
                .delete(endpoint)
                .json(&json)
                .headers(headers)),
            RequestMethod::Patch => Ok(self
                .http_client
                .patch(endpoint)
                .json(&json)
                .headers(headers)),
        }
    }

    /// Internal function to retrieve x-csrf-token
    fn x_csrf(&self) -> Result<String, crate::errors::RbxError> {
        if self.cookie.is_none() {
            return Err(RbxError::Unauthorized(
                "cannot get xcsrf token with unauthorized client",
            ));
        }
        const ENDPOINT: &str = "https://auth.roblox.com/v2/logout";
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&format!(
                ".ROBLOSECURITY={};",
                self.cookie.as_ref().unwrap()
            ))
            .unwrap(),
        );
        headers.insert(CONTENT_LENGTH, HeaderValue::from_str("0").unwrap());
        headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
        let resp = self.http_client.post(ENDPOINT).headers(headers).send()?;
        let token = resp.headers().get("x-csrf-token").unwrap_or_else(|| {
            panic!("did not recieve x-csrf-token");
        });
        Ok(token.to_str().unwrap().to_string())
    }

    /// Consume the cookie and session
    /// **THIS WILL REQUIRE YOU TO GET A NEW COOKIE**
    pub fn logout(&self) -> Result<(), RbxError> {
        const ENDPOINT: &str = "https://auth.roblox.com/v2/logout";
        let resp = self.new_authorized_request(None, ENDPOINT, Post)?.send()?;
        match resp.status() {
            StatusCode::OK => Ok(()),
            v => Err(RbxError::UnexpectedStatusCode(v.as_u16())),
        }
    }

    // pub fn get_status(&self, userid: i32) -> Result<String, reqwest::Error> {
    //     println!("{}", crate::TARGET);
    //     let mut headers = HeaderMap::new();
    //     headers.insert(CONTENT_LENGTH, HeaderValue::from_str("0").unwrap());
    //     headers.insert(COOKIE, HeaderValue::from_str(&format!(".ROBLOSECURITY={};", self.cookie)).unwrap());
    //     let _resp = self.http_client.get(&format!("https://www.roblox.com/users/profile/profileheader-json?userId={}", userid))
    //         .headers(headers)
    //         .send()?;
    //     if _resp.status() != StatusCode::OK {
    //         panic!("status not ok")
    //     }
    //     let json = serde_json::from_str::<Value>(&_resp.text().unwrap()).unwrap();
    //     Ok(json.get("UserStatus").unwrap().as_str().unwrap().to_owned())
    // }
    //
    // pub fn get_username_from_id(&self, userid: i32) -> Result<String, reqwest::Error> {
    //     let mut headers = HeaderMap::new();
    //     headers.insert(CONTENT_LENGTH, HeaderValue::from_str("0").unwrap());
    //     headers.insert(COOKIE, HeaderValue::from_str(&format!(".ROBLOSECURITY={};", self.cookie)).unwrap());
    //     let _resp = self.http_client.get(&format!("https://www.roblox.com/users/profile/profileheader-json?userId={}", userid))
    //         .headers(headers)
    //         .send()?;
    //     let json = serde_json::from_str::<Value>(&_resp.text().unwrap()).unwrap();
    //     Ok(json.get("ProfileUserName").unwrap().as_str().unwrap().to_owned())
    // }
}
