use derive_builder::Builder;
use reqwest::header::HeaderMap;
use serde::{Serialize, Deserialize};
use core::fmt;
use std::collections::HashMap;
use eyre::Report;
pub use colonparse::hashmap_from;

use crate::gd::levels::{calculate_difficulty, calculate_quality};

/// URL for the Geometry Dash servers.
pub const URL: &str = "http://www.boomlings.com"; // because of Rust limitations, both `URL` and `URL_DATABASE` need to be changed

/// URL for the Geometry Dash database, which is what most all API endpoints use.
pub const URL_DATABASE: &str = "https://www.boomlings.com/database";

/// This is a common string required in most request forms as the value for the `secret` parameter.
pub const SECRET_COMMON: &str = "Wmfd2893gb7";

/// Represents the level object returned by some API endpoints. 
/// 
/// See <https://boomlings.dev/resources/server/level>
#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default, PartialEq)]
pub struct Level {
    /// The ID of the level. Level object key 1.
    pub level_id: i32,

    /// The name of the level. Level object key 2.
    pub level_name: String,

    /// The description of the level, encoded in Base64. Level object key 3.
    pub description: String,

    /// The level string. Level object key 4.
    /// 
    /// This is only `Some(string)` when returned by the `downloadGJLevel22.php` endpoint.
    /// 
    /// > [!CAUTION] `unwrap()`ping this field will cause a panic if the level object didn't come from `downloadGJLevel22.php`,
    /// > since the value of the field is then `None`.
    pub level_string: Option<String>,

    /// Version of the level. Level object key 5.
    /// 
    /// Not to be confused with `game_version`.
    pub version: u16,

    /// Player ID of the level publisher. Level object key 6.
    pub player_id: i32,

    /// Difficulty denominator of the level. Level object key 8.
    /// 
    /// Divides `difficulty_numerator` to determine the level difficulty. 0 if N/A, 10 if a difficulty is assigned.
    /// 
    /// Both the [`calculate_difficulty()`] function and the [`Level`] method of the same name use this value for difficulty calculation.
    pub difficulty_denominator: u8,

    /// Difficulty numerator of the level. Level object key 9.
    /// 
    /// Divided by `difficulty_denominator` (if it's 10) to determine the level difficulty.
    /// - 0 = Unrated
    /// - 10 = Easy
    /// - 20 = Normal
    /// - 30 = Hard
    /// - 40 = Harder
    /// - 50 = Insane
    /// 
    /// If the `demon` field is `true`, this can be used to determine the demon difficulty.
    /// - 10 = Easy demon
    /// - 20 = Medium demon
    /// - 30 = Hard demon
    /// - 40 = Insane demon
    /// - 50 = Extreme demon
    /// 
    /// Both the [`calculate_difficulty()`] function and the [`Level`] method of the same name use this value for difficulty calculation.
    pub difficulty_numerator: i8,

    /// Amount of downloads of a level. Level object key 10.
    pub downloads: u32,

    /// Amount of peoeple who completed the level. Level object key 11.
    /// 
    /// Since game version 2.1, key 11 is not returned.
    /// 
    /// > [!CAUTION] `unwrap()`ping this field will cause a panic if the level object didn't come from `downloadGJLevel22.php`,
    /// > or if it came from a version later than 2.1. This is because the value of the field becomes `None`.
    pub completes: Option<u32>,

    /// ID of the included game song used in the level. Level object key 12.
    pub official_song: u8,

    /// GD version the level was uploaded in. Level object key 13.
    pub game_version: u8,

    /// Amount of likes the level has. Level object key 14.
    /// 
    /// In the servers, this value equals the amount of likes it has, minus the dislikes.
    pub likes: i32,

    /// How long the level is. Level object key 15.
    /// 
    /// Level object key 15 will have a value from 0 to 5. `gdutils` maps the value to a variant of the [`Length`] enum.
    /// 
    /// 0 => Tiny,
    /// 
    /// 1 => Short,
    /// 
    /// 2 => Medium,
    /// 
    /// 3 => Long,
    /// 
    /// 4 => XL,
    /// 
    /// 5 => Platformer,
    pub length: Length,

    /// How many dislikes the level has. Level object key 16.
    /// 
    /// Its value is always 0.
    pub dislikes: i32,

    /// Whether the level is a demon or not. Level object key 25.
    /// 
    /// Both the [`calculate_difficulty()`] function and the [`Level`] method of the same name use this value for difficulty calculation.
    pub demon: bool,

