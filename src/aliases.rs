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

#[derive(Debug, Clone, Eq, Hash, PartialEq, Deserialize)]
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

    pub fn get_all_aliases_named(&self) -> HashMap<AliasID, AliasName> {
        let mut res: HashMap<AliasID, AliasName> = HashMap::new();
        for (k, v) in self.aliases.iter() {
            res.insert(k.clone(), v.name.clone());
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
}
