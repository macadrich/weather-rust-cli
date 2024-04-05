use serde::{Deserialize, Serialize};
use prettytable::{Table, Row, Cell};

#[derive(Debug, Serialize, Deserialize)]
struct WeatherResponse {
    #[serde(default)]
    main: Option<Main>,
    wind: Option<Wind>,
    clouds: Option<Clouds>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    main: Main,
    wind: Wind,
    clouds: Clouds,
}

#[derive(Debug, Serialize, Deserialize)]
struct Main {
    temp: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Wind {
    speed: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Clouds {
    all: i32,
}

pub async fn weather(api_key: &str, city: &str) -> Result<(), Box<dyn std::error::Error>>{
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=imperial",
        city, api_key
    );

    let response_text = reqwest::get(&url).await?.text().await?;
    let json_data: WeatherResponse = serde_json::from_str(&response_text)?;

    let mut table = Table::new();
    let mut rows = Vec::new();
    let mut row_val = Vec::new();

    if let Some(main) = json_data.main {
        rows.push(Cell::new("City"));
        rows.push(Cell::new("Temperature (°F)"));

        row_val.push(Cell::new(city));
        row_val.push(Cell::new(&main.temp.to_string()));
    } else {
        println!("Main data not available for {}", city);
    }

    if let Some(wind) = json_data.wind {
        rows.push(Cell::new("Wind"));
        row_val.push(Cell::new(&wind.speed.to_string()));
    } else {
        println!("Wind data not available for {}", city);
    }
    
    if let Some(clouds) = json_data.clouds {
        rows.push(Cell::new("Clouds"));
        row_val.push(Cell::new(&clouds.all.to_string()));
    } else {
        println!("Clouds data not available for {}", city);
    }

    table.add_row(Row::new(rows));
    table.add_row(Row::new(row_val));
    table.printstd();

    Ok(())
}