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

const COOKIE: &'static str = "";
fn main() {
    let client = client::Client::new(COOKIE);
    let group = client.get_group(1);
    println!("{:?}", group);
}
