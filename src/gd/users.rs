use colonparse::{hashmap_from};
use std::collections::HashMap;
use eyre::Report;

use crate::gd::utils::{FriendState, FriendsState, GDQuery, GDQueryType, GDUtilsError, Gamemode, GroupState, MessageState, ModeratorType, SECRET_COMMON, URL_DATABASE, User, UserBuilder, post_request};

pub fn build_user<S: std::hash::BuildHasher>(user_hashmap: &HashMap<String, String, S>) -> Result<User, Report> {
    Ok(UserBuilder::default()
        .username(user_hashmap.get("1").unwrap().into())
        .user_id(user_hashmap.get("2").unwrap().parse()?)
        .stars(user_hashmap.get("3").unwrap().parse()?)
        .demon_count(user_hashmap.get("4").unwrap().parse()?)
        .ranking(user_hashmap.get("6").and_then(|s| s.parse().ok()))
        .account_highlight(user_hashmap.get("7").map(std::convert::Into::into))
        .creator_points(user_hashmap.get("8").unwrap().parse()?)
        .icon_id(user_hashmap.get("9").and_then(|s| s.parse().ok()))
        .color(user_hashmap.get("10").unwrap().parse()?)
        .color_2(user_hashmap.get("11").unwrap().parse()?)
        .ship_id(user_hashmap.get("12").and_then(|s| s.parse().ok()))
        .secret_coins(user_hashmap.get("13").unwrap().parse()?)
        .icon_type(user_hashmap.get("14").map(|s| Gamemode::new(s.parse::<usize>().unwrap()).unwrap()))
        .special(user_hashmap.get("15").and_then(|s| match s.as_str() { "0" => Some(false), "2" => Some(true), _ => None }))
        .account_id(user_hashmap.get("16").unwrap().parse()?)
        .user_coins(user_hashmap.get("17").unwrap().parse()?)
        .message_state(user_hashmap.get("18").map(|s| MessageState::new(s.parse().unwrap()).unwrap()))
        .friends_state(user_hashmap.get("19").map(|s| FriendsState::new(s.parse().unwrap()).unwrap()))
        .youtube(user_hashmap.get("20").and_then(|s| if s.is_empty() { None } else { Some(s.into()) }))
        .acc_icon(user_hashmap.get("21").and_then(|s| s.parse().ok()))
        .acc_ship(user_hashmap.get("22").and_then(|s| s.parse().ok()))
        .acc_ball(user_hashmap.get("23").and_then(|s| s.parse().ok()))
        .acc_bird(user_hashmap.get("24").and_then(|s| s.parse().ok()))
        .acc_dart(user_hashmap.get("25").and_then(|s| s.parse().ok()))
        .acc_robot(user_hashmap.get("26").and_then(|s| s.parse().ok()))
        .acc_streak(user_hashmap.get("27").and_then(|s| s.parse().ok()))
        .acc_glow(if user_hashmap.get("28").is_some() { Some(user_hashmap.get("28").is_some_and(String::is_empty)) } else { None })
        .is_registered(if user_hashmap.get("29").is_some() { Some(user_hashmap.get("29").is_some_and(String::is_empty)) } else { None })
        .global_rank(user_hashmap.get("30").and_then(|s| s.parse().ok()))
        .friend_state(user_hashmap.get("31").map(|s| if s.is_empty() { FriendState::Unfriended } else { FriendState::new(s.parse().unwrap()).unwrap() }))
        .friend_request_id(user_hashmap.get("32").and_then(|s| s.parse().ok()))
        .friend_request_message(user_hashmap.get("35").map(std::convert::Into::into))
        .friend_request_age(user_hashmap.get("37").and_then(|s| s.parse().ok()))
        .messages(user_hashmap.get("38").and_then(|s| s.parse().ok()))
        .friend_requests(user_hashmap.get("39").and_then(|s| s.parse().ok()))
        .new_friends(user_hashmap.get("40").and_then(|s| s.parse().ok()))
        .new_friend_request(if user_hashmap.get("41").is_some() { Some(user_hashmap.get("41").is_some_and(String::is_empty)) } else { None })
        .age(user_hashmap.get("42").and_then(|s| s.parse().ok()))
        .acc_spider(user_hashmap.get("43").and_then(|s| s.parse().ok()))
        .twitter(user_hashmap.get("44").and_then(|s| if s.is_empty() { None } else { Some(s.into()) }))
        .twitch(user_hashmap.get("45").and_then(|s| if s.is_empty() { None } else { Some(s.into()) }))
        .diamonds(user_hashmap.get("46").unwrap().parse()?)
        .acc_explosion(user_hashmap.get("48").and_then(|s| s.parse().ok()))
        .moderator(user_hashmap.get("49").map(|s| ModeratorType::new(s.parse().unwrap()).unwrap()))
        .comment_history_state(user_hashmap.get("50").map(|s| MessageState::new(s.parse().unwrap()).unwrap()))
        .color_3(user_hashmap.get("51").unwrap().parse()?)
        .moons(user_hashmap.get("52").unwrap().parse()?)
        .acc_swing(user_hashmap.get("53").and_then(|s| s.parse().ok()))
        .acc_jetpack(user_hashmap.get("54").and_then(|s| s.parse().ok()))
        .demons(user_hashmap.get("55").map(|s| s.split(',').map(|s| s.parse().unwrap()).collect()))
        .classic_levels(user_hashmap.get("56").map(|s| s.split(',').map(|s| s.parse().unwrap()).collect()))
        .platformer_levels(user_hashmap.get("57").map(|s| s.split(',').map(|s| s.parse().unwrap()).collect()))
        .discord(user_hashmap.get("58").and_then(|s| if s.is_empty() { None } else { Some(s.into()) }))
        .instagram(user_hashmap.get("59").and_then(|s| if s.is_empty() { None } else { Some(s.into()) }))
        .tiktok(user_hashmap.get("60").and_then(|s| if s.is_empty() { None } else { Some(s.into()) }))
        .custom(user_hashmap.get("61").and_then(|s| if s.is_empty() { None } else { Some(s.into()) }))
        .group_state(user_hashmap.get("62").map(|s| GroupState::new(s.parse().unwrap()).unwrap()))
        .g_o(user_hashmap.get("63").and_then(|s| s.parse().ok()))
        .friends_following_since(user_hashmap.get("64").and_then(|s| s.parse().ok()))
        .followers(user_hashmap.get("67").and_then(|s| s.parse().ok()))
        .following(user_hashmap.get("68").and_then(|s| s.parse().ok()))
        .is_following(user_hashmap.get("69").and_then(|s| s.parse().ok()))
        .build()?
    )
}

