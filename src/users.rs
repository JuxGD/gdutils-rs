use crate::constants::*;

// Gets user data from playerID
async fn get_user_data(id: &str) -> String {
    let url = format!("{}/getGJUserInfo20.php", URL_DATABASE);

    let form = vec![("secret", SECRET_COMMON), ("targetAccountID", id)];

    let client = reqwest::Client::new();

    return String::from(client.post(url).form(&form).send().await.unwrap().text().await.unwrap());
}