// use base64::{Engine as _, engine};
use colonparse::hashmap_from;
use eyre::{Ok, Report};
use std::collections::HashMap;
use std::slice::GetDisjointMutError::IndexOutOfBounds;
// use xor_cryptor_keylen::XORCryptor;

use crate::gd::utils::{DemonDifficulty, Difficulty, Epic, GDQuery, GDQueryType, GDUtilsError, Length, Level, LevelBuilder, Quality, SECRET_COMMON, TimelyInfo, TimelyLevel, TimelyType, URL_DATABASE, post_request};
use crate::gd::chk;

/// Performs level difficulty calculation. Doesn't necessarily require a `Level`, just the parameters.
/// 
/// The `Level` struct has a function that uses its own parameters. If you have a variable of type `Level`, that'll be shorter and cleaner code.
/// 
/// # Errors
/// 
/// Will error out if `difficulty_numerator` or `difficulty_denominator` are any number past 5, if the former is less than 0, or if the latter is less than 1.
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
    Ok(LevelBuilder::default()
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
        .build()?)
}

/// Searches levels in the GD servers by ID or name. When searching by ID, it should always return just one level.
/// 
/// # Errors
/// 
/// Shouldn't error out, but it will if there's no data in the search results (i think).
/// 
/// Will also propagate errors from the functions it depends on.
pub async fn search_levels(query: GDQuery) -> Result<String, Report> {
        let url = format!("{URL_DATABASE}/getGJLevels21.php");
        let form = vec![("secret", SECRET_COMMON), ("str", &query.query), ("type", "0")];

        post_request(url, form).await?.split('#').next().map(|s| String::from(s)).ok_or_else(|| GDUtilsError::GenericError(String::from("Error retrieving level search data")).into())
}

/// `search_levels`, but it returns the results in a vector. Also gets rid of the page info.
/// 
/// # Errors
/// 
/// Will propagate errors from the functions it depends on.
pub async fn search_levels_vector(query: GDQuery) -> Result<Vec<String>, Report> {
    let result = search_levels(query).await?;
    let levels = result.split('|').collect::<Vec<&str>>();
    let mut levels_vector: Vec<_> = vec![];
    
    for level in levels {
        levels_vector.push(String::from(level));     
    };

    Ok(levels_vector)
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
            let level = hashmap_from(search_levels_vector(query).await?.first().ok_or_else(|| GDUtilsError::GenericError(String::from("Error getting first level in level search data")))?)?;
            let id = level.get("1").ok_or_else(|| GDUtilsError::GenericError("Retrieved level has no ID; can't download level!".into()))?;

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
    let level_hashmap = level_object_to_hashmap(&level_object)?;
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
    let search_results_vector = search_levels_vector(query).await?;
    let level_object = search_results_vector.get(index).ok_or_else(|| IndexOutOfBounds)?;
    let level_hashmap = level_object_to_hashmap(level_object)?;
    build_level(&level_hashmap)
}

/// Turns a given level object string into a hashmap.
/// 
/// # Errors
/// 
/// Propagates errors from `colonparse::hashmap_from`.
pub fn level_object_to_hashmap(level_object: &str) -> Result<HashMap<String, String>, Report> {
    hashmap_from(level_object)
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

    let level_hashmap = level_object_to_hashmap(&download_level(GDQuery { query: id.into(), query_type: GDQueryType::ID }).await?)?;

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

    Ok(TimelyInfo { timely_index: (timely_info.first().unwrap().parse()?), time_left: (timely_info.get(1).unwrap().parse()?) })
}

// pub async fn get_common_level_info(query: &str) -> CommonLevelInfo {
//     CommonLevelInfoBuilder::default()
//         .level_name()
//         .build()
//         .unwrap()
// }