const GCAL_PREFIX: &str = "https://calendar.google.com/calendar/ical/";
const GCAL_SUFFIX: &str = "/public/basic.ics";
const LANGUAGE: &str = "en";
use ureq;

pub trait GooglePublicCalendar {
    fn get_google_id(&self) -> String;

    fn construct_calendar_url(&self) -> String {
        let id = self.get_google_id();
        // cannot use # directly, convert to %23
        let name = format!("{}.{}%23holiday@group.v.calendar.google.com", LANGUAGE, id);
        format!("{}{}{}", GCAL_PREFIX, name, GCAL_SUFFIX)
    }

    fn fetch_calendar_web(&self) -> Result<String, ureq::Error> {
        let url = self.construct_calendar_url();
        ureq::get(url).call()?.body_mut().read_to_string()
    }
}
