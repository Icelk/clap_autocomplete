> Easy to integrate shell completion for Clap. Finds the user's shell and puts
> completion files in the appropriate locations.

A library which adds a subcommand to your command. It also checks the matches and returns whether or not the operation succeeded.

# Supported shells

This supports the same shells as [clap_complete](https://crates.io/crates/clap_complete).

It however only supports placing the completion files for Fish, Bash, and Zsh. If the user is using any other shell, they will have to pipe the output into a file.

# Documentation

Documentation for releases can be found on [docs.rs](https://docs.rs/clap_autocomplete).
Main branch documentation is found on [doc.icelk.dev](https://doc.icelk.dev/clap_autocomplete/clap_autocomplete/).

# Versions

-   0.1.x: Clap >= 3.1, < 4
-   0.2.x: Clap >= 3.1, < 4
-   0.3.x: Clap 4
-   0.4.x: Clap 4

# Changelog

## 0.4.0

-   Fix compilation on Windows by disabling automatic writing to shell completions.

## 0.3.0

-   Update to Clap 4

## 0.2.1

-   Faster fetching of shell
-   4 nested dependencies removed

## 0.2.0

-   Much faster fetching of shell.
-   20 less dependencies (removed `rayon`, which was included as part of `sysinfo` in the old shell fetch crate).
-   Better output of OS errors.
-   Status messages print to `stderr`.
-   Removed dependency on `clap`'s default features (e.g. `regex`).

# License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
