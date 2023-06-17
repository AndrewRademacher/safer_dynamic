use std::ffi::{CStr, CString};

use rcalc::{
    DYNO_add, DYNO_free_client, DYNO_free_response, DYNO_new_client, DYNO_request,
    DynoError_DYNO_ERROR_OK,
};

fn main() {
    unsafe { run() };
}

unsafe fn run() {
    let sum = DYNO_add(235, 84);
    println!("Sum: {sum}");

    let client = DYNO_new_client();
    let url = CString::new("https://httpbin.org/json").unwrap();
    let resp = DYNO_request(client, url.as_ptr());
    if resp.error != DynoError_DYNO_ERROR_OK {
        println!("Error: {}", resp.error);
    } else {
        println!("Status: {}", resp.status_code);
        println!("{}", CStr::from_ptr(resp.text).to_str().unwrap());
    }

    DYNO_free_response(resp);
    DYNO_free_client(client);
}
