//! Easy to integrate shell completion for Clap. Tries to choose the user's shell and put
//! completion files in the appropriate locations.
//!
//! For best results, please add [`ValueHint`]s to every argument that takes a value.
//! This greatly improves the shell completion experience.
//!
//! # Examples
//!
//! ```rust
//! // Create a command from the crate metadata
//! let mut command = clap::command!();
//! // Register `complete` subcommand
//! command = clap_autocomplete::add_subcommand(command);
//!
//! // Add other arguments and subcommands
//!
//! let command_copy = command.clone();
//! // Resolve the matches
//! let matches = command.get_matches();
//! if let Some(result) = clap_autocomplete::test_subcommand(&matches, command_copy) {
//!     if let Err(err) = result {
//!         eprintln!("Insufficient permissions: {err}");
//!         std::process::exit(1);
//!     } else {
//!         std::process::exit(0);
//!     }
//! } else {
//!     // Continue with the application logic
//! }
//! ```

use std::fs;
use std::fs::create_dir_all;
use std::io;

use clap::ValueHint;
use clap::{Arg, ArgMatches, Command};
use clap_complete::Shell;

/// Add the `complete` subcommand to your [`Command`].
#[must_use]
pub fn add_subcommand(command: Command) -> Command {
    command.subcommand(
        Command::new("complete")
            .about(
                "Generate completions for the detected/selected shell and put the completions in appropriate directories.\n\
                Currently supports Fish, Bash, Zsh, Elvish, and PowerShell. Fish, Bash, and Zsh are installed automatically (when not using the --print flag)."
            )
            .arg(
                Arg::new("print").short('p').long("print").help(
                    "Print the shell completion to stdout instead of writing to default file.\n\
                    Does nothing when using shells for which the installation location isn't implemented.",
                ),
            )
            .arg(
                Arg::new("shell")
                    .takes_value(true)
                    .short('s')
                    .long("shell")
                    .help("Explicitly choose which shell to output.")
                    .value_hint(ValueHint::Other)
            ),
    )
}
/// Check the [`ArgMatches`] for the subcommand added by [`add_subcommand`].
///
/// # Returns
///
/// Ignore if this returns [`None`].
/// Exit application when this returns [`Some`], and signal the user with the error, if any.
#[must_use = "check whether or not to exit"]
pub fn test_subcommand(matches: &ArgMatches, mut command: Command) -> Option<Result<(), String>> {
    matches.subcommand_matches("complete").map(|matches| {
        let shell = {
            let mut name = matches
                .value_of("shell")
                .map(Into::into)
                .ok_or(())
                .or_else(|()| {
                    println!("Getting your shell.");
                    query_shell::get_shell()
                        .map_err(|_| {
                            "failed to detect shell, please explicitly supply it".to_owned()
                        })
                        .map(|shell| shell.to_str().to_owned())
                })?;
            name.make_ascii_lowercase();
            match name.as_str() {
                "bash" => Shell::Bash,
                "fish" => Shell::Fish,
                "zsh" => Shell::Zsh,
                "pwsh" | "powershell" => Shell::PowerShell,
                "elvish" => Shell::Elvish,
                _ => return Err("unsupported explicit shell".into()),
            }
        };
        let bin_name = command
            .get_bin_name()
            .unwrap_or_else(|| command.get_name())
            .to_owned();

        if matches.is_present("print") || !matches!(shell, Shell::Fish | Shell::Bash | Shell::Zsh) {
            clap_complete::generate(shell, &mut command, bin_name, &mut io::stdout());
            Ok(())
        } else {
            let mut buffer = Vec::with_capacity(512);
            clap_complete::generate(shell, &mut command, &bin_name, &mut buffer);
            write_shell(shell, &buffer, &bin_name)
                .map_err(|err| format!("insufficient privileges: {}", err))?;
            Ok(())
        }
    })
}
fn write_shell(shell: Shell, data: &[u8], bin_name: &str) -> Result<(), io::Error> {
    let path = match shell {
        Shell::Fish => {
            let dirs = xdg::BaseDirectories::new()?;
            dirs.place_config_file(format!("fish/completions/{bin_name}.fish"))?
        }
        Shell::Bash => format!("/usr/share/bash-completion/completions/{bin_name}").into(),
        Shell::Zsh => format!("/usr/share/zsh/functions/Completion/Base/_{bin_name}").into(),
        _ => unreachable!("trying to write unsupported shell"),
    };

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    println!("Writing completions to {}", path.display());
    fs::write(path, data)?;
    Ok(())
}
