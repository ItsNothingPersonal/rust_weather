use anyhow::Result;
use chrono::Local;
use rust_weather::{ForeCast, RustWeather};
use type_cli::CLI;

#[tokio::main]
async fn main() -> Result<()> {
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
