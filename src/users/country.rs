use {remove_string, remove_string_sub};

use std::fmt;
use serde_json::Value;

pub struct Country {
    code: String,
    name: String,
    japanese_name: String,
}

impl Country {
    pub fn parse(object: &mut Value) -> Option<Self> {
        if let Some(mut object) = object.as_object_mut() {
            let country = Country {
                code: remove_string("code", &mut object).unwrap_or_default(),
                name: remove_string_sub("international", &mut object["names"]).unwrap_or_default(),
                japanese_name: remove_string_sub("japanese", &mut object["names"])
                    .unwrap_or_default(),
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

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
