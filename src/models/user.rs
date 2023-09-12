use std::borrow::Cow;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::Request;

use super::BasicListing;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub subreddit_id: String,
    pub approved_at_utc: Option<String>,
    pub author_is_blocked: bool,
    pub comment_type: Option<String>,
    pub link_title: String,
    pub mod_reason_by: Option<String>,
    pub banned_by: Option<String>,
    pub ups: u64,
    pub num_reports: Option<u64>,
    pub author_flair_type: String,
    pub total_awards_received: u64,
    pub subreddit: String,
    pub link_author: String,
    pub likes: Option<u64>,
    pub replies: String,
    pub user_reports: Vec<String>,
    pub saved: bool,
    pub id: String,
    pub banned_at_utc: Option<String>,
    pub mod_reason_title: Option<String>,
    pub gilded: u64,
    pub archived: bool,
    pub collapsed_reason_code: Option<String>,
    pub no_follow: bool,
    pub author: String,
    pub num_comments: u64,
    pub can_mod_post: bool,
    pub send_replies: bool,
    pub parent_id: String,
    pub score: u64,
    pub author_fullname: String,
    pub over_18: bool,
    pub report_reasons: Option<String>,
    pub removal_reason: Option<String>,
    pub approved_by: Option<String>,
    pub controversiality: u64,
    pub body: String,
}

#[derive(Serialize)]
pub struct Comments {
    #[serde(skip_serializing)]
    pub username: String,
}

impl Request for Comments {
    const METHOD: Method = Method::GET;

    type Response = BasicListing<Comment>;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/user/{}/comments", self.username))
    }
}
