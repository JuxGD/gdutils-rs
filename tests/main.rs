use gdutils::{aredl::{classic::get_aredl_level_info_by_position, users::get_aredl_user_info_by_name}, gd::{levels::{download_and_build_level, get_timely_info, search_and_build_level}, users::get_user, utils::{GDQuery, GDUtilsError, TimelyType}}};
use dotenvy::dotenv;

#[tokio::test]
async fn main() -> Result<(), eyre::Report> {
    dotenv()?;

    let aredl_api_token = std::env::var("AREDL_API_TOKEN")?; 

    let daily_info = get_timely_info(TimelyType::Daily).await?;
    
    println!("doing daily level now");
    let daily_level = download_and_build_level(GDQuery::new("-1")).await?;

    download_and_build_level(GDQuery::new("retray")).await?;
    
    let grief = download_and_build_level(GDQuery::new("grief")).await?;

    let auto_play_area = download_and_build_level(GDQuery::name("destructive auto")).await?;

    auto_play_area.calculate_quality();
    auto_play_area.calculate_difficulty()?;

    grief.calculate_difficulty()?;
    grief.calculate_quality();

    if daily_level.daily_number.unwrap() != daily_info.timely_index { Err(GDUtilsError::GenericError(String::from("Error: You are dumb")))? } else { println!("all good") }

    search_and_build_level(GDQuery::name("stereo madness copy"), 2).await?;

    get_user(GDQuery::name("idoweirdstuff")).await?;

    get_aredl_user_info_by_name("JuxGD").await?;

    println!("{:?}", get_aredl_level_info_by_position(1, &aredl_api_token).await?);

    search_and_build_level(GDQuery::name("retray"), 5).await?;

    search_and_build_level(GDQuery::id(&128), 0).await?;
    
    if search_and_build_level(GDQuery::id(&128), 1).await.is_err() { println!("all good!") };

    Ok(())
}