use std::collections::HashMap;
use std::fmt;
extern crate toml;
use array_tool::vec::Uniq;
use serde::Deserialize;
use std::str::Chars;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Deserialize)]
pub struct AliasName(String);
impl fmt::Display for AliasName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Deserialize, Ord, PartialOrd)]
pub struct AliasID(String);
impl fmt::Display for AliasID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for AliasID {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<AliasID> for String {
    fn from(s: AliasID) -> Self {
        s.0.to_string()
    }
}

impl From<AliasName> for String {
    fn from(s: AliasName) -> Self {
        s.0.to_string()
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Deserialize)]
pub struct FormatString(String);

impl FormatString {
    pub fn new(format: impl Into<String>) -> Self {
        Self(format.into())
    }
    pub fn chars(&self) -> Chars<'_> {
        self.0.chars()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Alias {
    format: Option<FormatString>,
    name: AliasName,
    calendars: Vec<String>,
    // default to false, by sheer luck
    // see https://github.com/serde-rs/serde/issues/368 for future fix
    #[serde(default)]
    hidden: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Aliases {
    aliases: HashMap<AliasID, Alias>,
}

impl Aliases {
    /*    pub fn load_from_file(path: &str) -> Self {
            // later
            todo!();
        }
    */

    fn load_string(string: &str) -> Self {
        let aliases: HashMap<AliasID, Alias> = toml::from_str(string).unwrap();
        Aliases { aliases }
    }

    pub fn load_hardcoded() -> Self {
        Self::load_string(include_str!("../data/aliases.toml"))
    }

    pub fn get_all_aliases(&self) -> Vec<AliasID> {
        self.aliases.clone().into_keys().collect()
    }

    pub fn get_public_aliases_index(&self) -> HashMap<AliasID, AliasName> {
        let mut res: HashMap<AliasID, AliasName> = HashMap::new();
        for (k, v) in self.aliases.iter() {
            if !v.hidden {
                res.insert(k.clone(), v.name.clone());
            }
        }
        res
    }

    pub fn get_all_calendars_to_create(&self) -> Vec<String> {
        self.aliases
            .clone()
            .into_values()
            .flat_map(|x| x.calendars)
            .collect::<Vec<_>>()
            .unique()
    }

    // TODO check arguments
    pub fn get_members(&self, alias: &AliasID) -> Option<Vec<String>> {
        self.aliases.get(alias).cloned().map(|x| x.calendars)
    }

    pub fn get_formatting(&self, alias: &AliasID) -> Option<FormatString> {
        if let Some(cal) = self.aliases.get(alias).cloned() {
            cal.format.clone()
        } else {
            None
        }
    }

    pub fn get_name(&self, alias: &AliasID) -> Option<AliasName> {
        if let Some(cal) = self.aliases.get(alias).cloned() {
            Some(cal.name.clone())
        } else {
            None
        }
    }

    /* pub fn generate_hardcoded() -> Self {
        // TODO use a hardcoded toml file
        let hash = HashMap::from([("ospo".to_owned(), vec!["fr".to_owned(),
                                                              "jp".to_owned(),
                                                              "us".to_owned(),
                                                              "cz".to_owned(),
                                                              "de".to_owned(),
                                                              "fi".to_owned(),
                                                              "ie".to_owned(),
                                                              "us".to_owned()])]);
        Self {
            aliases: hash
        }
    }
    */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_functions() {
        let s = r#"[cee]
            name='CEE'
            calendars=['fr', 'be', 'it', 'lu', 'nl', 'de']

            [benelux]
            name='Benelux'
            calendars=['be', 'lu', 'nl' ]"#;

        let al = Aliases::load_string(s);
        let aliases = al.get_all_aliases();
        assert_eq!(aliases.len(), 2);
        assert!(aliases.contains(&AliasID("benelux".to_string())));
        assert!(aliases.contains(&AliasID("cee".to_string())));
    }

    #[test]
    fn test_hardcoded() {
        use crate::country_calendar::CountryCalendar;
        use crate::google_public_calendar::GooglePublicCalendar;
        use crate::religion_calendar::ReligionCalendar;
        // will fail if the hardcoded toml is incorrect
        let aliases = Aliases::load_hardcoded();
        for c in aliases.get_all_calendars_to_create() {
            let code = c.as_str();
            // will panic if the alias can't be created
            if let Ok(cal) = CountryCalendar::try_from(code) {
                assert_ne!(
                    cal.get_google_id(),
                    "",
                    "country {code} is not matched by get_google_id function"
                );
            } else if let Ok(cal) = ReligionCalendar::try_from(code) {
                assert_ne!(
                    cal.get_google_id(),
                    "",
                    "religion {code} is not matched by get_google_id function"
                );
            } else {
                panic!("{}", format!("the country code '{code}' should be a valid iso country code or a religion code").as_str());
            }
        }
    }
}
