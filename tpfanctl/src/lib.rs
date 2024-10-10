use std::sync::OnceLock;

use color_eyre::owo_colors::OwoColorize;
use libtpfanspeed as libtpfs;

pub static VERSION: &str = "0.2.0";
pub static PRINT_ERRORS: OnceLock<bool> = OnceLock::new();
pub static PRETTY_PRINT: OnceLock<bool> = OnceLock::new();

pub fn version() {
    info(format!("tpfanctl version {}", VERSION.cyan().bold()))
}

pub fn err(err: libtpfs::error::Error) -> ! {
    if *PRINT_ERRORS.get().unwrap() {
        eprintln!("{}{}", "==> ERROR: ".red().bold(), err);

        if let Some(help) = err.help() {
            eprintln!("{}{}", "==> HELP: ".green().bold(), help)
        }
    }

    std::process::exit(1);
}

pub fn info<S: AsRef<str>>(txt: S) {
    eprintln!("{}{}", "==> INFO: ".blue().bold(), txt.as_ref())
}

#[derive(Default, Clone)]
pub struct Application {
    pretty_print: bool,
}

impl Application {
    pub fn new() -> Self {
        Self {
            pretty_print: *PRETTY_PRINT.get().unwrap(),
        }
    }

    fn get_temp_color(temp: u8) -> color_eyre::owo_colors::AnsiColors {
        // Temperature ranges include:
        // >40: blue
        // 40-55: green
        // 55-75: yellow
        // >75: red

        match temp {
            ..40 => color_eyre::owo_colors::AnsiColors::Blue,
            40..=55 => color_eyre::owo_colors::AnsiColors::Green,
            56..=75 => color_eyre::owo_colors::AnsiColors::Yellow,
            76.. => color_eyre::owo_colors::AnsiColors::Red,
        }
    }

    fn print_temp_progress_bar(temp: u8) {
        let filled_frac = temp as f64 / 100.0;
        let mut nsquares = (filled_frac * 20.0) as u8;

        if nsquares > 20 {
            nsquares = 20;
        }

        print!("[");
        for _ in 0..nsquares {
            print!("{}", '#'.color(Application::get_temp_color(temp)));
        }
        for _ in 0..(20 - nsquares) {
            print!("{}", '.'.dimmed())
        }
        print!("]");
    }

    pub fn get_temp(&self) {
        let temps = libtpfs::get_temps().unwrap_or_else(|e| err(e));

        if !self.pretty_print {
            println!("{temps}");
            return;
        }

        let avg = temps.avg;
        println!(
            "{} temperature: {}°C",
            "Average".green().bold(),
            avg.color(Application::get_temp_color(avg))
        );

        for (key, value) in temps.cores {
            print!("Core {}: ", key.green());
            Application::print_temp_progress_bar(value.temp);
            println!(
                " ({}°C)",
                value.temp.color(Application::get_temp_color(value.temp))
            );
        }
    }

    pub fn get_rpm(&self) {
        let rpm = libtpfs::get_rpm().unwrap_or_else(|e| err(e));

        if !self.pretty_print {
            println!("{rpm}");
            return;
        }

        println!(
            "Your fan is spinning at {} {}",
            rpm.green().bold(),
            "RPM".bold()
        );
    }

    pub fn set_fan(&self, fanspeed: libtpfs::FanSpeed) {
        let curr_fanspeed = libtpfs::get_fanspeed().unwrap_or_else(|e| err(e));

        if fanspeed.to_string() == curr_fanspeed && self.pretty_print {
            info(format!(
                "Your current fan speed is already {}!",
                fanspeed.yellow().bold()
            ));
            return;
        }

        libtpfs::set_fanspeed(fanspeed).unwrap_or_else(|e| err(e));

        if self.pretty_print {
            info(format!(
                "Your fan speed was set to {}",
                fanspeed.yellow().bold()
            ))
        }
    }

    pub fn get_fan(&self) {
        let fanspeed = libtpfs::get_fanspeed().unwrap_or_else(|e| err(e));

        if !self.pretty_print {
            println!("{fanspeed}");
            return;
        }

        let levels = [
            "auto",
            "0",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "full-speed",
            "disengaged",
        ];

        // auto, 0, 1, 2, 3, 4, 5, 6, 7, full-speed, disengaged
        // [*-*-*-*-*-*-*-*-*-*-*-*]

        println!("Your fan speed settings is {}\n", fanspeed.bold().yellow());

        print!("[");

        for (idx, level) in levels.iter().enumerate() {
            if level == &fanspeed {
                print!("{}", "#".yellow().bold());
            } else {
                print!("{}", "*".red());
            }

            if idx != levels.len() - 1 {
                print!("{}", "-".dimmed());
            } else {
                println!("]");
            }
        }

        let fanspeed_chars = fanspeed.chars().collect::<Vec<char>>();
        for c in " A 0 1 2 3 4 5 6 7 F D ".chars() {
            // TODO: find a better way to do this??
            if fanspeed_chars[0].to_ascii_uppercase() == c {
                print!("{}", c.to_string().green().bold());
            } else {
                print!("{}", c.to_string().dimmed());
            }
        }

        println!();
    }

    pub fn get_dash_once(&self) {
        println!("{}", "==============================".dimmed());
        println!("{}", "DASHBOARD".bold().cyan());
        println!("{}", "==============================".dimmed());
        self.get_temp();
        println!("{}", "==============================".dimmed());
        self.get_fan();
        println!("{}", "==============================".dimmed());
        self.get_rpm();
        println!("{}", "==============================".dimmed());
    }

    pub fn dash(&self) {
        self.get_dash_once();
    }
}
