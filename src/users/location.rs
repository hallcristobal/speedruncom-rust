use super::country_region::CountryRegion;
use super::country::Country;

use std::fmt;
use serde_json::Value;

pub struct Location {
    country: Country,
    region: Option<CountryRegion>,
}

impl Location {
    pub fn parse(object: &mut Value) -> Option<Self> {
        if let Some(mut object) = object.as_object_mut() {
            let location = Location {
                country: Country::parse(&mut object["country"]).unwrap(),
                region: CountryRegion::parse(&mut object["region"]),
            };
            Some(location)
        } else {
            None
        }
    }

    pub fn get_country(&self) -> &Country {
        &self.country
    }

    pub fn get_region(&self) -> &Option<CountryRegion> {
        &self.region
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(region) = self.region.as_ref() {
            write!(f, "{} {}", self.country.get_name(), region.get_name())
        } else {
            write!(f, "{}", self.country.get_name())
        }
    }
}
