use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "nd-cli, a terminal directory navigator", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(
        long,
        default_value_t = false,
        help = "Show directories starting with '.'"
    )]
    pub show_hidden: bool,

    #[arg(
        long,
        value_name = "PATH",
        help = "Start browsing from PATH instead of the current directory"
    )]
    pub start_dir: Option<PathBuf>,

    #[arg(long, default_value_t = false, help = "Disable ANSI colors")]
    pub no_color: bool,
}

#[derive(Subcommand, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Init {
        #[arg(value_enum, help = "Shell to emit init script for", default_value_t = Shell::Zsh)]
        shell: Shell,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
}
