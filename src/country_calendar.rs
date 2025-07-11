use codes_iso_3166::part_1::CountryCode;
use std::fmt;
use std::str::FromStr;

use crate::google_public_calendar::GooglePublicCalendar;

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
            CountryCode::AU => "australian",
            CountryCode::BE => "be",
            CountryCode::BR => "brazilian",
            CountryCode::CA => "canadian",
            CountryCode::CN => "china",
            CountryCode::CZ => "czech",
            CountryCode::DE => "german",
            CountryCode::FI => "finnish",
            CountryCode::FR => "french",
            CountryCode::GB => "uk",
            CountryCode::HR => "croatian",
            CountryCode::IE => "irish",
            // as surprising it may seems, that's what Google calendar use
            CountryCode::IL => "jewish",
            // not sure why this is different
            CountryCode::IN => "indian.official",
            CountryCode::IT => "italian",
            CountryCode::JP => "japanese",
            CountryCode::NL => "dutch",
            CountryCode::SK => "slovak",
            CountryCode::US => "usa",
            _ => "",
        };
        r.to_string()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let fr = CountryCalendar::try_from("fr");
        assert_eq!(fr.is_err(), false);

        let FR = CountryCalendar::try_from("FR");
        assert_eq!(FR.is_err(), false);

        let plop = CountryCalendar::try_from("plop");
        assert_eq!(plop.is_err(), true);

        // I hope no country will be created with that code
        let zz = CountryCalendar::try_from("ZZ");
        assert_eq!(zz.is_err(), true);
    }
}
