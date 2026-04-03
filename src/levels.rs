use crate::constants::{URL, SECRET_COMMON};
use crate::chk;

pub async fn get_timely(ltype: String) -> String {
    let url = format!("{}/database/getGJDailyLevel.php", URL);

    let ltype = ltype.to_string();

    let client = reqwest::Client::new();

    let response: String;

    if ltype == "2" {
        let chk = chk::generate(190838017.0, "59182".to_string());

        let form = [("secret", format!("{}", SECRET_COMMON)), ("type", ltype), ("chk", chk)];

        response = client.post(url)
            .header("User_Agent", "")
            .form(&form)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

    } else {
        let form = [("secret", format!("{}", SECRET_COMMON)), ("type", ltype)];
        response = client.post(url)
            .header("User_Agent", "")
            .form(&form)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    }

    return response;
}

pub async fn get_daily() -> String {


    return get_timely(0.to_string()).await.to_string();
}

pub async fn get_weekly() -> String {
    return get_timely(1.to_string()).await.to_string();
}

pub async fn get_event() -> String {
    return get_timely(2.to_string()).await.to_string();
}