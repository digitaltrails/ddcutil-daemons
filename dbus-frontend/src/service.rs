
use core::option;
use base64::Engine;
use base64::engine::general_purpose;
use ddcutil_backend::ddcutil;
use std::collections::HashMap;
use zbus::interface;
use zbus::object_server::SignalEmitter;

const DETECT_ALL: u32 = 8;
const EDID_PREFIX_ALLOWED: u32 = 1;

/// The main service object. Holds all state and (eventually).
pub struct DdcutilService {
    pub(crate) dynamic_sleep: bool,
    pub(crate) output_level: u32,
}

// ── Private helpers (not part of the D-Bus interface) ──────────────
impl DdcutilService {
    /// Shared body for `detect` and `list_detected`.
    ///
    /// When `force_redetect` is true, a rescan is triggered before listing.
    /// Returns `(status, displays, error_status, error_message)`.
    fn list_displays_impl(
        &self,
        flags: u32,
        force_redetect: bool,
    ) -> (
        i32,
        Vec<(i32, i32, i32, String, String, String, u16, String, u32)>,
        i32,
        String,
    ) {
        // Map a ddcutil error into the D-Bus (code, message) pair.
        let err_result = |e: ddcutil::Error| -> (i32, String) {
            let code: i32 = e.status_code().try_into().unwrap_or(0);
            let fn_name: &str = if force_redetect {
                "detect"
            } else {
                "list_displays"
            };
            (code, format!("{}: {}", fn_name, e))
        };

        // Step 1 (optional): force a redetect.
        if force_redetect {
            if let Err(e) = ddcutil::redetect() {
                let (code, msg) = err_result(e);
                return (0, Vec::new(), code, msg);
            }
        }

        // Step 2: list the detected displays.
        let all = flags & DETECT_ALL != 0;
        let list = match ddcutil::list_displays(all) {
            Ok(list) => list,
            Err(e) => {
                let (code, msg) = err_result(e);
                return (0, Vec::new(), code, msg);
            }
        };

        // Step 3: map each varlink display into the D-Bus tuple.
        let result_vector: Vec<_> = list
            .into_iter()
            .map(|disp| {
                (
                    disp.display_number,
                    disp.usb_bus,
                    disp.usb_device,
                    disp.manufacturer_id,
                    disp.model_name,
                    disp.serial_number,
                    disp.product_code,
                    general_purpose::STANDARD.encode(disp.edid_bytes),
                    ddcutil::edid_serial_number(&disp.edid_bytes),
                )
            })
            .collect();

        (0, result_vector, 0, String::new())
    }
}

#[interface(name = "com.ddcutil.DdcutilInterface")]
impl DdcutilService {
    // ── Methods ────────────────────────────────────────────────────────

    /// Restarts the service.
    fn restart(
        &mut self,
        text_options: &str,
        syslog_level: u32,
        flags: u32,
    ) -> (i32, String) {
        // TODO:
        println!(
            "Restart called: options={}, level={}, flags={}",
            text_options, syslog_level, flags
        );
        (0, String::new())
    }

    /// Detects connected displays.
    fn detect(
        &self,
        flags: u32,
    ) -> (
        i32,
        Vec<(i32, i32, i32, String, String, String, u16, String, u32)>,
        i32,
        String,
    ) {
        self.list_displays_impl(flags, true)
    }

    /// Lists already-detected displays.
    fn list_detected(
        &self,
        flags: u32,
    ) -> (
        i32,
        Vec<(i32, i32, i32, String, String, String, u16, String, u32)>,
        i32,
        String,
    ) {
        self.list_displays_impl(flags, false)
    }

    /// Gets a single VCP value.
    fn get_vcp(
        &self,
        display_number: i32,
        edid_txt: &str,
        vcp_code: u8,
        flags: u32,
    ) -> (u16, u16, String, i32, String) {

        let err_result = |e: ddcutil::Error| -> (u16, u16, String, i32, String) {
            let code: i32 = e.status_code().try_into().unwrap_or(0);
            (0, 0, "".to_string(), code, format!("get_vcp: {}", e))
        };

        let dref = match ddcutil::find_display(
            Option::Some(display_number.into()),
            Option::Some(edid_txt),
            flags & EDID_PREFIX_ALLOWED != 0) {
            Ok(dref) => dref,
            Err(e) => return err_result(e),
        };

        let handle = match ddcutil::open_display(dref) {
            Ok(handle) => handle,
            Err(e) => return err_result(e),
        };

        match ddcutil::get_vcp(&handle, vcp_code as u8) {
            Ok((current, max, formatted)) => (current as u16, max as u16, formatted, 0, "".to_string()),
            Err(e) => err_result(e),
        }
    }

    /// Gets multiple VCP values in one call.
    fn get_multiple_vcp(
        &self,
        display_number: i32,
        edid_txt: &str,
        vcp_codes: &[u8],
        flags: u32,
    ) -> (Vec<(u8, u16, u16, String)>, i32, String) {
        // TODO: call ddcutil backend
        (vec![], 0, String::new())
    }

    /// Sets a VCP value.
    fn set_vcp(
        &mut self,
        display_number: i32,
        edid_txt: &str,
        vcp_code: u8,
        vcp_new_value: u16,
        flags: u32,
    ) -> (i32, String) {
        // TODO: call ddcutil backend
        (0, String::new())
    }

