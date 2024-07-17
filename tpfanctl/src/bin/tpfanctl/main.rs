use clap::{Parser, Subcommand};
use libtpfanspeed as libtpfs;
use tpfanctl::*;

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Print the CPU temperatures")]
    Temp,
    #[command(about = "Print the fan's RPM")]
    Rpm,
    #[command(about = "Print/Modify the fan's speed setting")]
    Fan { fanspeed: Option<String> },
    #[command(about = "Print this program's version")]
    Version,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple ThinkPad Fan control tool. pass `-h` for help.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(
        short = 'q',
        long,
        default_value_t = false,
        help = "do not print any information, except for errors."
    )]
    quiet: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "do not print anything, including errors."
    )]
    extra_quiet: bool,
}

fn main() {
    color_eyre::install().unwrap();

    let args = Args::parse();
    let app = Application::new();

    let loglevel = if args.quiet {
        LogLevel::Error
    } else if args.extra_quiet {
        LogLevel::Quiet
    } else {
        LogLevel::Help
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
        Command::Version => info(format!("tpfanctl version {VERSION}")),
    }
}
