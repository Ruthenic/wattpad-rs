// FIXME: we can likely deduplicate a lot of these structs, assuming we structure our queries correctly

use crate::raw_api::{get, get_text};

use anyhow::{Context, Result};
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use serde_repr::Deserialize_repr;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum Copyright {
    ALL_RIGHTS_RESERVED = 1,
    PUBLIC_DOMAIN = 2,
    CC_BY = 3,
    CC_BY_NC = 4,
    CC_BY_NC_ND = 5,
    CC_BY_NC_SA = 6,
    CC_BY_SA = 7,
    CC_BY_ND = 8,
}

impl fmt::Display for Copyright {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Copyright::ALL_RIGHTS_RESERVED => "All Rights Reserved",
            Copyright::PUBLIC_DOMAIN => "Public Domain",
            Copyright::CC_BY => "CC-BY",
            Copyright::CC_BY_NC => "CC-BY-NC",
            Copyright::CC_BY_NC_ND => "CC-BY-NC-ND",
            Copyright::CC_BY_NC_SA => "CC-BY-NC-SA",
            Copyright::CC_BY_SA => "CC-BY-SA",
            Copyright::CC_BY_ND => "CC-BY-ND",
        };

        write!(f, "{}", val)
    }
}

#[derive(Clone, Debug, Deserialize_repr, PartialEq)]
#[repr(i8)]
pub enum Category {
    None = -1,
    UnknownValue = 0,
    TeenFiction = 1,
    Poetry = 2,
    Fantasy = 3,
    Romance = 4,
    ScienceFiction = 5,
    Fanfiction = 6,
    Humor = 7,
    MysteryOrThriller = 8, // FIXME: dumbass name
    Horror = 9,
    // There is no 10
    Adventure = 11,
    Paranormal = 12,
    Spiritual = 13,
    Action = 14,
    // There is also no 15
    NonFiction = 16,
    ShortStory = 17,
    Vampire = 18,
    Random = 19, // ??????
    // There is also also no 20
    GeneralFiction = 21,
    Werewolf = 22,
    HistoricalFiction = 23,
    ChickLit = 24, // WHAT THE FUCK IS CHICKLIT
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Category::None => "None",
            Category::UnknownValue => "Unknown",
            Category::TeenFiction => "Teen Fiction",
            Category::Poetry => "Poetry",
            Category::Fantasy => "Fantasy",
            Category::Romance => "Romance",
            Category::ScienceFiction => "Science Fiction",
            Category::Fanfiction => "Fanfiction",
            Category::Humor => "Humor",
            Category::MysteryOrThriller => "Mystery / Thriller",
            Category::Horror => "Horror",
            Category::Adventure => "Adventure",
            Category::Paranormal => "Paranormal",
            Category::Spiritual => "Spiritual",
            Category::Action => "Action",
            Category::NonFiction => "Non-Fiction",
            Category::ShortStory => "Short Story",
            Category::Vampire => "Vampire",
            Category::Random => "Random",
            Category::GeneralFiction => "General Fiction",
            Category::Werewolf => "Werewolf",
            Category::HistoricalFiction => "Historical Fiction",
            Category::ChickLit => "ChickLit",
        };

        write!(f, "{}", val)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Story {
    /// The ID of the story
    pub id: String,
    /// The title of the story
    pub title: String,
    /// The approximate length of the story in characters
    pub length: i64,
    /// The creation date in ISO format
    pub create_date: String,
    /// The date of the last modification to the story in ISO format
    pub modify_date: String,
    /// The amount of votes on the story
    pub vote_count: i64,
    /// The amount of people who have read the story
    pub read_count: i64,
    /// The total amount of comments on the story
    pub comment_count: i64,
    /// The language of the story
    pub language: Language,
    /// (DO NOT USE UNLESS YOU KNOW WHAT YOU'RE DOING)
    /// The story struct's internal representation of a user
    pub _user: FakeUser,
    /// The description of the story
    pub description: String,
    /// The link to the (image) cover of the story
    pub cover: String,
    /// Whether the story is marked as completed or not
    pub completed: bool,
    /// A list of categories the work falls under (only the first seems to be an actual category?)
    pub categories: Vec<Category>,
    /// A list of tags applied to the story
    pub tags: Vec<String>,
    /// The rating of the story (1 is non-mature, 4 is mature, I don't know if other values even show up)
    pub rating: i64, // FIXME: figure out what the numbers mean MASON WHAT DO THEY MEAN
    /// The licensing of the story
    pub copyright: Copyright,
    /// The hardlink to the story on wattpad.com
    pub url: String,
    /// The amount of parts the story has
    pub num_parts: i64,
    /// (DO NOT USE UNLESS YOU KNOW WHAT YOU'RE DOING)
    /// The story struct's internal list of Parts
    pub _parts: Vec<Part>,
    /// Whether the story is deleted (I guess???)
    pub deleted: bool,
    /// Where the story ranks in popularity on it's tags
    pub tag_rankings: Vec<TagRanking>,
    #[serde(rename = "highlight_colour")]
    /// The highlight color of the story
    pub highlight_color: String,
    /// Whether the story is promoted by Wattpad or not
    pub promoted: bool,
    /// If the story does not show ads
    pub is_ad_exempt: bool,
    #[serde(rename = "story_text_url")]
    /// A struct that shows where the URL to get the text of the story is
    pub story_text_url: TextURL,
    /// Whether the story requires payment to view
    pub is_paywalled: bool,
    /// The type of payment required (ie upfront, subscription I guess?)
    pub paid_model: String,
    #[serde(skip_deserializing)]
    client: Client,
}

