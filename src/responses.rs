// FIXME: we can likely deduplicate a lot of these structs, assuming we structure our queries correctly

use crate::rawAPI::get;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum Copyright {
    ALL_RIGHTS_RESERVED = 1,
    PUBLIC_DOMAIN = 2,
    CC_BY = 3,
    CC_BY_NC = 4,
    CC_BY_NC_ND = 5,
    CC_BY_NC_SA = 6,
    CC_BY_SA = 7,
    CC_BY_ND = 8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Story {
    pub id: String,
    pub title: String,
    pub length: i64,
    pub create_date: String,
    pub modify_date: String,
    pub vote_count: i64,
    pub read_count: i64,
    pub comment_count: i64,
    pub language: Language,
    pub _user: FakeUser,
    pub description: String,
    pub cover: String, // FIXME: is this a URL?
    pub completed: bool,
    pub categories: Vec<i64>, // FIXME: type these with enum
    pub tags: Vec<String>,
    pub rating: i64, // FIXME: figure out what the numbers mean MASON WHAT DO THEY MEAN
    pub copyright: i64,
    pub url: String,
    pub num_parts: i64,
    pub last_published_part: LastPublishedPart, // FIXME: see top: this can definitely be replaced with a normal Part
    pub parts: Vec<Part>,
    pub deleted: bool,
    pub tag_rankings: Vec<TagRanking>,
    #[serde(rename = "highlight_colour")]
    pub highlight_color: String,
    pub promoted: bool,
    pub is_ad_exempt: bool,
    #[serde(rename = "story_text_url")]
    pub story_text_url: TextURL,
    pub is_paywalled: bool,
    pub paid_model: String,
    #[serde(skip_deserializing)]
    client: Client,
}

impl Story {
    pub fn from_json_value(val: Value, client: Client) -> Result<Story> {
        let mut story = serde_json::from_value::<Story>(val)?;
        story.client = client;
        Ok(story)
    }
}

impl Story {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub create_date: String,
    pub modify_date: String,
    pub length: i64,
    pub video_id: String,
    pub photo_url: String,
    pub comment_count: i64,
    pub vote_count: i64,
    pub read_count: i64,
    pub word_count: i64,
    #[serde(rename = "text_url")]
    pub text_url: TextURL,
    pub deleted: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "name")]
    pub display_name: String,
    pub username: String,
    pub avatar: String,
    pub is_private: bool,
    pub background_url: String,
    pub follower: bool,
    pub following: bool,
    // ????????????????????????
    pub follower_request: String,
    pub following_request: String,
    pub safety: Safety,
    pub description: String,
    pub gender_code: String,
    pub language: i64,
    pub create_date: String,
    pub location: String,
    pub verified: bool,
    pub ambassador: bool,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub votes_recieved: i64,
    pub num_stories_published: i64,
    pub num_following: i64,
    pub num_followers: i64,
    pub num_lists: i64,
    pub verified_email: bool,
    #[serde(rename = "is_staff")]
    pub is_staff: bool,
    #[serde(rename = "highlight_colour")]
    pub highlight_color: String,
    pub programs: Programs,
    pub external_id: String,
    pub show_social_network: String,
    #[serde(skip_deserializing)]
    pub client: Option<Client>,
}

#[derive(Deserialize, Debug)]
pub struct FakeUser {
    pub avatar: String,
    pub fullname: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Language {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LastPublishedPart {
    pub create_date: String,
}

#[derive(Deserialize, Debug)]
pub struct TextURL {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct TagRanking {
    pub name: String,
    pub rank: i64,
    pub total: i64,
}

#[derive(Deserialize, Debug)]
pub struct Programs {
    pub wattpad_starts: bool,
    pub wattpad_circle: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Safety {
    pub is_muted: bool,
    pub is_blocked: bool,
}
