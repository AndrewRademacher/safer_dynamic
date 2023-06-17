#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString};

include!(concat!(env!("OUT_DIR"), "/dynomath.rs"));

pub struct Client(*mut DynoClient_t);

impl Client {
    pub fn new() -> Self {
        Self(unsafe { DYNO_new_client() })
    }

    pub fn request(&self, url: &str) -> Response {
        let url = CString::new(url).unwrap();
        Response(unsafe { DYNO_request(self.0, url.as_ptr()) })
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe { DYNO_free_client(self.0) }
    }
}

pub struct Response(DynoResponse_t);

impl Response {
    pub fn error(&self) -> DynoError_t {
        self.0.error
    }

    pub fn status_code(&self) -> i16 {
        self.0.status_code
    }

    pub fn text(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.text).to_str().unwrap() }
    }
}

impl Drop for Response {
    fn drop(&mut self) {
        unsafe { DYNO_free_response(self.0) };
    }
}
