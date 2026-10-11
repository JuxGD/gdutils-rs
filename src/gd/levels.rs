use colonparse::hashmap_from;
use eyre::OptionExt;
use eyre::eyre;
use std::collections::HashMap;
use eyre::Error;

use crate::gd::utils::*;
use crate::gd::chk;

pub fn calculate_difficulty<S: std::hash::BuildHasher>(level_hashmap: &HashMap<String, String, S>) -> Result<String, Error> {

    let numerator = level_hashmap.get("9").unwrap().parse::<i32>()?;
    let denominator = level_hashmap.get("8").unwrap().parse::<i32>()?;

    let demon: bool = !level_hashmap.get("17").unwrap().is_empty();

    if level_hashmap.get("25").unwrap().is_empty() {

    } else {
        return Ok(String::from("Auto"));
    }

    let difficulty: String;

    if denominator == 0 {
        difficulty = String::from("N/A");
    } else if !demon {
        difficulty = match numerator / denominator {
            0 => String::from("Unrated")
        ,   1 => String::from("Easy")
        ,   2 => String::from("Medium")
        ,   3 => String::from("Hard")
        ,   4 => String::from("Harder")
        ,   5 => String::from("Insane")
        ,   _  => String::from("somethingelse")
        }
    } else if demon {
        difficulty = match numerator / denominator {
            1 => String::from("Easy Demon")
        ,   2 => String::from("Medium Demon")
        ,   3 => String::from("Hard Demon")
        ,   4 => String::from("Insane Demon")
        ,   5 => String::from("Extreme Demon")
        ,   _  => String::from("somethingelse")
        }
    } else {
        difficulty = String::from("somethingelse");
    }

    Ok(difficulty)
 }

pub fn calculate_quality<S: std::hash::BuildHasher>(level_hashmap: &HashMap<String, String, S>) -> Result<String, Error> {

    let feature_score = level_hashmap.get("19").ok_or_else(|| Error::msg("Error getting feature score from level hashmap"))?.parse::<i32>()?;

    let quality: String = match level_hashmap.get("42").ok_or_else(|| Error::msg("Error getting epic value from level hashmap"))?.parse::<i32>()? {
        0 => if feature_score == 0 {
            String::from("Normal")
            } else {
                String::from("Featured")
            },
        1 => String::from("Epic"),
        2 => String::from("Legendary"),
        3 => String::from("Mythic"),
        _ => String::from("somethingelse"),
    };

    Ok(quality)
}

// fn string_to_bool(string: &str) -> bool {
//     !(string == "0" || string.is_empty())
// }

pub fn build_level<S: std::hash::BuildHasher>(level_hashmap: &HashMap<String, String, S>) -> Result<Level, Error> {
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
        .completes(Some(level_hashmap.get("11").unwrap_or(&String::from("0")).parse()?).unwrap())
        .official_song(level_hashmap.get("12").unwrap().parse()?)
        .game_version(level_hashmap.get("13").unwrap().parse()?)
        .likes(level_hashmap.get("14").unwrap().parse()?)
        .length(level_hashmap.get("15").unwrap().parse()?)
        .dislikes(Some(level_hashmap.get("16").unwrap_or(&String::from("0")).parse()?).unwrap())
        .demon(level_hashmap.get("17").map(|s| if s == "0" {false} else {true}).unwrap_or(false))
        .stars(level_hashmap.get("18").unwrap().parse()?)
        .feature_score(level_hashmap.get("19").unwrap().parse()?)
        .auto(level_hashmap.get("25").map(|s| if s == "0" {false} else {true}).unwrap_or(false))
        .password(Some(String::from(level_hashmap.get("27").unwrap())))
        .upload_date(Some(String::from(level_hashmap.get("28").unwrap())))
        .update_date(Some(String::from(level_hashmap.get("29").unwrap())))
        .copied_id(level_hashmap.get("30").unwrap().parse()?)
        .two_player(level_hashmap.get("31").map(|s| if s == "0" {false} else {true}).unwrap_or(false))
        .custom_song_id(level_hashmap.get("35").unwrap().parse()?)
        .extra_string(String::from(level_hashmap.get("36").unwrap()))
        .coins(level_hashmap.get("37").unwrap().parse()?)
        .verified_coins(level_hashmap.get("38").map(|s| if s == "0" {false} else {true}).unwrap_or(false))
        .stars_requested(level_hashmap.get("39").unwrap().parse()?)
        .low_detail_mode(Some(level_hashmap.get("40").map(|s| if s == "0" {false} else {true}).unwrap_or(false)))
        .daily_number(Some(level_hashmap.get("41").unwrap_or(&String::from("0")).parse()?))
        .epic(level_hashmap.get("42").unwrap().parse()?)
        .demon_difficulty(level_hashmap.get("43").unwrap().parse()?)
        .is_gauntlet(Some(level_hashmap.get("44").map(|s| if s == "0" {false} else {true}).unwrap_or(false)))
        .objects(level_hashmap.get("45").unwrap().parse()?)
        .editor_time(level_hashmap.get("46").unwrap().parse()?)
        .editor_time_copies(level_hashmap.get("47").unwrap().parse()?)
        .settings_string(Some(String::from(level_hashmap.get("48").unwrap_or(&String::from("")))))
        .song_ids(Some(level_hashmap.get("52").unwrap_or(&String::from("0")).split(',').collect::<Vec<&str>>().iter().map(|s| -> Result<i32, Error> {Ok(s.parse::<i32>()?)}).collect::<Result<Vec<i32>, Error>>()?))
        .sfx_ids(Some(level_hashmap.get("53").unwrap_or(&String::from("0")).split(',').collect::<Vec<&str>>().iter().map(|s| -> Result<i32, Error> {Ok(s.parse::<i32>()?)}).collect::<Result<Vec<i32>, Error>>()?))
        .song_size(Some(level_hashmap.get("54").unwrap_or(&String::from("0")).parse()?))
        .verification_time(Some(level_hashmap.get("57").unwrap().parse()?))
        .build()?)
}

