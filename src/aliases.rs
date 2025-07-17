use std::collections::HashMap;
extern crate toml;
use array_tool::vec::Uniq;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Alias {
    name: String,
    calendars: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Aliases {
    aliases: HashMap<String, Alias>,
}

impl Aliases {
    /*    pub fn load_from_file(path: &str) -> Self {
            // later
            todo!();
        }
    */

    fn load_string(string: &str) -> Self {
        let aliases: HashMap<String, Alias> = toml::from_str(string).unwrap();
        Aliases { aliases }
    }

    pub fn load_hardcoded() -> Self {
        Self::load_string(include_str!("../data/aliases.toml"))
    }

    pub fn get_all_aliases(&self) -> Vec<String> {
        self.aliases.clone().into_keys().collect()
    }

    pub fn get_all_calendars_to_create(&self) -> Vec<String> {
        self.aliases
            .clone()
            .into_values()
            .map(|x| x.calendars)
            .flatten()
            .collect::<Vec<_>>()
            .unique()
    }

    // TODO check arguments
    pub fn get_members(&self, alias: &String) -> Option<Vec<String>> {
        self.aliases.get(alias).cloned().map(|x| x.calendars)
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
        assert!(aliases.contains(&"benelux".to_string()));
        assert!(aliases.contains(&"cee".to_string()));
    }
}
