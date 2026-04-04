use colonparse::hashmap_from;
use std::fmt::{Debug, Formatter};

use crate::constants::{URL_DATABASE, SECRET_COMMON};
use crate::chk;
// use crate::users;

// Represents level information like name, id,
// song used, author, rating, etc.
#[derive(Debug)]
pub struct Level {
    pub name: String,
    // author: String,
    pub id: i32,
    pub song: String,
    pub rating: String,
    pub stars: i32,
    pub demon: String,
    pub likes: i32,
    pub downloads: i32
}

// Represents timely level (daily, weekly, etc) specific data,
// AKA timely index (eg daily level #) and time left in seconds.
// For event levels, time_left can be ignored.
// Leaving a low value is recommended to save a few bits.
// Really doesn't matter though
#[derive(Debug)]
pub struct TimelyLevel {
    timely_index: i32,
    time_left: i32,
}

pub async fn get_level(id: String) -> Level {
    let url = format!("{}/downloadGJLevel22.php", URL_DATABASE);
    
    let client = reqwest::Client::new();

    let form = vec![("secret", SECRET_COMMON), ("levelID", &id)];

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

    let split_response = response.split("#").collect::<Vec<&str>>();
    let level_hashmap = hashmap_from(String::from(split_response[0]));
    // let author = level_hashmap["6"]; (todo)
    let level = Level {
        name: String::from(level_hashmap.get("2").unwrap())
        // author: level_hashmap["playerID"],
    ,   song: String::from(level_hashmap.get("35").unwrap())
    ,   id: level_hashmap.get("1").unwrap().parse::<i32>().unwrap()
    ,   rating: match level_hashmap["10"].parse::<i32>().unwrap() / level_hashmap.get("8").unwrap().parse::<i32>().unwrap() {
            0 => String::from("unrated")
        ,   10 => String::from("easy")
        ,   20 => String::from("medium")
        ,   30 => String::from("hard")
        ,   40 => String::from("harder")
        ,   50 => String::from("insane")
        ,   _ => String::from("somethingelse?")
        }
    ,   stars: level_hashmap["18"].parse::<i32>().unwrap()
    ,   demon: String::from(level_hashmap.get("17").unwrap())
    ,   likes: level_hashmap["14"].parse::<i32>().unwrap()
    ,   downloads: level_hashmap["10"].parse::<i32>().unwrap()
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

pub async fn get_daily() -> TimelyLevel {
    let daily = get_timely(String::from("0")).await;

    let index: i32 = daily[0..=3].parse().expect("REASON");
    let time_left: i32 = daily[5..].parse().expect("REASON");

    let info = TimelyLevel {
        timely_index: index,
        time_left: time_left
    };

    return info;
}

pub async fn get_weekly() -> TimelyLevel {
    let weekly = get_timely(String::from("1")).await;

    let index: i32 = weekly[3..=5].parse().expect("REASON");
    let time_left: i32 = weekly[7..].parse().expect("REASON");

    let info = TimelyLevel {
        timely_index: index,
        time_left: time_left
    };

    return info;
}

pub async fn get_event() -> TimelyLevel {
    let event = get_timely(String::from("2")).await;

    let index: i32 = event[3..=5].parse().expect("REASON");
    let time_left: i32 = event[7..].parse().expect("REASON");

    let info = TimelyLevel {
        timely_index: index,
        time_left: time_left
    };
    
    return info;
}