/// Searches levels by ID or by name
/// 
/// When searching by ID, it should always return just one level
///
/// To get the level(s list) in vector form, use `search_levels_vector`.
/// This will let you do `[0]` to get the first level in the list.
pub async fn search_levels(query: GDQuery) -> Result<String, Error> {
        let url = format!("{URL_DATABASE}/getGJLevels21.php");
        let form = vec![("secret", SECRET_COMMON), ("str", &query.query), ("type", "0")];
        post_request(url, form).await
}

pub async fn search_levels_vector(query: GDQuery) -> Result<Vec<String>, Error> {
    let result = search_levels(query).await?;
    let data = result.split('#').next().ok_or_eyre("Error retrieving level search data")?;
    let levels = data.split('|').collect::<Vec<&str>>();
    let mut levels_vector: Vec<_> = vec![];
    
    for level in levels {
        levels_vector.push(String::from(level));     
    }

    Ok(levels_vector)
}

/// Gets the full level object as a string, from name. It will return the first level it finds, matching that name.
pub async fn download_level_by_name(name: &str) -> Result<String, Error> {
    let query = GDQuery {
        query: String::from(name),
        query_type: GDQueryType::Name,
    };

    let levels = search_levels_vector(query).await?;

    let level_hashmap = hashmap_from(Clone::clone(&levels[0]));

    download_level_by_id(&level_hashmap.get("1").unwrap()).await
}

/// Turns a given level object string into a hashmap.
pub fn level_object_to_hashmap(level_object: String) -> Result<HashMap<String, String>, Error> {
    Ok(hashmap_from(String::from(level_object.split('#').collect::<Vec<_>>()[0])))
    // let level_object = search_levels(query).await?;

    // if level_object == "-1" {
    //     Err(Report::msg("Invalid response from server (-1) while trying to search levels"))
    // } else {
    //     let split_object = level_object.split('#').collect::<Vec<&str>>();

    //     let level_list = split_object[0].split('|').collect::<Vec<&str>>();
    //     let level_hashmap = hashmap_from(String::from(level_list[0]));

    //     let author_list = split_object[1].split('|').collect::<Vec<&str>>();
    //     let mut new_author_list: Vec<&str> = Vec::new();
    //     for n in level_list {
    //         let level_hashmap = hashmap_from(String::from(n));
    //         let player_id = String::from(level_hashmap.get("6").unwrap().unwrap());
            
    //         for author in &author_list {
    //             let author_player_id = author.split(':').collect::<Vec<&str>>()[0];
    //             if author_player_id == player_id {
    //                 new_author_list.push(author);
    //             }
    //         }
    //     };

    //     let author = new_author_list[0].split(":").collect::<Vec<&str>>();

    //     build_level(level_hashmap)
    // }
}

