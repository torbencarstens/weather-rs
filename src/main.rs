extern crate weather;

use weather::{OpenWeatherMap, WeatherApi};

fn main() {
    let mut owm = OpenWeatherMap::new("<TOKEN>".as_bytes(), None).unwrap();
    let weather = owm.get_city_weather(String::from("Darmstadt"));
    println!("{:#?}", weather);
}
