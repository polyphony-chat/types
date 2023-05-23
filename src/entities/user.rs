use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
#[cfg(feature = "sqlx")]
use sqlx::{FromRow, Type};

use crate::{
    errors::Error,
    utils::Snowflake, //util::{email::adjust_email, entities::user_setting::UserSettings},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub accent_color: Option<u8>,
    pub banner: Option<String>,
    pub theme_colors: Option<Vec<u8>>,
    pub pronouns: Option<String>,
    pub phone: Option<String>,
    pub desktop: bool,
    pub mobile: bool,
    pub premium: bool,
    pub premium_type: u8,
    pub bot: bool,
    pub bio: String,
    pub system: bool,
    pub nsfw_allowed: bool,
    pub mfa_enabled: bool,
    pub webauthn_enabled: bool,
    #[serde(skip)]
    pub totp_secret: Option<String>,
    #[serde(skip)]
    pub totp_last_ticket: Option<String>,
    pub created_at: DateTime<Utc>,
    pub premium_since: Option<DateTime<Utc>>,
    pub verified: bool,
    pub disabled: bool,
    pub deleted: bool,
    pub email: Option<String>,
    pub flags: String,
    pub public_flags: u16,
    pub purchased_flags: u16,
    pub premium_usage_flags: u16,
    pub rights: String,
    #[cfg(feature = "sqlx")]
    pub relationship_ids: sqlx::types::Json<Vec<String>>,
    #[cfg(not(feature = "sqlx"))]
    pub relationship_ids: Vec<String>,
    #[cfg(feature = "sqlx")]
    pub connected_account_ids: sqlx::types::Json<Vec<String>>,
    #[cfg(not(feature = "sqlx"))]
    pub connected_account_ids: Vec<String>,
    #[cfg(feature = "sqlx")]
    pub data: sqlx::types::Json<UserData>,
    #[cfg(not(feature = "sqlx"))]
    pub data: UserData,
    #[cfg(feature = "sqlx")]
    pub fingerprints: sqlx::types::Json<Vec<String>>,
    #[cfg(not(feature = "sqlx"))]
    pub fingerprints: Vec<String>,
    // pub settings: UserSettings,
    pub extended_settings: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "sqlx", derive(Type))]
pub struct UserData {
    pub valid_tokens_since: DateTime<Utc>,
    pub hash: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Snowflake::generate(),
            username: String::new(),
            discriminator: String::new(),
            avatar: None,
            accent_color: None,
            banner: None,
            theme_colors: None,
            pronouns: None,
            phone: None,
            desktop: false,
            mobile: false,
            premium: false,
            premium_type: 0,
            bot: false,
            bio: String::new(),
            system: false,
            nsfw_allowed: false,
            mfa_enabled: false,
            webauthn_enabled: false,
            totp_secret: None,
            totp_last_ticket: None,
            created_at: Utc::now(),
            premium_since: None,
            verified: false,
            disabled: false,
            deleted: false,
            email: None,
            flags: String::from("0"),
            public_flags: 0,
            purchased_flags: 0,
            premium_usage_flags: 0,
            rights: String::new(),
            relationship_ids: Default::default(),
            connected_account_ids: Default::default(),
            data: Default::default(),
            fingerprints: Default::default(),
            //settings: UserSettings::default(),
            extended_settings: Value::Object(Map::new()),
        }
    }
}

impl User {
    pub fn validate(&mut self) -> Result<(), Error> {
        /*if let Some(email) = self.email.as_ref() {
            self.email = Some(adjust_email(email)?);
        }

        if self.discriminator.len() < 4 {
            self.discriminator = format!("{:0>4}", self.discriminator);
        }
        Ok(())*/
        todo!()
    }

    pub async fn generate_discriminator(_username: &str) -> Result<String, Error> {
        todo!()
    }
}

impl User {
    pub fn to_public_user(self) -> PublicUser {
        PublicUser::from(self)
    }
}

#[cfg(feature = "sqlx")]
impl User {
    pub async fn get_by_id(
        conn: &mut sqlx::MySqlConnection,
        id: &Snowflake,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(conn)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn find_by_user_and_discrim(
        conn: &mut sqlx::MySqlConnection,
        user: &str,
        discrim: &str,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM users WHERE username = ? AND discriminator = ?")
            .bind(user)
            .bind(discrim)
            .fetch_optional(conn)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_user_by_email_or_phone(
        conn: &mut sqlx::MySqlConnection,
        email: &str,
        phone: &str,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM users WHERE email = ? OR phone = ? LIMIT 1")
            .bind(email)
            .bind(phone)
            .fetch_optional(conn)
            .await
            .map_err(Error::SQLX)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub accent_color: Option<u8>,
    pub banner: Option<String>,
    pub theme_colors: Option<Vec<u8>>,
    pub pronouns: Option<String>,
    pub bot: bool,
    pub bio: String,
    pub premium_type: u8,
    pub premium_since: Option<DateTime<Utc>>,
    pub public_flags: u16,
}

impl From<User> for PublicUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            discriminator: value.discriminator,
            avatar: value.avatar,
            accent_color: value.accent_color,
            banner: value.banner,
            theme_colors: value.theme_colors,
            pronouns: value.pronouns,
            bot: value.bot,
            bio: value.bio,
            premium_type: value.premium_type,
            premium_since: value.premium_since,
            public_flags: value.public_flags,
        }
    }
}

const CUSTOM_USER_FLAG_OFFSET: u64 = 1 << 32;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    #[cfg_attr(feature = "sqlx", derive(Type))]
    pub struct UserFlags: u64 {
        const DISCORD_EMPLOYEE = 1 << 0;
        const PARTNERED_SERVER_OWNER = 1 << 1;
        const HYPESQUAD_EVENTS = 1 << 2;
        const BUGHUNTER_LEVEL_1 =1 << 3;
        const MFA_SMS = 1 << 4;
        const PREMIUM_PROMO_DISMISSED = 1 << 5;
        const HOUSE_BRAVERY = 1 << 6;
        const HOUSE_BRILLIANCE = 1 << 7;
        const HOUSE_BALANCE = 1 << 8;
        const EARLY_SUPPORTER = 1 << 9;
        const TEAM_USER = 1 << 10;
        const TRUST_AND_SAFETY = 1 << 11;
        const SYSTEM = 1 << 12;
        const HAS_UNREAD_URGENT_MESSAGES = 1 << 13;
        const BUGHUNTER_LEVEL_2 = 1 << 14;
        const UNDERAGE_DELETED = 1 << 15;
        const VERIFIED_BOT = 1 << 16;
        const EARLY_VERIFIED_BOT_DEVELOPER = 1 << 17;
        const CERTIFIED_MODERATOR = 1 << 18;
        const BOT_HTTP_INTERACTIONS = 1 << 19;
    }
}
