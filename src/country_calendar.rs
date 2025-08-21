use codes_iso_3166::part_1::CountryCode;
use country_emoji::flag;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::google_public_calendar::{GooglePublicCalendar, GooglePublicCalendarError};
use icalendar::Calendar;

/* TODO
 * load calendar from disk, or from the binary with include_dir
 * add a Calendar type that generate a url, download the calendar
 * manage a cache
 *
 */

#[derive(Debug)]
pub struct CountryCalendar {
    iso_3166_code: CountryCode,
    //    google_cal_id: String,
}

impl GooglePublicCalendar for CountryCalendar {
    fn get_google_id(&self) -> String {
        let r = match self.iso_3166_code {
            CountryCode::AL => "al",
            CountryCode::AU => "australian",
            CountryCode::BE => "be",
            CountryCode::BR => "brazilian",
            CountryCode::CA => "canadian",
            CountryCode::CL => "cl",
            CountryCode::CN => "china",
            CountryCode::CZ => "czech",
            CountryCode::DE => "german",
            CountryCode::DK => "danish",
            CountryCode::FI => "finnish",
            CountryCode::FR => "french",
            CountryCode::GB => "uk",
            CountryCode::GH => "gh",
            CountryCode::GT => "gt",
            CountryCode::HR => "croatian",
            CountryCode::IE => "irish",
            // as surprising it may seems, that's what Google calendar use
            CountryCode::IL => "jewish",
            // not sure why this is different
            CountryCode::IN => "indian.official",
            CountryCode::IT => "italian",
            CountryCode::JP => "japanese",
            CountryCode::KE => "ke",
            CountryCode::KP => "kp",
            CountryCode::KR => "south_korea",
            CountryCode::LB => "lb",
            CountryCode::NL => "dutch",
            CountryCode::PT => "portuguese",
            CountryCode::SA => "sa",
            CountryCode::SK => "slovak",
            CountryCode::US => "usa",
            _ => "",
        };
        r.to_string()
    }

    fn get_short_name(&self) -> String {
        self.iso_3166_code.alpha_2_code().to_string()
    }

    fn get_formatting_hashmap(&self) -> HashMap<String, String> {
        let mut res = HashMap::new();
        let short_name = self.iso_3166_code.short_name();
        if let Some(emoji) = flag(short_name) {
            res.insert("emoji".to_owned(), emoji);
        }
        res.insert("name".to_owned(), String::from(short_name));

        res.insert("iso_code".to_owned(), self.get_short_name());
        // ("emoji".to_owned(), flag(self.iso_3166_code.full_name())),
        res
    }
}

impl CountryCalendar {
    fn new(code: CountryCode) -> Self {
        Self {
            iso_3166_code: code,
        }
    }
}

impl fmt::Display for CountryCalendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.iso_3166_code, self.get_google_id())
    }
}

impl TryFrom<&str> for CountryCalendar {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let code = value.to_string().to_uppercase();
        if let Ok(v) = CountryCode::from_str(code.as_str()) {
            Ok(CountryCalendar::new(v))
        } else {
            Err(())
        }
    }
}

impl TryFrom<CountryCalendar> for Calendar {
    type Error = GooglePublicCalendarError;
    fn try_from(c: CountryCalendar) -> Result<Self, Self::Error> {
        c.to_ical()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let fr = CountryCalendar::try_from("fr");
        assert_eq!(fr.is_err(), false);
        assert_eq!(fr.unwrap().get_short_name(), "FR");

        let fr_capital = CountryCalendar::try_from("FR");
        assert_eq!(fr_capital.is_err(), false);

        let plop = CountryCalendar::try_from("plop");
        assert_eq!(plop.is_err(), true);

        // I hope no country will be created with that code
        let zz = CountryCalendar::try_from("ZZ");
        assert_eq!(zz.is_err(), true);
    }

    #[test]
    fn test_get_formatting_hashmap() {
        let fr = CountryCalendar::try_from("fr").unwrap();
        let hm = fr.get_formatting_hashmap();
        assert_eq!(hm.get("iso_code"), Some("FR".to_owned()).as_ref());
        assert_eq!(hm.get("name"), Some("France".to_owned()).as_ref());
        assert_eq!(hm.get("emoji"), Some("🇫🇷".to_owned()).as_ref());
    }
}
