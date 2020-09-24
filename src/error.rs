use std::fmt;
//use thiserror::Error;
//type NewResult<()> = Result<(),Box<dyn FacebookError>>
//#[derive(Error,Debug)]
#[derive(Debug)]
pub enum RedditError{
    SerdeError,
    //         #[error("serde error")]
    //         SerdeError ( serde_json::Error) ,
    // #[error("invalid URI")]
    // HyperInvalidUri ( #[from] hyper::http::uri::InvalidUri ),

    HyperInvalidUri,
    // #[error("hyper error")]
    // HyperError (#[from]  hyper::error::Error),

     ReqwestError,
   // #[error("Uri error")]
   // HyperUriErr( #[from] <hyper::Uri as std::str>::Error),


  //  #[error("std error")]
  //  StdError( #[from] std::error::Error)
  //  #[error("other")]
  //  Other(#[from] anyhow::Error),
}

impl std::error::Error for RedditError{}

impl fmt::Display for RedditError{
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result{
        match self{
            RedditError::SerdeError => { write!( f ,"Serde Error") } ,
            RedditError::HyperInvalidUri => { write!(f,"Hyper uri error")},
            RedditError::ReqwestError => { write!(f,"Reqwest error")},
//            FacebookError::

        }
    }
}

impl From<serde_json::Error> for RedditError{
    fn from(_:serde_json::Error) -> Self{
            RedditError::SerdeError
    }
}
impl From<reqwest::Error> for RedditError{
    fn from(_:reqwest::Error) -> Self{
            RedditError::ReqwestError
    }
}
