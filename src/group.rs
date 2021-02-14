use reqwest::blocking::Client as HttpClient;
#[derive(Debug)]
pub struct Group {
    http_client: HttpClient,
    cookie: String,
    group_id: u32,
    group_name: String,
    group_description: String,
    // owner: Option<User>
}

impl Group {
    pub fn new(
        group_id: u32,
        group_name: String,
        group_description: String,
        http_client: HttpClient,
        cookie: String,
    ) -> Self {
        // TODO: Add parameters Option<String> for owner username and Option<u32> for owner userid then create user object from that info
        Self {
            group_id,
            cookie,
            group_name,
            group_description,
            http_client,
        }
    }

    pub fn pay(&self, id: u32, amount: u64) /* -> u16 */ {}

    pub fn exile(&self, id: u32) /* -> u16 */ {}

    pub fn set_rank(&self, id: u32, rank_id: u32) /* -> u16 */ {}

    pub fn promote(&self, id: u32) /* -> (Role, Role) */ {}

    pub fn demote(&self, id: u32) /* -> (Role, Role) */ {}

    pub fn change_rank(&self, id: u32, change: i32) /* -> (Role, Role) */ {}

    pub fn set_rank_by_id(&self, id: u32, role_id: u32) /* -> u16 */ {}

    pub fn get_group_roles(&self) /* Vec<Role> */ {}

    pub fn get_role_in_group(&self, id: u32) /* -> Role */ {}

    pub fn post_shout(&self, message: &'_ str) /* -> Role */ {}

    pub fn get_funds(&self) /* -> u16 */ {}

    pub fn get_join_requests(&self) /* -> Vec<JoinRequest> */ {}

    pub fn get_audit_logs(&self, action: Option<&'_ str>) {}

    pub fn get_members(&self) /* -> Vec<User> */ {}

    pub fn leave(&self) /* -> u16 */ {}
}
