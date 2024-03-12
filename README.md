# clap-nested-commands

Rust macros to automate the command definitions for nested commands in a `clap` CLI.

## Installation

Add `clap-nested-commands` to your project's `Cargo.toml`:

```toml
[dependencies]
clap = { version = "x.y.z", features = ["derive"] }
clap-nested-commands = "0.*"
```

### Async commands

If your CLI commands are `async`, you also need the `anyhow` dependency:

```toml
[dependencies]
anyhow = "x.y.z"
clap = { version = "x.y.z", features = ["derive"] }
clap-nested-commands = "0.*"
```

## Usage

For example, imagine a CLI command like `my-cli project user add --email name@domain.com`.

With `clap-nested-commands`, you structure this CLI as follows:

```
examples/sync_commands
├── Cargo.toml
├── cli_context.rs
├── src
│   └── commands
│       ├── mod.rs
│       └── project
│           ├── mod.rs
│           ├── task
│           │   ├── add.rs
│           │   ├── mod.rs
│           │   └── remove.rs
│           └── user
│               ├── add.rs
│               ├── mod.rs
│               └── remove.rs
└── main.rs
```

### Sync commands

Use the following pattern in your `src/commands/**/mod.rs` files:

```rust
// src/commands/project/user
use clap::{Args, Subcommand};
use clap_nested_commands::generate_sync_commands;
use crate::cli_context::CliContext;

// A list of sub-command modules
mod add;
mod remove;

/// User commands
#[derive(Debug, Args)]
pub struct Command {
    #[command(subcommand)]
    pub command: Commands,
}

generate_sync_commands!(add, remove);
```

Individual commands look like this:

```rust
// src/commands/project/user/add.rs
use clap::Args;
use crate::cli_context::CliContext;

/// Add a user
#[derive(Debug, Args)]
pub struct Command {
    /// The email address of the user to add
    pub email: String,
}

pub fn execute(_cli_context: &CliContext, _cmd: Command) {
    // TODO: Add command logic
}
```

### Async commands

*The only difference compared to sync commands is the use of the `generate_async_commands` macro.*

Use the following pattern in your `src/commands/**/mod.rs` files:

```rust
// src/commands/project/user
use clap::{Args, Subcommand};
use clap_nested_commands::generate_async_commands;
use crate::cli_context::CliContext;

// A list of sub-command modules
mod add;
mod remove;

/// User commands
#[derive(Debug, Args)]
pub struct Command {
    #[command(subcommand)]
    pub command: Commands,
}

generate_async_commands!(add, remove);
```

Individual commands look like this:

```rust
// src/commands/project/user/add.rs
use anyhow::Error;
use clap::Args;
use crate::cli_context::CliContext;

/// Add a user
#[derive(Debug, Args)]
pub struct Command {
    /// The email address of the user to add
    pub email: String,
}

pub async fn execute(_cli_context: &CliContext, _cmd: Command) -> Result<(), Error> {
    // Add command logic
    Ok(())
}
```

## Examples

See `./examples` for both a sync and an async example. To run them, use the following commands:

* `cargo run --example async_commands <sub-commands>`
  * E.g. `cargo run --example async_commands project user add --email name@domain.com`
* `cargo run --example sync_commands <sub-commands>`
  * E.g. `cargo run --example sync_commands project user add --email name@domain.com`
  