mod cli;
use cli::Cli;
use libc::sigset_t;
use nix::sys::signal::{SigSet, Signal};
use owo_colors::OwoColorize;
use owo_colors::Stream::Stdout as OwoStdout;
use procfs::process::Process;
use procfs::process::Status;
use std::io::{self, Write};
use std::mem::MaybeUninit;

// Include the examples content generated at build time
include!(concat!(env!("OUT_DIR"), "/examples_content.rs"));

/// A macro that handles the signal string formatting with name, colors, and
/// spacing.
macro_rules! format_signal_str {
    ($display:expr, $signals:expr, $name:expr, $color_fn:expr) => {
        if !$display || $signals.is_empty() {
            "".to_string()
        } else {
            format!(
                "{}={} ",
                $name.if_supports_color(OwoStdout, $color_fn),
                $signals
            )
        }
    };
}

fn main() {
    let cli = Cli::process_args_resetting_defaults_if_flags_were_provided();

    // If --examples flag is provided, display the examples and exit
    if cli.examples {
        // Prevent panics on pipe errors
        let _ = writeln!(io::stdout(), "{}", EXAMPLES_CONTENT);
        return;
    }

    set_color_override(&cli);

    let proc_statuses = {
        let mut vec = procfs::process::all_processes()
            .expect("Failed to get processes")
            .filter_map(Result::ok)
            .filter_map(|prc| prc.status().ok())
            .filter(get_filter_status(&cli))
            .collect::<Vec<_>>();
        vec.sort_by_key(|status| status.pid);
        vec
    };

    proc_statuses.iter().for_each(|status| {
        let pid = format!("{}", status.pid);
        let pid = pid.if_supports_color(OwoStdout, |t| t.bright_yellow());

        let name = get_name(status);
        let name = name.if_supports_color(OwoStdout, |t| t.bright_black());

        let pending = sigset_to_strings(status.shdpnd).join(",");
        let blocked = sigset_to_strings(status.sigblk).join(",");
        let ignored = sigset_to_strings(status.sigign).join(",");
        let caught = sigset_to_strings(status.sigcgt).join(",");

        let ignored = format_signal_str!(cli.ignored, ignored, "ignored", |t| t.bright_blue());
        let caught = format_signal_str!(cli.caught, caught, "caught", |t| t.bright_magenta());
        let blocked = format_signal_str!(cli.blocked, blocked, "blocked", |t| t.bright_red());
        let pending = format_signal_str!(cli.pending, pending, "pending", |t| t.bright_green());

        let is_empty =
            ignored.is_empty() && caught.is_empty() && blocked.is_empty() && pending.is_empty();

        let display = !is_empty || cli.empty;
        if display {
            let output = format!("{pid} {name} {ignored}{caught}{blocked}{pending}");
            let output = output.trim_end();
            let _ = writeln!(io::stdout(), "{output}");
        }
    });
}

/// Falls back to auto-detection if no color override flag is set
/// using owo-color's "supports-colors" feature (see Cargo.toml)
fn set_color_override(cli: &Cli) {
    if cli.color {
        owo_colors::set_override(true);
    } else if cli.no_color {
        owo_colors::set_override(false);
    }
}

fn get_filter_status(cli: &Cli) -> impl FnMut(&Status) -> bool {
    move |status: &Status| {
        (cli.pending && status.shdpnd != 0)
            || (cli.blocked && status.sigblk != 0)
            || (cli.caught && status.sigcgt != 0)
            || (cli.ignored && status.sigign != 0)
            || (cli.empty
                && status.sigign == 0
                && status.sigcgt == 0
                && status.sigblk == 0
                && status.shdpnd == 0)
    }
}

/// Get the name of a process
fn get_name(status: &Status) -> String {
    let name = status.name.trim();

    // The Linux kernel truncates process names to 15 characters, so if its hit
    // that mark, we should infer the name from the command line instead.
    if name.len() == 15 {
        Process::new(status.pid)
            .ok()
            .and_then(|process| process.cmdline().ok())
            .and_then(|cmdline| {
                cmdline
                    .first()
                    .map(|cmd| cmd.rsplit('/').next().unwrap_or(cmd).to_string())
            })
            .unwrap_or_else(|| name.to_string())
    } else {
        name.to_string()
    }
}

fn sigset_to_strings(sigset: u64) -> Vec<String> {
    let sigset = sigset_from_u64(sigset);
    let sigset = unsafe { SigSet::from_sigset_t_unchecked(sigset) };
    let mut sigset: Vec<(Signal, String)> = sigset
        .iter()
        .map(|s| (s, s.to_string().replace("SIG", "")))
        .collect();
    // Sort by signal number lowest first because that's the order Linux will
    // fire the handlers in when multiple signals transition out of the pending
    // state.
    sigset.sort_by_key(|(s, _)| *s);
    sigset
        .iter()
        .map(|(_, s)| s)
        .enumerate()
        .map(|(i, s)| {
            if i % 2 == 0 {
                s.if_supports_color(OwoStdout, |t| t.white()).to_string()
            } else {
                s.if_supports_color(OwoStdout, |t| t.white()).to_string()
            }
        })
        .collect()
}

// Warning: This only works on platforms where sigset_t starts with a u64[16]
// (true on Linux/glibc). Not portable or future-proof. Use with care.
fn sigset_from_u64(mask: u64) -> sigset_t {
    let mut sigset = unsafe { MaybeUninit::<sigset_t>::zeroed().assume_init() };
    unsafe {
        let raw = &mut sigset as *mut sigset_t as *mut u64;
        *raw = mask;
    }
    sigset
}
