use colonparse::hashmap_from;

use crate::{constants::*};
use std::fmt::{Debug};

#[derive(Debug)]
pub struct User {
    pub username: String
,   pub stars: i32
,   pub moons: i32
,   pub diamonds: i32
,   pub usercoins: i32
,   pub secretcoins: i32
,   pub demons: i32
,   pub top: i32
,   pub ctpoints: i32
}

// Gets user data from playerID
pub async fn get_user(id: &str) -> User {
    let url = format!("{}/getGJUserInfo20.php", URL_DATABASE);

    let form = vec![("secret", SECRET_COMMON), ("targetAccountID", id)];

    let client = reqwest::Client::new();

    let request = String::from(client.post(url)
        .form(&form)
        .header("User_Agent", "")
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
    ,   stars: user_hashmap["3"].parse::<i32>().unwrap()
    ,   moons: user_hashmap["52"].parse::<i32>().unwrap()
    ,   diamonds: user_hashmap["46"].parse::<i32>().unwrap()
    ,   usercoins: user_hashmap["17"].parse::<i32>().unwrap()
    ,   secretcoins: user_hashmap["13"].parse::<i32>().unwrap()
    ,   demons: user_hashmap["4"].parse::<i32>().unwrap()
    ,   top: user_hashmap["30"].parse::<i32>().unwrap()
    ,   ctpoints: user_hashmap["8"].parse::<i32>().unwrap()
    };

    return user;
}