    /// Amount of stars given by beating the level. Level object key 18.
    pub stars: u8,

    /// Feature score of the level. Level object key 19.
    /// 
    /// Despite the name, it only represents the place where the level appears in the "Featured" section ingame.
    /// The higher this number, the further up the level is in the "Featured" levels list.
    pub feature_score: i32,

    /// Amount of votes for a star rating in the level. Level object key 20. 
    /// 
    /// The keys for the [`Level`] fields `star_ratings`, `star_ratings_sum`, `min_star_ratings`, `max_star_ratings`, `demon_votes`,
    /// `record_string`, `rate_stars`, `rate_feature` and `rate_user` were only returned by the official GD servers when "extras" was enabled.
    /// Since 2022, they are only returned if RobTop is the one requesting the info from the servers.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping these will probably result in a panic.
    pub star_ratings: Option<u32>,

    /// Sum of the stars in the star rating votes in the level. Level object key 21.
    /// 
    /// The keys for the [`Level`] fields `star_ratings`, `star_ratings_sum`, `min_star_ratings`, `max_star_ratings`, `demon_votes`,
    /// `record_string`, `rate_stars`, `rate_feature` and `rate_user` were only returned by the official GD servers when "extras" was enabled.
    /// Since 2022, they are only returned if RobTop is the one requesting the info from the servers.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping these will probably result in a panic.
    pub star_ratings_sum: Option<u32>,

    /// Amount of users who voted for a 1 star difficulty rating (Auto) in the level. Level object key 22.
    /// 
    /// The keys for the `Level` fields `star_ratings`, `star_ratings_sum`, `min_star_ratings`, `max_star_ratings`, `demon_votes`,
    /// `record_string`, `rate_stars`, `rate_feature` and `rate_user` were only returned by the official GD servers when "extras" was enabled.
    /// Since 2022, they are only returned if RobTop is the one requesting the info from the servers.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping these will probably result in a panic.
    pub min_star_ratings: Option<u32>,

    /// Amount of users who voted for a 10 star difficulty rating (Demon) in the level. Level object key 23.
    /// 
    /// The keys for the `Level` fields `star_ratings`, `star_ratings_sum`, `min_star_ratings`, `max_star_ratings`, `demon_votes`,
    /// `record_string`, `rate_stars`, `rate_feature` and `rate_user` were only returned by the official GD servers when "extras" was enabled.
    /// Since 2022, they are only returned if RobTop is the one requesting the info from the servers.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping these will probably result in a panic.
    pub max_star_ratings: Option<u32>,

    /// Amount of people who voted for a demon difficulty (before GD 2.0) or a 10 star difficulty rating (from 2.0 onward) in the level. Level object key 24.
    ///
    /// The keys for the `Level` fields `star_ratings`, `star_ratings_sum`, `min_star_ratings`, `max_star_ratings`, `demon_votes`,
    /// `record_string`, `rate_stars`, `rate_feature` and `rate_user` were only returned by the official GD servers when "extras" was enabled.
    /// Since 2022, they are only returned if RobTop is the one requesting the info from the servers.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping these will probably result in a panic.
    pub demon_votes: Option<i32>,

    /// Whether a level is auto or not. Level object key 25.
    /// 
    /// Both the `calculate_difficulty()` function and the `Level` method of the same name use this value for difficulty calculation.
    pub auto: bool,

    /// Replay of the level, captured during verification. Level object key 26.
    /// 
    /// The keys for the `Level` fields `star_ratings`, `star_ratings_sum`, `min_star_ratings`, `max_star_ratings`, `demon_votes`,
    /// `record_string`, `rate_stars`, `rate_feature` and `rate_user` were only returned by the official GD servers when "extras" was enabled.
    /// Since 2022, they are only returned if RobTop is the one requesting the info from the servers.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping these will probably result in a panic.
    pub record_string: Option<String>,

    /// The password for copying the level, encrypted with XOR then encoded with Base64. Level object key 27.
    /// 
    /// To decrypt the password, the string must be DeCoded with Base64, and its result decrypted with XOR with the key "26364".
    /// 
    /// Levels without passwords have the value `"Aw=="`. Doing the decryption/DeCoding process, this results in "1".
    /// 
    /// The convenience function `decode_password()`, and the `Level` method of the same name, automate the DeCoding/decryption process.
    /// 
    /// > is that a geometry dash reference
    /// 
    /// > [!CAUTION] `unwrap()`ping this field will cause a panic if the level object didn't come from `downloadGJLevel22.php`,
    /// > since the value of the field is then `None`.
    pub password: Option<String>,

