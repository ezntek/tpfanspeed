use clap::{Parser, Subcommand};
use libtpfanspeed as libtpfs;
use tpfanctl::*;

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Print the dashboard")]
    Dash,
    #[command(about = "Print the CPU temperatures")]
    Temp,
    #[command(about = "Print the fan's RPM")]
    Rpm,
    #[command(about = "Print/Modify the fan's speed setting")]
    Fan { fanspeed: Option<String> },
    #[command(about = "Print this program's version")]
    Version,
}

/// NOTE: The `version` option is not used, as it will be customized
#[derive(Parser, Debug)]
#[command(author, about = "A simple ThinkPad Fan control tool. pass `-h` for help.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(
        short = 'q',
        long,
        default_value_t = true,
        help = "do not print any errors, nor help."
    )]
    quiet: bool,

    #[arg(
        short = 'D',
        long,
        default_value_t = false,
        help = "do not pretty-print data."
    )]
    disable_pretty_print: bool,
}

fn main() {
    color_eyre::install().unwrap();

    let args = Args::parse();

    PRINT_ERRORS.set(args.quiet).unwrap();
    PRETTY_PRINT.set(!args.disable_pretty_print).unwrap();

    let app = Application::new();

    match args.command {
        Command::Dash => app.dash(),
        Command::Temp => app.get_temp(),
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
        Command::Rpm => app.get_rpm(),
        Command::Version => version(),
    }
}
