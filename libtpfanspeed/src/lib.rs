use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{ErrorKind, Read, Write},
};

use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct CoreTemperature {
    pub temp: u8,
    pub max: u8,
    pub critical: u8,
}

#[derive(Debug, Clone)]
pub struct Temperatures {
    pub avg: u8,
    pub cores: HashMap<String, CoreTemperature>,
}

#[derive(Debug, Clone, Copy)]
pub enum FanSpeed {
    Level(u8),
    FullSpeed,
    Disengaged,
    Auto,
}

impl CoreTemperature {
    pub fn new(temp: u8, max: u8, critical: u8) -> Self {
        Self {
            temp,
            max,
            critical,
        }
    }
}

impl Temperatures {
    pub fn new() -> Self {
        Self {
            avg: 0,
            cores: HashMap::new(),
        }
    }
}

impl ToString for FanSpeed {
    fn to_string(&self) -> String {
        use FanSpeed as F;

        match &self {
            F::Auto => "auto".to_string(),
            F::Disengaged => "disengaged".to_string(),
            F::Level(val) => val.to_string(),
            F::FullSpeed => "full-speed".to_string(),
        }
    }
}

pub fn set_fanspeed(fs: FanSpeed) {
    let err = OpenOptions::new()
        .append(true)
        .read(true)
        .open("/proc/acpi/ibm/fan");
    let mut file = match err {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("file not found! are you not using a ThinkPad?"),
            ErrorKind::PermissionDenied => panic!("permission denied"),
            _ => panic!("{}", e),
        },
        Ok(f) => f,
    };

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    if !s.contains("command") {
        panic!("no way to control the fan speed, did you set your kernel parameters correctly?");
    }

    let err = file.write(format!("level {}", fs.to_string()).as_bytes());

    match err {
        Ok(_) => (),
        Err(e) => match e.kind() {
            ErrorKind::InvalidInput => {
                panic!("failed to set the fan speed! did you set your kernel parameters correctly?")
            }
            _ => panic!("{e}"),
        },
    }
}

pub fn get_rpm() -> u16 {
    let err = OpenOptions::new().read(true).open("/proc/acpi/ibm/fan");
    let mut file = match err {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("file not found! are you not using a ThinkPad?"),
            ErrorKind::PermissionDenied => panic!("permission denied"),
            _ => panic!("{}", e),
        },
        Ok(f) => f,
    };

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    let rpm = s.split('\n').collect::<Vec<&str>>()[1] // get second line
        .split_once(":")
        .unwrap()
        .1
        .trim();

    rpm.parse::<u16>().unwrap()
}

pub fn get_temps() -> Temperatures {
    let output = std::process::Command::new("sensors")
        .arg("-j")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut res = Temperatures::new();

    let json_value: serde_json::Value =
        serde_json::from_str(&stdout).expect("failed to parse JSON");

    res.avg = json_value
        .get("thinkpad-isa-0000")
        .expect("failed to find key")
        .get("CPU")
        .expect("failed to find key")
        .get("temp1_input")
        .expect("failed to find key")
        .as_f64()
        .expect("expected a float, got something else") as u8;

    let coretemps = json_value
        .get("coretemp-isa-0000")
        .expect("failed to find key")
        .as_object()
        .expect("failed to find key");

    for (key, value) in coretemps {
        let k = key.as_str();

        if !k.contains("Core ") {
            continue;
        }

        let coreid = k
            .split_once(" ")
            .expect("expected Core and an ID")
            .1
            .parse::<u8>()
            .expect("ID should be a valid number");

        let temp = value
            .get(format!("temp{}_input", coreid + 2))
            .expect("core id somehow invalid?")
            .as_f64()
            .expect("expected a float, got something else") as u8;

        let max = value
            .get(format!("temp{}_max", coreid + 2))
            .expect("core id somehow invalid?")
            .as_f64()
            .expect("expected a float, got something else") as u8;

        let critical = value
            .get(format!("temp{}_crit", coreid + 2))
            .expect("core id somehow invalid?")
            .as_f64()
            .expect("expected a float, got something else") as u8;

        let coretemp = CoreTemperature::new(temp, max, critical);

        res.cores.insert(k.to_owned(), coretemp);
    }

    res
}

pub fn get_cores() -> Vec<u8> {
    // modern ThinkPads may have weird core ids

    let output = std::process::Command::new("sensors")
        .arg("-j")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    let json_value: Value = serde_json::from_str(&stdout).unwrap();

    let coretemps = json_value
        .get("coretemp-isa-0000")
        .expect("failed to find key")
        .as_object()
        .expect("failed to find key");

    let mut res = Vec::new();

    for (key, value) in coretemps {
        let k = key.as_str();

        if !k.contains("Core ") {
            continue;
        }

        let coreid = k
            .split_once(" ")
            .expect("expected Core and an ID")
            .1
            .parse::<u8>()
            .expect("ID should be a valid number");

        res.push(coreid)
    }

    res
}
