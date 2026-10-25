use gdutils::{aredl::{classic::get_aredl_gd_info, users::get_aredl_user_info_by_name}, gd::{levels::{download_and_build_level, get_timely_info, search_and_build_level}, users::get_user, utils::{GDQuery, GDUtilsError, TimelyType}}};

#[tokio::test]
async fn main() -> Result<(), eyre::Report> {
    let daily_info = get_timely_info(TimelyType::Daily).await?;
    
    println!("doing daily level now");
    let daily_level = download_and_build_level(GDQuery::new("-1")).await?;

    download_and_build_level(GDQuery::new("retray")).await?;
    
    let grief = download_and_build_level(GDQuery::new("grief")).await?;

    let auto_play_area = download_and_build_level(GDQuery::new("destructive auto")).await?;

    auto_play_area.calculate_quality();
    auto_play_area.calculate_difficulty()?;

    grief.calculate_difficulty()?;
    grief.calculate_quality();

    if daily_level.daily_number.unwrap() != daily_info.timely_index { Err(GDUtilsError::GenericError(String::from("Error: You are dumb")))? } else { println!("all good") }

    search_and_build_level(GDQuery::name("stereo madness copy"), 2).await?;

    get_user(GDQuery::name("idoweirdstuff")).await?;

    get_aredl_user_info_by_name("JuxGD").await?;

    Ok(())
}