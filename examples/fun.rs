use intercomm::{make_command, Convention};
use io_lifetimes::OwnedFd;
use std::io::Write;
use std::sync::Arc;

fn main() {
    let file = std::fs::File::open("Cargo.toml").unwrap();

    let output = make_command(
        "target/debug/examples/debug",
        &["--use-fd".into(), Arc::new(OwnedFd::from(file)).into()],
        &[],
        Convention::Implicit,
    )
    .output()
    .unwrap();

    println!("status: {}", output.status);
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
}
