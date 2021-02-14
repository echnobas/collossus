use crate::{
    errors::RbxError,
    group::Group,
    request::{
        new_authorized_request,
        new_request,
    },
};
use reqwest::blocking::Client as HttpClient;
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
        let uri: &str = &format!("https://groups.roblox.com/v1/groups/{}/", 1);
        // let response = self.http_client.get(uri).send()?;
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
                Ok(Group::new(
                    group_id as u32,
                    group_name.to_owned(),
                    group_description.to_owned(),
                    self.http_client.clone(),
                    self.cookie.clone(),
                ))
            }
            unknown => panic!("unknown {}", unknown),
        }
    }

    pub fn get_user_by_username(&self, username: &'_ str) /* -> User */ {}

    pub fn get_user_by_id(&self, id: u32) /* -> User */ {}

    pub fn get_user(&self, username: Option<&'_ str>, id: Option<u32>) /* -> User */
    {
        if username.is_some() {
            self.get_user_by_username(username.unwrap());
        } else {
            self.get_user_by_id(id.unwrap())
        }
    }

    pub fn get_friends(&self) /* -> Vec<User> */ {}

    pub fn change_status(&self, status: &'_ str) /* -> u16 */ {}
}
