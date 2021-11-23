extern crate ichor;

use ichor::Ichor;

#[ctor::ctor]
fn init() {
    dotenv::dotenv().ok();
}

macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e).unwrap()
    };
}

macro_rules! new_client {
    () => {
        Ichor::new(
            ichor::BASE_URL,
            ichor::API_VERSION,
            &std::env::var("ITCH_API_KEY").unwrap(),
        )
    };
}

#[test]
fn credentials_info() {
    let client = new_client!();
    assert_eq!(aw!(client.credentials_info()).r#type, "key");
}

#[test]
fn me() {
    let client = new_client!();
    aw!(client.me());
}

#[test]
fn my_games() {
    let client = new_client!();
    aw!(client.my_games());
}

#[test]
fn game() {
    let client = new_client!();
    aw!(client.game("1289068"));
}

#[test]
fn download_keys() {
    let client = new_client!();
    aw!(client.download_keys(1289068u32, ichor::DownloadKeysType::UserId, "5119994"));
}

#[test]
fn purchases() {
    let client = new_client!();
    aw!(client.purchases(1289068u32, ichor::PurchasesType::UserId, "5119994"));
}
