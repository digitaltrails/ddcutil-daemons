// SPDX-FileCopyrightText: 2026 Contributors to ddc-ci-daemons <https://github.com/digitaltrails/ddc-ci-daemons>
// SPDX-License-Identifier: GPL-2.0-or-later

fn main() {
    println!("cargo:rerun-if-changed=varlink/local.ddc-ci.service.varlink");
    varlink_generator::cargo_build("varlink/local.ddc-ci.service.varlink");
}
