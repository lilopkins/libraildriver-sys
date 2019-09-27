#![allow(non_snake_case)]
extern crate libc;
extern crate libloading as lib;

use libc::{c_int, c_float};

#[cfg(target_arch = "x86")]
unsafe fn get_library() -> lib::Library {
    lib::Library::new("RailDriver.dll").unwrap()
}

#[cfg(target_arch = "x86_64")]
unsafe fn get_library() -> lib::Library {
    lib::Library::new("RailDriver64.dll").unwrap()
}

pub unsafe fn GetRailSimValue(id: c_int, modifier: c_int) -> c_float {
    let rd = get_library();
    rd.get::<unsafe extern fn(c_int, c_int) -> c_float>(b"GetRailSimValue").unwrap()(id, modifier)
}
pub unsafe fn SetRailSimValue(id: c_int, value: c_int) {
    let rd = get_library();
    rd.get::<unsafe extern fn(c_int, c_int)>(b"SetRailSimValue").unwrap()(id, value)
}

pub unsafe fn SetRailDriverConnected(isConnected: bool) {
    let rd = get_library();
    rd.get::<unsafe extern fn(bool)>(b"SetRailDriverConnected").unwrap()(isConnected)
}
pub unsafe fn GetNextRailDriverId(start: c_int) -> c_int {
    let rd = get_library();
    rd.get::<unsafe extern fn(c_int) -> c_int>(b"GetNextRailDriverId").unwrap()(start)
}
pub unsafe fn GetRailDriverValue(id: c_int) -> c_float {
    let rd = get_library();
    rd.get::<unsafe extern fn(c_int) -> c_float>(b"GetRailDriverValue").unwrap()(id)
}

pub unsafe fn GetRailSimConnected() -> bool {
    let rd = get_library();
    rd.get::<unsafe extern fn() -> bool>(b"GetRailSimConnected").unwrap()()
}
pub unsafe fn SetRailSimConnected(isConnected: bool) {
    let rd = get_library();
    rd.get::<unsafe extern fn(bool)>(b"SetRailSimConnected").unwrap()(isConnected)
}
pub unsafe fn GetRailSimLocoChanged() -> bool {
    let rd = get_library();
    rd.get::<unsafe extern fn() -> bool>(b"GetRailSimLocoChanged").unwrap()()
}
pub unsafe fn GetRailSimCombinedThrottleBrake() -> bool {
    let rd = get_library();
    rd.get::<unsafe extern fn() -> bool>(b"GetRailSimCombinedThrottleBrake").unwrap()()
}