    /// The date the level was uploaded. Level object key 28.
    /// 
    /// > [!CAUTION] `unwrap()`ping this field will cause a panic if the level object didn't come from `downloadGJLevel22.php`,
    /// > since the value of the field is then `None`.
    pub upload_date: Option<String>,

    /// The date the level was uploaded. Level object key 29.
    /// 
    /// > [!CAUTION] `unwrap()`ping this field will cause a panic if the level object didn't come from `downloadGJLevel22.php`,
    /// > since the value of the field is then `None`.
    pub update_date: Option<String>,

    /// The ID of the level this level is a copy of. Level object key 30.
    /// 
    /// This value will be `0` for levels that don't copy another level.
    pub copied_id: i32,

    /// Whether the level is two-player or not. Level object key 31.
    pub two_player: bool,

    /// Star rating from the last GD moderator to send the level to RobTop. Level object key 32.
    /// 
    /// The keys for the `Level` fields `star_ratings`, `star_ratings_sum`, `min_star_ratings`, `max_star_ratings`, `demon_votes`,
    /// `record_string`, `rate_stars`, `rate_feature` and `rate_user` were only returned by the official GD servers when "extras" was enabled.
    /// Since 2022, they are only returned if RobTop is the one requesting the info from the servers.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping these will probably result in a panic.
    pub rate_stars: Option<u8>,

    /// Whether the last GD moderator to send the level thought it was feature-worthy or not. Level object key 33.
    /// 
    /// The keys for the `Level` fields `star_ratings`, `star_ratings_sum`, `min_star_ratings`, `max_star_ratings`, `demon_votes`,
    /// `record_string`, `rate_stars`, `rate_feature` and `rate_user` were only returned by the official GD servers when "extras" was enabled.
    /// Since 2022, they are only returned if RobTop is the one requesting the info from the servers.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping these will probably result in a panic.
    pub rate_feature: Option<bool>,

    /// Account ID of the last GD moderator was who sent the level. Level object key 34.
    /// 
    /// In the past, level object key 34 may have referred to a "secret" (string used to verify) used when sending a level.
    /// 
    /// The keys for the `Level` fields `star_ratings`, `star_ratings_sum`, `min_star_ratings`, `max_star_ratings`, `demon_votes`,
    /// `record_string`, `rate_stars`, `rate_feature` and `rate_user` were only returned by the official GD servers when "extras" was enabled.
    /// Since 2022, they are only returned if RobTop is the one requesting the info from the servers.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping these will probably result in a panic.
    pub rate_user: Option<String>,

    /// The song ID of the main custom song used in the level. Level object key 35.
    /// 
    /// If the level uses an included Geometry Dash song, the value will be `0`.
    pub custom_song_id: i32,

    /// The `extraString` passed when uploading the level. Level object key 36.
    /// 
    /// > [!CAUTION] `unwrap()`ping this field will cause a panic if the level object didn't come from `downloadGJLevel22.php`,
    /// > since the value of the field is then `None`.
    pub extra_string: Option<String>,

    /// Amount of coins the level has. Level object key 37.
    pub coins: u8,

    /// Whether the coins of the level are verified or not. Level object key 38.
    pub verified_coins: bool,

    /// The amount of stars the level author requested for a rating. Level object key 39.
    pub stars_requested: u8,

    /// Whether the level has low detail mode available or not. Level object key 40.
    /// 
    /// > [!CAUTION] `unwrap()`ping this field will cause a panic if the level object didn't come from `downloadGJLevel22.php`,
    /// > since the value of the field is then `None`.
    pub low_detail_mode: Option<bool>,

