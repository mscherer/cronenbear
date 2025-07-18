use crate::aliases::AliasID;
use crate::aliases::AliasName;
use crate::consts::{DATE, GIT_REV};
use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    calendars: HashMap<AliasID, AliasName>,
    date: String,
    git_rev: String,
}

impl IndexTemplate {
    pub fn new(calendars: HashMap<AliasID, AliasName>) -> Self {
        Self {
            calendars,
            date: String::from(DATE),
            git_rev: String::from(GIT_REV),
        }
    }
}
