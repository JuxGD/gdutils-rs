use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use simple_datetime_rs::{DateTime};

pub const URL_AREDL: &str = "https://api.aredl.net/v2/api/aredl";
pub const URL_AREPL: &str = " https://api.aredl.net/v2/api/arepl";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AREDLLevelInfo {
    pub id: String
,   pub name: String
,   pub position: i32
,   pub publisher: HashMap<String, String>
,   pub points: i32
,   pub legacy: bool
,   pub level_id: i32
,   pub two_player: bool
,   pub tags: Option<Vec<String>>
,   pub description: Option<String>
,   pub song: Option<i32>
,   pub edel_enjoyment: Option<f32>
,   pub is_edel_pending: bool
,   pub gddl_tier: Option<f32>
,   pub nlw_tier: Option<String>
,   pub verifications: Option<Vec<AREDLVerification>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AREDLVerification {
    pub id: String,
    pub submitted_by: AREDLUser
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AREDLUser {
    pub id: String,
    pub username: String,
    pub global_name: String,
    pub discord_id: Option<String>,
    pub placeholder: Option<bool>,
    pub description: Option<String>,
    pub country: Option<i32>,
    pub ban_level: Option<i32>,
    pub discord_avatar: Option<String>,
    pub discord_banner: Option<String>,
    pub discord_accent_color: Option<i32>,
    pub created_at: Option<DateTime>,
    pub background_level: Option<i32>
}