use anyhow::Result;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use type_cli::CLI;

#[derive(CLI)]
pub struct RustWeather {
    #[named(short = "a")]
    pub api_key: String,

    #[named(short = "z")]
    pub zip: String,

    #[named(short = "c")]
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForeCast {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    pub main: Main,
    visibility: i64,
    wind: Wind,
    clouds: Clouds,
    dt: f64,
    sys: Sys,
    id: i32,
    pub name: String,
    cod: f64,
}

impl ForeCast {
    pub async fn get(zip: &str, country: &str, api_key: &str) -> Result<Self> {
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
pub struct Main {
    pub temp: f64,
    pressure: f64,
    humidity: f64,
    #[serde(alias = "tempMin")]
    pub temp_min: f64,
    #[serde(alias = "tempMax")]
    pub temp_max: f64,
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