    /// The daily, weekly or event level number of this level. Level object key 41.
    /// 
    /// Level object key 41 is only returned by `downloadGJLevel22.php` AND if the level has been a daily, weekly or event level.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping this field will cause a panic if the level object didn't come from `downloadGJLevel22.php`
    /// > or if the level hasn't been a daily, weekly or event level, since the value of the field is then `None`.
    pub daily_number: Option<i32>,
    pub epic: Epic,
    pub demon_difficulty: DemonDifficulty,
    pub is_gauntlet: bool,
    pub objects: u16,
    pub editor_time: u32,
    pub editor_time_copies: u32,
    pub settings_string: Option<String>,
    pub song_ids: Option<Vec<i32>>,
    pub sfx_ids: Option<Vec<i32>>,
    pub song_size: Option<f32>,
    pub verification_time: Option<i32>,
    pub exact_upload_time: Option<i32>,
    pub exact_update_time: Option<i32>
}

impl Level {
    /// Calculates the difficulty of a `Level`, returning a variant of the `Difficulty` enum. It makes use of the independent function of the same name.
    pub fn calculate_difficulty(&self) -> Result<Difficulty, Report> {
        calculate_difficulty(self.difficulty_numerator, self.difficulty_denominator, self.auto, self.demon)
    }

    /// Calculates the quality of a `Level`, returning a variant of the `Quality` enum. It makes use of the independent function of the same name.
    pub fn calculate_quality(&self) -> Quality {
        calculate_quality(self.feature_score, self.epic)
    }

    // TODO: fix decode_password so the function commented out below actually works

    // /// DeCodes and decrypts the value of the `password` field.
    // /// 
    // /// ## Errors
    // /// 
    // /// This will error out if the `Level` it's executed on doesn't have a password.
    // pub fn decode_password(&self) -> Result<String, Report> {
    //     if self.password.is_some() {
    //         decode_password(&self.password.unwrap())
    //     } else {
    //         Err(GDUtilsError::GenericError(String::from("This `Level`'s `password` is `None`!")).into())
    //     }
    // }
}

/// Represents possible timely types.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum TimelyType {
    #[default]
    Daily,
    Weekly,
    Event
}

/// Represents timely level (daily, weekly, etc) specific data,
/// AKA timely index (eg daily level #) and time left in seconds.
/// For event levels, `time_left` should be ignored.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Builder, Default, PartialEq, Eq)]
pub struct TimelyInfo {
    pub timely_index: i32
,   pub time_left: i32
}

/// Convenience struct that represents a timely level's level object and info
pub struct TimelyLevel {
    pub level: Level,
    pub info: TimelyInfo
}

/// Convenience enum that represents difficulty. Returned by the two `calculate_difficulty()` functions.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum Difficulty {
    #[default] NA, Unrated, Easy, Normal, Hard, Harder, Insane, EasyDemon, MediumDemon, HardDemon, InsaneDemon, ExtremeDemon, Auto
}

/// Represents demon difficulty. Type of the `demon_difficulty` of a `Level`
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum DemonDifficulty {
    EasyDemon, MediumDemon, #[default] HardDemon, InsaneDemon, ExtremeDemon
}

impl DemonDifficulty {
    
    /// Maps the possible values of level object key 43 to the `DemonDifficulty` variants.
    /// 
    /// # Errors
    /// 
    /// Errors out if the value doesn't map to anything.
    #[must_use]
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            0 => Self::HardDemon,
            3 => Self::EasyDemon,
            4 => Self::MediumDemon,
            5 => Self::InsaneDemon,
            6 => Self::ExtremeDemon,
            _ => return Err(GDUtilsError::UnknownDifficultyError.into())
        })
    }
}

/// Represents level length. Type of the `length` of a `Level`
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum Length {
    Tiny, Short, #[default] Medium, Long, XL, Plat
}

impl Length {

    /// Maps the possible values of level object key 15 to the `Length` variants.
    /// 
    /// # Errors
    /// 
    /// Errors out if the value doesn't map to anything.
    #[must_use]
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            0 => Self::Tiny,
            1 => Self::Short,
            2 => Self::Medium,
            3 => Self::Long,
            4 => Self::XL,
            5 => Self::Plat,
            _ => return Err(GDUtilsError::UnknownLevelLengthError.into())
        })
    }
}

/// Represents epic rating. Type of `epic` of a `Level`
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum Epic {
    #[default] None, Epic, Legendary, Mythic
}

impl Epic {

    /// Maps the possible values of level object key 42 to the `Epic` variants.
    /// 
    /// # Errors
    /// 
    /// Errors out if the value doesn't map to anything.
    #[must_use]
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            0 => Self::None,
            1 => Self::Epic,
            2 => Self::Legendary,
            3 => Self::Mythic,
            _ => return Err(GDUtilsError::UnknownEpicRatingError.into())
        })
    }
}

