use crate::aliases::AliasID;
use crate::aliases::AliasName;
use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    calendars: HashMap<AliasID, AliasName>,
}

impl IndexTemplate {
    pub fn new(calendars: HashMap<AliasID, AliasName>) -> Self {
        Self { calendars }
    }
}
