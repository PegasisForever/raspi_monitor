use std::fs;
use failure::Error;
use json_minimal::*;

fn read_file_as_float(name: &str) -> Result<f32, Error> {
    let content = fs::read_to_string(name)?;
    let parsed = content.trim().parse::<f32>()?;

    Ok(parsed)
}

fn add_json_f32(json: &mut Json, key: &str, value: f32) {
    json.add(
        Json::OBJECT {
            name: String::from(key),
            value: Box::new(
                Json::NUMBER(value as f64)
            ),
        }
    );
}

fn main() {
    let mut json = Json::new();
    add_json_f32(&mut json, "v", 1.0);

    if let Ok(temp) = read_file_as_float("/sys/class/thermal/thermal_zone0/temp") {
        add_json_f32(&mut json, "temp", temp / 1000.0);
    }

    println!("{}", json.print());
}
