pub mod error;

use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{self, Read, Write},
};

use error::*;

use serde_json::Value;

#[derive(Debug, Clone, Copy, Default)]
pub struct CoreTemperature {
    pub temp: u8,
    pub max: u8,
    pub critical: u8,
}

#[derive(Debug, Clone, Default)]
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

const VALID_SPEEDS: &'static str =
    "Valid fan speeds range from 0-7, auto, full-speed and disengaged";

impl std::fmt::Display for FanSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FanSpeed as F;

        let s = match &self {
            F::Auto => "auto".to_string(),
            F::Disengaged => "disengaged".to_string(),
            F::Level(val) => val.to_string(),
            F::FullSpeed => "full-speed".to_string(),
        };

        write!(f, "{s}")
    }
}

impl std::fmt::Display for CoreTemperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}°C (max {}, crit {})",
            self.temp, self.max, self.critical
        )
    }
}

impl std::fmt::Display for Temperatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Average: {}°C", self.avg)?;

        for (i, (corename, coretemp)) in self.cores.iter().enumerate() {
            write!(f, "{}: {}", corename, coretemp)?;

            if i != self.cores.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
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

impl FanSpeed {
    pub fn from_string(value: String) -> Result<Self, Error> {
        let parsed = value.parse::<u8>();

        use std::num::IntErrorKind as I;
        match parsed {
            Err(e) => match e.kind() {
                I::NegOverflow => Err(err!(ValueTooLow, VALID_SPEEDS, "{} is negative", value)),
                I::PosOverflow => Err(err!(
                    ValueTooHigh,
                    VALID_SPEEDS,
                    "Fan speed {} is too high",
                    value
                )),
                I::InvalidDigit => match value.as_ref() {
                    "disengaged" => Ok(Self::Disengaged),
                    "auto" => Ok(Self::Auto),
                    "full-speed" => Ok(Self::FullSpeed),
                    _ => Err(err!(
                        InvalidValue,
                        VALID_SPEEDS,
                        "{} is an invalid fan speed setting",
                        value
                    )),
                },
                _ => Err(err!(
                    InvalidValue,
                    VALID_SPEEDS,
                    "{} is an invalid fan speed setting",
                    value
                )),
            },
            Ok(num) => {
                if (1..=7).contains(&num) {
                    Ok(Self::Level(num))
                } else {
                    Err(err!(
                        InvalidValue,
                        VALID_SPEEDS,
                        "{} is an invalid fan speed setting",
                        value
                    ))
                }
            }
        }
    }
}

pub fn set_fanspeed(fs: FanSpeed) -> Result<(), Error> {
    let err = OpenOptions::new()
        .append(true)
        .read(true)
        .open("/proc/acpi/ibm/fan");

    let mut file = match err {
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                return Err(err!(
                    FileNotFound,
                    "Did you load thinkpad_acpi?",
                    "File not /proc/acpi/ibm/fan not found."
                ))
            }
            io::ErrorKind::PermissionDenied => {
                return Err(err!(
                    PermissionDenied,
                    "Do you have root permissions?",
                    "while trying to write to /proc/acpi/ibm/fan"
                ))
            }
            _ => return Err(generic_err!(e)),
        },
        Ok(f) => f,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => (),
        Err(e) => return Err(generic_err!(e)),
    };

    if !s.contains("command") {
        Err(err!(
            FanControlDisabled,
            "Did you load thinkpad_acpi with fan_control=1?",
            "Can't control the fan speed"
        ))
    } else {
        let err = file.write(format!("level {fs}").as_bytes());

        match err {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                io::ErrorKind::InvalidInput => Err(err!(
                    FanControlDisabled,
                    "Did you load thinkpad_acpi with fan_control=1",
                    "Can't control the fan speed. "
                )),
                _ => panic!("{e}"),
            },
        }
    }
}

pub fn get_rpm() -> Result<u16, Error> {
    let err = OpenOptions::new().read(true).open("/proc/acpi/ibm/fan");

    let mut file = match err {
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                return Err(err!(
                    FileNotFound,
                    "Did you load thinkpad_acpi?",
                    "File not /proc/acpi/ibm/fan not found"
                ))
            }
            io::ErrorKind::PermissionDenied => {
                return Err(err!(
                    PermissionDenied,
                    "Do you have sufficient permissions?",
                    "while trying to read from /proc/acpi/ibm/fan"
                ))
            }
            _ => return Err(generic_err!(e)),
        },
        Ok(f) => f,
    };

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    let rpm = s.split('\n').collect::<Vec<&str>>()[1] // get second line
        .split_once(':')
        .unwrap()
        .1
        .trim();

    Ok(rpm.parse::<u16>().expect("Failed to parse RPM"))
}

pub fn get_fanspeed() -> Result<String, Error> {
    let err = OpenOptions::new().read(true).open("/proc/acpi/ibm/fan");

    let mut file = match err {
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                return Err(err!(
                    FileNotFound,
                    "Did you load thinkpad_acpi?",
                    "File not /proc/acpi/ibm/fan not found"
                ))
            }
            io::ErrorKind::PermissionDenied => {
                return Err(err!(
                    PermissionDenied,
                    "Do you have sufficient permissions?",
                    "while trying to read from /proc/acpi/ibm/fan"
                ))
            }
            _ => return Err(generic_err!(e)),
        },
        Ok(f) => f,
    };

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    let fanspeed = s.split('\n').collect::<Vec<&str>>()[2] // get third line
        .split_once(':')
        .unwrap()
        .1
        .trim();

    Ok(fanspeed.into())
}

pub fn get_temps() -> Result<Temperatures, Error> {
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
            .split_once(' ')
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

    Ok(res)
}

pub fn get_cores() -> Result<Vec<u8>, ErrorKind> {
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

    for key in coretemps.keys() {
        let k = key.as_str();

        if !k.contains("Core ") {
            continue;
        }

        let coreid = k
            .split_once(' ')
            .expect("expected Core and an ID")
            .1
            .parse::<u8>()
            .expect("ID should be a valid number");

        res.push(coreid)
    }

    Ok(res)
}
