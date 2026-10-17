use colonparse::hashmap_from;
use eyre::Report;
use std::collections::HashMap;

use crate::gd::utils::{Level, LevelBuilder, GDQuery, URL_DATABASE, SECRET_COMMON, post_request, GDUtilsError, GDQueryType, TimelyType, TimelyLevel, TimelyInfo};
use crate::gd::chk;

pub fn calculate_difficulty(numerator: i32, denominator: i32, auto: bool, demon: bool) -> Result<String, Report> {
    if !auto {
        let difficulty = if denominator == 0 {
            String::from("N/A")
        } else if !demon {
            match numerator / denominator {
                0 => String::from("Unrated")
            ,   1 => String::from("Easy")
            ,   2 => String::from("Medium")
            ,   3 => String::from("Hard")
            ,   4 => String::from("Harder")
            ,   5 => String::from("Insane")
            ,   _  => String::from("somethingelse")
            }
        } else if demon {
            match numerator / denominator {
                1 => String::from("Easy Demon")
            ,   2 => String::from("Medium Demon")
            ,   3 => String::from("Hard Demon")
            ,   4 => String::from("Insane Demon")
            ,   5 => String::from("Extreme Demon")
            ,   _  => String::from("Unknown Demon")
            }
        } else {
            String::from("Unknown Demon")
        };

        Ok(difficulty)
    } else {
        Ok(String::from("Auto"))
    }

 }

pub fn calculate_quality(feature_score: i32, epic: i32) -> Result<String, Report> {

    let quality: String = match epic {
        0 => if feature_score == 0 {
            String::from("Normal")
            } else {
                String::from("Featured")
            },
        1 => String::from("Epic"),
        2 => String::from("Legendary"),
        3 => String::from("Mythic"),
        _ => String::from("Unknown"),
    };

    Ok(quality)
}

// fn string_to_bool(string: &str) -> bool {
//     !(string == "0" || string.is_empty())
// }

pub fn build_level<S: std::hash::BuildHasher>(level_hashmap: &HashMap<String, String, S>) -> Result<Level, Report> {
    Ok(LevelBuilder::default()
        .level_id(level_hashmap.get("1").unwrap().parse()?)
        .level_name(String::from(level_hashmap.get("2").unwrap()))
        .description(String::from(level_hashmap.get("3").unwrap()))
        .level_string(Some(String::from(level_hashmap.get("4").unwrap())))
        .version(level_hashmap.get("5").unwrap().parse()?)
        .player_id(level_hashmap.get("6").unwrap().parse()?)
        .difficulty_denominator(level_hashmap.get("8").unwrap().parse()?)
        .difficulty_numerator(level_hashmap.get("9").unwrap().parse()?)
        .downloads(level_hashmap.get("10").unwrap().parse()?)
        .completes(level_hashmap.get("11").unwrap_or(&String::from("0")).parse()?)
        .official_song(level_hashmap.get("12").unwrap().parse()?)
        .game_version(level_hashmap.get("13").unwrap().parse()?)
        .likes(level_hashmap.get("14").unwrap().parse()?)
        .length(level_hashmap.get("15").unwrap().parse()?)
        .dislikes(level_hashmap.get("16").unwrap_or(&String::from("0")).parse()?)
        .demon(level_hashmap.get("17").is_some_and(String::is_empty))
        .stars(level_hashmap.get("18").unwrap().parse()?)
        .feature_score(level_hashmap.get("19").unwrap().parse()?)
        .auto(level_hashmap.get("25").is_some_and(String::is_empty))
        .password(Some(String::from(level_hashmap.get("27").unwrap())))
        .upload_date(Some(String::from(level_hashmap.get("28").unwrap())))
        .update_date(Some(String::from(level_hashmap.get("29").unwrap())))
        .copied_id(level_hashmap.get("30").unwrap().parse()?)
        .two_player(level_hashmap.get("31").is_some_and(String::is_empty))
        .custom_song_id(level_hashmap.get("35").unwrap().parse()?)
        .extra_string(String::from(level_hashmap.get("36").unwrap()))
        .coins(level_hashmap.get("37").unwrap().parse()?)
        .verified_coins(level_hashmap.get("38").is_some_and(String::is_empty))
        .stars_requested(level_hashmap.get("39").unwrap().parse()?)
        .low_detail_mode(Some(level_hashmap.get("40").is_some_and(String::is_empty)))
        .daily_number(Some(level_hashmap.get("41").unwrap_or(&String::from("0")).parse()?))
        .epic(level_hashmap.get("42").unwrap().parse()?)
        .demon_difficulty(level_hashmap.get("43").unwrap().parse()?)
        .is_gauntlet(Some(level_hashmap.get("44").is_some_and(String::is_empty)))
        .objects(level_hashmap.get("45").unwrap().parse()?)
        .editor_time(level_hashmap.get("46").map(|s| if s.is_empty() { 0 } else { s.parse().unwrap() }).unwrap())
        .editor_time_copies(level_hashmap.get("47").map(|s| if s.is_empty() { 0 } else { s.parse().unwrap() }).unwrap())
        .settings_string(Some(String::from(level_hashmap.get("48").unwrap_or(&String::new()))))
        .song_ids(Some(level_hashmap.get("52").unwrap_or(&String::from("0")).split(',').collect::<Vec<&str>>().iter().map(|s| {Ok(s.parse::<i32>()?)}).collect::<Result<Vec<i32>, Report>>()?))
        .sfx_ids(Some(level_hashmap.get("53").unwrap_or(&String::from("0")).split(',').collect::<Vec<&str>>().iter().map(|s| {Ok(s.parse::<i32>()?)}).collect::<Result<Vec<i32>, Report>>()?))
        .song_size(Some(level_hashmap.get("54").unwrap_or(&String::from("0")).parse()?))
        .verification_time(Some(level_hashmap.get("57").map(|s| if s.is_empty() { 0 } else { s.parse().unwrap() }).unwrap()))
        .build()?)
}

