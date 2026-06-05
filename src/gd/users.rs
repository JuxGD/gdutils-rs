use colonparse::hashmap_from;
use std::collections::HashMap;
use eyre::Error;

use crate::gd::utils::*;

pub fn build_user<S: std::hash::BuildHasher>(user_hashmap: &HashMap<String, String, S>) -> Result<User, Error> {
    Ok(UserBuilder::default()
        .username(String::from(&user_hashmap["58"]))
        .user_id(user_hashmap["2"].parse()?)
        .stars(user_hashmap["3"].parse()?)
        .demon_count(user_hashmap["4"].parse()?)
        .ranking(Some(user_hashmap["6"].parse()?))
        .account_highlight(user_hashmap["7"].parse()?)
        .creator_points(user_hashmap["8"].parse()?)
        .icon_id(Some(user_hashmap["9"].parse()?))
        .color(user_hashmap["10"].parse()?)
        .color_2(user_hashmap["11"].parse()?)
        .ship_id(Some(user_hashmap["12"].parse()?))
        .secret_coins(user_hashmap["13"].parse()?)
        .icon_type(Some(user_hashmap["14"].parse()?))
        .special(Some(user_hashmap["15"].parse()?))
        .account_id(user_hashmap["16"].parse()?)
        .user_coins(user_hashmap["17"].parse()?)
        .message_state(Some(user_hashmap["18"].parse()?))
        .friends_state(Some(user_hashmap["19"].parse()?))
        .youtube(Some(String::from(&user_hashmap["20"])))
        .acc_icon(Some(user_hashmap["21"].parse()?))
        .acc_ship(Some(user_hashmap["22"].parse()?))
        .acc_ball(Some(user_hashmap["23"].parse()?))
        .acc_bird(Some(user_hashmap["24"].parse()?))
        .acc_dart(Some(user_hashmap["25"].parse()?))
        .acc_robot(Some(user_hashmap["26"].parse()?))
        .acc_streak(Some(user_hashmap["27"].parse()?))
        .acc_glow(Some(user_hashmap["28"].parse()?))
        .is_registered(Some(user_hashmap["29"].parse()?))
        .global_rank(Some(user_hashmap["30"].parse()?))
        .friend_state(Some(user_hashmap["31"].parse()?))
        .friend_request_id(Some(user_hashmap["32"].parse()?))
        .friend_request_message(Some(user_hashmap["35"].parse()?))
        .friend_request_age(Some(user_hashmap["37"].parse()?))
        .messages(Some(user_hashmap["38"].parse()?))
        .friend_requests(Some(user_hashmap["39"].parse()?))
        .new_friends(Some(user_hashmap["40"].parse()?))
        .new_friend_request(Some(user_hashmap["41"].parse()?))
        .age(Some(user_hashmap["42"].parse()?))
        .acc_spider(Some(user_hashmap["43"].parse()?))
        .twitter(Some(user_hashmap["44"].parse()?))
        .twitch(Some(user_hashmap["45"].parse()?))
        .diamonds(user_hashmap["46"].parse()?)
        .acc_explosion(Some(user_hashmap["48"].parse()?))
        .moderator(Some(user_hashmap["49"].parse()?))
        .comment_history_state(Some(user_hashmap["50"].parse()?))
        .color_3(user_hashmap["51"].parse()?)
        .moons(user_hashmap["52"].parse()?)
        .acc_swing(Some(user_hashmap["53"].parse()?))
        .acc_jetpack(Some(user_hashmap["54"].parse()?))
        .demons(Some(user_hashmap["55"].split(',').map(String::from).collect()))
        .classic_levels(Some(user_hashmap["56"].split(',').map(String::from).collect()))
        .platformer_levels(Some(user_hashmap["57"].split(',').map(String::from).collect()))
        .discord(Some(String::from(&user_hashmap["58"])))
        .instagram(Some(String::from(&user_hashmap["59"])))
        .tiktok(Some(String::from(&user_hashmap["60"])))
        .custom(Some(String::from(&user_hashmap["61"])))
        .build()?)
}

pub async fn get_user_object(query: GDQuery) -> Result<String, Error> {
    let url = format!("{URL_DATABASE}/getGJUserInfo20.php");
    let form: Vec<(&str, &str)> = match query.query_type {
        GDQueryType::ID => vec![("secret", SECRET_COMMON), ("targetAccountID", &query.query)],
        GDQueryType::Name => vec![("secret", SECRET_COMMON), ("str", &query.query)]
    };

    post_request(url, form).await
}

pub async fn get_user_object_hashmap(query: GDQuery) -> Result<HashMap<String, String>, Error> {
    let user_object = get_user_object(query).await;

    Ok(hashmap_from(user_object?))
}

pub async fn get_user(query: GDQuery) -> Result<User, Error> {
    let user_hashmap = get_user_object_hashmap(query).await?;

    build_user(&user_hashmap)
}