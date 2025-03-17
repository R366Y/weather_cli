use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub generationtime_ms: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub utc_offset_seconds: i32,
    pub current: CurrentWeather,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub time: String,
    pub temperature_2m: f64,
    pub relative_humidity_2m: i32,
    pub apparent_temperature: f64,
    pub precipitation: f64,
    pub rain: f64,
    pub showers: f64,
    pub snowfall: f64,
    pub weather_code: i32,
    pub cloud_cover: i32,
    pub pressure_msl: f64,
    pub wind_speed_10m: f64,
    pub wind_direction_10m: i32,
    pub wind_gusts_10m: f64,
}

// Helper function to convert weather code to description
pub fn weather_code_to_description(code: i32) -> &'static str {
    match code {
        0 => "Clear sky",
        1..=3 => "Partly cloudy",
        45 | 48 => "Fog",
        51..=55 => "Drizzle",
        56..=57 => "Freezing drizzle",
        61..=65 => "Rain",
        66..=67 => "Freezing rain",
        71..=77 => "Snow",
        80..=82 => "Rain showers",
        85..=86 => "Snow showers",
        95..=99 => "Thunderstorm",
        _ => "Unknown"
    }
}