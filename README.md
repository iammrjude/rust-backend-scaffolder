# ForgeIT

A Rust CLI tool that scaffolds backend projects using Cargo.  
It creates a new Cargo binary project, adds a selected web framework dependency, optionally adds extra crates, generates a framework-specific `main.rs`, and creates common module directories.

---

## Package Info

- **Name:** forgeit
- **Version:** 0.1.0
- **Edition:** 2024
- **Authors:**
  - Abel Osaretin <contact.abel321@gmail.com>
  - Jude Abara <judeabara@gmail.com>
  - Eregha Thompson <thompsoneregha00@gmail.com>
  - Ibrahim Sunday <whizibrahim@gmail.com>

---

## Dependencies

Declared in `Cargo.toml`:

```toml
clap = { version = "4.5.60", features = ["derive"] }
```

### Notes

- `clap` is used for CLI argument parsing

---

## Usage

### Scaffold a New Project

```bash
forgeit scaffold --name <project_name> --framework <framework>
```

Example:

```bash
forgeit scaffold --name my_app --framework axum
```

With additional dependencies:

```bash
forgeit scaffold
  --name my_app
  --framework actix-web
  --deps dotenvy --deps tracing
```

```bash
forgeit scaffold -n my_app -f actix-web -d dotenvy -d tracing

```

---

## What the Scaffold Command Does

When you run `scaffold`, the tool:

1. Runs `cargo new <project_name>`
2. Adds the selected framework using `cargo add`
3. Adds any extra dependencies passed via `--deps`
4. Overwrites `src/main.rs` with framework-specific starter code
5. If the framework is `axum` or `actix-web`, it also adds:
   - `serde` with the `derive` feature
   - `tokio` with the `full` feature
6. Creates the following module directories under `src/`, each with an empty `mod.rs` file:
   - `services`
   - `models`
   - `handlers`
   - `routes`

---

## Supported Frameworks

List supported frameworks:

```bash
forgeit list
```

Output:

```bash
Available frameworks:
  - axum
  - actix-web
```

### Important Behavior

- Any framework name will still be added as a dependency
- Unsupported framework names receive a default `Hello, world!` `main.rs`

---

## Generated Project Structure

```tree
<project_name>/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── services/
    │   └── mod.rs
    ├── models/
    │   └── mod.rs
    ├── handlers/
    │   └── mod.rs
    └── routes/
        └── mod.rs
```

All `mod.rs` files are empty.

---

## Framework-Specific main.rs

### Axum

- Uses `#[tokio::main]`
- Binds to `127.0.0.1:3000`
- Single `/` route returning `"Hello from Axum 🦀!"`

### Actix-web

- Uses `#[actix_web::main]`
- Binds to `127.0.0.1:3000`
- Single `/` route returning `"Hello from Actix-web 🦀!"`

---

## Add a Dependency

Adds a crate to the **current working directory’s Cargo project**.

```bash
forgeit add <crate_name>
```

Example:

```bash
forgeit add serde
```

Add a specific version:

```bash
forgeit add serde --version 1.0.197
```

Behavior:

- Default (`latest`): `cargo add <crate>`
- Specific version: `cargo add <crate>@<version>`

---

## Known Limitations

- No use of `git2` yet
- No framework validation
- No configuration files generated
- No database or Docker setup
- No tests
- `add` command does not target a specific project directory
- Minimal error handling

---

## License

MIT
