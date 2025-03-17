use crate::models::WeatherResponse;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct GeocodingResult{
    results: Option<Vec<Location>>
}

#[derive(Debug, Deserialize, Clone)]
struct Location {
    latitude: f64,
    longitude: f64,
    name: String,
}

pub fn get_weather(city: &str, unit: &str) -> Result<(WeatherResponse, String), Box<dyn Error>> {
    // Step 1: Geocode the city name to get coordinates
    let geocoding_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
        city
    );
    let geocoding_resp = reqwest::blocking::get(&geocoding_url)?
        .json::<GeocodingResult>()?;

    // Extract coordinates from the first result
    let location = match geocoding_resp.results {
        Some(results) if !results.is_empty() => results[0].clone(),
        _ => return Err("City not found".into()),
    };

    // For simplicity, we'll use a geocoding approach that's directly supported by Open-Meteo
    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,precipitation,rain,showers,snowfall,weather_code,cloud_cover,pressure_msl,wind_speed_10m,wind_direction_10m,wind_gusts_10m&timezone=auto&forecast_days=1&temperature_unit={}",
        location.latitude,
        location.longitude,
        if unit == "metric" { "celsius" } else { "fahrenheit" }
    );

    // Make the request and parse the response
    let response = reqwest::blocking::get(&weather_url)?
        .json::<WeatherResponse>()?;
    Ok((response, location.name))
}