    /// Sets a VCP value with a client context string.
    fn set_vcp_with_context(
        &mut self,
        display_number: i32,
        edid_txt: &str,
        vcp_code: u8,
        vcp_new_value: u16,
        client_context: &str,
        flags: u32,
    ) -> (i32, String) {
        // TODO: call ddcutil backend
        (0, String::new())
    }

    /// Gets metadata for a VCP code.
    fn get_vcp_metadata(
        &self,
        display_number: i32,
        edid_txt: &str,
        vcp_code: u8,
        flags: u32,
    ) -> (String, String, bool, bool, bool, bool, bool, i32, String) {
        // TODO: call ddcutil backend
        (
            "Feature".into(),
            "Description".into(),
            false,
            false,
            true,
            false,
            false,
            0,
            String::new(),
        )
    }

    /// Gets the capabilities string for a display.
    fn get_capabilities_string(
        &self,
        display_number: i32,
        edid_txt: &str,
        flags: u32,
    ) -> (String, i32, String) {
        // TODO: call ddcutil backend
        (String::new(), 0, String::new())
    }

    /// Gets parsed capabilities metadata.
    fn get_capabilities_metadata(
        &self,
        display_number: i32,
        edid_txt: &str,
        flags: u32,
    ) -> (
        String,
        u8,
        u8,
        HashMap<u8, String>,
        HashMap<u8, (String, String, HashMap<u8, String>)>,
        i32,
        String,
    ) {
        // TODO: call ddcutil backend
        (
            String::new(),
            0,
            0,
            HashMap::new(),
            HashMap::new(),
            0,
            String::new(),
        )
    }

    /// Gets the current state of a display.
    fn get_display_state(
        &self,
        display_number: i32,
        edid_txt: &str,
        flags: u32,
    ) -> (i32, String) {
        // TODO: call ddcutil backend
        (0, String::new())
    }

    /// Gets the current sleep multiplier.
    fn get_sleep_multiplier(
        &self,
        display_number: i32,
        edid_txt: &str,
        flags: u32,
    ) -> (f64, i32, String) {
        // TODO: call ddcutil backend
        (1.0, 0, String::new())
    }

    /// Sets the sleep multiplier.
    fn set_sleep_multiplier(
        &mut self,
        display_number: i32,
        edid_txt: &str,
        new_multiplier: f64,
        flags: u32,
    ) -> (i32, String) {
        // TODO: call ddcutil backend
        (0, String::new())
    }

    // ── Signals ────────────────────────────────────────────────────────
    // Signals must be declared async, even with the blocking API.
    // The `&SignalEmitter<'_>` first parameter is mandatory.

    #[zbus(signal)]
    async fn connected_displays_changed(
        signal_emitter: &SignalEmitter<'_>,
        edid_txt: &str,
        event_type: i32,
        flags: u32,
    ) -> zbus::Result<()> {}

    #[zbus(signal)]
    async fn vcp_value_changed(
        signal_emitter: &SignalEmitter<'_>,
        display_number: i32,
        edid_txt: &str,
        vcp_code: u8,
        vcp_new_value: u16,
        source_client_name: &str,
        source_client_context: &str,
        flags: u32,
    ) -> zbus::Result<()> {}

    #[zbus(signal)]
    async fn service_initialized(
        signal_emitter: &SignalEmitter<'_>,
        flags: u32,
    ) -> zbus::Result<()> {}

    // ── Properties ─────────────────────────────────────────────────────

    #[zbus(property)]
    fn attributes_returned_by_detect(&self) -> Vec<String> {
        vec![]
    }

    #[zbus(property)]
    fn status_values(&self) -> HashMap<i32, String> {
        HashMap::new()
    }

    #[zbus(property)]
    fn ddcutil_version(&self) -> &str {
        "0.0.0"
    }

    #[zbus(property)]
    fn ddcutil_dynamic_sleep(&self) -> bool {
        self.dynamic_sleep
    }

    #[zbus(property)]
    fn set_ddcutil_dynamic_sleep(&mut self, value: bool) {
        self.dynamic_sleep = value;
    }

    #[zbus(property)]
    fn ddcutil_output_level(&self) -> u32 {
        self.output_level
    }

    #[zbus(property)]
    fn set_ddcutil_output_level(&mut self, value: u32) {
        self.output_level = value;
    }

    #[zbus(property)]
    fn display_event_types(&self) -> HashMap<i32, String> {
        HashMap::new()
    }

    #[zbus(property)]
    fn service_interface_version(&self) -> &str {
        "1.0"
    }

    #[zbus(property)]
    fn service_info_logging(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn set_service_info_logging(&mut self, _value: bool) {}

    #[zbus(property)]
    fn service_emit_connectivity_signals(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn set_service_emit_connectivity_signals(&mut self, _value: bool) {}

    #[zbus(property)]
    fn service_emit_signals(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn set_service_emit_signals(&mut self, _value: bool) {}

    #[zbus(property)]
    fn service_flag_options(&self) -> HashMap<i32, String> {
        HashMap::new()
    }

    #[zbus(property)]
    fn service_parameters_locked(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn service_poll_interval(&self) -> u32 {
        0
    }

    #[zbus(property)]
    fn set_service_poll_interval(&mut self, _value: u32) {}

    #[zbus(property)]
    fn service_poll_cascade_interval(&self) -> f64 {
        0.0
    }

    #[zbus(property)]
    fn set_service_poll_cascade_interval(&mut self, _value: f64) {}
}

