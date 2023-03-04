mod rawAPI;
mod responses;

use rawAPI::get;

use anyhow::{Context, Result};
use regex::Regex;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, Url,
};
use responses::{Part, Story, User};

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
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .context("Failed to build client")?;
        Ok(Wattpad { client })
    }

    pub async fn get_story(self, id: &str) -> Result<Story> {
        let res = get(
            format!("/api/v3/stories/{}", id), vec![
                ("drafts", "0"),
                ("mature", "1"),
                ("include_deleted", "1"),
                ("fields", "id,title,length,createDate,modifyDate,voteCount,readCount,commentCount,url,promoted,sponsor,language,user,description,cover,highlight_colour,completed,isPaywalled,paidModel,categories,numParts,readingPosition,deleted,dateAdded,lastPublishedPart(createDate),tags,copyright,rating,story_text_url(text),,parts(id,title,voteCount,commentCount,videoId,readCount,photoUrl,createDate,modifyDate,length,voted,deleted,text_url(text),dedication,url,wordCount),isAdExempt,tagRankings")
            ],
            false,
            &self.client,
        )
        .await?;

        Story::from_json_value(res, self.client)
    }

    pub async fn get_author_from_story(self, story: Story) -> Result<User> {
        let res = get(format!("/api/v3/users/{}", story._user.fullname), vec![("fields", "username,description,avatar,name,email,genderCode,language,birthdate,verified,isPrivate,ambassador,is_staff,follower,following,backgroundUrl,votesReceived,numFollowing,numFollowers,createDate,followerRequest,website,facebook,twitter,followingRequest,numStoriesPublished,numLists,location,externalId,programs,showSocialNetwork,verified_email,has_accepted_latest_tos,email_reverification_status,highlight_colour,safety(isMuted,isBlocked),has_writer_subscription")], false, &self.client).await?;

        Ok(serde_json::from_value::<User>(res)?)
    }
}