/// Searches levels by ID or by name
/// 
/// When searching by ID, it should always return just one level
///
/// To get the level(s list) in vector form, use `search_levels_vector`.
/// This will let you do `[0]` to get the first level in the list.
pub async fn search_levels(query: GDQuery) -> Result<String, Report> {
        let url = format!("{URL_DATABASE}/getGJLevels21.php");
        let form = vec![("secret", SECRET_COMMON), ("str", &query.query), ("type", "0")];
        post_request(url, form).await
}

pub async fn search_levels_vector(query: GDQuery) -> Result<Vec<String>, Report> {
    let result = search_levels(query).await?;
    let data = result.split('#').next().ok_or_else(|| GDUtilsError::GenericError("Error retrieving level search data".to_string()))?;
    let levels = data.split('|').collect::<Vec<&str>>();
    let mut levels_vector: Vec<_> = vec![];
    
    for level in levels {
        levels_vector.push(String::from(level));     
    };

    Ok(levels_vector)
}

/// Gets the full level object as a string, from name. It will return the first level it finds, matching that name.
pub async fn download_level_by_name(name: &str) -> Result<String, Report> {
    let query = GDQuery {
        query: String::from(name),
        query_type: GDQueryType::Name,
    };

    let levels = search_levels_vector(query).await?;

    let level_hashmap = hashmap_from(&levels[0])?;

    download_level_by_id(level_hashmap.get("1").unwrap()).await
}

/// Turns a given level object string into a hashmap.
pub fn level_object_to_hashmap(level_object: &str) -> Result<HashMap<String, String>, Report> {
    hashmap_from(&String::from(level_object.split('#').next().unwrap())).map_or_else(
        |_| Err(GDUtilsError::GenericError(String::from("`colonparse` error. The level object is likely not valid!")).into()),
        Ok
    )
}

pub async fn download_level_by_id(id: &str) -> Result<String, Report> {
    let url = format!("{URL_DATABASE}/downloadGJLevel22.php");
    let form = vec![("secret", SECRET_COMMON), ("levelID", id)];

    post_request(url, form).await
}

pub async fn get_timely_level(timely_type: TimelyType) -> Result<TimelyLevel, Report> {
    let id: &str;
    let ltype: &str;
    match timely_type {
        TimelyType::Daily => { id = "-1"; ltype = "0" },
        TimelyType::Weekly => { id = "-2"; ltype = "1" },
        TimelyType::Event => { id = "-3"; ltype = "2" },
        TimelyType::Unknown => return Err(GDUtilsError::UnknownTimelyTypeError().into())
    }

    let chk = if timely_type == TimelyType::Event { chk::generate_reward_chk(190_838_017, "59182")? } else { String::new() };

    let url = format!("{URL_DATABASE}/getGJDailyLevel.php");
    let form = vec![("secret", SECRET_COMMON), ("type", ltype), ("chk", &chk)];

    let timely_info_string = post_request(url, form).await?;
    
    let timely_info_vector_full = timely_info_string.split('|').collect::<Vec<&str>>();

    let level_hashmap = level_object_to_hashmap(&download_level_by_id(id).await?)?;

    let timely_index = timely_info_vector_full.first().ok_or_else(|| GDUtilsError::GenericError(String::from("Error getting timely index")))?.parse();
    let time_left = timely_info_vector_full.get(1).ok_or_else(|| GDUtilsError::GenericError(String::from("Error getting time left for timely")))?.parse();

    Ok(TimelyLevel {
        level: build_level(&level_hashmap)?,
        timely_index: timely_index?,
        time_left: time_left?
    })
}

pub async fn get_timely_info(timely_type: TimelyType) -> Result<TimelyInfo, Report> {
    let ltype = match timely_type {
        TimelyType::Daily => "0",
        TimelyType::Weekly => "1",
        TimelyType::Event => "2",
        TimelyType::Unknown => return Err(GDUtilsError::UnknownTimelyTypeError().into())
    };

    let url = format!("{URL_DATABASE}/getGJDailyLevel.php");

    let chk = if timely_type == TimelyType::Event { chk::generate_reward_chk(190_838_017, "59182")? } else { String::new() };

    let form = vec![("secret", SECRET_COMMON), ("type", ltype), ("chk", &chk)];

    let response = post_request(url, form).await?;
    let timely_info: Vec<&str> = response.split('|').collect();

    Ok(TimelyInfo { timely_index: (timely_info.first().unwrap().parse()?), time_left: (timely_info.get(1).unwrap().parse()?) })
}

// pub async fn get_common_level_info(query: &str) -> CommonLevelInfo {
//     CommonLevelInfoBuilder::default()
//         .level_name()
//         .build()
//         .unwrap()
// }