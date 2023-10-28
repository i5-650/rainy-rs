use serde_json::Value;
use reqwest;


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
    let lat = 45.750000;
    let long = 4.850000;
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

async fn fetch_weather(lat: f32, long: f32) -> Option<Value> {
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


    return Some(parsed);
}

fn map_data(data: Value) -> CurrentWeather {
    return CurrentWeather {
        temperature: data["current"]["temperature_2m"].to_string() + remove_quote(&data["current_units"]["temperature_2m"]).as_str(),
        precipitaion: data["current"]["precipitation"].to_string() + remove_quote(&data["current_units"]["precipitation"]).as_str(),
        rain: data["current"]["rain"].to_string() + remove_quote(&data["current_units"]["rain"]).as_str(),
        wind: data["current"]["windspeed_10m"].to_string() + remove_quote(&data["current_units"]["windspeed_10m"]).as_str(),
        wind_direction: convert_to_compass(data["current"]["winddirection_10m"].as_i64().expect("Problem with wind direction"))
    };
}

fn remove_quote(input: &Value) -> String {
    return input.to_string().replace("\"", "").to_string();
}



fn convert_to_compass(degree: i64) -> String {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let index = (degree as f32 / 45.0).round() as usize % 8;
    return String::from(directions[index]);
}
