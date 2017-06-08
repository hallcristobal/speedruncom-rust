#![allow(dead_code)]
extern crate hyper;
#[macro_use]
extern crate url;
extern crate chrono;
extern crate serde;
extern crate serde_json;

pub mod users;
pub mod speedruncom_client;
pub mod string_helpers;
pub mod element_description;
pub mod element_type;

use serde_json::{Map, Value};
use url::percent_encoding::SIMPLE_ENCODE_SET;
use url::percent_encoding;

pub fn remove_string(key: &str, object: &mut Map<String, Value>) -> Option<String> {
    if let Some(Value::String(value)) = object.remove(key) {
        Some(value)
    } else {
        None
    }
}

pub fn remove_string_sub(key: &str, object: &mut Value) -> Option<String> {
    if let Some(mut object) = object.as_object_mut() {
        remove_string(key, &mut object)
    } else {
        None
    }
}

define_encode_set! {
    pub ENCODE_SET = [SIMPLE_ENCODE_SET] | {
        '!','*','\'','(',')',';',':','@','&','=','+','$',',','/','?','#','[',']'
    }
}

pub fn encode_string(data_string: &str) -> String {
    percent_encoding::utf8_percent_encode(data_string, ENCODE_SET).collect::<String>()
}
