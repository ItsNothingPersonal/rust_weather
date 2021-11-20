use chrono::Local;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use type_cli::CLI;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let RustWeather {
        api_key,
        zip,
        country,
    } = RustWeather::process();

    let res = ForeCast::get(&zip, &country, &api_key).await?;
    
    println!(
        "{}",
        format!(
            "Weather in {location} {icon}",
            location = res.name,
            icon = "\u{1F308}"
        )
    );
    println!("---------------------------------------");
    println!("{}", Local::now().format("%d.%m.%Y %H:%M:%S"));
    println!(
        "{}",
        format!(
            "Now: {temp_current} °C, Min: {temp_min} °C, Max: {temp_max} °C",
            temp_current = res.main.temp,
            temp_min = res.main.temp_min,
            temp_max = res.main.temp_max,
        )
    );

    Ok(())
}

#[derive(CLI)]
struct RustWeather {
    #[named(short = "a")]
    api_key: String,

    #[named(short = "z")]
    zip: String,

    #[named(short = "c")]
    country: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ForeCast {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: i64,
    wind: Wind,
    clouds: Clouds,
    dt: f64,
    sys: Sys,
    id: i32,
    name: String,
    cod: f64,
}

impl ForeCast {
    async fn get(zip: &str, country: &str, api_key: &str) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?zip={zip},{country}&units=metric&appid={api_key}",
            zip = zip,
            country = country,
            api_key = api_key,
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<ForeCast>().await?;

        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f64,
    pressure: f64,
    humidity: f64,
    #[serde(alias = "tempMin")]
    temp_min: f64,
    #[serde(alias = "tempMax")]
    temp_max: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    #[serde(alias = "type")]
    sys_type: i32,
    id: i32,
    message: Option<f64>,
    country: String,
    sunrise: i32,
    sunset: i32,
}
