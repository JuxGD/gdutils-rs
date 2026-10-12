use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fmt;

pub const URL_AREDL: &str = "https://api.aredl.net/v2/api/aredl";
pub const URL_AREPL: &str = " https://api.aredl.net/v2/api/arepl";
pub const URL_USERS: &str = "https://api.aredl.net/v2/api/users";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum AREDLStatus {
    Pending,
    MainList,
    Legacy,
    Removed,

    #[default]
    Unknown
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AREDLLevelInfo {
    pub id: String
,   pub name: String
,   pub position: i32
,   pub publisher_id: Option<String>
,   pub points: i32
,   pub status: AREDLStatus
,   pub requires_raw_footage: bool
,   pub level_id: i32
,   pub two_player: bool
,   pub tags: Option<Vec<String>>
,   pub description: Option<String>
,   pub song: Option<i32>
,   pub edel_enjoyment: Option<f32>
,   pub is_edel_pending: bool
,   pub gddl_tier: Option<f32>
,   pub nlw_tier: Option<String>
,   pub publisher: Option<HashMap<String, String>>
,   pub verifications: Option<Vec<AREDLVerification>>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AREDLVerification {
    pub id: String,
    pub submitted_by: AREDLUser
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AREDLUser {
    pub id: String,
    pub username: String,
    pub global_name: String,
    pub discord_id: Option<String>,
    pub placeholder: bool,
    pub description: Option<String>,
    pub country: Option<i32>,
    pub ban_level: i32,
    pub discord_avatar: Option<String>,
    pub discord_avatar_decoration: Option<String>,
    pub created_at: String,
    pub background_level: Option<i32>,
    pub featured_badge_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AREDLUserResolved {
    pub id: String,
    pub username: String,
    pub global_name: String,
    pub discord_id: Option<String>,
    pub placeholder: bool,
    pub description: Option<String>,
    pub country: Option<i32>,
    pub ban_level: i32,
    pub discord_avatar: Option<String>,
    pub discord_avatar_decoration: Option<String>,
    pub created_at: String,
    pub background_level: Option<i32>,
    pub featured_badge_code: Option<String>,
    pub clan: Option<AREDLClan>,
    pub roles: Vec<AREDLRoles>,
    pub scopes: Vec<String>,
    pub badges: Vec<AREDLBadge>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AREDLClan {
    id: String,
    global_name: String,
    description: Option<String>,
    created_at: String,
    updated_at: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AREDLRoles {
    pub id: i32,
    pub privilege_level: i32,
    pub role_desc: String,
    pub hide: bool,
    pub inherits_from_role_id: Option<i32>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AREDLBadge {
    pub badge_code: String,
    pub description: Option<String>,
    pub unlocked_at: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AREDLUserSearchResults {
    pub data: Vec<AREDLUser>,
    pub page: u16,
    pub per_page: u16,
    pub pages: u16,
    pub count: u32,
}

#[derive(Debug)]
pub enum GDUtilsAREDLError {
    NumberTooHighError()
}

impl fmt::Display for GDUtilsAREDLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GDUtilsAREDLError::NumberTooHighError() => write!(f, "The requested AREDL position does not exist")
        }
    }
}

impl Error for GDUtilsAREDLError {}