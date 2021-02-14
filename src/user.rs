use crate::{
    errors::RbxError,
    request::*,
};
use reqwest::blocking::Client as HttpClient;
use serde_json::Value;

#[derive(Debug)]
pub struct User {
    http_client: HttpClient,
    cookie: String,
    id: u32,
    username: String,
}

impl User {
    pub fn new(id: u32, username: String, http_client: HttpClient, cookie: String) -> Self {
        Self {
            id,
            username,
            http_client,
            cookie,
        }
    }

    pub fn send_message(&self, subject: &'_ str, body: &'_ str) /* -> Message */ {}

    pub fn get_role_in_group(&self, group_id: u32) /* -> Role */ {}

    pub fn get_friends(&self) /* Vec<User> */ {}

    pub fn block(&self) /* -> u16 */ {}

    pub fn unblock(&self) /* -> u16 */ {}

    pub fn follow(&self) /* -> u16 */ {}

    pub fn unfollow(&self) /* -> u16 */ {}

    pub fn get_detailed_user(&self) /* -> DetailedUser */ {}

    pub fn get_gamepasses(&self, cursor: Option<String>) /* -> Vec<Gamepass> */ {}

    pub fn has_gamepass(&self, gamepass_id: u32) /* -> bool */ {}

    pub fn get_groups(&self) /* -> Vec<GroupMember> */ {}

    pub fn get_status(&self) -> Result<String, RbxError> {
        let uri = &format!("https://users.roblox.com/v1/users/{}/status", self.id);
        let response = new_request(&self.http_client, None, uri, "GET").send()?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<Value>()?;
                let status = json
                    .get("status")
                    .ok_or(RbxError::FieldMissing("status"))?
                    .as_str()
                    .unwrap();
                Ok(status.to_owned())
            }
            unexpected => Err(RbxError::UnexpectedStatusCode(unexpected)),
        }
    }
}
