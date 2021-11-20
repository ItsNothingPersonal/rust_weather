# rust_weather

![Build](https://github.com/ItsNothingPersonal/rust_weather/actions/workflows/rust.yml/badge.svg)
![Security Audit](https://github.com/ItsNothingPersonal/rust_weather/actions/workflows/security_audit.yml/badge.svg)

rust_weather is a small cli tool that pulls weatherdata from https://openweathermap.org for your local area.
This is my rust version of the dart program from [this article by Marvin Knabe](https://medium.com/@m_knabe/create-a-command-line-app-with-dart-8633d3d4a437).

## Installation

1. Create an openweathermap.org account (free tier is enough) over at https://openweathermap.org
2. Install rust for your operating system from https://www.rust-lang.org/tools/install
3. Clone the repository
4. Run cargo to build the project

```bash
cargo build --release
```

5. Run the resulting binary and pass the required parameters

```bash
rust_weather.exe --zip <your-zip-code> --country <your-country-code> --api-key <your-api-key>
```

6. Profit

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