/// Convenience enum that represents difficulty. Returned by the two `calculate_quality()` functions.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum Quality {
    #[default] Normal, Featured, Epic, Legendary, Mythic
}

/// This struct is used to build a search query for the GD servers.
/// 
/// `GDQuery::new()` can be used to quickly create a `GDQuery`.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct GDQuery {
    pub query: String,
    pub query_type: GDQueryType
}

/// Represents query types (ID, name). Type of `query_type` in `GDQuery`.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum GDQueryType {
    ID,

    #[default]
    Name
}

impl GDQuery {

    /// Creates a `GDQuery` for you based on the string given to it, figuring out `query_type` by itself, returns said `GDQuery`.
    /// 
    /// If this doesn't have the desired behavior, creating a `GDQuery` manually is simple enough.
    /// 
    /// There's also `GDQuery::name` and `GDQuery::id`.
    #[must_use]
    pub fn new(query: &str) -> Self {
        Self {
            query: String::from(query),
            query_type: if check_if_numeric(query) || ["-1", "-2", "-3"].contains(&query) {
                GDQueryType::ID
            } else {
                GDQueryType::Name
            }
        }
    }

    /// Creates a `GDQuery` of `query_type` `GDQueryType::Name` for you based on the string given to it, returns said `GDQuery`.
    #[must_use]
    pub fn name(query: &str) -> Self {
        Self {
            query: String::from(query),
            query_type: GDQueryType::Name
        }
    }

    /// Creates a `GDQuery` of `query_type` `GDQueryType::ID` for you based on the string given to it, returns said `GDQuery`.
    #[must_use]
    pub fn id(query: &i32) -> Self {
        Self {
            query: query.to_string(),
            query_type: GDQueryType::ID
        }
    }
}

/// Represents user information like username, ID, demons beaten, etc.
/// 
/// See <https://boomlings.dev/resources/server/user>
#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default, PartialEq, Eq)]
pub struct User {
    pub username: String,
    pub user_id: i32,
    pub stars: u32,
    pub demon_count: u32,
    pub ranking: Option<u32>,
    pub account_highlight: Option<String>,
    pub creator_points: u16,
    pub icon_id: Option<i32>,
    pub color: i32,
    pub color_2: i32,
    pub ship_id: Option<i32>,
    pub secret_coins: u8,
    pub icon_type: Option<Gamemode>,
    pub special: Option<bool>,
    pub account_id: i32,
    pub user_coins: u32,
    pub message_state: Option<MessageState>,
    pub friends_state: Option<FriendsState>,
    pub youtube: Option<String>,
    pub acc_icon: Option<i32>,
    pub acc_ship: Option<i32>,
    pub acc_ball: Option<i32>,
    pub acc_bird: Option<i32>,
    pub acc_dart: Option<i32>,
    pub acc_robot: Option<i32>,
    pub acc_streak: Option<i32>,
    pub acc_glow: Option<bool>,
    pub is_registered: Option<bool>,
    pub global_rank: Option<u32>,
    pub friend_state: Option<FriendState>,
    pub friend_request_id: Option<u32>,
    pub friend_request_message: Option<String>,
    pub friend_request_age: Option<u32>,
    pub messages: Option<u32>,
    pub friend_requests: Option<u32>,
    pub new_friends: Option<u32>,
    pub new_friend_request: Option<bool>,
    pub age: Option<String>,
    pub acc_spider: Option<i32>,
    pub twitter: Option<String>,
    pub twitch: Option<String>,
    pub diamonds: u32,
    pub acc_explosion: Option<i32>,

    /// The staff level of the user. User object key 49.
    /// 
    /// > [!CAUTION] Because of the above, `unwrap()`ping this field will cause a panic if the level object didn't come from `getGJUserInfo20.php`
    /// > or if the level hasn't been a daily, weekly or event level, since the value of the field is then `None`.
    pub moderator: Option<ModeratorType>,
    pub comment_history_state: Option<MessageState>, // they're the same according to boomlings.dev
    pub color_3: i32,
    pub moons: u32,
    pub acc_swing: Option<i32>,
    pub acc_jetpack: Option<i32>,
    pub demons: Option<Vec<u16>>,
    pub classic_levels: Option<Vec<u16>>,
    pub platformer_levels: Option<Vec<u16>>,
    pub discord: Option<String>,
    pub instagram: Option<String>,
    pub tiktok: Option<String>,
    pub custom: Option<String>,
    pub group_state: Option<GroupState>,
    pub g_o: Option<i128>,
    pub friends_following_since: Option<i32>,
    pub followers: Option<i32>,
    pub following: Option<i32>,
    pub is_following: Option<bool>
}

