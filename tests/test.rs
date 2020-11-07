#[cfg(feature = "sync")]
#[test]
fn it_works() {
    use collossus::{
        Group,
        RbxClient,
    };
    let c = RbxClient::new_without_cookie();
    let g = Group::new(1, &c).unwrap();
    assert_eq!(g.id, 1);
}
