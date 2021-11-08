use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=migrations");

    // Only git env variables when we do a release
    if std::env::var("PROFILE").unwrap_or("debug".into()) == "debug" {
        return;
    }

    let git_head_long = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_head_long = String::from_utf8(git_head_long.stdout).unwrap();

    let git_head_short = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let git_head_short = String::from_utf8(git_head_short.stdout).unwrap();

    println!("cargo:rustc-env=GIT_HEAD_LONG={}", git_head_long);
    println!("cargo:rustc-env=GIT_HEAD_SHORT={}", git_head_short);
}
