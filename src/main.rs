use std::process::{Command, Stdio};
use regex::Regex;

fn main() {

    let output = Command::new("ioreg")
                     .arg("-rc")
                     .arg("AppleSmartBattery")
                     .stdout(Stdio::piped())
                     .output()
                     .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let mut re = Regex::new(r#""CurrentCapacity" = (\d+)"#).unwrap();
    let mut caps = re.captures(&stdout).unwrap();
    let current = caps.get(1).map_or("", |m| m.as_str());

    re = Regex::new(r#""MaxCapacity" = (\d+)"#).unwrap();
    caps = re.captures(&stdout).unwrap();
    let maximum = caps.get(1).map_or("", |m| m.as_str());

    let pct = (100.0 * 
            current.parse::<f32>().unwrap() / maximum.parse::<f32>().unwrap()
            ) as i32;

    let color_green = "%{[32m%}";
    let color_yellow = "%{[1;33m%}";
    let color_red = "%{[31m%}";
    let color_reset = "%{[00m%}";
    let color = match pct {
        0..=33 =>  color_red,
        34..=60 =>  color_yellow,
        _ => color_green
    };

    println!("{}{}{}",color, pct, color_reset);

}