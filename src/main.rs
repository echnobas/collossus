#![allow(dead_code, unused_variables)]
// #![forbid(
//     clippy::all,
//     clippy::pedantic,
//     clippy::nursery,
//     clippy::cargo,
// )]

mod client;
mod errors;
mod group;
mod request;
mod user;

const COOKIE: &'static str = "";
fn main() {
    let client = client::Client::new(COOKIE);
    let group = client.get_group(1174414);
    println!("{:?}", group);
    let user = client.get_user(Some("ninjacraft0304"), None);
    println!("{:?}", user.unwrap().get_status());
}
