//! Adds the `git rev` as environment variable `GIT_HEAD_SHORT` during the build
//! process, if hthe build environment variable `PROFILE` is not `debug` during
//! the build process.

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=migrations");

    // Only git env variables when we do a release
    if std::env::var("PROFILE").unwrap_or_else(|_| "debug".into()) == "debug" {
        return;
    }

    let git_head_short = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let git_head_short = String::from_utf8(git_head_short.stdout).unwrap();

    println!("cargo:rustc-env=GIT_HEAD_SHORT={}", git_head_short);
}
