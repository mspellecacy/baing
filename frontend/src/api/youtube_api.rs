use std::error;
use gloo::console::{console, console_dbg, debug};
use gloo::net::http::Method;
use reqwest::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, REFERRER_POLICY};
use serde_json::json;
use wasm_bindgen::JsValue;
use web_sys::RequestMode;
use yewdux::log;
use common::model::core::{YTChannel};
// POST https://www.youtube.com/youtubei/v1/browse?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8
// Content-Type: application/json
//
// {
// "context": {
// "client": {
// "clientName": "WEB",
// "clientVersion": "2.20201210.01.00",
// "originalUrl": "https://www.youtube.com/",
// "platform": "DESKTOP",
// "clientFormFactor": "UNKNOWN_FORM_FACTOR",
// "newVisitorCookie": true
// }
// },
// "browseId": "UC4PooiX37Pld1T8J5SYT-SQ",
// "params": "EgZ2aWRlb3M%3D"
// }

pub async fn api_yt_channel_details(
    yt_channel: &mut YTChannel,
) -> Result<YTChannel, Box<dyn error::Error>> {
    console!("MOUTH SHITS MOUTH SHITS".to_string());
    let endpoint = "https://www.youtube.com/youtubei/v1/browse";
    let channel_id = yt_channel.channel_id.as_str();
    let body = json!({
          "context": {
            "client": {
              "clientName": "WEB",
              "clientVersion": "2.20201210.01.00",
              "originalUrl": "http://youtube.com",
              "platform": "DESKTOP",
              "clientFormFactor": "UNKNOWN_FORM_FACTOR",
              "newVisitorCookie": true
            }
          },
          "browseId": format!("{}", channel_id),
          "params": "EgZ2aWRlb3M%3D"
        });

    console!("I shit in your mouth".to_string());

    //headers.append("Content-Type", "application/json");
    //headers.append("Access-Control-Request-Headers", "content-type");




    // let res = reqwasm::http::Request::post(endpoint)
    //     .mode(RequestMode::NoCors)
    //     .headers(headers)
    //     .body(body);

    let client = reqwest::Client::new();
    let res = client
        .post(endpoint)
        .body(body.to_string())
        .fetch_mode_no_cors()
        ;

    let res = match res.build() {
        Ok(r) => {
            r
        }
        Err(e) => {
            console!(format!("{e:?}"));
            panic!("Farts?");
        }
    };

    console!(format!("{res:?}"));

    match client.execute(res).await {
        Ok(r) => {
            let farts = r.json::<serde_json::Value>().await.unwrap();
            console!(format!("{farts:?}"));
        }
        Err(e) => {
            console!(format!("{e:?}"));
        }
    }

    // let res_body: serde_json::Value = res.json::<serde_json::Value>().await?;
    //
    // let channel_name: serde_json::Value = res_body["header"]["c4TabbedHeaderRenderer"]["title"].clone();
    //
    // debug!("Channel Name - {:?}", channel_name.as_str());

    Ok(yt_channel.to_owned())
}