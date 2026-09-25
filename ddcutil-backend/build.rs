// SPDX-FileCopyrightText: 2026 Contributors to ddcutil-daemons <https://github.com/digitaltrails/ddcutil-daemons>
// SPDX-License-Identifier: GPL-2.0-or-later

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    println!("cargo:rustc-link-lib=ddcutil");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings for libddcutil");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
