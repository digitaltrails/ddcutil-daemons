// SPDX-FileCopyrightText: 2026 Contributors to ddc-ci-daemons <https://github.com/digitaltrails/ddc-ci-daemons>
// SPDX-License-Identifier: GPL-2.0-or-later

///! DDC CI D-Bus service

mod ddc_ci_dbus_service;
use log::{info};
use zbus::blocking::connection;
use ddc_ci_dbus_service::DdcCiDbusService;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();

    // Initialize the service object.
    let service = DdcCiDbusService {
        dynamic_sleep: false,
        output_level: 0,
    };

    // Blocking builder: request the well-known name and register the
    // interface at the correct object path in one fluent chain.
    let _connection = connection::Builder::session()?
        .name(DdcCiDbusService::SERVICE_NAME)?
        .serve_at(
            DdcCiDbusService::OBJECT_PATH,
            service
        )?
        .build()?;

    let interface_name = <DdcCiDbusService as zbus::object_server::Interface>::name();
    info!("Running D-Bus Service: {}; Object: {}; Interface: {}",
        DdcCiDbusService::SERVICE_NAME,
        DdcCiDbusService::OBJECT_PATH,
        interface_name);

    // Block forever 
    // TODO: shutdown handling.
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}