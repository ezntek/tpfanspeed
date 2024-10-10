use clap::Parser;
use libtpfanspeed::FanSpeed;
use tpfanctl::*;

#[derive(Parser)]
#[command(author, version, long_about = None, about = "A condensed version of the tpfanctl utility, that only sets the fan speed.")]
struct Args {
    #[arg(help = "The fan speed in question")]
    fanspeed: String,
}

fn main() {
    color_eyre::install().unwrap();
    let args = Args::parse();

    PRINT_ERRORS.set(true).unwrap();
    PRETTY_PRINT.set(true).unwrap();

    let fs = FanSpeed::from_string(args.fanspeed).unwrap_or_else(|e| err(e));
    Application::new().set_fan(fs);
}
