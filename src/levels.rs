use crate::constants::{URL_DATABASE, SECRET_COMMON};
use crate::chk;

// Represents level information like name, id,
// song used, author, rating, etc.
struct Level {
    name: String,
    author: String,
    id: String,
    song: String,
    rating: String,
    demon: String,
    likes: i32,
    downloads: i32
}

// Represents timely level (daily, weekly, etc) specific data,
// AKA timely index (eg daily level #) and time left in seconds.
// For event levels, time_left can be ignored.
// Leaving a low value is recommended to save a few bits.
// Really doesn't matter though
struct TimelyLevel {
    timely_index: i32,
    time_left: i32,
}

pub async fn get_level(id: String) -> Level {
    let url = format!("{}/downloadGjLevel22.php", URL_DATABASE);
    
    let client = reqwest::Client::new();

    let mut form = Vec::new();
    form.push(("secret", SECRET_COMMON));
    form.push(("levelID", &id));

    let response: String = client.post(url)
        .header("User Agent", "")
        .form(&form)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
    ;

    let info = response;

    todo!();
}

fn check_type(ltype: &String) -> String {
    if ltype == "2" {
        return chk::generate(190838017.0, "59182".to_string());
    } else {
        return "".to_string();
    }
}

pub async fn get_timely(ltype: String) -> String {
    let url = format!("{}/getGJDailyLevel.php", URL_DATABASE);

    let ltype = ltype.to_string();

    let client = reqwest::Client::new();

    let chk = check_type(&ltype);

    let mut form = Vec::new();
    form.push(("secret", SECRET_COMMON));
    form.push(("type", &ltype));
    form.push(("chk", &chk));

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
    let daily = get_timely(0.to_string()).await.to_string();

    let index: i32 = daily[0..3].parse().expect("REASON");
    let time_left: i32 = daily[5..].parse().expect("REASON");

    todo!();
}

pub async fn get_weekly() -> TimelyLevel {
    let weekly = get_timely(1.to_string()).await.to_string();

    let index: i32 = weekly[3..5].parse().expect("REASON");
    let time_left: i32 = weekly[7..].parse().expect("REASON");

    let info = TimelyLevel {
        timely_index: index,
        time_left: time_left
    };

    return info;
}

pub async fn get_event() -> TimelyLevel {
    let event = get_timely(2.to_string()).await.to_string();

    let index: i32 = event[3..5].parse().expect("REASON");
    let time_left: i32 = event[7..].parse().expect("REASON");

    let info = TimelyLevel {
        timely_index: index,
        time_left: time_left
    };
    
    return info;
}