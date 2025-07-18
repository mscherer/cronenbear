use crate::aliases::FormatString;
use crate::google_public_calendar::GooglePublicCalendar;
use icalendar::CalendarComponent::Event;
use icalendar::Component;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MergedCalendar {
    events: Vec<icalendar::Event>,
    name: String,
}

fn format_summary(
    summary: &str,
    format: &FormatString,
    format_hashmap: &HashMap<String, String>,
) -> String {
    let mut res = String::new();
    let mut var = String::new();
    let mut in_format = false;

    for c in format.chars() {
        if in_format {
            if c == '}' {
                in_format = false;
                let v = if var == "summary" {
                    summary.to_owned()
                } else {
                    format_hashmap
                        .get(&var)
                        .unwrap_or(&format!("{{{var}}}"))
                        .to_string()
                };
                res.push_str(&v);
                var.clear();
            } else {
                var.push(c);
            }
        } else if c == '{' {
            in_format = true;
        } else {
            res.push(c);
        }
    }
    if in_format {
        res.push('{');
        res.push_str(&var);
    }
    res
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
        let format_hm = calendar.get_formatting_hashmap();
        for e in &calendar.to_ical().unwrap().components {
            if let Event(event) = e {
                // TODO make sure this work if language is set to something else than english
                if event
                    .get_description()
                    .is_none_or(|v| !v.contains("Observance"))
                {
                    let mut e = event.clone();
                    let summary = String::from(e.get_summary().unwrap_or("Public holiday"));

                    let s = if let Some(f) = format {
                        format_summary(&summary, f, &format_hm)
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

    #[test]
    fn test_format_summary() {
        let mut format_hm = HashMap::new();
        format_hm.insert(
            String::from("name"),
            String::from("Sir Lancelot of Camelot"),
        );
        format_hm.insert(String::from("color"), String::from("blue"));

        let summary = String::from("maybe more");
        let format = FormatString::new("test: {name} like {color} and {summary}");
        let res = format_summary(&summary, &format, &format_hm);
        assert_eq!(
            res,
            "test: Sir Lancelot of Camelot like blue and maybe more"
        );
    }
}
