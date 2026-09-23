// SPDX-FileCopyrightText: 2026 Contributors to ddcutil-varlink <https://github.com/digitaltrails/ddcutil-varlink>
// SPDX-License-Identifier: GPL-2.0-or-later

mod service;

use base64::Engine;
use base64::Engine as _;

use zbus::
blocking::connection;

use service::DdcutilService;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise the service object.
    let service = DdcutilService {
        dynamic_sleep: false,
        output_level: 0,
    };

    // Blocking builder: request the well-known name and register the
    // interface at the correct object path in one fluent chain.
    let _connection = connection::Builder::session()?
        .name("com.ddcutil.DdcutilService")?
        .serve_at(
            "/com/ddcutil/DdcutilObject",
            service
        )?
        .build()?;

    println!("DdcutilService running on D-Bus...");

    // Block forever (replace with your main loop / shutdown handling).
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}