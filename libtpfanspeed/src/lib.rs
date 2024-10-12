pub mod error;

use error::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs::OpenOptions,
    io::{self, Read, Write},
};

// Represents the output of `sensors -j`. Type alias for clarity.
pub type SensorsOutput = String;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CoreTemperature {
    pub temp: u8,
    pub max: u8,
    pub critical: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Temperatures {
    pub avg: u8,
    pub cores: BTreeMap<u8, CoreTemperature>,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FanSpeed {
    Level(u8),
    FullSpeed,
    Disengaged,
    Auto,
}

const VALID_SPEEDS: &str = "Valid fan speeds range from 0-7, auto, full-speed and disengaged";

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
            cores: BTreeMap::new(),
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

pub fn get_sensors_output() -> Result<SensorsOutput, Error> {
    let err = std::process::Command::new("sensors").arg("-j").output();
    let output = match err {
        Ok(output) => output,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                return Err(err!(
                    FileNotFound,
                    "Do you have lm_sensors installed?",
                    "Could not access sensors command"
                ))
            }
            _ => panic!("{}", e),
        },
    };

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn get_temps_from_sensors_output(sensors_output: SensorsOutput) -> Result<Temperatures, Error> {
    let mut res = Temperatures::new();
    let json_value: serde_json::Value =
        serde_json::from_str(&sensors_output).expect("failed to parse JSON");

    // get individual core temperatures
    let coretemps = json_value
        .get("coretemp-isa-0000")
        .expect("failed to find key")
        .as_object()
        .expect("failed to find key");

    for (key, value) in coretemps {
        let k = key.as_str();

        // ignore other data
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

        let core_id = k
            .split_whitespace()
            .nth(1)
            .and_then(|temp| temp.parse::<u8>().ok())
            .expect("something is seriously wrong. the core id doesn't begin with core!");

        res.cores.insert(core_id, coretemp);
    }

    // get average temperature if `Package id 0` is a key
    if let Some(package_id_0) = json_value
        .get("coretemp-isa-0000")
        .expect("failed to find key")
        .get("Package id 0")
    {
        res.avg = package_id_0
            .get("temp1_input")
            .expect("failed to find key")
            .as_f64()
            .expect("expected a float, got something else") as u8;
    } else {
        // manually calculate average
        let temps_sum = res.cores.iter().map(|(_, v)| v.temp).sum::<u8>();
        res.avg = temps_sum / res.cores.len() as u8;
    }

    Ok(res)
}

pub fn get_core_temp_from_sensors_output(
    sensors_output: SensorsOutput,
    core_id: u8,
) -> Result<CoreTemperature, Error> {
    let json_value: serde_json::Value =
        serde_json::from_str(&sensors_output).expect("failed to parse JSON");

    // get individual core temperatures
    let coretemps = json_value
        .get("coretemp-isa-0000")
        .expect("failed to find key")
        .as_object()
        .expect("failed to find key");

    let core_temp = coretemps.get(&format!("Core {core_id}"));
    if let Some(value) = core_temp {
        let temp = value
            .get(format!("temp{}_input", core_id + 2))
            .expect("core id somehow invalid?")
            .as_f64()
            .expect("expected a float, got something else") as u8;

        let max = value
            .get(format!("temp{}_max", core_id + 2))
            .expect("core id somehow invalid?")
            .as_f64()
            .expect("expected a float, got something else") as u8;

        let critical = value
            .get(format!("temp{}_crit", core_id + 2))
            .expect("core id somehow invalid?")
            .as_f64()
            .expect("expected a float, got something else") as u8;

        let coretemp = CoreTemperature::new(temp, max, critical);
        Ok(coretemp)
    } else {
        Err(err!(InvalidValue, "Core {} is not valid!", core_id))
    }
}

pub fn get_temps() -> Result<Temperatures, Error> {
    let sensors_output = get_sensors_output()?;
    get_temps_from_sensors_output(sensors_output) // Propagate both the result and error
}

pub fn get_core_temp(core_id: u8) -> Result<CoreTemperature, Error> {
    let sensors_output = get_sensors_output()?;
    get_core_temp_from_sensors_output(sensors_output, core_id)
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
