use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Context<'c> {
    pub name: Cow<'c, str>,
    pub version: Cow<'c, str>,
    pub url: Cow<'c, str>,
}
