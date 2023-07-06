# rust-vosp

A Very Opinionated Starting Point for Rust projects.

This is the basics:

- configuration through environment vars with defaults
- simple logging
- Cli module with one command to print the configuration to console as json

Just fork, clone, and start your own app using this as a foundation. (or don't, I made this for me)

Design considerations:

- Each module is self contained and can be dropped directly into existing projects
- Modules are either initialied on use, or by calling an init function in main
- Configuration is accessed through the static: `config::SETTINGS` using dot notation e.g.

```rust
use crate::config::SETTINGS;
fn main() {
    let foo: String = SETTINGS.hostname;
    println!("{}", foo)
}
```