/// Represents gamemode. Type of `icon_type` of a `User` (wrapped in `Option`).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum Gamemode {
    #[default] Cube, Ship, Ball, UFO, Wave, Robot, Spider, Swing
}

impl Gamemode {

    /// Maps the possible values of user object key 14 to the `Gamemode` variants.
    /// 
    /// # Errors
    /// 
    /// Errors out if the value doesn't map to anything.
    #[must_use]
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            0 => Self::Cube,
            1 => Self::Ship,
            2 => Self::Ball,
            3 => Self::UFO,
            4 => Self::Wave,
            5 => Self::Robot,
            6 => Self::Spider,
            7 => Self::Swing,
            _ => return Err(GDUtilsError::UnknownGamemodeError.into())
        })
    }
}

/// Represents message state. Type of `message_state` of a `User` (wrapped in `Option`).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum MessageState {
    #[default] All, FriendsOnly, None
}

impl MessageState {
    
    /// Maps the possible values of user object key 18 to the `MessageState` variants.
    /// 
    /// # Errors
    /// 
    /// Errors out if the value doesn't map to anything.
    #[must_use]
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            0 => Self::All,
            1 => Self::FriendsOnly,
            2 => Self::None,
            _ => return Err(GDUtilsError::UnknownMessageStateError.into())
        })
    }
}

/// Represents friends state. Type of `friends_state` of a `User` (wrapped in `Option`).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum FriendsState {
    #[default] All, None, AllWithAttachments
}

impl FriendsState {

    /// Maps the possible values of user object key 19 to the `FriendsState` variants.
    /// 
    /// # Errors
    /// 
    /// Errors out if the value doesn't map to anything.
    #[must_use]
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            0 => Self::All,
            1 => Self::None,
            2 => Self::AllWithAttachments,
            _ => return Err(GDUtilsError::UnknownFriendsStateError.into())
        })
    }
}

/// Represents friend state. Type of `friend_state` of a `User` (wrapped in `Option`).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum FriendState {
    #[default] Unfriended, Friended, FriendRequestSent, FriendRequestReceived
}

impl FriendState {
    
    /// Maps the possible values of user object key 19 to the `FriendState` variants.
    /// 
    /// # Errors
    /// 
    /// Errors out if the value doesn't map to anything.
    #[must_use]
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            0 => Self::Unfriended,
            1 => Self::Friended,
            3 => Self::FriendRequestSent,
            4 => Self::FriendRequestReceived,
            _ => return Err(GDUtilsError::UnknownFriendStateError.into())
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum ModeratorType {
    #[default] NotMod, Mod, ElderMod, LeaderboardMod, Unknown
}

impl ModeratorType {
    /// Represents moderator type. Type of `moderator` of a `User` (wrapped in `Option`).
    /// 
    /// Maps the possible values of user object key 49 to the `ModeratorType` variants.
    /// 
    /// # Errors
    /// 
    /// Errors out if the value doesn't map to anything.
    #[must_use]
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            0 => Self::NotMod,
            1 => Self::Mod,
            2 => Self::ElderMod,
            3 => Self::LeaderboardMod,
            _ => return Err(GDUtilsError::UnknownModeratorTypeError.into())
        })
    }
}

/// Represents group state. Type of `group_state` of a `User` (wrapped in `Option`).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub enum GroupState {
    Friends, #[default] None
}

impl GroupState {

    /// Represents moderator type. Type of `group_state` of a `User` (wrapped in `Option`).
    /// 
    /// Maps the possible values of user object key 62 to the `ModeratorType` variants.
    /// 
    /// # Errors
    /// 
    /// Errors out if the value doesn't map to anything.
    #[must_use]
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            1 => Self::Friends,
            2 => Self::None,
            _ => return Err(GDUtilsError::UnknownGroupStateError.into())
        })
    }
}

/// Simply checks if a string is numeric.
#[must_use]
pub fn check_if_numeric(string: &str) -> bool {
    for c in string.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    
    true
}

