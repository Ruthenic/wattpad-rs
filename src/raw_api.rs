use anyhow::Result;
use reqwest::{Client, Url};
use serde_json::Value;

pub async fn get(
    path: String,
    params: Vec<(&str, &str)>,
    use_api: bool,
    client: &Client,
) -> Result<Value> {
    //FIXME: these should be user configurable incase someone reimplements the wattpad backend and hosts it themselves for some ungodly reason (definitely not me :trollface:)
    let url = if use_api {
        Url::parse_with_params(&format!("https://api.wattpad.com{}", path), params.iter())?
    } else {
        Url::parse_with_params(&format!("https://www.wattpad.com{}", path), params.iter())?
    };
    let resp = client.get(url).send().await?;
    Ok(resp.json::<Value>().await?)
}

pub async fn get_text(
    path: String,
    params: Vec<(&str, &str)>,
    use_api: bool,
    client: &Client,
) -> Result<String> {
    //FIXME: these should be user configurable incase someone reimplements the wattpad backend and hosts it themselves for some ungodly reason (definitely not me :trollface:)
    let url = if use_api {
        Url::parse_with_params(&format!("https://api.wattpad.com{}", path), params.iter())?
    } else {
        Url::parse_with_params(&format!("https://www.wattpad.com{}", path), params.iter())?
    };
    let resp = client.get(url).send().await?;

    Ok(resp.text().await?)
}
