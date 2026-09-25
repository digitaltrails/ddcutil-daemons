// SPDX-FileCopyrightText: 2026 Contributors to ddcutil-daemons <https://github.com/digitaltrails/ddcutil-daemons>
// SPDX-License-Identifier: GPL-2.0-or-later

fn main() {
    println!("cargo:rerun-if-changed=varlink/com.ddcutil.service.varlink");
    varlink_generator::cargo_build("varlink/com.ddcutil.service.varlink");
}