impl Story {
    pub async fn from_id(id: String, client: &Client) -> Result<Story> {
        let res = get(
            format!("/api/v3/stories/{}", id), vec![
                ("drafts", "0"),
                ("mature", "1"),
                ("include_deleted", "1"),
                ("fields", "id,title,length,createDate,modifyDate,voteCount,readCount,commentCount,url,promoted,sponsor,language,user,description,cover,highlight_colour,completed,isPaywalled,paidModel,categories,numParts,readingPosition,deleted,dateAdded,lastPublishedPart(createDate),tags,copyright,rating,story_text_url(text),,parts(id,title,voteCount,commentCount,videoId,readCount,photoUrl,createDate,modifyDate,length,voted,deleted,text_url(text),dedication,url,wordCount),isAdExempt,tagRankings")
            ],
            false,
            client,
        )
        .await?;

        Story::from_json_value(res, client)
    }

    pub fn from_json_value(val: Value, client: &Client) -> Result<Story> {
        let mut story = serde_json::from_value::<Story>(val)?;
        story.client = client.clone();
        Ok(story)
    }

    pub async fn get_author(&self) -> Result<User> {
        let res = get(format!("/api/v3/users/{}", self._user.fullname), vec![("fields", "username,description,avatar,name,email,genderCode,language,birthdate,verified,isPrivate,ambassador,is_staff,follower,following,backgroundUrl,votesReceived,numFollowing,numFollowers,createDate,followerRequest,website,facebook,twitter,followingRequest,numStoriesPublished,numLists,location,externalId,programs,showSocialNetwork,verified_email,has_accepted_latest_tos,email_reverification_status,highlight_colour,safety(isMuted,isBlocked),has_writer_subscription")], false, &self.client).await?;

        User::from_json_value(res, &self.client)
    }

    pub async fn get_parts(&self) -> Result<Vec<Part>> {
        let mut new_parts = self._parts.clone();

        for (idx, _) in new_parts.clone().iter().enumerate() {
            new_parts[idx].html = get_text(
                "/apiv2/storytext".to_string(),
                vec![("id", new_parts[idx].id.to_string().as_str())],
                false,
                &self.client,
            )
            .await?
            .to_string();
            new_parts[idx].client = self.client.clone();
        }

        Ok(new_parts)
    }

    pub async fn get_part(&self, index: usize) -> Result<Part> {
        // FIXME: this error handling is probably terrible
        let mut new_part = self
            ._parts
            .get(index)
            .context("Invalid part index?")?
            .clone();

        new_part.html = get_text(
            "/apiv2/storytext".to_string(),
            vec![("id", new_part.id.to_string().as_str())],
            false,
            &self.client,
        )
        .await?
        .to_string();
        new_part.client = self.client.clone();

        Ok(new_part)
    }
}

#[derive(Deserialize, Debug, Clone)]
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
    #[serde(skip_deserializing)]
    client: Client,
    #[serde(skip_deserializing)]
    html: String,
}

impl Part {
    pub async fn from_id(id: String, client: &Client) -> Result<Part> {
        let res = get(
            format!("/api/v3/story_parts/{}", id), vec![
                ("fields", "id,title,voteCount,commentCount,videoId,readCount,photoUrl,createDate,modifyDate,length,voted,deleted,text_url(text),dedication,url,wordCount")
            ],
            false,
            client,
        )
        .await?;

        Part::from_json_value(res, client)
    }

    pub fn from_json_value(val: Value, client: &Client) -> Result<Part> {
        let mut part = serde_json::from_value::<Part>(val)?;
        part.client = client.clone();
        Ok(part)
    }

