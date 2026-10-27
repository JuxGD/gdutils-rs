use crate::{aredl::utils::{AREDLLevelInfo, GDUtilsAREDLError, URL_AREPL}, gd::{levels::{build_level, download_level}, utils::{GDQuery, GDUtilsError, Level, get_request, hashmap_from}}};
use reqwest::header::{ACCEPT, HeaderMap};
use eyre::Report;

pub async fn get_aredl_list(api_key: &str) -> Result<Vec<AREDLLevelInfo>, Report> {
    let url = format!("{URL_AREPL}/levels");
    let form: Vec<(&str, &str)> = vec![];
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "applications/json".parse()?);
    headers.insert("api-key", api_key.parse()?);
    
    let response = get_request(url, form, headers).await?;
    
    Ok(serde_json::from_str(&response)?)
}

pub async fn get_aredl_level_info_by_position(position: usize, api_key: &str) -> Result<AREDLLevelInfo, Report> {    
    let list = get_aredl_list(api_key).await?;

    if position > list.len() {
        return Err(GDUtilsAREDLError::NumberTooHighError().into())
    }

    let level = list.get(position - 1).ok_or(GDUtilsError::GenericError(String::from("Something went wrong getting the requested AREDL position")))?;

    Ok(level.to_owned())
}

pub async fn get_aredl_gd_info(position: usize, api_key: &str) -> Result<Level, Report> {
    let list = get_aredl_list(api_key).await?;

    if position > list.len() {
        return Err(GDUtilsAREDLError::NumberTooHighError().into())
    }

    build_level(&hashmap_from(&String::from(download_level(GDQuery::new(&list[position - 1].level_id.to_string())).await?))?)
}