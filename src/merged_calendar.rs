use crate::google_public_calendar::GooglePublicCalendar;
use icalendar::Calendar;
use icalendar::CalendarComponent::Event;
use icalendar::Component;

pub struct MergedCalendar {
    calendars: Vec<Calendar>,
    name: String,
}

impl IntoIterator for MergedCalendar {
    type Item = Calendar;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.calendars.into_iter()
    }
}

impl MergedCalendar {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            calendars: Vec::new(),
        }
    }

    // TODO remove the unwrap
    pub fn add<T: GooglePublicCalendar>(&mut self, calendar: T) {
        self.calendars.push(calendar.to_ical().unwrap())
    }

    pub fn get_name(&self) -> String {
        self.name.to_string().clone()
    }
}

impl From<MergedCalendar> for icalendar::Calendar {
    fn from(merged_calendar: MergedCalendar) -> icalendar::Calendar {
        let mut new_cal = Calendar::new(); //.name("example calendar")
        let name = merged_calendar.get_name();
        for calendar in merged_calendar {
            for e in calendar.components {
                if let Event(event) = e {
                    // TODO make sure this work if language is set to something else than english
                    if event
                        .get_description()
                        .is_none_or(|v| !v.contains("Observance"))
                    {
                        new_cal.push(event.clone());
                    }
                }
            }
        }
        new_cal.name(&name).done()
    }
}
