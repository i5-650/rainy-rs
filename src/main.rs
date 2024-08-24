use std::{println, format, env};

use serde_json::Value;


#[derive(Debug)]
struct CurrentWeather {
    temperature: String,
    precipitaion: String,
    rain :String,
    wind :String,
    wind_direction: String
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You must provide a city as an argument:\nrainy London");
        return;
    }

    let city_name = &args[1];
    let (lat, long) = fetch_city_position(city_name).await;
    let values = fetch_weather(lat, long).await;
    if values.is_some() {
        let data = map_data(values.unwrap());
        println!("Temperature: {}", data.temperature);
        println!("Précipitaion: {}", data.precipitaion);
        println!("Pluie: {}", data.rain);
        println!("Vent: {}", data.wind);
        println!("Direction du vent: {}", data.wind_direction);
    } else {
        println!("[Err] No data read");
    }
}


async fn fetch_city_position(city_name :&String) -> (f64, f64) {
    let url = format!("https://geocode.maps.co/search?q={}", city_name);

    let resp = reqwest::get(url).await;

    let content = match resp {
        Ok(response) => response,
        Err(e) => {
            println!("[Err] Failed to get a response: {}", e);
            return (0.0,0.0);
        }
    };

    let json_data = match content.text().await {
        Ok(json_content) => json_content,
        Err(e) => {
            println!("[Err] Request content failed to load: {}", e);
            return (0.0, 0.0);
        }
    };

    let parsed = match serde_json::from_str::<Value>(json_data.as_str()) {
        Ok(data) => data,
        Err(e) => {
            println!("[Err] Failed to parse request content: {}", e);
            return (0.0, 0.0);
        }
    };

    let lat = remove_quote(&parsed[0]["lat"]).parse::<f64>().expect("Failed to parse latitude");
    let lon = remove_quote(&parsed[0]["lon"]).parse::<f64>().expect("Failed to parse longitude");


    (lat, lon)
}

async fn fetch_weather(lat: f64, long: f64) -> Option<Value> {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,precipitation,rain,windspeed_10m,winddirection_10m&timezone=Europe%2FBerlin", lat, long);
    let resp = reqwest::get(url).await;

    let content = match resp {
        Ok(response) => response,
        Err(e) => {
            println!("[Err] Failed to get a response: {}", e);
            return None;
        }
    };

    let json_data = match content.text().await {
        Ok(json_content) => json_content,
        Err(e) => {
            println!("Request content failed to load: {}", e);
            return None;
        }
    };

    let parsed = match serde_json::from_str::<Value>(json_data.as_str()) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to parse request content: {}", e);
            return None;
        }
    };


    Some(parsed)
}

fn map_data(data: Value) -> CurrentWeather {
    CurrentWeather {
        temperature: data["current"]["temperature_2m"].to_string() + remove_quote(&data["current_units"]["temperature_2m"]).as_str(),
        precipitaion: data["current"]["precipitation"].to_string() + remove_quote(&data["current_units"]["precipitation"]).as_str(),
        rain: data["current"]["rain"].to_string() + remove_quote(&data["current_units"]["rain"]).as_str(),
        wind: data["current"]["windspeed_10m"].to_string() + remove_quote(&data["current_units"]["windspeed_10m"]).as_str(),
        wind_direction: convert_to_compass(data["current"]["winddirection_10m"].as_i64().expect("Problem with wind direction"))
    }
}

#[inline(always)]
fn remove_quote(input: &Value) -> String {
    input.to_string().replace("\"", "").to_string()
}


#[inline(always)]
fn convert_to_compass(degree: i64) -> String {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let index = (degree as f32 / 45.0).round() as usize % 8;
    String::from(directions[index])
}
