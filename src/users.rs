use colonparse::hashmap_from;

use crate::{constants::*};
use std::fmt::{Debug};

#[derive(Debug)]
pub struct UserInfo {
    pub username: String
,   pub stars: i32
,   pub moons: i32
,   pub diamonds: i32
,   pub usercoins: i32
,   pub secretcoins: i32
,   pub demons: i32
,   pub top: i32
,   pub ctpoints: i32
,   pub account_id: i32
,   pub player_id: i32
,   pub is_valid: bool
}

enum Query {
    ID,
    Username
}

fn check_if_numeric(string: &str) -> bool {
    for c in string.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

// Gets user info from accountID
pub async fn get_user_info(query: &str) -> UserInfo {
    let query_type: Query = if check_if_numeric(query) {
        Query::ID
    } else {
        Query::Username
    };

    match query_type {
        Query::ID => return from_id(query).await,
        Query::Username => return from_username(query).await
    };
}

async fn from_id(id: &str) -> UserInfo {
    let url = format!("{}/getGJUserInfo20.php", URL_DATABASE);
    let form: Vec<(&str, &str)> = vec![("secret", SECRET_COMMON), ("targetAccountID", id)];

    let client = reqwest::Client::new();

    let response = String::from(client.post(url)
        .form(&form)
        .header("User_Agent", "")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
    );

    if response == "-1" {
        return UserInfo {
            username: String::from("0")
        ,   stars: 0
        ,   moons: 0
        ,   diamonds: 0
        ,   usercoins: 0
        ,   secretcoins: 0
        ,   demons: 0
        ,   top: 0
        ,   ctpoints: 0
        ,   account_id: 0
        ,   player_id: 0
        ,   is_valid: false
        }
    }

    let user_hashmap = hashmap_from(response);

    let user = UserInfo {
        username: String::from(user_hashmap.get("1").unwrap())
    ,   stars: user_hashmap["3"].parse::<i32>().unwrap()
    ,   moons: user_hashmap["52"].parse::<i32>().unwrap()
    ,   diamonds: user_hashmap["46"].parse::<i32>().unwrap()
    ,   usercoins: user_hashmap["17"].parse::<i32>().unwrap()
    ,   secretcoins: user_hashmap["13"].parse::<i32>().unwrap()
    ,   demons: user_hashmap["4"].parse::<i32>().unwrap()
    ,   top: user_hashmap["30"].parse::<i32>().unwrap()
    ,   ctpoints: user_hashmap["8"].parse::<i32>().unwrap()
    ,   account_id: user_hashmap["16"].parse::<i32>().unwrap()
    ,   player_id: user_hashmap["2"].parse::<i32>().unwrap()
    ,   is_valid: true
    };

    return user;
}

async fn from_username(username: &str) -> UserInfo {
    let url = format!("{}/getGJUsers20.php", URL_DATABASE);
    let form: Vec<(&str, &str)> = vec![("secret", SECRET_COMMON), ("str", username)];

    let client = reqwest::Client::new();

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
        return UserInfo {
            username: String::from("0")
        ,   stars: 0
        ,   moons: 0
        ,   diamonds: 0
        ,   usercoins: 0
        ,   secretcoins: 0
        ,   demons: 0
        ,   top: 0
        ,   ctpoints: 0
        ,   account_id: 0
        ,   player_id: 0
        ,   is_valid: false
        }
    }

    let user_hashmap = hashmap_from(response);
    let account_id = user_hashmap.get("16").unwrap();
    
    return from_id(account_id).await;
}