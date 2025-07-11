const GCAL_PREFIX: &str = "https://calendar.google.com/calendar/ical/";
const GCAL_SUFFIX: &str = "/public/basic.ics";
const LANGUAGE: &str = "en";

pub trait GooglePublicCalendar {
    fn get_google_id(&self) -> String;

    fn construct_calendar_url(&self) -> String {
        let id = self.get_google_id();
        let name = format!("{}.{}#holiday@group.v.calendar.google.com", LANGUAGE, id);
        format!("{}{}{}", GCAL_PREFIX, name, GCAL_SUFFIX)
    }
}
