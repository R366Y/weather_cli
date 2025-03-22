use crate::models::{weather_code_to_description, WeatherResponse};
use crate::ascii_art::get_weather_art;
use colored::*;

pub fn display_weather(weather: &WeatherResponse, unit_symbol: &str) {
    println!("\n{}", "Current Weather".bold().underline());

    // Weather conditions based on weather code
    let weather_code = weather.current.weather_code;
    let ascii_art = get_weather_art(weather_code);
    let description = weather_code_to_description(weather_code);

    // Print ASCII art for current weather
    let colored_art = match weather_code {
        0 => ascii_art.yellow(),                  // Clear sky - yellow
        1..=3 => ascii_art.white(),               // Partly cloudy - white
        45 | 48 => ascii_art.white().dimmed(),    // Fog - dimmed white
        51..=67 => ascii_art.blue(),              // Drizzle and rain - blue
        71..=77 => ascii_art.white().bold(),      // Snow - bold white
        80..=82 => ascii_art.bright_blue(),       // Rain showers - bright blue
        85..=86 => ascii_art.bright_white(),      // Snow showers - bright white
        95..=99 => ascii_art.bright_yellow(),     // Thunderstorm - bright yellow
        _ => ascii_art.normal()                   // Unknown - normal color
    };

    println!("{}", colored_art);
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
