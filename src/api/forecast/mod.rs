use super::{Coordinates, Clouds, Main, WeatherResponse, Wind};

#[derive(Debug, Deserialize)]
pub struct Response {
    cod: String,
    message: f64,
    cnt: u64,
    list: Vec<WeatherEntry>,
    city: City
}

#[derive(Debug, Deserialize)]
struct System {
    pod: String
}

#[derive(Debug, Deserialize)]
struct WeatherEntry {
    dt: u64,
    main: Main,
    weather: Vec<WeatherResponse>,
    clouds: Clouds,
    wind: Wind,
    sys: System,
    dt_txt: String
}

#[derive(Debug, Deserialize)]
struct City {
    id: u64,
    name: String,
    coord: Coordinates,
    country: String
}
