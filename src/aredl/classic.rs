use crate::{aredl::{constants::URL_AREDL}, gd::levels::{LevelInfo, get_level_info}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AREDLLevelInfo {
    pub id: String
,   pub name: String
,   pub position: i32
,   pub publisher_id: String
,   pub points: i32
,   pub legacy: bool
,   pub level_id: i32
,   pub two_player: bool
,   pub tags: Option<Vec<String>>
,   pub description: Option<String>
,   pub song: Option<i32>
,   pub edel_enjoyment: Option<f32>
,   pub is_edel_pending: bool
,   pub gddl_tier: Option<f32>
,   pub nlw_tier: Option<String>
}

pub async fn get_aredl_list() -> Vec<AREDLLevelInfo> {
    let client = reqwest::Client::new();
    let response = client.get(format!("{}/levels", URL_AREDL))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let list: Vec<AREDLLevelInfo> = serde_json::from_str(&response).expect("Error deserializing request response");
    return list;
}

pub async fn get_aredl_level_info(position: usize) -> AREDLLevelInfo {
    let list = get_aredl_list().await;
    return list[position - 1].clone()
}

pub async fn get_aredl_gd_info(position: usize) -> LevelInfo {
    let list = get_aredl_list().await;

    return get_level_info(&String::from(list[position - 1].level_id.to_string())).await;
}