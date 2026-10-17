use colonparse::{hashmap_from, tuples_vector_from};
use std::collections::HashMap;
use eyre::Report;

use crate::gd::utils::{User, UserBuilder, GDQuery, URL_DATABASE, GDQueryType, SECRET_COMMON, post_request, GDUtilsError};

pub fn build_user<S: std::hash::BuildHasher>(user_hashmap: &HashMap<String, String, S>) -> Result<User, Report> {
    Ok(UserBuilder::default()
        .username(String::from(user_hashmap.get("1").unwrap()))
        .user_id(user_hashmap.get("2").unwrap().parse()?)
        .stars(user_hashmap.get("3").unwrap().parse()?)
        .demon_count(user_hashmap.get("4").unwrap().parse()?)
        .ranking(Some(user_hashmap.get("6").unwrap_or(&String::from("0")).parse()?))
        .account_highlight(Some(user_hashmap.get("7").unwrap_or(&String::from("0")).to_owned()))
        .creator_points(user_hashmap.get("8").unwrap().parse()?)
        .icon_id(Some(user_hashmap.get("9").unwrap_or(&String::from("0")).parse()?))
        .color(user_hashmap.get("10").unwrap().parse()?)
        .color_2(user_hashmap.get("11").unwrap().parse()?)
        .ship_id(Some(user_hashmap.get("12").unwrap_or(&String::from("0")).parse()?))
        .secret_coins(user_hashmap.get("13").unwrap().parse()?)
        .icon_type(Some(user_hashmap.get("14").unwrap_or(&String::from("0")).parse()?))
        .special(Some(user_hashmap.get("15").unwrap_or(&String::from("0")).parse()?))
        .account_id(user_hashmap.get("16").unwrap().parse()?)
        .user_coins(user_hashmap.get("17").unwrap().parse()?)
        .message_state(Some(user_hashmap.get("18").unwrap_or(&String::from("0")).parse()?))
        .friends_state(Some(user_hashmap.get("19").unwrap_or(&String::from("0")).parse()?))
        .youtube(Some(String::from(user_hashmap.get("20").unwrap_or(&String::new()))))
        .acc_icon(Some(user_hashmap.get("21").unwrap_or(&String::from("0")).parse()?))
        .acc_ship(Some(user_hashmap.get("22").unwrap_or(&String::from("0")).parse()?))
        .acc_ball(Some(user_hashmap.get("23").unwrap_or(&String::from("0")).parse()?))
        .acc_bird(Some(user_hashmap.get("24").unwrap_or(&String::from("0")).parse()?))
        .acc_dart(Some(user_hashmap.get("25").unwrap_or(&String::from("0")).parse()?))
        .acc_robot(Some(user_hashmap.get("26").unwrap_or(&String::from("0")).parse()?))
        .acc_streak(Some(user_hashmap.get("27").unwrap_or(&String::from("0")).parse()?))
        .acc_glow(Some(user_hashmap.get("28").is_some_and(std::string::String::is_empty)))
        .is_registered(Some(user_hashmap.get("29").is_some_and(std::string::String::is_empty)))
        .global_rank(Some(user_hashmap.get("30").unwrap_or(&String::from("0")).parse()?))
        .friend_state(Some(user_hashmap.get("31").unwrap_or(&String::from("0")).parse()?))
        .friend_request_id(Some(user_hashmap.get("32").unwrap_or(&String::from("0")).parse()?))
        .friend_request_message(Some(user_hashmap.get("35").unwrap_or(&String::from("0")).parse()?))
        .friend_request_age(Some(user_hashmap.get("37").unwrap_or(&String::from("0")).parse()?))
        .messages(Some(user_hashmap.get("38").unwrap_or(&String::from("0")).parse()?))
        .friend_requests(Some(user_hashmap.get("39").unwrap_or(&String::from("0")).parse()?))
        .new_friends(Some(user_hashmap.get("40").unwrap_or(&String::from("0")).parse()?))
        .new_friend_request(Some(user_hashmap.get("41").is_some_and(std::string::String::is_empty)))
        .age(Some(user_hashmap.get("42").unwrap_or(&String::from("0")).parse()?))
        .acc_spider(Some(user_hashmap.get("43").unwrap_or(&String::from("0")).parse()?))
        .twitter(Some(user_hashmap.get("44").unwrap_or(&String::from("0")).parse()?))
        .twitch(Some(user_hashmap.get("45").unwrap_or(&String::from("0")).parse()?))
        .diamonds(user_hashmap.get("46").unwrap().parse()?)
        .acc_explosion(Some(user_hashmap.get("48").unwrap_or(&String::from("0")).parse()?))
        .moderator(Some(user_hashmap.get("49").unwrap_or(&String::from("0")).parse()?))
        .comment_history_state(Some(user_hashmap.get("50").unwrap_or(&String::from("0")).parse()?))
        .color_3(user_hashmap.get("51").unwrap().parse()?)
        .moons(user_hashmap.get("52").unwrap().parse()?)
        .acc_swing(Some(user_hashmap.get("53").unwrap_or(&String::from("0")).parse()?))
        .acc_jetpack(Some(user_hashmap.get("54").unwrap_or(&String::from("0")).parse()?))
        .demons(Some(user_hashmap.get("55").unwrap_or(&String::from("0")).split(',').map(String::from).collect()))
        .classic_levels(Some(user_hashmap.get("56").unwrap_or(&String::from("0")).split(',').map(String::from).collect()))
        .platformer_levels(Some(user_hashmap.get("57").unwrap_or(&String::from("0")).split(',').map(String::from).collect()))
        .discord(Some(String::from(user_hashmap.get("58").unwrap_or(&String::from("0")))))
        .instagram(Some(String::from(user_hashmap.get("59").unwrap_or(&String::from("0")))))
        .tiktok(Some(String::from(user_hashmap.get("60").unwrap_or(&String::from("0")))))
        .custom(Some(String::from(user_hashmap.get("61").unwrap_or(&String::from("0")))))
        .build()?)
}

pub async fn get_user_object(query: GDQuery) -> Result<String, Report> {
    let user_result: Vec<(String, String)>; // don't mind this

    let url = format!("{URL_DATABASE}/getGJUserInfo20.php");
    let form: Vec<(&str, &str)> = match query.query_type {
        GDQueryType::ID => vec![("secret", SECRET_COMMON), ("targetAccountID", &query.query)],
        GDQueryType::Name => {
            let url = format!("{URL_DATABASE}/getGJUsers20.php");
            let form: Vec<(&str, &str)> = vec![("secret", SECRET_COMMON), ("str", &query.query)];
            
            let search_results = post_request(url, form).await?;
            
            user_result = tuples_vector_from(&search_results)?;

            vec![("secret", SECRET_COMMON), ("targetAccountID", &user_result[11].1)]
        },
        GDQueryType::Unknown => return Err(GDUtilsError::UnknownGDQueryTypeError().into())
    };

    post_request(url, form).await
}

pub async fn get_user_object_hashmap(query: GDQuery) -> Result<HashMap<String, String>, Report> {
    let user_object = get_user_object(query).await;

    hashmap_from(&user_object?)
}

pub async fn get_user(query: GDQuery) -> Result<User, Report> {
    let user_hashmap = get_user_object_hashmap(query).await?;

    build_user(&user_hashmap)
}