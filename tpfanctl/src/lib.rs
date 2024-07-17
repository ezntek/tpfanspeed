use std::sync::OnceLock;

use colored::Colorize;
use libtpfanspeed as libtpfs;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum LogLevel {
    Quiet = 0,
    Error = 1,
    Help = 2,
}

#[derive(Default, Clone)]
pub struct Application {}

pub static VERSION: &str = "0.1.0";
pub static LOGLEVEL: OnceLock<LogLevel> = OnceLock::new();

pub fn err(err: libtpfs::error::Error) -> ! {
    if LOGLEVEL.get().unwrap() >= &LogLevel::Error {
        eprintln!("{}{}", "==> ERROR: ".red().bold(), err);

        if LOGLEVEL.get().unwrap() >= &LogLevel::Help {
            if let Some(help) = err.help() {
                eprintln!("{}{}", "==> HELP: ".green().bold(), help)
            }
        }
    }
    std::process::exit(1);
}

pub fn info<S: AsRef<str>>(txt: S) {
    eprintln!("{}{}", "==> INFO: ".blue().bold(), txt.as_ref())
}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub fn temp(&self) {
        let temps = libtpfs::get_temps().unwrap_or_else(|e| err(e));
        println!("{temps}");
    }

    pub fn rpm(&self) {
        let rpm = libtpfs::get_rpm().unwrap_or_else(|e| err(e));
        println!("{rpm}");
    }

    pub fn set_fan(&self, fanspeed: libtpfs::FanSpeed) {
        libtpfs::set_fanspeed(fanspeed).unwrap_or_else(|e| err(e));
        info(format!("fan speed set to {fanspeed}"))
    }

    pub fn get_fan(&self) {
        let fanspeed = libtpfs::get_fanspeed().unwrap_or_else(|e| err(e));
        println!("{fanspeed}");
    }
}