// Gets info for a timely level by downloading.
// pub async fn get_timely_level_info(id: &str) -> Result<Level, Error> {
//     let url = format!("{URL_DATABASE}/downloadGJLevel22.php");
//     let client = reqwest::Client::new();
//     let form = vec![("secret", SECRET_COMMON), ("levelID", id)];

//     let response: String = client.post(url)
//         .header("User_Agent", "")
//         .form(&form)
//         .send()
//         .await?
//         .text()
//         .await?
//     ;

//     if response == "-1" {
//         Ok(LevelBuilder::default().build().unwrap())
//     } else {
//         let split_response = response.split('#').collect::<Vec<&str>>();
//         let level_hashmap = hashmap_from(String::from(split_response[0]));

//         Ok(build_level(&level_hashmap)?)
//     }
// }

pub async fn download_level_by_id(id: &str) -> Result<String, Error> {
    let url = format!("{URL_DATABASE}/downloadGJLevel22.php");
    let form = vec![("secret", SECRET_COMMON), ("levelID", id)];

    post_request(url, form).await
}

pub async fn get_timely_level(timely_type: TimelyType) -> Result<TimelyLevel, Error> {
    let id: &str;
    let ltype: &str;
    match timely_type {
        TimelyType::Daily => { id = "-1"; ltype = "0" },
        TimelyType::Weekly => { id = "-2"; ltype = "1" },
        TimelyType::Event => { id = "-3"; ltype = "2" },
        TimelyType::Unknown => return Err(eyre!("Unknown timely level type"))
    }

    let chk = if timely_type == TimelyType::Event { chk::generate_reward_chk(190_838_017, "59182")? } else { String::new() };

    let url = format!("{URL_DATABASE}/getGJDailyLevel.php");
    let form = vec![("secret", SECRET_COMMON), ("type", ltype), ("chk", &chk)];

    let timely_info_string = post_request(url, form).await?;
    
    let timely_info_vector_full = timely_info_string.split('|').collect::<Vec<&str>>();

    let level_hashmap = level_object_to_hashmap(download_level_by_id(id).await?)?;

    let timely_index = timely_info_vector_full.get(0).ok_or_eyre("Error getting timely index")?.parse().to_owned();
    let time_left = timely_info_vector_full.get(1).ok_or_eyre("Error getting time left for timely")?.parse().to_owned();

    Ok(TimelyLevel {
        level: build_level(&level_hashmap)?,
        timely_index: timely_index?,
        time_left: time_left?
    })
}

// pub async fn get_timely_info(ltype: String) -> Result<String, Error> {
//     let url = format!("{}/getGJDailyLevel.php", URL_DATABASE);

//     let client = reqwest::Client::new();

//     let chk = check_type(&ltype);

//     let form = vec![("secret", SECRET_COMMON), ("type", &ltype), ("chk", &chk)];

//     Ok(post_request(url, form).await?)
    
// }

// pub async fn get_daily() -> TimelyLevel {
//     let daily = get_timely(String::from("0")).await;

//     let index: i32 = daily[0..=3].parse().expect("Couldn't parse daily index");
//     let time_left: i32 = daily[5..].parse().expect("Couldn't parse time left");

//     let level_info = get_timely_level_info("-1").await;

//     TimelyLevel {
//         level: level_info
//     ,   timely_index: index
//     ,   time_left
//     }
// }

// pub async fn get_weekly() -> TimelyLevel {
//     let weekly = get_timely(String::from("1")).await;

//     let index: i32 = weekly[3..=5].parse().expect("Couldn't parse weekly index");
//     let time_left: i32 = weekly[7..].parse().expect("Couldn't parse time left");

//     let level_info = get_timely_level_info("-2").await;

//     TimelyLevel {
//         level: level_info
//     ,   timely_index: index
//     ,   time_left
//     }
// }

// pub async fn get_event() -> TimelyLevel {
//     let event = get_timely(String::from("2")).await;

//     let index: i32 = event[3..=5].parse().expect("Couldn't parse event index");
//     let time_left: i32 = event[7..8].parse().expect("Couldn't parse time left");

//     let level_info = get_timely_level_info("-3").await;

//     TimelyLevel {
//         level: level_info
//     ,   timely_index: index
//     ,   time_left
//     }
// }

// pub async fn get_common_level_info(query: &str) -> CommonLevelInfo {
//     CommonLevelInfoBuilder::default()
//         .level_name()
//         .build()
//         .unwrap()
// }