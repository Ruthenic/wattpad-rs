mod raw_api;
mod responses;

use raw_api::get;

use anyhow::{Context, Result};
use regex::Regex;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    redirect::Policy,
    Client, Proxy, Url,
};
use responses::{Part, Search, Story, User};
pub use responses::{SearchSort, SearchType};

pub struct Wattpad {
    client: Client,
}

impl Wattpad {
    async fn get_auth_token() -> Result<String> {
        // we can just use a temporary client here, no special handling needed
        let tmp_client = Client::new();

        // FIXME: should be configurable, see comment in rawAPI
        let res = tmp_client.get("https://wattpad.com").send().await?;
        let res = res.text().await?;

        // unwrapping this should be safe (unless our regex somehow breaks)
        let regex = Regex::new("wattpad\\.apiAuthKey = ('|\")(.*)('|\")").unwrap();

        Ok(regex
            .captures(res.as_str())
            .context("Failed to get captures for Wattpad API token")?
            .get(2)
            .context("Failed to get first capture for Wattpad API token")?
            .as_str()
            .to_string())
    }

    pub async fn new() -> Result<Wattpad> {
        // FIXME: these should be customizable question mark?
        let mut headers = HeaderMap::new();

        headers.insert(
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_static("en-US,en;q=0.5"),
        );

        let auth_token = Wattpad::get_auth_token().await?;
        let auth_token = auth_token.as_str();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(auth_token)
                .context("Failed to construct authorization header")?,
        );

        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:108.0) Gecko/20100101 Firefox/108.0")
            .cookie_store(true)
            .default_headers(headers)
            .redirect(Policy::limited(10))
            .build()
            .context("Failed to build client")?;

        Ok(Wattpad { client })
    }

    pub async fn get_story(&self, id: &str) -> Result<Story> {
        Story::from_id(id.to_string(), &self.client).await
    }

    pub async fn search(
        &self,
        query: &str,
        search_type: SearchType,
        search_sort: SearchSort,
        limit: i64,
    ) -> Result<Search> {
        Ok(Search {
            query: query.to_string(),
            search_type,
            search_sort,
            limit,
            client: &self.client,
        })
    }
}
