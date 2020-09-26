
use reqwest::Error;
use std::env;
use serde_json::{Value};
use serde::{Serialize,Deserialize};

mod error;
use error::RedditError;

mod Oauth;
use Oauth::*;
mod endpoints;
use endpoints::*;


static REDDIT_API_URL:&str = "https://www.reddit.com/api/v1/";
static REDDIT_URL:&str = "https://www.reddit.com/";


#[cfg(test)]
mod tests {
use super::*;
use tokio::prelude::*;
    #[test]
    // fn oauth_reddit_test() {

    //     let MyappOuth = OauthReddit::new();

    //     let MyredappOuth = MyappOuth 
    //                 .with_client_id(Some(env::var("Redid").unwrap().to_string()))
    //                 .with_response_type(Some("code".to_string()))
    //                 .with_client_secret(Some(env::var("Redsecret").unwrap().to_string()))
    //                 .with_duration(Duration::temporary)
    //                 .with_redirect_url(Some("http://localhost:8000/reddit"))
    //                 .with_scope(Scope::read)
    //                 .with_state("RANDOM_STRING");
    //     println!("{:?}",MyredappOuth);
    //     tokio_test::block_on(MyredappOuth.authorize()) ;       
        
    // }

    #[test]
    fn search_subreddit_test(){

        let path = Path::new(Some("r/rust".to_string()),"async".to_string());
         
        tokio_test::block_on(search(path));
    }
}
