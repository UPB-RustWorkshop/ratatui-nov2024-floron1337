use chrono::{DateTime, Local};
use serde::Deserialize;


struct CityInfo {
    // TODO: define elements in the structure
    name: String,
    weather_list: Vec<Weather>
}

struct Weather{
    temp: f32,
    feels_like: f32,
    weather: String
}
 

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    cod: String,
    cnt: i32,
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
pub fn get_data(city: &String) -> String{
    let api_url = "https://api.openweathermap.org/data/2.5/forecast?q=bucharest&units=metric&cnt=7&appid=f50a4563082e03e30bca7f5a23ffb481";
    match reqwest::blocking::get(api_url) {
        Ok(response) => {
            let weather_response = response.json::<WeatherResponse>();
            let weather_list = weather_response
                                                                            .unwrap().list.iter()
                                                                            .map(|forecast|{
                                                                                Weather{
                                                                                    temp: forecast.main.temp,
                                                                                    feels_like: forecast.main.feels_like,
                                                                                    weather: forecast.weather.first().map_or("Unknown".to_string(), |w| w.main.clone()),
                                                                                }
                                                                            })
                                                                            .collect::<Vec<Weather>>();
            
            //TODO: Finish
            return String::from("");
            //return weather_response.unwrap().list[0].main.temp.to_string();
        },
        Err(error) => {
            // Handle error
            return String::from(error.to_string());
        }
    }
}