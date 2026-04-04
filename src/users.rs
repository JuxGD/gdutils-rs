use colonparse::hashmap_from;

use crate::{constants::*, levels::Level};
use std::fmt::{Debug};

#[derive(Debug)]
struct User {
    username: String
,   stars: i32
,   moons: i32
,   diamonds: i32
,   demons: i32
,   top: i32
,   ctpoints: i32
}

// Gets user data from playerID
pub async fn get_user(id: &str) -> User {
    let url = format!("{}/getGJUserInfo20.php", URL_DATABASE);

    let form = vec![("secret", SECRET_COMMON), ("targetAccountID", id)];

    let client = reqwest::Client::new();

    let request = String::from(client.post(url)
        .form(&form)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
    );

    let user_hashmap = hashmap_from(request);

    let user = User {
        username: String::from(user_hashmap.get("1").unwrap())
    ,   stars: user_hashmap["1"].parse::<i32>().unwrap()
    ,   moons: user_hashmap["52"].parse::<i32>().unwrap()
    ,   diamonds: user_hashmap["46"].parse::<i32>().unwrap()
    ,   demons: user_hashmap["4"].parse::<i32>().unwrap()
    ,   top: user_hashmap["6"].parse::<i32>().unwrap()
    ,   ctpoints: user_hashmap["8"].parse::<i32>().unwrap()
    };

    return user;
}