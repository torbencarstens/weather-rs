extern crate chrono;

use self::chrono::{DateTime, NaiveDateTime};
use self::chrono::prelude::Utc;
use super::{City, Clouds, Coordinates, Main, Rain, Temperature, WeatherResponse, Weather, Wind};

#[derive(Debug, Deserialize)]
pub struct Response {
    coord: Coordinates,
    sys: System,
    weather: Vec<WeatherResponse>,
    main: Main,
    base: String,
    visibility: f32,
    wind: Wind,
    rain: Option<Rain>,
    clouds: Clouds,
    dt: i64,
    name: String,
    cod: i64
}

impl Into<Weather> for Response {
    fn into(self) -> Weather {
        let city = City::new(0, self.name, self.coord, self.sys.country);
        let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(self.dt, 0), Utc);
        let sunrise_time = Some(DateTime::from_utc(NaiveDateTime::from_timestamp(self.sys.sunrise, 0), Utc));
        let sunset_time = Some(DateTime::from_utc(NaiveDateTime::from_timestamp(self.sys.sunset, 0), Utc));
        let weather: &WeatherResponse = self.weather.get(0).unwrap();
        let status = &weather.main;
        let description = &weather.description;
        let rain = self.rain;
        let wind = self.wind;
        let humidity = self.main.humidity;
        let temperature = Temperature::from_celcius(self.main.temp as f64);
        let temperature_min = Temperature::from_celcius(self.main.temp_min as f64);
        let temperature_max = Temperature::from_celcius(self.main.temp_max as f64);
        let sea_level = self.main.sea_level;
        let ground_level = self.main.grnd_level;

        Weather::new(
            city,
            timestamp,
            status.to_string(),
            description.to_string(),
            sunrise_time,
            sunset_time,
            rain,
            wind,
            humidity,
            temperature,
            temperature_min,
            temperature_max,
            sea_level,
            ground_level
        )
    }
}

#[derive(Debug, Deserialize)]
struct System {
    country: String,
    sunrise: i64,
    sunset: i64
}
