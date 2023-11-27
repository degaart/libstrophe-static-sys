#![allow(unused)]

use std::path::Path;
use std::process::Command;
use std::{env, path::PathBuf};
use std::fs;

const TARBALL: &str = "libstrophe-0.12.3.tar.bz2";
const EXTRACT_DIR: &str = "libstrophe-0.12.3";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR")
        .unwrap_or(
            env::current_dir()
                .unwrap()
                .join("target")
                .display()
                .to_string()
        ));
    let prefix = out_dir.join("prefix");

    println!("cargo:rerun-if-changed={TARBALL}");
    println!("cargo:rustc-link-search={}", prefix.join("lib").display());

    if !prefix.join("include").join("strophe.h").exists() {
        let ret = Command::new("tar")
            .arg("jxf")
            .arg(TARBALL)
            .arg("-C")
            .arg(&out_dir)
            .status()
            .unwrap();
        if !ret.success() {
            panic!("Extraction failed");
        }

        let extract_dir = out_dir.join(EXTRACT_DIR);

        let configure = extract_dir
            .join("configure");
        let ret = Command::new(&configure)
            .arg("--disable-shared")
            .arg("--disable-dependency-tracking")
            .arg("--disable-examples")
            .arg("--disable-tls")
            .arg("--with-pic")
            .arg(&format!("--prefix={}", prefix.display()))
            .current_dir(&out_dir)
            .status()
            .unwrap();
        if !ret.success() {
            panic!("configure failed");
        }

        let ret = Command::new("make")
            .arg(&format!("-j{}", num_cpus::get()))
            .current_dir(&out_dir)
            .status()
            .unwrap();
        if !ret.success() {
            panic!("build failed");
        }

        let ret = Command::new("make")
            .arg("install")
            .current_dir(&out_dir)
            .status()
            .unwrap();
        if !ret.success() {
            panic!("install failed");
        }
    }
}

