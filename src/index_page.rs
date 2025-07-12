use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    calendars: Vec<String>, 
}

impl IndexTemplate {
    pub fn new(calendars: Vec<String>) -> Self {
        Self {
            calendars: calendars.clone()
        }
    }
}