/// Makes a simple POST request with a custom URL and form.
pub async fn post_request(url: String, form: Vec<(&str, &str)>) -> Result<String, reqwest::Error> {
    reqwest::Client::new().post(url)
        .form(&form)
        .header("User_Agent", "")
        .send()
        .await?
        .text()
        .await
}

/// Makes a simple POST request with a custom URL, form and headers.
pub async fn post_request_custom_headers(url: String, form: Vec<(&str, &str)>, headers: HeaderMap) -> Result<String, reqwest::Error> {
    reqwest::Client::new().post(url)
        .form(&form)
        .headers(headers)
        .send()
        .await?
        .text()
        .await
}

/// Makes a simple GET request with a custom url, query and headers. 
pub async fn get_request(url: String, query: Vec<(&str, &str)>, headers: HeaderMap) -> Result<String, reqwest::Error> {
    reqwest::Client::new().get(url)
        .query(&query)
        .headers(headers)
        .send()
        .await?
        .text()
        .await
}

/// Error types
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GDUtilsError {
    UnknownDifficultyError,
    UnknownLevelLengthError,
    UnknownEpicRatingError,
    UnknownGamemodeError,
    UnknownMessageStateError,
    UnknownFriendsStateError,
    UnknownFriendStateError,
    NoTimelyLevelError(TimelyType),
    UnknownTimelyTypeError,
    UnknownGDQueryTypeError,
    UnknownGroupStateError,
    UnknownModeratorTypeError,
    UnknownNongTypeError,
    UnknownNewSongTypeError,
    EvenSeparatorAmountError(String),

    /// Used when Jux is too lazy to just add a new enum variant
    GenericError(String),
}

impl fmt::Display for GDUtilsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoTimelyLevelError(t) => {
                let timely = match t {
                    TimelyType::Daily => "daily",
                    TimelyType::Weekly => "weekly",
                    TimelyType::Event => "event"
                };

                write!(f, "There is no {} level!", timely)
            },

            Self::UnknownTimelyTypeError => write!(f, "Unknown timely level type!"),
            Self::UnknownEpicRatingError => write!(f, "Unknown epic rating type!"),
            Self::UnknownFriendStateError => write!(f, "Unknown friend state!"),
            Self::UnknownFriendsStateError => write!(f, "Unknown friends state!"),
            Self::UnknownGamemodeError => write!(f, "Unknown gamemode!"),
            Self::UnknownLevelLengthError => write!(f, "Unknown level length type!"),
            Self::UnknownMessageStateError => write!(f, "Unknown message state!"),
            Self::UnknownGDQueryTypeError => write!(f, "Unknown GD query type!"),
            Self::UnknownDifficultyError => write!(f, "Unknown level difficulty!"),
            Self::UnknownGroupStateError => write!(f, "Unknown group state!"),
            Self::UnknownModeratorTypeError => write!(f, "Unknown moderator type!"),
            Self::UnknownNongTypeError => write!(f, "Unknown NONG type!"),
            Self::UnknownNewSongTypeError => write!(f, "Unknown new song type!"),
            Self::EvenSeparatorAmountError(e) => write!(f, "Provided string has even amount of `{e}`!"),
            Self::GenericError(e) => write!(f, "Error: {e}"),
        }
    }
}

impl std::error::Error for GDUtilsError {}

impl From<GDUtilsError> for std::fmt::Error {
    fn from(_: GDUtilsError) -> Self {
        Self
    }
}

pub fn song_tuples_vector_from(string: &str) -> Result<Vec<(String, String)>, Report> {
    let mut pairs: Vec<(String, String)> = vec![];
    let split_string = string.split("~|~").collect::<Vec<&str>>();

    if split_string.len() % 2 != 0 {
        return Err(GDUtilsError::EvenSeparatorAmountError(String::from("~|~")).into());
    }

    for e in (0..split_string.len()).step_by(2) {
        let pair = (String::from(split_string[e]), String::from(split_string[e + 1]));
        pairs.push(pair);
    };

    Ok(pairs)
}

/// Uses `tuples_vector_from()`` to make a hashmap. It's a direct conversion.
/// 
/// This shouldn't error out or panic unless a bad string is fed into it.
pub fn song_hashmap_from(string: &str) -> Result<HashMap<String, String>, Report> {
    let mut hashmap = HashMap::new();
    let tuple = song_tuples_vector_from(string)?;

    for (key, value) in tuple {
        hashmap.insert(key, value);
    };

    Ok(hashmap)
}