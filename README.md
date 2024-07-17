# TPFanSpeed

*Just another ThinkPad fan control utility*.

This one is written in Rust and GTK (no [relm4](relm4.org), etc.).

## Dependencies

 * `lm_sensors` (whichever package from your distribution that provides the `sensors` command)

## the CLI

The CLI interface contains 2 binaries, `tpfanctl` and `setfan`. The help pages for these binaries are as follows:

```
A simple ThinkPad Fan control tool. pass `-h` for help.

Usage: tpfanctl [OPTIONS] <COMMAND>

Commands:
  temp     Print the CPU temperatures
  rpm      Print the fan's RPM
  fan      Print/Modify the fan's speed setting
  version  Print this program's version
  help     Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet        do not print any information, except for errors.
      --extra-quiet  do not print anything, including errors.
  -h, --help         Print help
  -V, --version      Print version
```

```
A condensed version of the tpfanctl utility, that only sets the fan speed.

Usage: setfan <FANSPEED>

Arguments:
  <FANSPEED>  The fan speed in question

Options:
  -h, --help     Print helpA condensed version of the tpfanctl utility, that only sets the fan speed.

Usage: setfan <FANSPEED>

Arguments:
  <FANSPEED>  The fan speed in question

Options:
  -h, --help     Print help
  -V, --version  Print version
  -V, --version  Print version
```

## the GUI

This one is still WIP. checkout the `gui` branch to see its development. 
