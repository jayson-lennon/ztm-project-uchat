// forces the `embed_migrations!` macro to run when migrations get updated
fn main() {
    println!("cargo:rerun-if-changed=../migrations");
}
