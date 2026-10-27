// use base64::{Engine as _, engine};
use crate::gd::utils::{hashmap_from, song_hashmap_from};
use derive_builder::Builder;
use eyre::Report;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use xor_cryptor_keylen::XORCryptor;

use crate::gd::utils::{DemonDifficulty, Difficulty, Epic, GDQuery, GDQueryType, GDUtilsError, Length, Level, LevelBuilder, Quality, SECRET_COMMON, TimelyInfo, TimelyLevel, TimelyType, URL_DATABASE, post_request};
use crate::gd::chk;

#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default, PartialEq)]
pub struct LevelSearchResults {
    pub levels: Vec<Level>,
    pub creators: Vec<CreatorInfo>,
    pub songs: Vec<Song>,
    pub page_info: PageInfo,
    pub hash: String
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default, PartialEq, Eq)]
pub struct CreatorInfo {
    pub user_id: i32,
    pub username: String,
    pub account_id: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default, PartialEq, Eq)]
pub struct PageInfo {
    pub total: i32,
    pub offset: i32,
    pub amount: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default, PartialEq)]
pub struct Song {
    pub id: i32,
    pub name: String,
    pub artist_id: i32,
    pub artist_name: String,
    pub size: f32,
    pub video_id: String,
    pub youtube_url: String,
    pub is_verified: bool,
    pub song_priority: Option<i32>,
    pub link: String,
    pub nong_enum: Option<NongType>,
    pub extra_artist_ids: Option<Vec<i32>>,
    pub new: Option<bool>,
    pub new_type: Option<NewSongType>,
    pub extra_artist_names: Option<Vec<(i32, String)>>,
    pub download_soundtrack_override: String
}

