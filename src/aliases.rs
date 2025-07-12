use std::collections::HashMap;
extern crate serde_derive;
extern crate toml;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Aliases {
    aliases: HashMap<String,Vec<String>>,

}

impl Aliases {
    pub fn load_from_file(path: &str) -> Self {
        // later
        todo!();
    }

    pub fn load_hardcoded() -> Self {
        let aliases: Aliases = toml::from_str(include_str!("../data/aliases.toml")).unwrap();
        aliases
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
