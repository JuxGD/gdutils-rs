use reqwest::{header::{HeaderMap, USER_AGENT}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct TimelyBody {
    secret: String,
    weekly: String
}

pub async fn get_timely(weekly: i8) -> String {
    let url = "http://www.boomlings.com/database/getGJDailyLevel.php";

    let form = TimelyBody {
        secret: "Wmfd2893gb7".to_string(),
        weekly: format!("{}", weekly)
    };

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "".parse().unwrap());

    let client = reqwest::Client::new();

    let response: String = client.post(url)
        .json(&form)
        .header("User_Agent", "")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    return response;
}

pub async fn get_weekly() -> String {
    return get_timely(0).await.to_string();
}

pub async fn get_daily() -> String {
    return get_timely(1).await.to_string();
}

pub async fn get_event() -> String {
    return get_timely(2).await.to_string();
}