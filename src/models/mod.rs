pub mod auth;
pub mod inbox;
pub mod me;
pub mod user;

use serde::Deserialize;

/// Basic structure of a Reddit response.
/// See: <https://github.com/reddit-archive/reddit/wiki/JSON>
#[derive(Deserialize, Debug)]
pub struct BasicThing<T> {
    /// An identifier that specifies the type of object that this is.
    pub kind: String,
    /// The data contained by this struct. This will vary depending on the type parameter
    /// because each endpoint returns different contents.
    pub data: T,
}

/// Used to paginate content that is too long to display in one go.
/// https://www.reddit.com/dev/api#listings
#[derive(Deserialize, Debug)]
pub struct Listing<T> {
    /// The fullname of the listing that follows after this page.
    pub after: Option<String>,
    pub dist: Option<u64>,
    /// Modhash
    pub modhash: Option<String>,
    pub geo_filter: String,
    /// The fullname of the listing that follows before this page.
    pub before: Option<String>,
    /// A list of `things` that this Listing wraps.
    pub children: Vec<T>,
}

pub type BasicListing<T> = BasicThing<Listing<BasicThing<T>>>;

/// SubredditResponse
#[derive(Debug, Deserialize)]
pub struct DataType<T> {
    pub kind: Option<String>,
    /// Data about subreddit.
    pub data: T,
}

pub fn flatten<T>(thing: BasicListing<T>) -> Vec<T> {
    thing.data.children.into_iter().map(|x| x.data).collect()
}
