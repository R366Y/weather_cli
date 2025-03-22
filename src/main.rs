mod api;
mod models;
mod display;
mod ascii_art;

use clap::{Parser, ValueEnum};
use std::fmt;
use std::fmt::Formatter;
use std::process;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Unit {
    Metric,
    Imperial,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Metric => write!(f, "metric"),
            Unit::Imperial => write!(f, "imperial")
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli{
    // city name (e.g. "London", "Rome")
    #[arg(short, long)]
    city: String,

    // Temperature unit
    #[arg(short, long, value_enum, default_value_t = Unit::Metric)]
    unit: Unit,
}


fn main() {
    let cli = Cli::parse();

    let unit_symbol = match cli.unit {
        Unit::Metric => "°C",
        Unit::Imperial => "°F",
    };

    match api::get_weather(&cli.city, &cli.unit.to_string()) {
        Ok((weather, city_name)) => {
            println!("Weather for: {}", city_name);
            display::display_weather(&weather, unit_symbol);
        },
        Err(e) => {
            eprintln!("Error fetching weather data: {}", e);
            process::exit(1);
        }
    }
}
