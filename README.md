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

## `git tag`

Over time, the `Cargo.lock` and `Cargo.toml` files will diverge from what is
shown in the videos. This will cause lots of extraneous data to be displayed
when trying to view the `git diff` output between your code and what is shown
in the videos. To get useful `diff` output use this command:

```sh
git diff VIDEO_GIT_TAG ':(exclude)Cargo.lock' ':(exclude)*/Cargo.toml'`
```

and substitute VIDEO_GIT_TAG with the one shown in the video (use double quotes
" " if you are on Windows). This will ignore these files in the diff output,
allowing you to see the actual code changes between videos.

## Updates / Compiler errors

Throughout the videos for this project we use the `cargo add` command to add
dependencies. This pulls in the most recent version of the dependency which
will likely differ from the version that was pulled while recording the videos.
APIs change over time, so some adjustments are necessary while creating this project.

Here is a list of potential issues you may encounter, and how to update your
code to fix them.

### `nutype`

```text
error: Unknown validation rule `present`
 --> shared/domain/src/post.rs:5:19
  |
5 | #[nutype(validate(present, max_len = 50))]
  |                   ^^^^^^^
```

```text
error[E0599]: no variant or associated item named `Missing` found for enum `UsernameError` in the current scope
  --> shared/domain/src/user.rs:14:28
   |
7  | #[nutype(validate(not_empty, min_len = 3, max_len = 30))]
   | --------------------------------------------------------- variant or associated item `Missing` not found for this enum
...
14 |             UsernameError::Missing => "User name cannot be empty.",
   |                            ^^^^^^^ variant or associated item not found in `UsernameError`
```

The `nutype` crate now uses the `not_empty` rule instead of `present`. To fix
this, two changes need to be made:

1. Replace all `#[nutype(validate(present, ...))]` with
   `#[nutype(validate(not_empty, ...))]`
2. In all `impl` blocks where we create `fn formatted_error`, change
   `StructError::Missing` to `StructError::Empty`.