pub async fn get_user_object(query: GDQuery) -> Result<String, Report> {
    let user_result: HashMap<String, String>;

    let url = format!("{URL_DATABASE}/getGJUserInfo20.php");
    let form: Vec<(&str, &str)> = match query.query_type {
        GDQueryType::ID => vec![("secret", SECRET_COMMON), ("targetAccountID", &query.query)],
        GDQueryType::Name => {
            let url = format!("{URL_DATABASE}/getGJUsers20.php");
            let form: Vec<(&str, &str)> = vec![("secret", SECRET_COMMON), ("str", &query.query)];
            
            let search_results = post_request(url, form).await?;
            
            user_result = hashmap_from(search_results.split('#').next().ok_or_else(|| GDUtilsError::GenericError(String::from("The results are empty!")))?)?;

            vec![("secret", SECRET_COMMON), ("targetAccountID", user_result.get("16").ok_or_else(|| GDUtilsError::GenericError(String::from("User key 16 not found!")))?)]
        }
    };

    post_request(url, form).await.map_err(std::convert::Into::into)
}

pub async fn search_user(query: GDQuery) -> Result<String, Report> {
    let url = format!("{URL_DATABASE}/getGJUsers20.php");
    let form: Vec<(&str, &str)> =  vec![("secret", SECRET_COMMON), ("str", &query.query)];

    post_request(url, form).await.map_err(std::convert::Into::<Report>::into)
}

pub async fn get_user_object_hashmap(query: GDQuery) -> Result<HashMap<String, String>, Report> {
    let user_object = get_user_object(query).await;

    hashmap_from(&user_object?)
}

pub async fn search_user_object_hashmap(query: GDQuery) -> Result<HashMap<String, String>, Report> {
    let user_object = search_user(query).await?;

    hashmap_from(&user_object)
}

pub async fn get_user_info(query: GDQuery) -> Result<User, Report> {
    let user_hashmap = get_user_object_hashmap(query).await?;

    build_user(&user_hashmap)
}

pub async fn get_user(query: GDQuery) -> Result<User, Report> {
    let user_hashmap = search_user_object_hashmap(query).await?;

    build_user(&user_hashmap)
}