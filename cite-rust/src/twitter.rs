use async_trait::async_trait;
// use reqwest::Error;
use crate::{Plugin, GenericError};
// use std::error::Error;
use std::collections::HashMap;
use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth2Token, BearerToken};
use twitter_v2::query::{TweetField, UserField};

/*
let auth = BearerToken::new(std::env::var("APP_BEARER_TOKEN").unwrap());
let tweet = TwitterApi::new(auth)
    .get_tweet(1261326399320715264)
    .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
    .send()
    .await?
    .into_data()
    .expect("this tweet should exist");
assert_eq!(tweet.id, 1261326399320715264);
assert_eq!(tweet.author_id.unwrap(), 2244994945);
assert_eq!(tweet.created_at.unwrap(), datetime!(2020-05-15 16:03:42 UTC));

let auth: Oauth2Token = serde_json::from_str(&stored_oauth2_token)?;
let my_followers = TwitterApi::new(auth)
    .with_user_ctx()
    .await?
    .get_my_followers()
    .user_fields([UserField::Username])
    .max_results(20)
    .send()
    .await?
    .into_data();
*/

// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html

pub enum Twitter {
    ScreenName(String),
    Url(String)
}

#[async_trait]
impl Plugin for Twitter {
    async fn fill(&self) -> Result<HashMap<String,String>, GenericError> {
        let auth = BearerToken::new(std::env::var("APP_BEARER_TOKEN").unwrap());
        let tweet = TwitterApi::new(auth)
            .get_tweet(1261326399320715264)
            .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
            .send()
            .await?
            .into_data()
            .expect("this tweet should exist");
        assert_eq!(tweet.id, 1261326399320715264);
        assert_eq!(tweet.author_id.unwrap(), 2244994945);
        // assert_eq!(tweet.created_at.unwrap(), datetime!(2020-05-15 16:03:42 UTC));

        // let auth: Oauth2Token = serde_json::from_str(&stored_oauth2_token)?;
        // let my_followers = TwitterApi::new(auth)
        //     .with_user_ctx()
        //     .await?
        //     .get_my_followers()
        //     .user_fields([UserField::Username])
        //     .max_results(20)
        //     .send()
        //     .await?
        //     .into_data();
        // let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
        // let mut body = String::new();
        // res.read_to_string(&mut body)?;

        // println!("Status: {}", res.status());
        // println!("Headers:\n{:#?}", res.headers());
        // println!("Body:\n{}", body);
       Ok(HashMap::new())
    }
}
