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
    google_cal_id: String,
}

impl GooglePublicCalendar for CountryCalendar {
    fn get_google_id(&self) -> String {
        self.google_cal_id.clone()
    }
}

impl CountryCalendar {
    fn new(code: CountryCode) -> Self {
        Self {
            iso_3166_code: code,
            // TODO load from the disk
            google_cal_id: "".to_owned(),
        }
    }
}

impl fmt::Display for CountryCalendar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.iso_3166_code, self.google_cal_id)
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
