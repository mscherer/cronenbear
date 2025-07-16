use crate::google_public_calendar::{GooglePublicCalendar, GooglePublicCalendarError};
use icalendar::Calendar;

// the only one that appear in the interface as far as I see
#[derive(Debug)]
pub enum ReligionCode {
    Christianism,
    Judaism,
    Islamic,
    OrthodoxChristianism,
    Hinduism,
}

#[derive(Debug)]
pub struct ReligionCalendar {
    religion_code: ReligionCode,
}

impl ReligionCalendar {
    pub fn new(code: ReligionCode) -> Self {
        Self {
            religion_code: code,
        }
    }
}
impl GooglePublicCalendar for ReligionCalendar {
    fn get_google_id(&self) -> String {
        let r = match self.religion_code {
            ReligionCode::Christianism => "christian",
            ReligionCode::Hinduism => "hinduism",
            ReligionCode::Islamic => "islamic",
            ReligionCode::Judaism => "judaism",
            ReligionCode::OrthodoxChristianism => "orthodox_christianity",
        };
        r.to_string()
    }

    fn get_short_name(&self) -> String {
        "".to_string()
    }
}

impl TryFrom<ReligionCalendar> for Calendar {
    type Error = GooglePublicCalendarError;
    fn try_from(c: ReligionCalendar) -> Result<Self, Self::Error> {
        c.to_ical()
    }
}
