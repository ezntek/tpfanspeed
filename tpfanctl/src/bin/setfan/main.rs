use clap::Parser;
use libtpfanspeed::FanSpeed;
use tpfanctl::*;

#[derive(Parser)]
#[command(author, version, long_about = None, about = "A condensed version of the tpfanctl utility, that only sets the fan speed.")]
struct Args {
    #[arg(help = "The fan speed in question")]
    fanspeed: String,

    #[arg(
        help = "print the version of this application",
        short = 'v',
        long = "version",
        default_value_t = false
    )]
    print_version: bool,
}

fn main() {
    color_eyre::install().unwrap();
    let args = Args::parse();

    if args.print_version {
        info(format!("tpfanctl version {VERSION}"));
        std::process::exit(0);
    }

    LOGLEVEL.set(LogLevel::Help).unwrap();

    let fs = FanSpeed::from_string(args.fanspeed).unwrap_or_else(|e| err(e));
    Application::new().set_fan(fs);
}
