use std::fs;
use failure::Error;
use json_minimal::*;
use std::env;
use std::process::Command;
use systemstat::{System, Platform, saturating_sub_bytes};
use std::time::{SystemTime, UNIX_EPOCH};
use fs2::statvfs;

fn read_file_as_float(name: &str) -> Result<f32, Error> {
    let content = fs::read_to_string(name)?;
    let parsed = content.trim().parse::<f32>()?;

    Ok(parsed)
}

fn read_file(name: &str) -> Result<String, Error> {
    let content = fs::read_to_string(name)?;
    Ok(content)
}

fn add_json_f64(json: &mut Json, key: &str, value: f64) {
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

fn get_current_millis() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&("info".into())) {
        print!("{}", run_command("uname -a".into()));
        print!("{}", run_command("grep -hv ^# /etc/*release".into()));
    } else {
        let sys = System::new();
        let mut json = Json::new();
        add_json_f64(&mut json, "v", 1.0);
        add_json_f64(&mut json, "time", get_current_millis());

        if let Ok(temp) = sys.cpu_temp() {
            add_json_f64(&mut json, "cpu_temp", temp as f64);
        }
        if let Ok(mem) = sys.memory() {
            let mem_used = (saturating_sub_bytes(mem.total, mem.free).as_u64() as f64) / 1024.0;
            let mem_total = (mem.total.as_u64() as f64) / 1024.0;
            add_json_f64(&mut json, "mem_used_kb", mem_used);
            add_json_f64(&mut json, "mem_total_kb", mem_total);
        }
        if let Ok(load) = sys.load_average() {
            add_json_f64(&mut json, "load_1", load.one as f64);
            add_json_f64(&mut json, "load_5", load.five as f64);
            add_json_f64(&mut json, "load_15", load.fifteen as f64);
        }
        {
            let file_content = read_file("/proc/stat").unwrap();
            let first_line: &str = file_content.split("\n").next().unwrap();
            let cpu_times: Vec<f64> = first_line.split_whitespace()
                .skip(1)
                .map(|num| num.parse::<f64>().unwrap())
                .collect();
            let idle_time = cpu_times[3];
            let total_time = cpu_times.iter().sum::<f64>();
            add_json_f64(&mut json, "cpu_idle_time", idle_time);
            add_json_f64(&mut json, "cpu_total_time", total_time);
        }
        if let Ok(cpu_hertz) = read_file_as_float("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq") {
            add_json_f64(&mut json, "cpu_mhz", (cpu_hertz as f64) / 1024.0);
        }
        if let Ok(cpu_min_hertz) = read_file_as_float("/sys/devices/system/cpu/cpu0/cpufreq/scaling_min_freq") {
            add_json_f64(&mut json, "cpu_min_mhz", (cpu_min_hertz as f64) / 1024.0);
        }
        if let Ok(cpu_max_hertz) = read_file_as_float("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq") {
            add_json_f64(&mut json, "cpu_max_mhz", (cpu_max_hertz as f64) / 1024.0);
        }
        {
            let file_content = read_file("/proc/net/dev").unwrap();
            let mut total_received_bytes = 0.0;
            let mut total_sent_bytes = 0.0;
            file_content.split("\n")
                .skip(2)
                .for_each(|line: &str| {
                    let nums = line.split_whitespace().collect::<Vec<&str>>();
                    if nums.is_empty() || nums[0] == "lo:" { return; }
                    let mid = nums.len() / 2;
                    total_received_bytes += nums[1].parse::<f64>().unwrap();
                    total_sent_bytes += nums[mid + 1].parse::<f64>().unwrap();
                });
            add_json_f64(&mut json, "received_bytes", total_received_bytes);
            add_json_f64(&mut json, "sent_bytes", total_sent_bytes);
        }
        if let Ok(file_content) = read_file("/proc/swaps") {
            let mut swap_total_kb = 0.0;
            let mut swap_used_kb = 0.0;
            file_content.trim()
                .split("\n")
                .skip(1)
                .for_each(|line: &str| {
                    let mut iter = line
                        .split_whitespace()
                        .skip(2);
                    swap_total_kb += iter.next().unwrap().parse::<f64>().unwrap();
                    swap_used_kb = iter.next().unwrap().parse::<f64>().unwrap();
                });
            add_json_f64(&mut json, "swap_total_kb", swap_total_kb);
            add_json_f64(&mut json, "swap_used_kb", swap_used_kb);
        }
        if let Ok(file_content) = read_file("/proc/diskstats") {
            let mut total_disk_read_kb = 0.0;
            let mut total_disk_write_kb = 0.0;
            let mut last_device = "aaaa";
            file_content.trim()
                .split("\n")
                .map(|line: &str| {
                    line.split_whitespace()
                        .collect::<Vec<&str>>()
                })
                .for_each(|line: Vec<&str>| {
                    let name = line[2];
                    if !name.starts_with("loop") &&
                        !name.starts_with("ram") &&
                        !name.starts_with(last_device) {
                        last_device = name;
                        total_disk_read_kb += line[5].parse::<f64>().unwrap() / 2.0;
                        total_disk_write_kb += line[9].parse::<f64>().unwrap() / 2.0;
                    }
                });
            add_json_f64(&mut json, "total_disk_read_kb", total_disk_read_kb);
            add_json_f64(&mut json, "total_disk_write_kb", total_disk_write_kb);
        }
        if let Ok(fs_stats) = statvfs("/") {
            add_json_f64(&mut json, "root_used_kb", (fs_stats.total_space() - fs_stats.free_space()) as f64 / 1024.0);
            add_json_f64(&mut json, "root_total_kb", fs_stats.total_space() as f64 / 1024.0);
        }

        println!("{}", json.print());
    }
}
