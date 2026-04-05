use colonparse::hashmap_from;
use std::fmt::{Debug};
use std::collections::HashMap;

use crate::constants::{URL_DATABASE, SECRET_COMMON};
use crate::chk;
// use crate::users;

// Represents level information like name, id,
// song used, author, rating, etc.
#[derive(Debug)]
pub struct LevelInfo {
    pub name: String
,   pub id: i32
,   pub author: String
,   pub author_account_id: i32
,   pub rating: String
,   pub quality: String
,   pub feature_score: i32
,   pub stars: i32
,   pub coins: i32
,   pub verified_coins: bool
,   pub likes: i32
,   pub downloads: i32
,   pub song_id: i32
,   pub is_valid: bool
}

#[derive(Debug)]
pub struct Level {
    pub info: LevelInfo
,   pub string: String
}

// Represents timely level (daily, weekly, etc) specific data,
// AKA timely index (eg daily level #) and time left in seconds.
// For event levels, time_left can be ignored.
// Leaving a low value is recommended to save a few bits.
// Really doesn't matter though
#[derive(Debug)]
pub struct TimelyLevelInfo {
    pub info: LevelInfo
,   pub timely_index: i32
,   pub time_left: i32
}

fn calculate_difficulty(level_hashmap: &HashMap<String, String>) -> String {

    let numerator = level_hashmap["9"].parse::<i32>().unwrap();
    let denominator = level_hashmap["8"].parse::<i32>().unwrap();
   
    let demon: bool;

    if level_hashmap["17"] == "" {
        demon = false;
    } else {
        demon = true;
    }

    if level_hashmap["25"] == "" {

    } else {
        return String::from("Auto");
    }

    let difficulty: String;

    if denominator == 0 {
        difficulty = String::from("N/A")
    } else if demon == false {
        difficulty = match numerator / denominator {
            0 => String::from("Unrated")
        ,   1 => String::from("Easy")
        ,   2 => String::from("Medium")
        ,   3 => String::from("Hard")
        ,   4 => String::from("Harder")
        ,   5 => String::from("Insane")
        ,   _  => String::from("somethingelse")
        }
    } else if demon == true {
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

    return difficulty;
 }

fn calculate_quality(level_hashmap: &HashMap<String, String>) -> String {

    let quality: String;

    let feature_score = level_hashmap.get("19").unwrap().parse::<i32>().unwrap();

    quality = match level_hashmap.get("42").unwrap().parse::<i32>().unwrap() {
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

    return quality

}

fn string_to_bool(string: &str) -> bool {
    if string == "0" {
        return false;
    } else if string == "" {
        return false;
    } else {
        return true;
    }
}

// Gets info for a level by searching.
//
// `query` can be a level name like "bloodbath"/
// In that case, it'll return the information
// for the first level in the search results.
// For specific levels with a name similar
// to the top result, make `query` an ID string
// instead, like "128"
pub async fn get_level_info(query: &str) -> LevelInfo {
    let url = format!("{}/getGJLevels21.php", URL_DATABASE);
    let client = reqwest::Client::new();
    let form = vec![("secret", SECRET_COMMON), ("str", query), ("type", "0")];

    let response = client.post(url)
        .form(&form)
        .header("User_Agent", "")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
    ;

    if response == "-1" {
        return LevelInfo {
            name: String::from("0")
        ,   id: 0
        ,   author: String::from("0")
        ,   author_account_id: 0
        ,   rating: String::from("0")
        ,   quality: String::from("0")
        ,   feature_score: 0
        ,   stars: 0
        ,   coins: 0
        ,   verified_coins: false
        ,   likes: 0
        ,   downloads: 0
        ,   song_id: 0
        ,   is_valid: false
        }
    };

    let split_response = response.split("#").collect::<Vec<&str>>();

    let level_list = split_response[0].split("|").collect::<Vec<&str>>();

    let level_hashmap = hashmap_from(String::from(level_list[0]));
    let author_list = split_response[1].split("|").collect::<Vec<&str>>();
    let author = author_list[0].split(":").collect::<Vec<&str>>();

    let level_info = LevelInfo {
        name: String::from(level_hashmap.get("2").unwrap())
    ,   id: level_hashmap.get("1").unwrap().parse::<i32>().unwrap()
    ,   author: String::from(author[1])
    ,   author_account_id: author[2].parse::<i32>().unwrap()
    ,   rating: calculate_difficulty(&level_hashmap)
    ,   quality: calculate_quality(&level_hashmap)
    ,   feature_score: level_hashmap.get("19").unwrap().parse::<i32>().unwrap()
    ,   stars: level_hashmap.get("18").unwrap().parse::<i32>().unwrap()
    ,   coins: level_hashmap.get("37").unwrap().parse::<i32>().unwrap()
    ,   verified_coins: string_to_bool(level_hashmap.get("38").unwrap())
    ,   likes: level_hashmap.get("14").unwrap().parse::<i32>().unwrap()
    ,   downloads: level_hashmap.get("10").unwrap().parse::<i32>().unwrap()
    ,   song_id: level_hashmap.get("35").unwrap().parse::<i32>().unwrap()
    ,   is_valid: true
    };

    return level_info;
}

// Gets info for a timely level by downloading.
pub async fn get_timely_level_info(id: &str) -> LevelInfo {
    let url = format!("{}/downloadGJLevel22.php", URL_DATABASE);
    let client = reqwest::Client::new();
    let form = vec![("secret", SECRET_COMMON), ("levelID", id)];

    let response: String = client.post(url)
        .header("User_Agent", "")
        .form(&form)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
    ;

    if response == "-1" {
        return LevelInfo {
            name: String::from("0")
        ,   id: 0
        ,   author: String::from("0")
        ,   author_account_id: 0
        ,   rating: String::from("0")
        ,   quality: String::from("0")
        ,   feature_score: 0
        ,   stars: 0
        ,   coins: 0
        ,   verified_coins: false
        ,   likes: 0
        ,   downloads: 0
        ,   song_id: 0
        ,   is_valid: false
        }
    }

    let split_response = response.split("#").collect::<Vec<&str>>();
    let level_hashmap = hashmap_from(String::from(split_response[0]));

    let author = split_response[3].split(":").collect::<Vec<&str>>();

    let level_info = LevelInfo {
        name: String::from(level_hashmap.get("2").unwrap())
    ,   id: level_hashmap.get("1").unwrap().parse::<i32>().unwrap()
    ,   author: String::from(author[1])
    ,   author_account_id: author[2].parse::<i32>().unwrap()
    ,   rating: calculate_difficulty(&level_hashmap)
    ,   quality: calculate_quality(&level_hashmap)
    ,   feature_score: level_hashmap.get("19").unwrap().parse::<i32>().unwrap()
    ,   stars: level_hashmap["18"].parse::<i32>().unwrap()
    ,   coins: level_hashmap.get("37").unwrap().parse::<i32>().unwrap()
    ,   verified_coins: string_to_bool(level_hashmap.get("38").unwrap())
    ,   likes: level_hashmap["14"].parse::<i32>().unwrap()
    ,   downloads: level_hashmap["10"].parse::<i32>().unwrap()
    ,   song_id: level_hashmap.get("35").unwrap().parse::<i32>().unwrap()
    ,   is_valid: true
    };

    return level_info;
}

pub async fn download_level(id: &str) -> Level {
    let url = format!("{}/downloadGJLevel22.php", URL_DATABASE);
    let client = reqwest::Client::new();
    let form = vec![("secret", SECRET_COMMON), ("levelID", id)];

    let response: String = client.post(url)
        .header("User_Agent", "")
        .form(&form)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
    ;

    if response == "-1" {
        return Level {
            info: LevelInfo {
                name: String::from("0")
            ,   id: 0
            ,   author: String::from("0")
            ,   author_account_id: 0
            ,   rating: String::from("0")
            ,   quality: String::from("0")
            ,   feature_score: 0
            ,   stars: 0
            ,   coins: 0
            ,   verified_coins: false
            ,   likes: 0
            ,   downloads: 0
            ,   song_id: 0
            ,   is_valid: false
            }
        ,   string: String::from("0")
        }
    }

    let split_response = response.split("#").collect::<Vec<&str>>();

    let level_info = get_level_info(id).await;

    let level_hashmap = hashmap_from(String::from(split_response[0]));
    let level_string = String::from(level_hashmap.get("4").unwrap());

    let level = Level {
        info: level_info,
        string: level_string
    };

    return level;
}

fn check_type(ltype: &String) -> String {
    if ltype == "2" {
        return chk::generate(190838017.0, String::from("59182"));
    } else {
        return String::from("");
    }
}

pub async fn get_timely(ltype: String) -> String {
    let url = format!("{}/getGJDailyLevel.php", URL_DATABASE);

    let ltype = String::from(ltype);

    let client = reqwest::Client::new();

    let chk = check_type(&ltype);

    let form = vec![("secret", SECRET_COMMON), ("type", &ltype), ("chk", &chk)];

    let response: String = client.post(url)
        .header("User_Agent", "")
        .form(&form)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
    ;

    return response;
}

pub async fn get_daily() -> TimelyLevelInfo {
    let daily = get_timely(String::from("0")).await;

    let index: i32 = daily[0..=3].parse().expect("REASON");
    let time_left: i32 = daily[5..].parse().expect("REASON");

    let level_info = get_timely_level_info("-1").await;

    let daily_info = TimelyLevelInfo {
        info: level_info
    ,   timely_index: index
    ,   time_left: time_left
    };

    return daily_info;
}

pub async fn get_weekly() -> TimelyLevelInfo {
    let weekly = get_timely(String::from("1")).await;

    let index: i32 = weekly[3..=5].parse().expect("REASON");
    let time_left: i32 = weekly[7..].parse().expect("REASON");

    let level_info = get_timely_level_info("-2").await;

    let weekly_info = TimelyLevelInfo {
        info: level_info
    ,   timely_index: index
    ,   time_left: time_left
    };

    return weekly_info;
}

pub async fn get_event() -> TimelyLevelInfo {
    let event = get_timely(String::from("2")).await;

    let index: i32 = event[3..=5].parse().expect("REASON");
    let time_left: i32 = event[7..8].parse().expect("REASON");

    let level_info = get_timely_level_info("-3").await;

    let info = TimelyLevelInfo {
        info: level_info
    ,   timely_index: index
    ,   time_left: time_left
    };
    
    return info;
}