use libc::sigset_t;
use nix::sys::signal::SigSet;
use owo_colors::OwoColorize;
use owo_colors::Stream::Stdout as OwoStdout;
use std::mem::MaybeUninit;

/// A macro that handles the signal string formatting with name, colors, and spacing.
macro_rules! format_signal_str {
    ($signals:expr, $name:expr, $color_fn:expr) => {
        if $signals.is_empty() {
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
    let proc_statuses = {
        let mut vec = procfs::process::all_processes()
            .expect("Failed to get processes")
            .filter_map(Result::ok)
            .filter_map(|prc| prc.status().ok())
            .filter(|status| status.shdpnd != 0 || status.sigblk != 0)
            .collect::<Vec<_>>();
        vec.sort_by_key(|status| status.pid);
        vec
    };

    proc_statuses.iter().for_each(|status| {
        let pid = format!("{}", status.pid);
        let pid = pid.if_supports_color(OwoStdout, |t| t.bright_yellow());
        let name = status.name.trim();
        let name = name.if_supports_color(OwoStdout, |t| t.bright_black());

        let pending = sigset_to_strings(status.shdpnd).join(",");
        let blocked = sigset_to_strings(status.sigblk).join(",");
        let ignored = sigset_to_strings(status.sigign).join(",");
        let caught = sigset_to_strings(status.sigcgt).join(",");

        let ignored = format_signal_str!(ignored, "ignored", |t| t.bright_blue());
        let caught = format_signal_str!(caught, "caught", |t| t.bright_magenta());
        let blocked = format_signal_str!(blocked, "blocked", |t| t.bright_red());
        let pending = format_signal_str!(pending, "pending", |t| t.bright_green());

        println!("{pid} {name} {ignored}{caught}{blocked}{pending}",);
    });
}

fn sigset_to_strings(sigset: u64) -> Vec<String> {
    let sigset = sigset_from_u64(sigset);
    let sigset = unsafe { SigSet::from_sigset_t_unchecked(sigset) };
    let mut sigset: Vec<String> = sigset
        .iter()
        .map(|s| s.to_string().replace("SIG", ""))
        .collect();
    sigset.sort();
    sigset
        .iter()
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

// Warning: This only works on platforms where sigset_t starts with a u64[16] (true on Linux/glibc). Not portable or future-proof. Use with care.
fn sigset_from_u64(mask: u64) -> sigset_t {
    let mut sigset = unsafe { MaybeUninit::<sigset_t>::zeroed().assume_init() };
    unsafe {
        let raw = &mut sigset as *mut sigset_t as *mut u64;
        *raw = mask;
    }
    sigset
}
