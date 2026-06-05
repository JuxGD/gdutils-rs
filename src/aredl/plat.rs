use crate::{aredl::utils::{AREDLLevelInfo, URL_AREPL}, gd::{levels::*, utils::{Level, post_request_no_form}}};
use eyre::Error;

pub async fn get_aredl_list() -> Result<Vec<AREDLLevelInfo>, Error> {
    let url = format!("{URL_AREPL}/levels");

    let response = post_request_no_form(url).await?;

    Ok(serde_json::from_str(&response)?)
}

pub async fn get_aredl_level_info_by_position(position: usize) -> Result<AREDLLevelInfo, Error> {    
    let list = get_aredl_list().await?;
    let url = format!("{URL_AREPL}/levels/{}", list[position - 1].id);

    let response = post_request_no_form(url).await?;

    Ok(serde_json::from_str(&response)?)
}

pub async fn get_aredl_gd_info(position: usize) -> Result<Level, Error> {
    let list = get_aredl_list().await?;

    build_level(&level_object_to_hashmap(download_level_by_id(&list[position - 1].level_id.to_string()).await?)?)
}