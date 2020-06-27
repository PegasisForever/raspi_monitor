use std::fs;
use failure::Error;
use json_minimal::*;
use std::env;
use std::process::Command;
use systemstat::{System, Platform, saturating_sub_bytes};

fn read_file_as_float(name: &str) -> Result<f32, Error> {
    let content = fs::read_to_string(name)?;
    let parsed = content.trim().parse::<f32>()?;

    Ok(parsed)
}

fn read_file(name: &str) -> Result<String, Error> {
    let content = fs::read_to_string(name)?;
    Ok(content)
}

fn add_json_f32(json: &mut Json, key: &str, value: f32) {
    json.add(
        Json::OBJECT {
            name: String::from(key),
            value: Box::new(
                Json::NUMBER(value)
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
        let sys = System::new();
        let mut json = Json::new();
        add_json_f32(&mut json, "v", 1.0);

        if let Ok(temp) = sys.cpu_temp() {
            add_json_f32(&mut json, "cpu_temp", temp);
        }
        if let Ok(mem) = sys.memory() {
            let mem_used = (saturating_sub_bytes(mem.total, mem.free).as_u64() as f32) / 1024.0 / 1024.0;
            let mem_total = (mem.total.as_u64() as f32) / 1024.0 / 1024.0;
            add_json_f32(&mut json, "mem_used_mb", mem_used);
            add_json_f32(&mut json, "mem_total_mb", mem_total);
        }
        if let Ok(load) = sys.load_average() {
            add_json_f32(&mut json, "load_1", load.one);
            add_json_f32(&mut json, "load_5", load.five);
            add_json_f32(&mut json, "load_15", load.fifteen);
        }
        {
            let stdout = read_file("/proc/stat").unwrap();
            let first_line: &str = stdout.split("\n").next().unwrap();
            let cpu_times: Vec<f32> = first_line.split(" ")
                .skip(2)
                .map(|num| num.parse::<f32>().unwrap())
                .collect();
            let idle_time = cpu_times[3];
            let total_time = cpu_times.iter().sum::<f32>();
            add_json_f32(&mut json, "cpu_idle_time", idle_time);
            add_json_f32(&mut json, "cpu_total_time", total_time);
        }
        if let Ok(cpu_hertz) = read_file_as_float("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq") {
            add_json_f32(&mut json, "cpu_mhz", cpu_hertz / 1024.0);
        }
        if let Ok(cpu_min_hertz) = read_file_as_float("/sys/devices/system/cpu/cpu0/cpufreq/scaling_min_freq") {
            add_json_f32(&mut json, "cpu_min_mhz", cpu_min_hertz / 1024.0);
        }
        if let Ok(cpu_max_hertz) = read_file_as_float("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq") {
            add_json_f32(&mut json, "cpu_max_mhz", cpu_max_hertz / 1024.0);
        }

        println!("{}", json.print());
    }
}
