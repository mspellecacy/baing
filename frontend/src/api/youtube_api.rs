use std::error;
use common::model::core::{YTChannel};

const API_BASE: &str = "https://api.themoviedb.org/3";

pub async fn api_yt_channel_details(
    key: &str,
    yt_channel: &mut YTChannel,
) -> Result<YTChannel, Box<dyn error::Error>> {

    todo!()
}