pub struct Artist {
    pub id: i32,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum NongType {
    #[default] None, NCS, Chompo
}

impl NongType {
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            0 => Self::None,
            1 => Self::NCS,
            2 => Self::Chompo,
            _ => return Err(GDUtilsError::UnknownNongTypeError.into())
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum NewSongType {
    #[default] Yellow, Blue
}

impl NewSongType {
    pub fn new(value: usize) -> Result<Self, Report> {
        Ok(match value {
            0 => Self::Yellow,
            1 => Self::Blue,
            _ => return Err(GDUtilsError::UnknownNewSongTypeError.into())
        })
    }
}
/// Performs level difficulty calculation. Doesn't necessarily require a [`Level`], just the parameters.
/// 
/// The [`Level`] struct has a function that uses its own parameters. If you have a variable of type [`Level`], that'll be shorter and cleaner code.
/// 
/// # Errors
/// 
/// Will error out if [`difficulty_numerator`] or [`difficulty_denominator`] are any number past 5, if the former is less than 0, or if the latter is less than 1.
pub fn calculate_difficulty(difficulty_numerator: i8, difficulty_denominator: u8, auto: bool, demon: bool) -> Result<Difficulty, Report> {
    let difficulty = if auto {
        Difficulty::Auto
    } else {
        if difficulty_denominator == 0 {
            Difficulty::NA
        } else if !demon {
            match difficulty_numerator as u8 / difficulty_denominator {
                0 => Difficulty::Unrated,
                1 => Difficulty::Easy,
                2 => Difficulty::Normal,
                3 => Difficulty::Hard,
                4 => Difficulty::Harder,
                5 => Difficulty::Insane,
                _ => return Err(GDUtilsError::UnknownDifficultyError.into())

            }
        } else {
            match difficulty_numerator as u8 / difficulty_denominator {
                1 => Difficulty::EasyDemon,
                2 => Difficulty::MediumDemon,
                3 => Difficulty::HardDemon,
                4 => Difficulty::InsaneDemon,
                5 => Difficulty::ExtremeDemon,
                _ => return Err(GDUtilsError::UnknownDifficultyError.into())
            }
        }
    };

    Ok(difficulty)
 }

/// Performs level quality calculation. Doesn't necessarily require a `Level`, just the parameters.
/// 
/// The `Level` struct has a function that uses its own parameters. If you have a variable of type `Level`, that'll be shorter and cleaner code.
pub fn calculate_quality(feature_score: i32, epic: Epic) -> Quality {
    match epic {
        Epic::None => if feature_score == 0 {
            Quality::Normal
            } else {
                Quality::Featured
            },
        Epic::Epic => Quality::Epic,
        Epic::Legendary => Quality::Legendary,
        Epic::Mythic => Quality::Mythic
    }
}

// TODO: fix the function commented out below, required imports are commented out at the top of the file

// /// Performs level password DeCodification. Doesn't necessarily require a `Level`, just the parameters.
// /// 
// /// The `Level` struct has a function that uses its own parameters. If you have a variable of type `Level`, that'll be shorter and cleaner code.
// /// 
// /// # Errors
// /// 
// /// - `from_utf8` returns `Result`. It may error out returning a `FromUtf8Error`, propagating to this function.
// /// - `encrypt_v2`, from the `xor_cryptor` crate also returns `Result`,
// /// but it's unlikely to fail within the context of this function as it's being given a set key.
// pub fn decode_password(password: &str) -> Result<String, Report> {
//     let base64_decoded = engine::general_purpose::URL_SAFE.decode(password)?;
    
//     println!("{base64_decoded:?}");

//     let xrc = XORCryptor::new("26364")?;

//     let xor_decrypted = xrc.decrypt_vec(base64_decoded);

//     Ok(String::from_utf8_lossy_owned(xor_decrypted))
// }

/// Builds a `Level` from a level hashmap. In order to get this hashmap from an existing level, use the function `level_object_to_hashmap()` is provided.
/// 
/// # Errors
/// 
/// Errors out if any of the keys' values are invalid, or from errors propagating from functions it depends on.
/// Testing has been done and it's not supposed to error out, please report any errors.
/// 
/// # Panics
/// 
/// Panics for most keys, if they aren't present. Testing has been done and it's not supposed to panic, please report any panics.
pub fn build_level<S: std::hash::BuildHasher>(level_hashmap: &HashMap<String, String, S>) -> Result<Level, Report> {
    LevelBuilder::default()
        .level_id(level_hashmap.get("1").unwrap().parse()?)
        .level_name(level_hashmap.get("2").unwrap().into())
        .description(level_hashmap.get("3").unwrap().into())
        .level_string(level_hashmap.get("4").map(|s| s.into()))
        .version(level_hashmap.get("5").unwrap().parse()?)
        .player_id(level_hashmap.get("6").unwrap().parse()?)
        .difficulty_denominator(level_hashmap.get("8").unwrap().parse()?)
        .difficulty_numerator(level_hashmap.get("9").unwrap().parse()?)
        .downloads(level_hashmap.get("10").unwrap().parse()?)
        .completes(level_hashmap.get("11").and_then(|s| s.parse().ok()))
        .official_song(level_hashmap.get("12").unwrap().parse()?)
        .game_version(level_hashmap.get("13").unwrap().parse()?)
        .likes(level_hashmap.get("14").unwrap().parse()?)
        .length(level_hashmap.get("15").unwrap().parse::<usize>().map(Length::new)??)
        .dislikes(level_hashmap.get("16").unwrap_or(&"0".into()).parse()?)
        .demon(!level_hashmap.get("17").is_some_and(String::is_empty))
        .stars(level_hashmap.get("18").unwrap().parse()?)
        .feature_score(level_hashmap.get("19").unwrap().parse()?)
        .star_ratings(level_hashmap.get("20").and_then(|s| s.parse().ok()))
        .star_ratings_sum(level_hashmap.get("21").and_then(|s| s.parse().ok()))
        .min_star_ratings(level_hashmap.get("22").and_then(|s| s.parse().ok()))
        .max_star_ratings(level_hashmap.get("23").and_then(|s| s.parse().ok()))
        .demon_votes(level_hashmap.get("24").and_then(|s| s.parse().ok()))
        .auto(!level_hashmap.get("25").is_some_and(String::is_empty))
        .record_string(level_hashmap.get("26").and_then(|s| s.parse().ok()))
        .password(level_hashmap.get("27").and_then(|s| s.parse().ok()))
        .upload_date(level_hashmap.get("28").and_then(|s| s.parse().ok()))
        .update_date(level_hashmap.get("29").and_then(|s| s.parse().ok()))
        .copied_id(level_hashmap.get("30").unwrap().parse()?)
        .two_player(level_hashmap.get("31").is_some_and(String::is_empty))
        .rate_stars(level_hashmap.get("32").and_then(|s| s.parse().ok()))
        .rate_feature(if level_hashmap.get("33").is_some() { Some(level_hashmap.get("33").is_some_and(String::is_empty)) } else { None })
        .rate_user(level_hashmap.get("34").and_then(|s| s.parse().ok()))
        .custom_song_id(level_hashmap.get("35").unwrap().parse()?)
        .extra_string(level_hashmap.get("36").and_then(|s| s.parse().ok()))
        .coins(level_hashmap.get("37").unwrap().parse()?)
        .verified_coins(level_hashmap.get("38").is_some_and(String::is_empty))
        .stars_requested(level_hashmap.get("39").unwrap().parse()?)
        .low_detail_mode(if level_hashmap.get("40").is_some() { Some(level_hashmap.get("40").is_some_and(String::is_empty)) } else { None })
        .daily_number(if level_hashmap.get("41").is_none() { None } else { Some(level_hashmap.get("41").unwrap().parse()?) } )
        .epic(level_hashmap.get("42").unwrap().parse::<usize>().map(Epic::new)??)
        .demon_difficulty(level_hashmap.get("43").unwrap().parse::<usize>().map(DemonDifficulty::new)??)
        .is_gauntlet(level_hashmap.get("44").is_some_and(String::is_empty))
        .objects(level_hashmap.get("45").unwrap().parse()?)
        .editor_time(level_hashmap.get("46").map(|s| if s.is_empty() { 0 } else { s.parse().unwrap() }).unwrap())
        .editor_time_copies(level_hashmap.get("47").map(|s| if s.is_empty() { 0 } else { s.parse().unwrap() }).unwrap())
        .settings_string(level_hashmap.get("48").and_then(|s| s.parse().ok()))
        .song_ids(level_hashmap.get("52").map(|s| s.split(',').map(|s| s.parse().unwrap()).collect()))
        .sfx_ids(level_hashmap.get("53").map(|s| s.split(',').map(|s| s.parse().unwrap()).collect()))
        .song_size(level_hashmap.get("54").and_then(|s| s.parse().ok()))
        .verification_time(level_hashmap.get("57").and_then(|s| s.parse().ok()))
        .exact_upload_time(level_hashmap.get("62").and_then(|s| s.parse().ok()))
        .exact_update_time(level_hashmap.get("63").and_then(|s| s.parse().ok()))
        .build().map_err(|e| e.into())
}

pub fn build_creator_info(creator: &Vec<String>) -> Result<CreatorInfo, Report> {
    CreatorInfoBuilder::default()
        .user_id(creator.get(0).unwrap().parse()?)
        .username(creator.get(1).unwrap().into())
        .account_id(creator.get(2).unwrap().parse()?)
        .build().map_err(|e| e.into())
}

pub fn build_song(song_hashmap: &HashMap<String, String>) -> Result<Song, Report> {
    let extra_artist_names_vec: Option<Vec<&str>> = if song_hashmap.get("14").is_some() {
        Some(song_hashmap.get("14").unwrap().split(',').collect())
    } else {
        None
    };

    let mut extra_artist_names: Option<Vec<(i32, String)>> = Some(vec![]);

    match extra_artist_names_vec {
        Some(v) => for i in (0..v.len()).step_by(2) {
            extra_artist_names.as_mut().unwrap().push((v.get(i).unwrap().parse()?, String::from(*v.get(i + 1).unwrap())));
        },
        None => extra_artist_names = None
    };

    SongBuilder::default()
        .id(song_hashmap.get("1").unwrap().parse()?)
        .name(song_hashmap.get("2").unwrap().into())
        .artist_id(song_hashmap.get("3").unwrap().parse()?)
        .artist_name(song_hashmap.get("4").unwrap().into())
        .size(song_hashmap.get("5").unwrap().parse()?)
        .video_id(song_hashmap.get("6").unwrap().into())
        .youtube_url(song_hashmap.get("7").unwrap().into())
        .is_verified(song_hashmap.get("8").is_some_and(String::is_empty))
        .song_priority(song_hashmap.get("9").and_then(|s| Some(s.parse().unwrap())))
        .link(song_hashmap.get("10").unwrap().into())
        .nong_enum(song_hashmap.get("11").and_then(|s| Some(NongType::new(s.parse().unwrap()).unwrap())))
        .extra_artist_ids(song_hashmap.get("12").and_then(|s| Some(s.split('.').map(|s| s.parse().unwrap()).collect())))
        .new(if song_hashmap.get("13").is_some() { Some(song_hashmap.get("13").is_some_and(String::is_empty)) } else { None })
        .new_type(song_hashmap.get("14").and_then(|s| Some(NewSongType::new(s.parse().unwrap()).unwrap())))
        .extra_artist_names(extra_artist_names)
        .download_soundtrack_override(song_hashmap.get("16").unwrap().into())
        .build().map_err(|e| e.into())
}

pub fn build_page_info(page_info: Vec<String>) -> Result<PageInfo, Report> {
    PageInfoBuilder::default()
        .total(page_info.get(0).unwrap().parse()?)
        .offset(page_info.get(1).unwrap().parse()?)
        .amount(page_info.get(2).unwrap().parse()?)
        .build().map_err(|e| e.into())
}
/// Searches levels in the GD servers by ID or name. When searching by ID, it should always return just one level.
/// 
/// This doesn't return just the level results list, but also lists of level creators,  in the same order
/// 
/// # Errors
/// 
/// Shouldn't error out, but it will if there's no data in the search results (i think).
/// 
/// Will also propagate errors from the functions it depends on.
pub async fn search_levels(query: GDQuery) -> Result<LevelSearchResults, Report> {
    let url = format!("{URL_DATABASE}/getGJLevels21.php");
    let form = vec![("secret", SECRET_COMMON), ("str", &query.query), ("type", "0")];

    let results: Vec<String> = post_request(url, form).await?.split('#').map(|s| String::from(s)).collect();

    let levels: Vec<Level> = results.get(0).unwrap().split('|').map(|s| hashmap_from(s)).filter_map(Result::ok).map(|h| build_level(&h)).filter_map(Result::ok).collect();

    let creators_buffer: Vec<CreatorInfo> = results.get(1).unwrap().split('|').map(|s| build_creator_info(&s.split(':').map(|s| s.into()).collect())).filter_map(Result::ok).collect();
    let mut creators: Vec<CreatorInfo> = vec![];

    let songs_buffer: Vec<Song> = results.get(2).unwrap().split("~:~").map(|s| song_hashmap_from(s)).filter_map(Result::ok).map(|h| build_song(&h)).filter_map(Result::ok).collect();
    let mut songs: Vec<Song> = vec![];

    for level in &levels {
        for creator in &creators_buffer {
            if level.player_id == creator.user_id {
                creators.push(creator.to_owned());
                break;
            }
        }

        for song in &songs_buffer {
            if song.id == level.custom_song_id {
                songs.push(song.to_owned());
                break;
            }
        }
    }

    let page_info = build_page_info(results.get(3).unwrap().split(':').map(String::from).collect())?;

    LevelSearchResultsBuilder::default()
        .levels(levels)
        .creators(creators)
        .songs(songs)
        .page_info(page_info)
        .hash(results.get(4).unwrap().into())
        .build().map_err(|e| e.into())
}

/// Gets the full level object as a string. If `query`'s `query_type` is `GDQueryType::Name`, searches first to get the level ID, then downloads it using that.
/// 
/// # Errors
/// 
/// When using a `query` with `query_type` `GDQueryType::Name`, errors out if:
/// 
/// - there's no search results for `query.query`
/// - the retrieved level from the search results has no ID for whatever reason
///
/// It will also propagate errors from the functions it depends on.
pub async fn download_level(query: GDQuery) -> Result<String, Report> {
    let url = format!("{URL_DATABASE}/downloadGJLevel22.php");
    
    let response = match query.query_type {
        GDQueryType::Name => {
            let results = search_levels(query).await?;
            let level = results.levels.first().ok_or_else(|| GDUtilsError::GenericError(String::from("Error getting first level in level search data")))?;
            let id = &level.level_id.to_string();

            let form = vec![("secret", SECRET_COMMON), ("levelID", id)];

            post_request(url, form).await.map_err(std::convert::Into::<Report>::into)?
        },
        GDQueryType::ID => {
            let form = vec![("secret", SECRET_COMMON), ("levelID", &query.query)];

            post_request(url, form).await.map_err(std::convert::Into::<Report>::into)?
        }
    };

    response.split('#').next().map(|s| String::from(s)).ok_or_else(|| GDUtilsError::GenericError(String::from("Error retrieving level search data")).into())
}

/// Convenience function, downloads a level object through the `getGJLevel22.php` endpoint and builds it for you.
/// 
/// # Errors
/// 
/// The functions this uses may error out. See the docs for those.
/// 
/// The `build_level` function isn't being unwrapped since it returns the same type as this function,
/// but it may error as well, making this return error.
pub async fn download_and_build_level(query: GDQuery) -> Result<Level, Report> {
    let level_object = download_level(query).await?;
    let level_hashmap = hashmap_from(&level_object)?;
    build_level(&level_hashmap)
}

/// Searches levels through the `getGJLevels21.php` endpoint, lets you pick one and builds it for you.
/// 
/// # Errors
/// 
/// The functions this uses may error out. See the docs for those.
/// 
/// The `build_level` function isn't being unwrapped since it returns the same type as this function,
/// but it may error as well, making this return error.
pub async fn search_and_build_level(query: GDQuery, index: usize) -> Result<Level, Report> {
    let results = search_levels(query).await?;

    results.levels.get(index).ok_or(GDUtilsError::GenericError(String::from("Error getting level at requested index!"))).map(|l| l.to_owned()).map_err(|e| e.into())
}

/// Gets the timely level along with the info for it.
/// 
/// # Errors
/// 
/// Propagates errors from functions it depends on.
pub async fn get_timely_level(timely_type: TimelyType) -> Result<TimelyLevel, Report> {
    let id = match timely_type {
        TimelyType::Daily => "-1",
        TimelyType::Weekly => "-2",
        TimelyType::Event => "-3"
    };
    
    let timely_info = get_timely_info(timely_type).await?;

    let level_hashmap = hashmap_from(&download_level(GDQuery { query: id.into(), query_type: GDQueryType::ID }).await?)?;

    Ok(TimelyLevel { level: build_level(&level_hashmap)?, info: timely_info})
}

/// Gets the basic timely level info.
/// 
/// # Errors
/// 
/// Propagates errors from functions it depends on.
pub async fn get_timely_info(timely_type: TimelyType) -> Result<TimelyInfo, Report> {
    let ltype = match timely_type {
        TimelyType::Daily => "0",
        TimelyType::Weekly => "1",
        TimelyType::Event => "2"
    };

    let url = format!("{URL_DATABASE}/getGJDailyLevel.php");

    let chk = if timely_type == TimelyType::Event { chk::generate_reward_chk(190_838_017, "59182")? } else { String::new() };

    let form = vec![("secret", SECRET_COMMON), ("type", ltype), ("chk", &chk)];

    let response = post_request(url, form).await?;

    if response == "-1" { Err(GDUtilsError::NoTimelyLevelError(timely_type))? }

    let timely_info: Vec<&str> = response.split('|').collect();

    Ok(TimelyInfo {
        timely_index: (timely_info.first().ok_or_else(|| GDUtilsError::GenericError(String::from("Error unwrapping the first element of the timely info!")))?.parse()?),
        time_left: (timely_info.get(1).ok_or_else(|| GDUtilsError::GenericError(String::from("Error unwrapping the second element of the timely info!")))?.parse()?)
    })
}

// pub async fn get_common_level_info(query: &str) -> CommonLevelInfo {
//     CommonLevelInfoBuilder::default()
//         .level_name()
//         .build()
//         .unwrap()
// }