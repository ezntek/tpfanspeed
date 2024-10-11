# tpfanspeed

*Just another ThinkPad fan control utility*.

## Dependencies

 * `lm_sensors` (whichever package from your distribution that provides the `sensors` command)

## Building

 * `git clone https://github.com/ezntek/tpfanspeed --branch=cli`
 * `cargo build --release`

The binaries will then be in `target/release`. Copy the binaries to your desired locations afterwards.

Every "stable" release will have a prebuilt linux binary in the Releases section.

## libtpfanspeed - the Crate

This crate contains only utility functions (mostly no structs/classes) to get critical data about
fan speed, RPM, etc. by parsing `/proc/acpi/ibm/fan` and using `sensors -j` from `lm_sensors`.

## tpfanctl and setfan - the CLI

The CLI is relatively simple, it contains 2 binaries; `tpfanctl` which is the more comprehensive utility
with a "dashboards" and `setfan` which is just for setting the fan speed and nothing else.

The CLI help (accessed by issuing `/path/to/tpfanctl help` and `/path/to/setfan --help`, version `0.2.0-alpha`):

```
A simple ThinkPad Fan control tool. pass `-h` for help.

Usage: tpfanctl [OPTIONS] <COMMAND>

Commands:
  dash     Print the dashboard
  temp     Print the CPU temperatures
  rpm      Print the fan's RPM
  fan      Print/Modify the fan's speed setting
  version  Print this program's version
  help     Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet                 do not print any errors, nor help.
  -D, --disable-pretty-print  do not pretty-print data.
  -h, --help                  Print help
```

```
A condensed version of the tpfanctl utility, that only sets the fan speed.

Usage: setfan <FANSPEED>

Arguments:
  <FANSPEED>  The fan speed in question

Options:
  -h, --help  Print help
```

## tpfanspeed - the GUI

The is written in Rust and GTK (no [relm4](relm4.org), etc.).

The `cli` branch of the project will not include the GUI build for obvious reasons. It is simply a
frontend for the `libtpfanspeed` crate. check out the `main` branch for mre info.

