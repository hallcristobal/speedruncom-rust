use url::Url;
use std::time::Duration;
use std::str;
use super::element_description::ElementDescription;

use serde_json::{Value, Error as json_error};
use serde_json;

pub struct SpeedrunComClient {
    base_uri: Url,
    api_uri: Url,
    api_http_header_relation: String,
    access_token: Option<String>,
    user_agent: String,
    max_cache_elements: usize,
    timeout: Duration, 
    //cache: HashMap<Url, Box<Any>>,
    //Categories: CategoriesClient,
    //Games: GamesClient,
    //Guests: GuestsClient,
    //Leaderboards: LeaderboardsClient,
    //Levels: LevelsClient,
    //Notifications: NotificationsClient,
    //Platforms: PlatformsClient,
    //Regions: RegionsClient,
    //Runs: RunsClient,
    //Series: SeriesClient,
    //Users: UsersClient,
    //Variables: VariablesClient,
}

impl SpeedrunComClient {
    pub fn new(user_agent: Option<String>,
               access_token: Option<String>,
               max_cache_elements: Option<usize>,
               timeout: Option<Duration>)
               -> Self {
        let base_uri = "http://www.speedrun.com/";
        SpeedrunComClient {
            base_uri: Url::parse(base_uri).unwrap(),
            api_uri: Url::parse(base_uri).unwrap().join("api/v1/").unwrap(),
            api_http_header_relation: "alternate http://www.speedrun.com/api".to_string(),
            access_token: access_token,
            user_agent: user_agent.unwrap_or("SpeedRunComSharp/1.0".to_string()),
            max_cache_elements: max_cache_elements.unwrap_or(50),
            timeout: timeout.unwrap_or(Duration::new(30, 0)), 
            //cache: HashMap::new(),
        }
    }

    pub fn api_uri(&self) -> &Url {
        &self.api_uri
    }

    pub fn get_base_uri(&self) -> &Url {
        &self.base_uri
    }

    pub fn get_access_token(&self) -> &str {
        if let Some(token) = self.access_token.as_ref() {
            token
        } else {
            ""
        }
    }

    pub fn is_access_token_valid(&self) -> bool {
        if self.access_token.is_none() {
            false
        } else {
            true
        }
    }

    pub fn get(&self) -> &str {
        &self.api_http_header_relation
    }

    pub fn do_request(&self, uri: Url) -> Result<Value, json_error> {
        let v: Value = serde_json::from_str(uri.as_str())?;
        Ok(v)
    }

    pub fn do_paginated_request<T, V, F>(&self, uri: Url, f: F) -> Vec<T>
        where F: FnMut(V) -> T
    {
        Vec::new()
    }

    pub fn do_data_collection_request<T, V, F>(&self, uri: Url, f: F) -> Vec<T>
        where F: FnMut(V) -> T
    {
        Vec::new()
    }

    pub fn get_api_uri(&self, sub_uri: &str) -> Url {
        self.api_uri.join(sub_uri).unwrap()
    }

    pub fn get_element_description_from_site_uri(&self,
                                                 site_uri: &str)
                                                 -> Option<ElementDescription> {
        ElementDescription::parse(site_uri)
    }
}


#[test]
fn create_default() {
    let client = SpeedrunComClient::new(None, None, None, None);
    let ex_uri = "http://www.speedrun.com/api/v1/";
    assert_eq!(ex_uri, client.api_uri.into_string());
}
