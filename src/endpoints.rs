use crate::error::*;
use crate::*;
//static = 

#[derive(Debug)]
pub struct Path{
    subreddit:Option<String>,
    section:String
}

impl Path{

    pub fn new(subreddit:Option<String>,section:String) -> Self{
        Path{
            subreddit,
            section,
        }
    }
}

pub async fn search(path:Path) -> Result<(),RedditError>{

    let url = format!("{}{}/search.json?q={}&limit=1",REDDIT_URL,path.subreddit.unwrap(),path.section);
    println!("{:?}",url);
    let res = reqwest::get(&url).await?;
    let body = res.text().await?;
    println!("{}",body);
    Ok(())


}