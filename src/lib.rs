use serde::{Deserialize, Serialize};

pub const BASE_URL: &'static str = "https://itch.io/api";
pub const API_VERSION: u8 = 1;

pub struct Ichor {
    http: reqwest::Client,
    base_url: String,
    api_version: u8,
    api_key: String,
}

#[derive(Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MaybeEmptyList<T> {
    Empty {},
    List(Vec<T>),
}

fn default_false() -> bool {
    false
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    #[serde(default = "default_false")]
    pub gamer: bool,
    pub display_name: Option<String>,
    pub cover_url: Option<String>,
    pub url: String,
    #[serde(default = "default_false")]
    pub press_user: bool,
    #[serde(default = "default_false")]
    pub developer: bool,
    pub id: u32,
}

#[derive(Deserialize)]
pub struct CredentialsInfo {
    pub r#type: String,
    pub scopes: Option<MaybeEmptyList<String>>,
    pub expires_at: Option<String>,
}

#[derive(Deserialize)]
pub struct Me {
    pub user: User,
}

#[derive(Deserialize)]
pub struct MyGames {
    pub games: MaybeEmptyList<Game>,
}

#[derive(Deserialize)]
pub struct Game {
    pub purchases_count: Option<i64>,
    pub p_osx: bool,
    pub id: i64,
    pub published: Option<bool>,
    pub published_at: String,
    pub views_count: Option<i64>,
    pub url: String,
    pub can_be_bought: bool,
    pub p_android: bool,
    pub p_linux: bool,
    pub created_at: String,
    pub in_press_system: bool,
    pub has_demo: bool,
    pub user: User,
    pub title: String,
    pub downloads_count: Option<i64>,
    pub p_windows: bool,
    pub min_price: i64,
    pub classification: String,
    pub short_text: String,
    pub r#type: String,
    pub earnings: Option<MaybeEmptyList<Earning>>,
}

#[derive(Deserialize)]
pub struct Earning {
    pub currency: String,
    pub amount_formatted: String,
    pub amount: u32,
}

#[derive(Deserialize)]
pub struct DownloadKeys {
    pub download_key: DownloadKey,
}

#[derive(Deserialize)]
pub struct DownloadKey {
    pub id: u32,
    pub created_at: String,
    pub downloads: u32,
    pub key: String,
    pub game_id: u32,
    pub owner: Option<User>,
}

#[derive(Serialize)]
pub enum DownloadKeysType {
    #[serde(rename = "download_key")]
    DownloadKey,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "email")]
    Email,
}

#[derive(Deserialize)]
pub struct Purchases {
    pub purchases: MaybeEmptyList<Purchase>,
}

#[derive(Deserialize)]
pub struct Purchase {
    pub donation: bool,
    pub id: i64,
    pub email: String,
    pub created_at: String,
    pub source: String,
    pub currency: String,
    pub price: String,
    pub sale_rate: i64,
    pub game_id: i64,
}

#[derive(Serialize)]
pub enum PurchasesType {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "user_id")]
    UserId,
}

impl Ichor {
    pub fn new<S: Into<String>>(base_url: S, api_version: u8, api_key: S) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
            api_version,
            api_key: api_key.into(),
        }
    }

    fn get_full_base(&self) -> String {
        format!("{}/{}/{}", self.base_url, self.api_version, self.api_key)
    }

    pub async fn credentials_info(self) -> Result<CredentialsInfo, reqwest::Error> {
        let base = self.get_full_base();
        Ok(self
            .http
            .get(format!("{}/credentials/info", base))
            .send()
            .await?
            .error_for_status()?
            .json::<CredentialsInfo>()
            .await?)
    }

    pub async fn me(self) -> Result<Me, reqwest::Error> {
        let base = self.get_full_base();
        Ok(self
            .http
            .get(format!("{}/me", base))
            .send()
            .await?
            .error_for_status()?
            .json::<Me>()
            .await?)
    }

    pub async fn my_games(self) -> Result<MyGames, reqwest::Error> {
        let base = self.get_full_base();
        Ok(self
            .http
            .get(format!("{}/my-games", base))
            .send()
            .await?
            .error_for_status()?
            .json::<MyGames>()
            .await?) // TODO: figure out why games can be an empty map here
    }

    pub async fn game<S: Into<String>>(self, game_id: S) -> Result<Game, reqwest::Error> {
        #[derive(Deserialize)]
        struct GameResponse {
            game: Game,
        }

        let base = self.get_full_base();
        Ok(self
            .http
            .get(format!("{}/game/{}", base, game_id.into()))
            .send()
            .await?
            .error_for_status()?
            .json::<GameResponse>()
            .await?
            .game)
    }

    pub async fn download_keys<U: Into<u32>, S: Into<String>>(
        self,
        game_id: U,
        lookup_type: DownloadKeysType,
        lookup: S,
    ) -> Result<DownloadKeys, reqwest::Error> {
        let base = self.get_full_base();
        Ok(self
            .http
            .get(format!("{}/game/{}/download_keys", base, game_id.into()))
            .query(&[(lookup_type, lookup.into())])
            .send()
            .await?
            .error_for_status()?
            .json::<DownloadKeys>()
            .await?)
    }

    pub async fn purchases<U: Into<u32>, S: Into<String>>(
        self,
        game_id: U,
        lookup_type: PurchasesType,
        lookup: S,
    ) -> Result<Purchases, reqwest::Error> {
        let base = self.get_full_base();
        Ok(self
            .http
            .get(format!("{}/game/{}/purchases", base, game_id.into()))
            .query(&[(lookup_type, lookup.into())])
            .send()
            .await?
            .error_for_status()?
            .json::<Purchases>()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::Ichor;

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
                crate::BASE_URL,
                crate::API_VERSION,
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
        aw!(client.download_keys(1289068u32, crate::DownloadKeysType::UserId, "5119994"));
    }

    #[test]
    fn purchases() {
        let client = new_client!();
        aw!(client.purchases(1289068u32, crate::PurchasesType::UserId, "5119994"));
    }
}
