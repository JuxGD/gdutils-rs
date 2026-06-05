use derive_builder::Builder;
use serde::{Serialize, Deserialize};
use eyre::Error;

pub const URL: &str = "http://www.boomlings.com"; // because of Rust limitations, both URL and URL_DATABASE need to be changed
pub const URL_DATABASE: &str = "https://www.boomlings.com/database";
pub const SECRET_COMMON: &str = "Wmfd2893gb7";

/// Represents level information like name, id, song used, author, rating, etc.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct Level {
    pub level_id: i32
,   pub level_name: String
,   pub description: String
,   pub level_string: Option<String>
,   pub version: u16
,   pub player_id: i32
,   pub difficulty_denominator: u8
,   pub difficulty_numerator: u8
,   pub downloads: u32
,   pub set_completes: u32
,   pub official_song: u8
,   pub game_version: u8
,   pub likes: i32
,   pub length: u8
,   pub dislikes: i32
,   pub demon: bool
,   pub stars: u8
,   pub feature_score: i32
,   pub auto: bool
,   pub record_string: String
,   pub password: Option<String>
,   pub upload_date: Option<String>
,   pub update_date: Option<String>
,   pub copied_id: i32
,   pub two_player: bool
,   pub custom_song_id: i32
,   pub extra_string: String
,   pub coins: u8
,   pub verified_coins: bool
,   pub stars_requested: u8
,   pub low_detail_mode: Option<bool>
,   pub daily_number: Option<i32>
,   pub epic: u8
,   pub demon_difficulty: u8
,   pub is_gauntlet: bool
,   pub objects: u16
,   pub editor_time: u32
,   pub editor_time_copies: u32
,   pub settings_string: Option<String>
,   pub song_ids: Option<Vec<i32>>
,   pub sfx_ids: Option<Vec<i32>>
,   pub song_size: Option<i32>
,   pub verification_time: Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct CommonLevelInfo {
    pub level_name: String
,   pub level_id: i32
,   pub author: String
,   pub difficulty: String
,   pub quality: String
,   pub stars_moons: u8
,   pub coins: u8
,   pub verified_coins: bool
,   pub likes: i32
,   pub downloads: u32
,   pub song_id: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TimelyType {
    Daily,
    Weekly,
    Event
}

/// Represents timely level (daily, weekly, etc) specific data,
/// AKA timely index (eg daily level #) and time left in seconds.
/// For event levels, `time_left` should be ignored.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct TimelyLevel {
    pub level: Level
,   pub timely_index: i32
,   pub time_left: i32
}

/// USE `new()`! This is necessary for the compiler and analyzer to catch stuff
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GDQuery {
    pub query: String,
    pub query_type: GDQueryType
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum GDQueryType {
    ID,
    Name
}

impl GDQuery {
    pub fn new(query: &str) -> Self {
        Self {
            query: String::from(query),
            query_type: if check_if_numeric(query) {
                GDQueryType::ID
            } else {
                GDQueryType::Name
            }
        }
    }
}

/// Represents user information like username, ID, demons beaten, etc.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct User {
    pub username: String
,   pub user_id: i32
,   pub stars: u32
,   pub demon_count: u32
,   pub ranking: Option<u32>
,   pub account_highlight: u32
,   pub creator_points: u16
,   pub icon_id: Option<i32>
,   pub color: i32
,   pub color_2: i32
,   pub ship_id: Option<i32>
,   pub secret_coins: u8
,   pub icon_type: Option<i32>
,   pub special: Option<u8>
,   pub account_id: i32
,   pub user_coins: u32
,   pub message_state: Option<u8>
,   pub friends_state: Option<u8>
,   pub youtube: Option<String>
,   pub acc_icon: Option<i32>
,   pub acc_ship: Option<i32>
,   pub acc_ball: Option<i32>
,   pub acc_bird: Option<i32>
,   pub acc_dart: Option<i32>
,   pub acc_robot: Option<i32>
,   pub acc_streak: Option<i32>
,   pub acc_glow: Option<bool>
,   pub is_registered: Option<bool>
,   pub global_rank: Option<u32>
,   pub friend_state: Option<u8>
,   pub friend_request_id: Option<u32>
,   pub friend_request_message: Option<String>
,   pub friend_request_age: Option<u32>
,   pub messages: Option<u32>
,   pub friend_requests: Option<u32>
,   pub new_friends: Option<u32>
,   pub new_friend_request: Option<bool>
,   pub age: Option<String>
,   pub acc_spider: Option<i32>
,   pub twitter: Option<String>
,   pub twitch: Option<String>
,   pub diamonds: u32
,   pub acc_explosion: Option<i32>
,   pub moderator: Option<u8>
,   pub comment_history_state: Option<u8>
,   pub color_3: i32
,   pub moons: u32
,   pub acc_swing: Option<i32>
,   pub acc_jetpack: Option<i32>
,   pub demons: Option<Vec<String>>
,   pub classic_levels: Option<Vec<String>>
,   pub platformer_levels: Option<Vec<String>>
,   pub discord: Option<String>
,   pub instagram: Option<String>
,   pub tiktok: Option<String>
,   pub custom: Option<String>
}

pub fn check_if_numeric(string: &str) -> bool {
    for c in string.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

pub async fn post_request(url: String, form: Vec<(&str, &str)>) -> Result<String, Error> {
    Ok(reqwest::Client::new().post(url)
            .form(&form)
            .header("User_Agent", "")
            .send()
            .await?
            .text()
            .await?
    )
}

pub async fn post_request_no_form(url: String) -> Result<String, Error> {
    Ok(reqwest::Client::new().post(url)
            .header("User_Agent", "")
            .send()
            .await?
            .text()
            .await?
    )
}