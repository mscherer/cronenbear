use crate::google_public_calendar::{GooglePublicCalendar, GooglePublicCalendarError};
use bimap::BiMap;
use icalendar::Calendar;
use std::collections::HashMap;
use std::sync::LazyLock;

// the only one that appear in the interface as far as I see
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ReligionCode {
    Christianism,
    Judaism,
    Islamic,
    OrthodoxChristianism,
    Hinduism,
}

type ReligionDLT = BiMap<ReligionCode, String>;
pub fn generate_religion_dlt() -> ReligionDLT {
    let mut r = ReligionDLT::new();
    r.insert(ReligionCode::Christianism, "christian".to_owned());
    r.insert(ReligionCode::Hinduism, "hinduism".to_owned());
    r.insert(ReligionCode::Islamic, "islamic".to_owned());
    r.insert(ReligionCode::Judaism, "judaism".to_owned());
    r.insert(
        ReligionCode::OrthodoxChristianism,
        "orthodox_christianity".to_owned(),
    );
    r
}

static RELIGION_TABLE: LazyLock<ReligionDLT> = LazyLock::new(generate_religion_dlt);

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
        RELIGION_TABLE
            .get_by_left(&self.religion_code)
            .expect("all religions are here")
            .to_string()
    }

    fn get_formatting_hashmap(&self) -> HashMap<String, String> {
        let mut res = HashMap::new();
        let emoji = match self.religion_code {
            ReligionCode::Christianism => "✝️",
            ReligionCode::Judaism => "✡️",
            ReligionCode::Islamic => "☪️",
            ReligionCode::OrthodoxChristianism => "☦️",
            ReligionCode::Hinduism => "🕉️",
        };
        res.insert("emoji".to_owned(), emoji.to_owned());
        res
    }
}

impl TryFrom<&str> for ReligionCalendar {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let code = value.to_string().to_lowercase();

        if let Some(v) = RELIGION_TABLE.get_by_right(&code) {
            Ok(ReligionCalendar::new(*v))
        } else {
            Err(())
        }
    }
}

impl TryFrom<ReligionCalendar> for Calendar {
    type Error = GooglePublicCalendarError;
    fn try_from(c: ReligionCalendar) -> Result<Self, Self::Error> {
        c.to_ical()
    }
}
