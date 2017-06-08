use super::speedruncom_client::SpeedrunComClient;
use super::user::User;
use super::user_ordering::*;
use super::encode_string;
use super::string_helpers::*;
use super::element_type::ElementType;

use url::Url;

use serde_json::Value;

const NAME: &'static str = "users";

pub struct UsersClient {
    base_client: SpeedrunComClient,
}

impl UsersClient {
    pub fn new(base_client: SpeedrunComClient) -> Self {
        UsersClient { base_client: base_client }
    }

    pub fn get_user_from_site_uri(&self, site_uri: &str) -> Option<User> {
        let id = self.get_user_id_from_site_uri(site_uri);
        if let Some(id) = id {
            let user = self.get_user(&id);
            Some(user)
        } else {
            None
        }
    }

    pub fn get_user_id_from_site_uri(&self, site_uri: &str) -> Option<String> {
        if let Some(element_description) =
            self.base_client
                .get_element_description_from_site_uri(site_uri) {
            if element_description.get_type() != ElementType::User {
                None
            } else {
                Some(element_description.get_id())
            }
        } else {
            None
        }


    }

    pub fn get_user(&self, user_id: &str) -> User {
        let uri = self.get_users_uri(&format!("/{}", encode_string(user_id)));
        let mut result: Value = self.base_client.do_request(uri).unwrap();
        User::parse(&self.base_client, &mut result["data"]).unwrap()
    }

    pub fn get_users(&self,
                     name: Option<&str>,
                     twitch: Option<&str>,
                     hitbox: Option<&str>,
                     twitter: Option<&str>,
                     speedrunslive: Option<&str>,
                     elements_per_page: Option<usize>,
                     order_by: Option<UsersOrdering>)
                     -> Vec<User> {
        let mut parameters: Vec<String> = Vec::new();

        if let Some(name) = name {
            parameters.push(format!("name={}", encode_string(name)));
        }
        if let Some(twitch) = twitch {
            parameters.push(format!("twitch={}", encode_string(twitch)))
        }
        if let Some(hitbox) = hitbox {
            parameters.push(format!("hitbox={}", encode_string(hitbox)))
        }
        if let Some(twitter) = twitter {
            parameters.push(format!("twitter={}", encode_string(twitter)))
        }
        if let Some(speedrunslive) = speedrunslive {
            parameters.push(format!("speedrunslive={}", encode_string(speedrunslive)))
        }
        if let Some(elements_per_page) = elements_per_page {
            parameters.push(format!("max={}", elements_per_page))
        }
        if let Some(order_by) = order_by {
            parameters.extend(order_by.to_parameters());
        } else {
            parameters.extend(UsersOrdering::default().to_parameters());
        }

        let uri = self.get_users_uri(&vec_to_parameters(&parameters));
        self.base_client
            .do_paginated_request(uri, |mut v| User::parse(&self.base_client, &mut v).unwrap())
    }

    pub fn get_users_fuzzy(&self,
                           fuzzy_name: Option<&str>,
                           elements_per_page: Option<usize>,
                           order_by: Option<UsersOrdering>)
                           -> Vec<User> {
        let mut parameters: Vec<String> = Vec::new();
        if let Some(fuzzy_name) = fuzzy_name {
            parameters.push(format!("lookup={}", fuzzy_name));
        }
        if let Some(elements_per_page) = elements_per_page {
            parameters.push(format!("max={}", elements_per_page))
        }
        if let Some(order_by) = order_by {
            parameters.extend(order_by.to_parameters());
        } else {
            parameters.extend(UsersOrdering::default().to_parameters());
        }

        let uri = self.get_users_uri(&vec_to_parameters(&parameters));

        self.base_client
            .do_paginated_request(uri, |mut v| User::parse(&self.base_client, &mut v).unwrap())
    }

    pub fn get_users_uri(&self, sub_uri: &str) -> Url {
        self.base_client
            .get_api_uri(&format!("{}{}", NAME, sub_uri))
    }

    /*
    // TODO
    pub fn get_personal_bests(&self,
                              user_id: &str,
                              top: Option<usize>,
                              series_id: Option<&str>,
                              game_id: Option<&str>,
                              embeds: Option<RunEmbeds>)
                              -> Vec<Records> {
        let mut parameters = vec![format!({"{}", embeds})];

        if let Some(top) = top {
            parameters.push(format!("top={}", top));
        }
        if let Some(series_id) = series_id {
            parameters.push(format!("series={}", series_id));
        }
        if let Some(game_id) = game_id {
            parameters.push(format!("game={}", game_id));
        }

        let uri = self.get_users_uri(&format!("/{}/personal-bests{}",
                                              encode_string(user_id),
                                              vec_to_parameters(&parameters)));

        self.base_client
            .do_data_collection_request(uri,
                                        |mut v| Record::parse(&self.base_client, &mut v).unwrap())
    }
	*/
}
