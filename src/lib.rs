use anyhow::Result;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, type_cli::CLI)]
#[help = "rust_weather - a small cli tool to get weather data for zip codes"]
pub struct RustWeather {
    #[named(short = "a")]
    #[help = "the api key from https://openweathermap.org"]
    pub api_key: String,

    #[variadic]
    #[help = "zip codes you want to get data for"]
    pub zip: Vec<String>,

    #[named(short = "c")]
    #[help = "the country code of the zip codes, e.g. de"]
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

pub struct ForeCastClient {
    client: reqwest::Client,
}

impl ForeCastClient {
    pub fn new() -> Self {
        ForeCastClient {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&mut self, zip: &str, country: &str, api_key: &str) -> Result<ForeCast> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?zip={zip},{country}&units=metric&appid={api_key}",
            zip = zip,
            country = country,
            api_key = api_key,
        );

        let url_parsed = Url::parse(&url).unwrap();
        let res = self
            .client
            .get(url_parsed)
            .send()
            .await?
            .json::<ForeCast>()
            .await?;

        Ok(res)
    }
}

impl Default for ForeCastClient {
    fn default() -> Self {
        ForeCastClient::new()
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
