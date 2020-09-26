
use crate::error::*;
use crate::*;

//use error::RedditError;
#[derive(Debug)]
pub enum Duration{
    temporary,
    permanent,
}


#[derive(Debug)]
pub enum Scope{
    identify,
    edit,
    flair,
    history,
    modconfig,
    modflair,
    modlog,
    modposts,
    modwiki,
    mysubreddits,
    privatemessages,
    read,
    report,
    save,
    submit,
    subscribe,
    vote,
    wikiedit,
    wikiread,
}
#[derive(Debug)]
pub struct OauthReddit<'a>{

    client_id:Option<String>,
    response_type:Option<String>,
    state:&'a str,
    client_secret:Option<String>,
    redirect_url:Option<&'a str>,
    duration:Duration,
    scope:Scope,
    
}
impl<'a> OauthReddit<'a>{

    pub fn new() -> Self{
        OauthReddit{
            client_id:None,
            response_type:None,
            state:"RANDOM_STRING",
            client_secret:None,
            redirect_url:None,
            duration:Duration::temporary,
            scope:Scope::read,
        }
    }

    pub fn with_client_id(self,client_id:Option<String>)-> Self{

        OauthReddit{
            client_id,
            response_type:self.response_type,
            state:self.state,
            client_secret:self.client_secret,
            redirect_url:self.redirect_url,
            duration:self.duration,
            scope:self.scope,
        }
    }


    pub fn with_response_type(self,response_type:Option<String>)-> Self{

        OauthReddit{
            client_id:self.client_id,
            response_type,
            state:self.state,
            client_secret:self.client_secret,
            redirect_url:self.redirect_url,
            duration:self.duration,
            scope:self.scope,
        }
    }


    pub fn with_state(self,state:&'a str)-> Self{

        OauthReddit{
            client_id:self.client_id,
            response_type:self.response_type,
            state,
            client_secret:self.client_secret,
            redirect_url:self.redirect_url,
            duration:self.duration,
            scope:self.scope,
        }
    }


    pub fn with_client_secret(self,client_secret:Option<String>)-> Self{

        OauthReddit{
                client_id:self.client_id,
            response_type:self.response_type,
            state:self.state,
            client_secret,
            redirect_url:self.redirect_url,
            duration:self.duration,
            scope:self.scope,
        }
    }


    pub fn with_redirect_url(self,redirect_url:Option<&'a str>)-> Self{

        OauthReddit{
            client_id:self.client_id,
            response_type:self.response_type,
            state:self.state,
            client_secret:self.client_secret,
            redirect_url,
            duration:self.duration,
            scope:self.scope,
        }
    
    }

    pub fn with_duration(self,duration:Duration)-> Self{

        OauthReddit{
            client_id:self.client_id,
            response_type:self.response_type,
            state:self.state,
            client_secret:self.client_secret,
            redirect_url:self.redirect_url,
            duration,
            scope:self.scope,
        }

    }


    pub fn with_scope(self,scope:Scope)-> Self{

        OauthReddit{
            client_id:self.client_id,
            response_type:self.response_type,
            state:self.state,
            client_secret:self.client_secret,
            redirect_url:self.redirect_url,
            duration:self.duration,
            scope,
        }

    }

    pub fn build(self) -> Self{
        self
    }


    pub async fn authorize(&self)-> Result<(),RedditError>{
        let url = format!("{}authorize?client_id={}&response={}&state={}&redirect_url={}&duration={:?}&scope={:?}",REDDIT_API_URL,self.client_id.as_ref().unwrap(),self.response_type.as_ref().unwrap(),self.state,self.redirect_url.unwrap(),self.duration,self.scope);
        println!("{:?}",url);
        let res = reqwest::get(&url).await?;
        let body = res.text().await?;
        println!("{}",body);
        Ok(())
    }


}

