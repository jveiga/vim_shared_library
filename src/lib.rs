extern crate rustc_serialize;
use rustc_serialize::json;

use std::ffi::{CString, CStr};

use std::os::raw::{c_char,c_int};

fn default() -> CString {
    CString::new("").unwrap()
}

#[no_mangle]
pub fn hn(arg: *const c_char) -> c_int {
    let slice: &CStr = unsafe { CStr::from_ptr(arg) };
    let ret = slice.to_bytes().len();

    ret as i32
}


#[derive(RustcDecodable, RustcEncodable)]
pub struct Payload {
    data_int: i32,
}

#[no_mangle]
pub fn hi(arg: *const c_char) -> *const c_char {
    let slice: &CStr = unsafe { CStr::from_ptr(arg) };

    let s : String = slice.to_string_lossy().into_owned();

    let mut decoded: Payload = if let Ok(dec) = json::decode(&s) {
        dec
    } else {
        return default().into_raw();
    };

    decoded.data_int += 42;

    let encoded = json::encode(&decoded).unwrap();

    let c_to_print = CString::new(encoded).unwrap();

    c_to_print.into_raw()
}
