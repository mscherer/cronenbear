use crate::aliases::FormatString;
use crate::google_public_calendar::GooglePublicCalendar;
use icalendar::CalendarComponent::Event;
use icalendar::Component;

#[derive(Debug)]
pub struct MergedCalendar {
    events: Vec<icalendar::Event>,
    name: String,
}

impl MergedCalendar {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            events: Vec::new(),
        }
    }

    // TODO remove the unwrap
    pub fn add<T: GooglePublicCalendar>(&mut self, calendar: &T, format: &Option<FormatString>) {
        for e in &calendar.to_ical().unwrap().components {
            if let Event(event) = e {
                // TODO make sure this work if language is set to something else than english
                if event
                    .get_description()
                    .is_none_or(|v| !v.contains("Observance"))
                {
                    let mut e = event.clone();
                    let summary = String::from(e.get_summary().unwrap_or("Public holiday"));

                    let s = if let Some(_f) = format {
                        // TODO add some formating
                        summary
                    } else {
                        format!("{}: {}", calendar.get_short_name(), summary)
                    };
                    e.summary(s.as_str());
                    //println!("{}", calendar);
                    // TODO add a text to say which country is on holiday
                    self.events.push(e);
                }
            }
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string().clone()
    }

    pub fn generate_ical(&self) -> icalendar::Calendar {
        let mut i = icalendar::Calendar::new();
        for e in &self.events {
            let e2 = e.clone();
            i.push(e2);
        }
        i.name(&self.name);
        i.done()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let m = MergedCalendar::new("test");
        assert_eq!(m.get_name(), "test");
    }
}
