extern crate serde;

use std::fmt;
use serde::de;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    id: i64,
    pub main: String,
    pub description: String,
    icon: String
}

#[derive(Debug, Deserialize)]
pub struct Clouds {
    all: f64
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub temp_min: f64,
    pub temp_max: f64,
    pub sea_level: Option<f64>,
    pub grnd_level: Option<f64>
}
