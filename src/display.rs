use crate::models::{weather_code_to_description, WeatherResponse};
use colored::*;

pub fn display_weather(weather: &WeatherResponse, unit_symbol: &str) {
    println!("\n{}", "Current Weather".bold().underline());

    // Weather conditions based on weather code
    let description = weather_code_to_description(weather.current.weather_code);
    println!("Conditions: {}", description.to_string().cyan());

    // Temperature information
    println!(
        "Temperature: {}{}",
        weather.current.temperature_2m.to_string().yellow().bold(),
        unit_symbol
    );
    println!("Feels like: {}{}",
             weather.current.apparent_temperature.to_string().yellow(),
             unit_symbol);

    // Precipitation information
    println!("Precipitation: {} mm",
             weather.current.precipitation.to_string().blue());
    if weather.current.rain > 0.0 {
        println!("Rain: {} mm", weather.current.rain.to_string().blue());
    }
    if weather.current.snowfall > 0.0 {
        println!("Snowfall: {} cm", weather.current.snowfall.to_string().white().bold());
    }

    // Cloud cover and humidity
    println!("Cloud cover: {}%",
             weather.current.cloud_cover.to_string());
    println!("Humidity: {}%",
             weather.current.relative_humidity_2m.to_string().green());

    // Pressure
    println!("Pressure: {} hPa",
             weather.current.pressure_msl.to_string());

    // Wind information
    println!("Wind: {} km/h at {}°",
             weather.current.wind_speed_10m.to_string(),
             weather.current.wind_direction_10m.to_string());
    println!("Wind gusts: {} km/h",
             weather.current.wind_gusts_10m.to_string());
}
