#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

pub mod shared;

#[test]
fn test_fromclient_json() {
    use crate::shared::client_response::ClientResponse;
    use std::sync::Arc;

    let from_client = ClientResponse::Post {
        group_name: Arc::new("Dogs".to_string()),
        message: Arc::new("Samoyeds rock!".to_string()),
    };

    let json = serde_json::to_string(&from_client).unwrap();
    assert_eq!(
        json,
        r#"{"Post":{"group_name":"Dogs","message":"Samoyeds rock!"}}"#
    );

    assert_eq!(
        serde_json::from_str::<ClientResponse>(&json).unwrap(),
        from_client
    );
}
