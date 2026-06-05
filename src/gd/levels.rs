use colonparse::hashmap_from;
use std::collections::HashMap;
use eyre::Error;

use crate::gd::utils::*;
use crate::gd::chk;

pub fn calculate_difficulty<S: std::hash::BuildHasher>(level_hashmap: &HashMap<String, String, S>) -> Result<String, Error> {

    let numerator = level_hashmap["9"].parse::<i32>()?;
    let denominator = level_hashmap["8"].parse::<i32>()?;

    let demon: bool = !level_hashmap["17"].is_empty();

    if level_hashmap["25"].is_empty() {

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

    let feature_score = level_hashmap.get("19").unwrap().parse::<i32>()?;

    let quality: String = match level_hashmap.get("42").unwrap().parse::<i32>()? {
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
        .level_id(level_hashmap["1"].parse()?)
        .level_name(String::from(&level_hashmap["2"]))
        .description(String::from(&level_hashmap["3"]))
        .level_string(Some(String::from(&level_hashmap["4"])))
        .version(level_hashmap["5"].parse()?)
        .player_id(level_hashmap["6"].parse()?)
        .difficulty_denominator(level_hashmap["8"].parse()?)
        .difficulty_numerator(level_hashmap["9"].parse()?)
        .downloads(level_hashmap["10"].parse()?)
        .set_completes(level_hashmap["11"].parse()?)
        .official_song(level_hashmap["12"].parse()?)
        .game_version(level_hashmap["13"].parse()?)
        .likes(level_hashmap["14"].parse()?)
        .length(level_hashmap["15"].parse()?)
        .dislikes(level_hashmap["16"].parse()?)
        .demon(level_hashmap["17"].parse()?)
        .stars(level_hashmap["18"].parse()?)
        .feature_score(level_hashmap["19"].parse()?)
        .auto(level_hashmap["25"].parse()?)
        .record_string(String::from(&level_hashmap["26"]))
        .password(Some(String::from(&level_hashmap["27"])))
        .upload_date(Some(String::from(&level_hashmap["28"])))
        .update_date(Some(String::from(&level_hashmap["29"])))
        .copied_id(level_hashmap["30"].parse()?)
        .two_player(level_hashmap["31"].parse()?)
        .custom_song_id(level_hashmap["35"].parse()?)
        .extra_string(String::from(&level_hashmap["36"]))
        .coins(level_hashmap["37"].parse()?)
        .verified_coins(level_hashmap["38"].parse()?)
        .stars_requested(level_hashmap["39"].parse()?)
        .low_detail_mode(Some(level_hashmap["40"].parse()?))
        .daily_number(Some(level_hashmap["41"].parse()?))
        .epic(level_hashmap["42"].parse()?)
        .demon_difficulty(level_hashmap["43"].parse()?)
        .is_gauntlet(level_hashmap["44"].parse()?)
        .objects(level_hashmap["45"].parse()?)
        .editor_time(level_hashmap["46"].parse()?)
        .editor_time_copies(level_hashmap["47"].parse()?)
        .settings_string(Some(String::from(&level_hashmap["48"])))
        .song_ids(Some(level_hashmap["52"].split(',').collect::<Vec<&str>>().iter().map(|s| s.parse::<i32>().unwrap()).collect()))
        .sfx_ids(Some(level_hashmap["53"].split(',').collect::<Vec<&str>>().iter().map(|s| s.parse::<i32>().unwrap()).collect()))
        .song_size(Some(level_hashmap["54"].parse()?))
        .verification_time(Some(level_hashmap["57"].parse()?))
        .build()?)
}

fn check_type(ltype: &String) -> String {
    if ltype == "2" {
        chk::generate(190_838_017, "59182")
    } else {
        String::new()
    }
}

/// Searches levels by ID or by name
/// 
/// When searching by ID, it should always return just one level
///
/// To get the level(s list) in vector form, use `search_levels_vector`.
/// This will let you do `[0]` to get the first level in the list.
pub async fn search_levels(query: GDQuery) -> Result<String, Error> {
    match query.query_type {
        GDQueryType::ID => {
            let url = format!("{URL_DATABASE}/getGJLevels21.php");
            let form = vec![("secret", SECRET_COMMON), ("levelID", &query.query)];
            Ok(post_request(url, form).await?)
        },
        GDQueryType::Name => {
            let url = format!("{URL_DATABASE}/getGJLevels21.php");
            let form = vec![("secret", SECRET_COMMON), ("str", &query.query), ("type", "0")];
            let levels = post_request(url, form).await?;
            Ok(levels)
        }
    }
}

pub async fn search_levels_vector(query: GDQuery) -> Result<Vec<String>, Error> {
    let levels = search_levels(query).await?;
    let levels = levels.split('#').collect::<Vec<&str>>();
    let mut levels_vector: Vec<_> = vec![];
    
    for level in levels {
        levels_vector.push(String::from(level));     
    }
    Ok(levels_vector)
}

/// Gets the full level object as a string, from name.
/// When using a name for query, the function will return the first level it finds.
pub async fn download_level_by_name(name: String) -> Result<String, Error> {
    let query = GDQuery {
        query: name,
        query_type: GDQueryType::Name,
    };

    let levels = search_levels_vector(query).await?;
    Ok(Clone::clone(&levels[0]))
}

/// Turns a given level object string into a hashmap.
pub fn level_object_to_hashmap(level_object: String) -> Result<HashMap<String, String>, Error> {
    Ok(hashmap_from(level_object))
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
    //         let player_id = String::from(level_hashmap.get("6").unwrap());
            
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
        TimelyType::Daily => { id = "-1"; ltype = "1" },
        TimelyType::Weekly => { id = "-2"; ltype = "2" },
        TimelyType::Event => { id = "-3"; ltype = "3" }
    }

    let chk = check_type(&String::from(ltype));

    let url = format!("{URL_DATABASE}/getGJDailyLevel.php");
    let form = vec![("secret", SECRET_COMMON), ("type", ltype), ("chk", &chk)];

    let timely_info = post_request(url, form).await?.split('|').collect::<Vec<&str>>().iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let level_hashmap = level_object_to_hashmap(download_level_by_id(id).await?)?;

    let timely_index = timely_info[0];
    let time_left = timely_info[1];

    Ok(TimelyLevel {
        level: build_level(&level_hashmap)?,
        timely_index,
        time_left
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