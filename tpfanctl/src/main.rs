use std::sync::OnceLock;

use clap::{Parser, Subcommand};
use colored::Colorize;
use libtpfanspeed as libtpfs;

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Print the CPU temperatures")]
    Temp,
    #[command(about = "Print the fan's RPM")]
    Rpm,
    #[command(about = "Print/Modify the fan's speed setting")]
    Fan { fanspeed: Option<String> },
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum LogLevel {
    Quiet = 0,
    Error = 1,
    Info = 2,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple ThinkPad Fan control tool. pass `-h` for help.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(short = 'n', default_value_t = false)]
    noinfo: bool,

    #[arg(short = 'q', default_value_t = false)]
    quiet: bool,
}

struct Application {}
static LOGLEVEL: OnceLock<LogLevel> = OnceLock::new();

fn err(err: libtpfs::error::Error) -> ! {
    if LOGLEVEL.get().unwrap() >= &LogLevel::Error {
        eprintln!("{}{}", "==> ERROR: ".red().bold(), err);
        if let Some(help) = err.help() {
            eprintln!("{}{}", "==> HELP: ".green().bold(), help)
        }
    }
    std::process::exit(1);
}

fn info<S: AsRef<str>>(txt: S) {
    if LOGLEVEL.get().unwrap() >= &LogLevel::Info {
        eprintln!("{}{}", "==> INFO: ".blue().bold(), txt.as_ref())
    }
}

impl Application {
    fn new() -> Self {
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

fn main() {
    color_eyre::install().unwrap();

    let args = Args::parse();
    let app = Application::new();

    let loglevel = if args.quiet {
        LogLevel::Quiet
    } else if args.noinfo {
        LogLevel::Error
    } else {
        LogLevel::Info
    };

    LOGLEVEL.set(loglevel).unwrap();

    match args.command {
        Command::Temp => app.temp(),
        Command::Fan { fanspeed } => match fanspeed {
            Some(fs) => {
                let res = libtpfs::FanSpeed::from_string(fs);

                match res {
                    Ok(fs) => app.set_fan(fs),
                    Err(e) => err(e),
                }
            }
            None => app.get_fan(),
        },
        Command::Rpm => app.rpm(),
    }
}
