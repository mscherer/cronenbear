use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    calendars: Vec<String>, 
}

impl IndexTemplate {
    pub fn new() -> Self {
        Self {
            calendars: Vec::new()
        }
    }
}
