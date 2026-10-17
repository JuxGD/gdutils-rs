use crate::{aredl::utils::{AREDLUser, AREDLUserSearchResults, URL_USERS}, gd::utils::get_request};
use eyre::Report;
use reqwest::header::{ACCEPT, HeaderMap};

pub async fn get_aredl_user_info(id: &str) -> Result<AREDLUser, Report> {
    let url = format!("{URL_USERS}/{id}");
    let query: Vec<(&str, &str)> = vec![];
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);

    let response = get_request(url, query, headers).await?;

    Ok(serde_json::from_str(&response)?)
}

pub async fn search_aredl_users(name: &str) -> Result<AREDLUserSearchResults, Report> {
    let url = String::from(URL_USERS);
    let query: Vec<(&str, &str)> = vec![("name_filter", name)];
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);

    let response = get_request(url, query, headers).await?;

    Ok(serde_json::from_str(&response)?)
}

pub async fn get_aredl_user_info_by_name(name: &str) -> Result<AREDLUser, Report> {
    let users = search_aredl_users(name).await?;

    let user = &users.data[0];

    get_aredl_user_info(&user.id).await
}