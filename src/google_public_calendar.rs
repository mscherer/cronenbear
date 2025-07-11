use icalendar::Calendar;
use ureq;

const GCAL_PREFIX: &str = "https://calendar.google.com/calendar/ical/";
const GCAL_SUFFIX: &str = "/public/basic.ics";
const LANGUAGE: &str = "en";

#[derive(Debug)]
pub enum GooglePublicCalendarError {
    NetworkError,
    ParseError,
}

impl From<ureq::Error> for GooglePublicCalendarError {
    fn from(_e: ureq::Error) -> Self {
        GooglePublicCalendarError::NetworkError
    }
}

impl From<String> for GooglePublicCalendarError {
    fn from(_e: String) -> Self {
        GooglePublicCalendarError::ParseError
    }
}

pub trait GooglePublicCalendar {
    // uncomment once https://github.com/rust-lang/rust/issues/29661 is solved
    // eg, when "associated type defaults" is stable
    // type Error = GooglePublicCalendarError;
    fn get_google_id(&self) -> String;

    fn construct_calendar_url(&self) -> String {
        let id = self.get_google_id();
        // cannot use # directly, convert to %23
        let name = format!("{}.{}%23holiday@group.v.calendar.google.com", LANGUAGE, id);
        format!("{}{}{}", GCAL_PREFIX, name, GCAL_SUFFIX)
    }

    fn fetch_calendar_web(&self) -> Result<String, GooglePublicCalendarError> {
        let url = self.construct_calendar_url();
        Ok(ureq::get(url).call()?.body_mut().read_to_string()?)
    }

    // cannot use TryForm due to GooglePublicCalendar being
    // a trait, and try_from is not dyn safe or something
    fn to_ical(&self) -> Result<icalendar::Calendar, GooglePublicCalendarError> {
        let cal = self.fetch_calendar_web()?;
        let c: Calendar = cal.parse()?;
        Ok(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CountryCalendar;
    #[test]
    fn test_calendar_url() {
        let fr = CountryCalendar::try_from("fr").expect("weird error");
        assert_eq!(fr.construct_calendar_url(), "https://calendar.google.com/calendar/ical/en.french%23holiday@group.v.calendar.google.com/public/basic.ics");
    }
}
