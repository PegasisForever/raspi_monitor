use std::fs;
use failure::Error;
use json_minimal::*;
use std::env;
use std::process::Command;

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

fn run_command(cmd: String) -> String {
    let mut command = Command::new("sh");
    command.arg("-c");
    command.arg(cmd);
    let stdout = command.output().unwrap().stdout;
    String::from_utf8(stdout).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&("info".into())) {
        print!("{}", run_command("uname -a".into()));
        print!("{}", run_command("grep -H -v ^# /etc/*release".into()));
    } else {
        let mut json = Json::new();
        add_json_f32(&mut json, "v", 1.0);

        if let Ok(temp) = read_file_as_float("/sys/class/thermal/thermal_zone0/temp") {
            add_json_f32(&mut json, "temp", temp / 1000.0);
        }

        println!("{}", json.print());
    }
}
