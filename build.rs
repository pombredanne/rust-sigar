extern crate pkg_config;

use std::fs;
use std::process::{Command, Stdio};
use std::env;
use std::path::Path;

fn main() {
    match pkg_config::find_library("libsigar") {
        Ok(..) => return,
        Err(..) => {}
    }

    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_dir = env::var("OUT_DIR").unwrap();

    let src = Path::new(&cargo_dir[..]);
    let dst = Path::new(&output_dir[..]);
    let root = src.join("sigar");

    run(Command::new("sh")
            .arg("-c")
            .arg(&root.join("autogen.sh"))
            .current_dir(&root));

    let _ = fs::remove_dir_all(&dst.join("build-include"));
    let _ = fs::remove_dir_all(&dst.join("build-tests"));
    let _ = fs::remove_dir_all(&dst.join("build-src"));
    let _ = fs::remove_dir_all(&dst.join("build"));
    fs::create_dir(&dst.join("build")).unwrap();

    let mut config_opts = Vec::new();
    config_opts.push(format!("{:?}", root.join("configure")));
    config_opts.push(format!("--prefix={:?}", dst));

    run(Command::new("sh")
            .arg("-c")
            .arg(&config_opts.connect(" "))
            .current_dir(&dst.join("build")));

    run(Command::new(make())
            .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
            .current_dir(&dst.join("build")));

    run(Command::new(make())
            .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
            .arg("install")
            .current_dir(&dst.join("build")));

    let _ = fs::remove_file(root.join("test-driver"));

    println!("cargo:rustc-flags=-L {}/lib -l sigar", dst.display());
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}/include", dst.display());
}

fn make() -> &'static str {
    if cfg!(target_os = "freebsd") {"gmake"} else {"make"}
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}
