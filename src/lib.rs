use std::borrow::Cow;

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

pub mod client;
pub mod models;

pub trait Request: Serialize {
    const METHOD: Method;
    const PATH: &'static str = "";
    const REQUIRES_USER: bool = false;

    type Response: DeserializeOwned;

    fn path(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::PATH)
    }
}
