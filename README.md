> Easy to integrate shell completion for Clap. Finds the user's shell and puts
> completion files in the appropriate locations.

A library which adds a subcommand to your command. It also checks the matches and returns whether or not the operation succeeded.

# Supported shells

This supports the same shells as [clap_complete](https://crates.io/crates/clap_complete).

It however only supports placing the completion files for Fish, Bash, and Zsh. If the user is using any other shell, they will have to pipe the output into a file.

# Documentation

Documentation for releases can be found on [docs.rs](https://docs.rs/clap_autocomplete).
Main branch documentation is found on [doc.icelk.dev](https://doc.icelk.dev/clap_autocomplete/clap_autocomplete/).

# License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
