use anyhow::Result;
use rust_weather::{ForeCast, ForeCastClient, RustWeather};
use stybulate::{Cell, Headers, Style, Table};
use type_cli::CLI;

#[tokio::main]
async fn main() -> Result<()> {
    let RustWeather {
        api_key,
        zip,
        country,
    } = RustWeather::process();

    let mut forecast_client = ForeCastClient::default();
    let mut forecast_data: Vec<ForeCast> = Vec::new();

    for single_zip in zip.iter() {
        forecast_data.push(forecast_client.get(single_zip, &country, &api_key).await?);
    }

    let table_rows = forecast_data
        .iter()
        .map(|result| {
            let mut row = Vec::new();
            row.push(Cell::from(&result.name));
            row.push(Cell::from(&format!("{:.2} °", result.main.temp)));
            row.push(Cell::from(&format!("{:.2} °", result.main.temp_min)));
            row.push(Cell::from(&format!("{:.2} °", result.main.temp_max)));
            row
        })
        .collect();

    let table = Table::new(
        Style::FancyGithub,
        table_rows,
        Some(Headers::from(vec!["Location", "Now", "Min", "Max"])),
    );

    println!("{}", table.tabulate());

    Ok(())
}
