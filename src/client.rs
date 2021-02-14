use crate::{
    errors::RbxError,
    group::Group,
    request::{
        new_authorized_request,
        new_request,
    },
    user::User,
};
use reqwest::blocking::Client as HttpClient;
use serde_json::Value;
pub struct Client {
    http_client: HttpClient,
    cookie: String,
}

impl Client {
    pub fn new(cookie: &'_ str) -> Self {
        Self {
            http_client: HttpClient::new(),
            cookie: cookie.to_owned(),
        }
    }

    pub fn get_trades(&self) /* ->  Vec<TradeRequest> */ {}

    pub fn get_group(&self, group_id: u32) -> Result<Group, RbxError> {
        let uri: &str = &format!("https://groups.roblox.com/v1/groups/{}/", group_id);
        let response = new_request(&self.http_client, None, &uri, "GET").send()?;
        match response.status().as_u16() {
            400 => panic!("group does not exist"),
            429 => panic!("ratelimit"),
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .expect("decoding json failed?");
                let group_id = json
                    .get("id")
                    .expect("decode error, expected `id` but found None")
                    .as_u64()
                    .unwrap();
                let group_name = json
                    .get("name")
                    .expect("decode error, expected_`name` but found None")
                    .as_str()
                    .unwrap();
                let group_description = json
                    .get("description")
                    .expect("decode error, expected `description` but found None")
                    .as_str()
                    .unwrap();
                let owner = json.get("owner").ok_or(RbxError::FieldMissing("owner"))?;
                let owner_username = owner
                    .get("username")
                    .map(|item| -> Option<String> {
                        item.as_str().map(|item| {
                            item.to_owned()
                        })
                    }).flatten();
                let owner_id = owner
                    .get("userId")
                    .map(|item| -> Option<u32> {
                        item.as_u64().map(|item| -> u32 {
                            item as u32
                        })
                    })
                    .flatten();
                Ok(Group::new(
                    group_id as u32,
                    group_name.to_owned(),
                    group_description.to_owned(),
                    owner_id,
                    owner_username,
                    self.http_client.clone(),
                    self.cookie.clone(),
                ))
            }
            unknown => panic!("unknown {}", unknown),
        }
    }

    pub fn get_user_by_username(&self, username: &'_ str) -> Result<User, RbxError> {
        let uri = format!(
            "https://api.roblox.com/users/get-by-username?username={}",
            username
        );
        let response = new_request(&self.http_client, None, &uri, "GET").send()?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<Value>()?;
                let id = json
                    .get("Id")
                    .ok_or(RbxError::FieldMissing("Id"))?
                    .as_u64()
                    .unwrap();
                let username = json
                    .get("Username")
                    .ok_or(RbxError::FieldMissing("Username"))?
                    .as_str()
                    .unwrap();
                Ok(User::new(
                    id as u32,
                    username.to_owned(),
                    self.http_client.clone(),
                    self.cookie.clone(),
                ))
            }
            unexpected => Err(RbxError::UnexpectedStatusCode(unexpected)),
        }
    }

    pub fn get_user_by_id(&self, id: u32) /* -> Result<User, RbxError> */ {}

    pub fn get_user(&self, username: Option<&'_ str>, id: Option<u32>) -> Result<User, RbxError> {
        if username.is_some() {
            self.get_user_by_username(username.unwrap())
        } else {
            // self.get_user_by_id(id.unwrap());
            unimplemented!()
        }
    }

    pub fn get_friends(&self) /* -> Vec<User> */ {}

    pub fn change_status(&self, status: &'_ str) /* -> u16 */ {}
}
