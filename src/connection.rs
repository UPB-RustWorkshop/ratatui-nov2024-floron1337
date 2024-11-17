use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CityInfo {
    // TODO: define elements in the structure
    pub weather_list: Vec<Weather>
}

#[derive(Debug, Deserialize)]
pub struct Weather{
    pub temp: f32,
    pub feels_like: f32,
    pub weather: String
}
 

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    list: Vec<WeatherResponseListElement>,
}

#[derive(Debug, Deserialize)]
struct WeatherResponseListElement {
    main: WeatherResponseListElementMain,
    weather: Vec<WeatherResponseListElementWeatherElement>,  // This should be a Vec
}

#[derive(Debug, Deserialize)]
struct WeatherResponseListElementMain {
    temp: f32,
    feels_like: f32,
}

#[derive(Debug, Deserialize)]
struct WeatherResponseListElementWeatherElement {
    main: String,  // This field corresponds to a weather condition (e.g., "Clear", "Rain")
}

/// Method that is handling the request to the OpenWeather api
/// and parsing the response
///
/// Returns weather details about a certain city
pub fn get_data(city: &String) -> Result<CityInfo, String> {
    let api_url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&units=metric&cnt=7&appid=f50a4563082e03e30bca7f5a23ffb481",
        city
    );

    match reqwest::blocking::get(&api_url) {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<WeatherResponse>() {
                    Ok(weather_response) => {
                        let weather_list = weather_response
                            .list
                            .iter()
                            .map(|forecast| Weather {
                                temp: forecast.main.temp,
                                feels_like: forecast.main.feels_like,
                                weather: forecast
                                    .weather
                                    .first()
                                    .map_or("Unknown".to_string(), |w| w.main.clone()),
                            })
                            .collect::<Vec<Weather>>();

                        Ok(CityInfo { weather_list })
                    }
                    Err(_) => Err("Failed to parse the weather data.".to_string()),
                }
            } else {
                Err(format!(
                    "Failed to fetch data: HTTP {}",
                    response.status().as_u16()
                ))
            }
        }
        Err(_) => Err("Failed to connect to the API.".to_string()),
    }
}