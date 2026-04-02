use crate::constants::URL;
use crate::constants::SECRET_COMMON;

pub async fn get_timely(weekly: String) -> String {
    let url = format!("{}/getGJDailyLevel.php", URL);

    let weekly = weekly.to_string();

    let form = [("secret", format!("{}", SECRET_COMMON)), ("weekly", weekly)];

    let client = reqwest::Client::new();

    let response: String = client.post(url)
        .header("User_Agent", "")
        .form(&form)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    return response;
}

pub async fn get_daily() -> String {
    return get_timely("-1".to_string()).await.to_string();
}

pub async fn get_weekly() -> String {
    return get_timely("0".to_string()).await.to_string();
}

pub async fn get_event() -> String {
    return get_timely("-2".to_string()).await.to_string();
}