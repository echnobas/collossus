#[derive(Deserialize, Debug, Clone)]
pub struct Group {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isBuildersClubOnly")]
    pub is_builders_club_only: bool,
    #[serde(rename = "memberCount")]
    pub member_count: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "publicEntryAllowed")]
    pub public_entry_allowed: bool,
    #[serde(rename = "shout")]
    pub shout: Option<Shout>,
    // #[serde(rename = "owner")]
    // pub owner: Option<()>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Shout {
    pub content: String,
    pub created: String,
    pub updated: String,
    pub author: Option<()>,
}

impl Group {
    pub fn new(id: i32, client: &crate::RbxClient) -> Result<Self, crate::errors::RbxError> {
        let uri: &str = &format!("https://groups.roblox.com/v1/groups/{}", id);
        let request = client
            .new_request(None, uri, crate::rbx_client::RequestMethod::Get)
            .send()?;
        match request.status() {
            http::StatusCode::OK => {}
            _ => panic!("unexpected status code"),
        }
        let group = request.json().unwrap();
        Ok(group)
    }
}
