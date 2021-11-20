use type_cli::CLI;

fn main() {
    let RustWeather { zip, country } = RustWeather::process();
    eprint!("zip:{}, country:{} ", zip, country)
}

#[derive(CLI)]
struct RustWeather {
    #[named(short = "z")]
    zip: String,

    #[named(short = "c")]
    country: String,
}
