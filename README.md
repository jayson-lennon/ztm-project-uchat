# uChat Project Info

## Design

The `design/` directory contains a some design-related files:

| File                              | Purpose                                                                          |
| --------------------------------- | -------------------------------------------------------------------------------- |
| database.dbm                      | [pgModeler](https://pgmodeler.io/) database modeling file                        |
| database.svg                      | Visual overview of the database tables exported from `database.dbm`              |
| mockup.svg                        | Visual sample of how the final application should look                           |
| ui-elements.svg                   | Icons for the UI. These get exported to `frontend/static/icons`                  |
| wireframes-ui.svg                 | Overview of the pages available in the application                               |
| wireframes-modules.svg            | Shows the names of the Rust modules and the pages belonging to each              |
| wireframes-navigation.svg         | Overview of user flows mapped onto the pages of the application                  |
| wireframes-inkscape-composite.svg | The above `wireframes-*` files in a single [Inkscape](https://inkscape.org/) SVG |

## Initial Setup

If you are on Windows, using
[WSL](https://learn.microsoft.com/en-us/windows/wsl/install) is recommended to
manage build dependencies and tooling.

### Rust

If you haven't installed Rust yet, you can do so using
[rustup](https://rustup.rs/) and then install
[cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Compiling Rust for the browser also requires adding the `wasm32` compilation target:

```bash
rustup target add wasm32-unknown-unknown
```

### Database

This project uses [PostgreSQL](https://www.postgresql.org/) for the database.
Please follow the [official instructions](https://www.postgresql.org/download/)
for how to install PostgreSQL on your system.

### Trunk

[Trunk](https://trunkrs.dev/) is a tool to build and bundle Rust WASM
applications. Install with:

```bash
cargo install --locked trunk

# Apple M1 users also need to install this:
cargo install --locked wasm-bindgen-cli
```

### Diesel

[Diesel](https://diesel.rs/) is a Rust SQL query builder for working with the
database.

Make sure you have [installed PostgreSQL](https://www.postgresql.org/download/)
before proceeding.

Install Diesel with:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

If you receive build or linker errors, make sure you install `libpq`. This may
be packaged separately depending on your operating system and package manager
(`libpq-dev` on Ubuntu/WSL, for example).

### Create new database

Create a `.env` file in the workspace directory containing:

```bash
DATABASE_URL=postgres://DATABASE_USER:PASSWORD@localhost/uchat
TEST_DATABASE_URL=postgres://DATABASE_USER:PASSWORD@localhost/uchat_test
```

Substitute these:

- `DATABASE_USER`: role created to access PostgreSQL
- `PASSWORD`: your password to login to the database (omit `:PASSWORD` if
  not using a password)

After the `.env` is ready, run this command to create the database:

```bash
diesel setup
```

### npm

This project uses [Tailwind CSS](https://tailwindcss.com/) for utility classes.
To use Tailwind, you'll need to [install
npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) to use
the `npx` command.

### (Optional) `just`

[just](https://github.com/casey/just) is a command runner which can simplify
running different development commands. Install with:

```bash
cargo install just
```

## Commands

### Documentation

To build the documentation:

```bash
cargo doc -F docbuild
```

There is a minor bug in the published version of a transitive dependency.
Enabling the `docbuild` feature is a temporary workaround until the dependency
gets updated.

### Check / Clippy

Check the two different targets (frontend and backend):

```bash
cargo check -p frtonend --target wasm32-unknown-unknown
cargo check --workspace --exclude frontend
```

Run clippy for the two different targets (frontend and backend):

```bash
cargo check -p frtonend --target wasm32-unknown-unknown
cargo check --workspace --exclude frontend
```

### Project Init

This will check for the dependencies listed above and attempt to install the Rust
dependencies. Dependencies which require manual install will provide a link to
installation instructions.

```bash
cargo run -p project-init
```

### Development Server

To run a dev server for the frontend and open a new browser window:

```bash
trunk serve --open
```

To run the backend server:

```bash
cargo run -p uchat_server
```

### Build for production

To build the project for distribution:

```bash
trunk --config Trunk-release.toml build
cargo build --release --workspace --exclude frontend
```

### Migrations

To create database migrations, run:

```bash
diesel migration generate MIGRATION_NAME
```

The migrations will get created in `data/migrations/timestamp_MIGRATION_NAME/`.
Add your SQL for applying the migration to `up.sql` and the SQL for reverting
the migration to `down.sql`.

After adding your migration code to `up.sql` and `down.sql`, apply the
migration with:

```bash
diesel migration run
```

To make sure you `down.sql` works, run this command to revert and then reapply
the migration:

```bash
diesel migration redo
```

After creating a new migration, delete the testing database using:

```bash
psql -d postgres -c 'DROP DATABASE uchat_test;'
```
