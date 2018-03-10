#![feature(associated_type_defaults)]

extern crate reqwest;
extern crate url;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::{borrow, env, io};
use url::Url;

mod api;
mod models;

pub use models::*;
pub use api::{CurrentResponse, ForecastResponse};

pub trait WeatherApi<T> {
    fn new<R>(readable_token: R, base_url: Option<Url>) -> io::Result<T> where R: io::Read;

    fn get_current(city_name: String) -> Weather;

    fn get_forecast<R>(city_name: String, days: u64) -> R where R: ForecastIterator;
}

pub struct OpenWeatherMap {
    token: String,
    url: Url,
    query: QueryBuilder,
}

impl OpenWeatherMap {
    const BASE_URL: &'static str = "https://api.openweathermap.org/data/2.5/";
    const CURRENT_ENDPOINT: &'static str = "/weather";

    pub fn get_city_weather(&mut self, city_name: String) -> Option<Weather> {
        self.query.insert(&String::from("q"), &city_name);

        let mut response = self.send(Self::CURRENT_ENDPOINT.to_string()).unwrap();

        let weather: Weather = match response.json::<CurrentResponse>() {
            Ok(val) => val.into(),
            Err(e) => panic!() // TODO
        };

        Some(weather)
    }

    fn send(&mut self, endpoint: String) -> Option<reqwest::Response> {
        let url = self.url(endpoint);

        let val = match reqwest::Client::new().get(url).send() {
            Ok(val) => val,
            Err(e) => panic!(e)
        };

        Some(val)
    }

    pub fn url(&mut self, endpoint: String) -> Url {
        let token = self.token.clone();
        self.add_query_param(&String::from("appid"), &token);
        self.add_query_param(&String::from("units"), &String::from("metric"));

        self.build_url(endpoint)
    }

    fn add_query_param(&mut self, key: &String, value: &String) {
        self.query.insert(key, value);
    }

    fn build_url(&self, endpoint: String) -> Url {
        let mut url = self.url.clone();
        url.set_path(&(String::from(self.url.path()) + &endpoint));
        let query = self.query.clone();
        let query = query.build().unwrap_or(String::new());

        url.set_query(Some(&query));
        url
    }
}

impl WeatherApi<OpenWeatherMap> for OpenWeatherMap {
    fn new<R>(mut readable_token: R, base_url: Option<Url>) -> io::Result<Self> where R: io::Read {
        let mut token = String::new();
        readable_token.read_to_string(&mut token)?;

        let url = match base_url {
            Some(val) => val,
            None => {
                match Url::parse(Self::BASE_URL) {
                    Ok(val) => val,
                    Err(e) => return Err(io::Error::from(io::ErrorKind::InvalidInput))
                }
            }
        };

        Ok(
            OpenWeatherMap {
                token,
                url,
                query: QueryBuilder::empty(),
            }
        )
    }

    fn get_current(city_name: String) -> Weather {
        unimplemented!()
    }

    fn get_forecast<R>(city_name: String, days: u64) -> R where R: ForecastIterator {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
struct QueryBuilder {
    pairs: HashMap<String, String>
}

impl QueryBuilder {
    pub fn empty() -> Self {
        QueryBuilder {
            pairs: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: &String, value: &String) {
        self.pairs.insert(key.to_string(), value.to_string());
    }

    pub fn build(self) -> Option<String> {
        let mut url = Url::parse("https://example.com").unwrap();
        for (key, value) in self.pairs {
            url.query_pairs_mut().append_pair(key.as_ref(), value.as_ref());
        }

        match url.query() {
            Some(val) => Some(val.to_string()),
            _ => None
        }
    }
}
