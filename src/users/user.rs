use remove_string;
use super::user_role::UserRole;
use super::location::Location;
use super::user_name_style::UserNameStyle;
use super::speedruncom_client::SpeedrunComClient;

use chrono::{DateTime, FixedOffset};
use serde_json::Value;
use url::Url;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter, Result};

pub struct User {
    id: String,
    name: String,
    japanese_name: Option<String>,
    web_link: Url,
    name_style: UserNameStyle,
    role: UserRole,
    sign_up_date: DateTime<FixedOffset>,
    location: Location,
    twitch_profile: Option<Url>,
    hitbox_profile: Option<Url>,
    youtube_profile: Option<Url>,
    twitter_profile: Option<Url>,
    speed_runs_live_profile: Option<Url>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)
    }
}

impl User {
    pub fn parse(client: &SpeedrunComClient, object: &mut Value) -> Option<Self> {
        if let Some(mut object) = object.as_object_mut() {
            let user = User {
                id: remove_string("id", &mut object).unwrap_or_default(),
                name: if let Some(mut names) = object["names"].as_object_mut() {
                    remove_string("international", &mut names).unwrap_or_default()
                } else {
                    String::new()
                },
                japanese_name: if let Some(mut names) = object["names"].as_object_mut() {
                    remove_string("japanese", &mut names)
                } else {
                    None
                },
                web_link: parse_url(&object["weblink"]).unwrap(),
                name_style: UserNameStyle::parse(&mut object["name-style"]).unwrap(),
                role: parse_user_role(object["role"].as_str().unwrap_or_default()),
                sign_up_date: DateTime::parse_from_rfc3339(object["signup"].as_str().unwrap())
                    .unwrap(),
                location: Location::parse(&mut object["location"]).unwrap(),
                twitch_profile: parse_url(&object["twitch"]),
                hitbox_profile: parse_url(&object["hitbox"]),
                youtube_profile: parse_url(&object["youtube"]),
                twitter_profile: parse_url(&object["twitter"]),
                speed_runs_live_profile: parse_url(&object["speedrunslive"]), // TODO Parse links
            };
            Some(user)
        } else {
            None
        }
    }

    pub fn get_hash_code(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.id.hash(&mut hasher);
        hasher.finish()
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_japanese_name(&self) -> &Option<String> {
        &self.japanese_name
    }

    pub fn get_web_link(&self) -> &Url {
        &self.web_link
    }

    pub fn get_user_name_style(&self) -> &UserNameStyle {
        &self.name_style
    }

    pub fn get_role(&self) -> &UserRole {
        &self.role
    }

    pub fn get_sign_up_date(&self) -> &DateTime<FixedOffset> {
        &self.sign_up_date
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_twitch_profile(&self) -> &Option<Url> {
        &self.twitch_profile
    }

    pub fn get_hitbox_profile(&self) -> &Option<Url> {
        &self.hitbox_profile
    }

    pub fn get_youtube_profile(&self) -> &Option<Url> {
        &self.youtube_profile
    }

    pub fn get_twitter_profile(&self) -> &Option<Url> {
        &self.twitter_profile
    }

    pub fn get_speed_runs_live_profile(&self) -> &Option<Url> {
        &self.speed_runs_live_profile
    }
}

pub fn parse_url(object: &Value) -> Option<Url> {
    let object = if let Some(link) = object.as_object() {
        &link["uri"]
    } else {
        object
    };

    if let Some(uri) = object.as_str() {
        match Url::parse(uri) {
            Ok(url) => Some(url),
            _ => None,
        }
    } else {
        None
    }
}

pub fn parse_user_role(role: &str) -> UserRole {
    match role {
        "banned" => UserRole::Banned,
        "user" => UserRole::User,
        "trusted" => UserRole::Trusted,
        "moderator" => UserRole::Moderator,
        "admin" => UserRole::Admin,
        "programmer" => UserRole::Programmer,
        _ => UserRole::User,
    }
}
