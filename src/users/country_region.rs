use remove_string;

use std::fmt;
use serde_json::Value;

pub struct CountryRegion {
    code: String,
    name: String,
    japanese_name: String,
}

impl CountryRegion {
    pub fn parse(object: &mut Value) -> Option<Self> {
        if let Some(mut object) = object.as_object_mut() {
            let country = CountryRegion {
                code: remove_string("code", &mut object).unwrap_or_default(),
                name: if let Some(mut names) = object["names"].as_object_mut() {
                    remove_string("international", &mut names).unwrap_or_default()
                } else {
                    String::new()
                },
                japanese_name: if let Some(mut names) = object["names"].as_object_mut() {
                    remove_string("japanese", &mut names).unwrap_or_default()
                } else {
                    String::new()
                },
            };
            Some(country)
        } else {
            None
        }
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_japanese_name(&self) -> &str {
        &self.japanese_name
    }
}

impl fmt::Display for CountryRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