    pub fn get_paragraphs(&self) -> Result<Vec<Paragraph>> {
        // in theory this should never be able to error
        let regex = Regex::new("<p data-p-id=\"(.{32})\"(|\\s+[^>]*)>(.*?)</p\\s*>").unwrap();

        let mut thing: Vec<Paragraph> = vec![];

        for regex_match in regex.find_iter(self.html.as_str()) {
            let captures = regex
                .captures(regex_match.into())
                .context("Failed to get captures of paragraph")?;
            let attributes = captures
                .get(2)
                .map(|e| e.as_str())
                .context("Failed to get attributes of paragraph")?;
            let para = Paragraph {
                id: captures
                    .get(1)
                    .map(|e| e.as_str().to_string())
                    .context("Failed to get ID of paragraph")?,
                attributes: match attributes {
                    "" => None,
                    _ => Some(attributes.to_string()),
                },
                html: captures
                    .get(3)
                    .map(|e| e.as_str().to_string())
                    .context("Failed to get HTML of paragraph")?,
                client: self.client.clone(),
            };
            thing.push(para)
        }

        Ok(thing)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Paragraph {
    pub id: String,
    pub attributes: Option<String>,
    pub html: String,
    #[serde(skip_deserializing)]
    client: Client,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "name")]
    pub display_name: String,
    pub username: String,
    pub avatar: String,
    pub is_private: bool,
    pub background_url: String,
    pub follower: Option<bool>,
    pub following: Option<bool>,
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
    pub votes_recieved: Option<i64>,
    pub num_stories_published: i64,
    pub num_following: i64,
    pub num_followers: i64,
    pub num_lists: i64,
    pub verified_email: Option<bool>,
    #[serde(rename = "is_staff")]
    pub is_staff: bool,
    #[serde(rename = "highlight_colour")]
    pub highlight_color: String,
    pub programs: Programs,
    pub external_id: String,
    pub show_social_network: bool,
    #[serde(skip_deserializing)]
    pub client: Client,
}

impl User {
    pub fn from_json_value(val: Value, client: &Client) -> Result<User> {
        let mut user = serde_json::from_value::<User>(val)?;
        user.client = client.clone();
        Ok(user)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchType {
    Text,
    Title,
    Tag,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchSort {
    Hot,
    New,
}

// FIXME: we need to support multiple tags (somehow)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Search<'a> {
    pub(crate) query: String,
    pub(crate) search_type: SearchType,
    pub(crate) search_sort: SearchSort,
    pub(crate) limit: i64,
    pub(crate) client: &'a Client,
}

impl Search<'_> {
    pub async fn page(&self, page_number: i64) -> Result<Vec<Story>> {
        let search_result;
        match self.search_type {
            SearchType::Text | SearchType::Title => {
                // a bit o' jank
                let maybe_query = format!("title:{}", self.query);
                let query = if self.search_type == SearchType::Title {
                    maybe_query.as_str()
                } else {
                    self.query.as_str()
                };
                let limit = self.limit.to_string();
                let limit = limit.as_str();
                let offset = (self.limit * page_number).to_string();
                let offset = offset.as_str();

                let res = get(
                    "/v4/stories".to_string(),
                    vec![
                        ("fields", "stories(id)"),
                        ("query", query),
                        (
                            "filter",
                            match self.search_sort {
                                SearchSort::Hot => "hot",
                                SearchSort::New => "new",
                            },
                        ),
                        ("limit", limit),
                        ("offset", offset),
                        ("mature", "1"),
                    ],
                    false,
                    &self.client,
                )
                .await?;

                search_result = SearchResults::from_json_value(res)?;
            }
            SearchType::Tag => {
                let api_path = format!(
                    "/v5/{}list",
                    match self.search_sort {
                        SearchSort::Hot => "hot",
                        SearchSort::New => "new",
                    }
                );

                let limit = self.limit.to_string();
                let limit = limit.as_str();
                let offset = (self.limit * page_number).to_string();
                let offset = offset.as_str();

                let res = get(
                    api_path,
                    vec![
                        ("tags", &self.query),
                        ("offset", offset),
                        ("limit", limit),
                        ("mature", "1"),
                    ],
                    true,
                    &self.client,
                )
                .await?;

                search_result = SearchResults::from_json_value(res)?;
            }
        };

        let mut stories: Vec<Story> = vec![];

        for fake_story in search_result.stories {
            let story = Story::from_id(fake_story.id, &self.client).await?;
            stories.push(story)
        }
        Ok(stories)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct SearchResults {
    stories: Vec<SearchStory>,
}

impl SearchResults {
    pub fn from_json_value(val: Value) -> Result<SearchResults> {
        let results = serde_json::from_value::<SearchResults>(val)?;
        Ok(results)
    }
}

#[derive(Deserialize, Debug, Clone)]
struct SearchStory {
    id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FakeUser {
    pub avatar: String,
    pub fullname: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Language {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LastPublishedPart {
    pub create_date: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextURL {
    pub text: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TagRanking {
    pub name: String,
    pub rank: i64,
    pub total: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Programs {
    pub wattpad_starts: Option<bool>,
    pub wattpad_circle: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Safety {
    pub is_muted: bool,
    pub is_blocked: bool,
}
