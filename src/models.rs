extern crate chrono;

use std::fmt;
use self::chrono::DateTime;
use self::chrono::prelude::*;
use super::serde::de;

#[derive(Debug)]
pub struct Temperature {
    celcius_value: f64
}

impl Temperature {
    pub fn from_celcius(value: f64) -> Self {
        Temperature {
            celcius_value: value
        }
    }

    pub fn from_fahrenheit() -> Self {
        unimplemented!()
    }

    pub fn from_kelvin() -> Self {
        unimplemented!()
    }

    pub fn as_celsius(&self) -> f64 {
        self.celcius_value
    }

    pub fn as_fahrenheit(&self) -> f64 {
        unimplemented!()
    }

    pub fn as_kelvin(&self) -> f64 {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    lon: f64,
    lat: f64
}

#[derive(Debug)]
pub struct City {
    id: u64,
    name: String,
    coord: Coordinates,
    country: String
}

impl City {
    pub fn new(id: u64, name: String, coord: Coordinates, country: String) -> Self {
        City { id, name, coord, country }
    }
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    speed: f32,
    deg: f32
}

impl Wind {
    pub fn new(speed: f32, deg: f32) -> Self {
        Wind { speed, deg }
    }
}

#[derive(Debug, Deserialize)]
pub struct Rain {
    volume: f32
}

impl Rain {
    pub fn new(volume: f32) -> Self {
        Rain { volume }
    }
}

impl<'de> de::Visitor<'de> for Rain {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A float value for the rain volume in the last 3 hours")
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where
        E: de::Error, {
        Ok(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where
        E: de::Error, {
        Ok(v)
    }
}

#[derive(Debug)]
pub struct Weather {
    city: City,
    timestamp: DateTime<Utc>,
    status: String,
    description: String,
    sunrise_time: Option<DateTime<Utc>>,
    sunset_time: Option<DateTime<Utc>>,
    rain: Option<Rain>,
    wind: Wind,
    humidity: f32,
    temperature: Temperature,
    temperature_min: Temperature,
    temperature_max: Temperature,
    sea_level: Option<f64>,
    ground_level: Option<f64>,
}

impl Weather {
    pub fn new(city: City, timestamp: DateTime<Utc>, status: String, description: String, sunrise_time: Option<DateTime<Utc>>,
               sunset_time: Option<DateTime<Utc>>, rain: Option<Rain>, wind: Wind, humidity: f32, temperature: Temperature, temperature_min: Temperature,
               temperature_max: Temperature, sea_level: Option<f64>, ground_level: Option<f64>) -> Self {
        Weather {
            city, timestamp, status, description, sunrise_time,
            sunset_time, rain, wind, humidity, temperature, temperature_min,
            temperature_max, sea_level, ground_level
        }
    }
}

pub trait ForecastIterator: Iterator {
    type Item = Forecast;
}

#[derive(Debug)]
pub struct Forecast {
    timestamp: DateTime<Utc>,
    weathers: Vec<Weather>
}
