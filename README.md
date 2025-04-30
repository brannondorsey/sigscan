# Sigscan

List POSIX signal information for all processes.

![sigscan screenshot](.images/sigscan-screenshot.png)

## Features

* Easy to use (just `sigscan`)
* Blazingly fast (~95% of time spent in kernel space)
* Uses colors if output is detected as a terminal that supports them (or forcibly with `--(no-)colors`).

## Examples

### Basic usage

```bash
# Show all processes with caught, blocked, and pending signals (default)
sigscan

# Show only processes that are ignoring signals (omitted by default)
sigscan --ignored

# Show caught, blocked, pending, and ignored signals
sigscan --caught --blocked --pending --ignored

# The quivalent command using short argument notation
sigscan -cbpi

# You get the idea...
```

### Recipes

`sigscan` subscribes to the Unix philosophy of doing one thing and doing it well. It can be combined with other tools to answer more complex questions.

```bash
# Paginate out 
sigscan --color | less

# Get PIDs of processes that are blocking any type of signal
sigscan --blocked | cut -d" " -f1

# Find potential daemon processes (catching SIGHUP)
sigscan --caught | grep HUP | cut -d" " -f2

# Get the full command line of processes that are catching TERM
sigscan --caught | grep TERM | cut -d" " -f1 | xargs ps -o pid,cmd

# Find processes that might be zombies (blocking SIGCHLD)
sigscan --blocked | grep CHLD | awk '{print $1}' | xargs ps -o pid,ppid,state,cmd

# Continuously watch for processes with pending signals
watch --color -n 0.1 "sigscan --pending --color"
```

## Usage

```txt
$ sigscan --help
List POSIX signal information for all processes.

By default -c (--caught), -b (--blocked), and -p (--pending) are set.
However, any of these options you specify will override and reset the defaults.

Example: sigscan -cbp

Note: This tool only supports classic POSIX signals, not real-time signals
(e.g. SIGRTMIN and above).

Usage: sigscan [OPTIONS]

Options:
  -i, --ignored
          Show processes that are ignoring signals

  -c, --caught
          Show processes that are catching signals

  -b, --blocked
          Show processes that are blocking signals

  -p, --pending
          Show processes that have pending signals

      --cmdline
          Replace the binary name with the full value of /proc/$PID/cmdline surrounded by quotes

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Limitations

* Only classic POSIX signals are supported, no real-time signals.
* Relies on `/proc` and therefore only works on Linux.
