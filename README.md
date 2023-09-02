# rs-vosp-template

A Very Opinionated Starting Point for Rust projects.

For Use with `cargo generate`:

```
cargo install cargo-generate
cargo generate bladerunnerp/rs-vosp-template
```

This is the basics:

- configuration through environment vars with defaults
- simple logging
- Cli module with one command to print the configuration to console as json

Design considerations:

- Each module is self contained and can be dropped directly into existing projects
- Modules are either initialied on use, or by calling an init function in main
- Configuration is accessed through the static: `config::SETTINGS` using dot notation e.g.

```rust
use crate::config::SETTINGS;
fn main() {
    println!("{}", SETTINGS.listen_addr)
}
```

Example usage:

```bash
cargo run -- -p
```

```bash
{"listen_addr":"127.0.0.1","port":3000,"loglevel":"info"}
```
