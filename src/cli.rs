use clap::{ArgGroup, Parser};

const LONG_ABOUT: &str = "\
List POSIX signal information for all processes.

By default -c (--caught), -b (--blocked), and -p (--pending) are set.
However, any of these options you specify will override and reset the defaults.

Example: sigscan -cbp

Note: This tool only supports classic POSIX signals, not real-time signals
(e.g. SIGRTMIN and above).
";

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = LONG_ABOUT,
    next_line_help = true,
    group = ArgGroup::new("colors").multiple(false)
)]
pub struct Cli {
    /// Show processes that are ignoring signals
    #[arg(short, long, default_value_t = false)]
    pub ignored: bool,

    /// Show processes that are catching signals
    #[arg(short, long, default_value_t = true)]
    pub caught: bool,

    /// Show processes that are blocking signals
    #[arg(short, long, default_value_t = true)]
    pub blocked: bool,

    /// Show processes that have pending signals
    #[arg(short, long, default_value_t = true)]
    pub pending: bool,

    /// Replace the binary name with the full value of
    /// /proc/$PID/cmdline surrounded by quotes
    #[arg(long, default_value_t = false)]
    pub cmdline: bool,

    /// Force enable colored output (can also be set via FORCE_COLOR env var)
    #[arg(long, default_value_t = false, group = "colors")]
    pub color: bool,

    /// Force disable colored output (can also be set via NO_COLOR env var)
    #[arg(long, default_value_t = false, group = "colors")]
    pub no_color: bool,
}

impl Cli {
    /// Process command line args to nullify defaults when any flag is explicitly set
    pub fn process_args_resetting_defaults_if_flags_were_provided() -> Self {
        fn short_arg_present(arg: &str, letter: char) -> bool {
            arg.starts_with('-') && !arg.starts_with("--") && arg.contains(letter)
        }

        let mut cli = Self::parse();

        // Get the raw args as strings
        let raw_args: Vec<String> = std::env::args().collect();

        // Skip the first arg (program name)
        let args = &raw_args[1..];

        // Check if any args were provided at all
        if !args.is_empty() {
            // Flags we need to detect
            let mut has_ignored = false;
            let mut has_caught = false;
            let mut has_blocked = false;
            let mut has_pending = false;

            // Examine each argument
            for arg in args {
                if arg == "--ignored" || short_arg_present(arg, 'i') {
                    has_ignored = true;
                }
                if arg == "--caught" || short_arg_present(arg, 'c') {
                    has_caught = true;
                }
                if arg == "--blocked" || short_arg_present(arg, 'b') {
                    has_blocked = true;
                }
                if arg == "--pending" || short_arg_present(arg, 'p') {
                    has_pending = true;
                }
            }

            // If any flag was detected, reset the defaults for unspecified flags
            if has_ignored || has_caught || has_blocked || has_pending {
                cli.ignored = has_ignored;
                cli.caught = has_caught;
                cli.blocked = has_blocked;
                cli.pending = has_pending;
            }
        }

        cli
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
