#![allow(non_snake_case)]

extern crate libc;

#[link(name = "RailDriver")]
extern "C" {
    pub fn GetRailSimValue(id: libc::c_int, modifier: libc::c_int) -> libc::c_float;
    pub fn SetRailSimValue(id: libc::c_int, value: libc::c_int);

    pub fn SetRailDriverConnected(isConnected: bool);
    pub fn GetNextRailDriverId(start: libc::c_int) -> libc::c_int;
    pub fn GetRailDriverValue(id: libc::c_int) -> libc::c_float;

    pub fn GetRailSimConnected() -> bool;
    pub fn SetRailSimConnected(isConnected: bool);
    pub fn GetRailSimLocoChanged() -> bool;
    pub fn GetRailSimCombinedThrottleBrake() -> bool;
}
