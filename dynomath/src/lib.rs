#![allow(non_snake_case)]

use std::ffi::c_short;

use safer_ffi::prelude::*;

#[ffi_export]
pub fn DYNO_add(a: i64, b: i64) -> i64 {
    a + b
}

#[derive_ReprC]
#[repr(opaque)]
pub struct DynoClient {
    client: reqwest::Client,
}

#[derive_ReprC]
#[repr(C)]
pub struct Response {
    pub error: DynoError,
    pub status_code: c_short,
    pub text: char_p::Box,
}

#[ffi_export]
pub fn DYNO_new_client() -> repr_c::Box<DynoClient> {
    Box::new(DynoClient {
        client: reqwest::Client::new(),
    })
    .into()
}

/// Free the previously created dyno client.
#[ffi_export]
pub fn DYNO_free_client(client: repr_c::Box<DynoClient>) {
    drop(client)
}

#[ffi_export]
pub fn DYNO_request(client: &mut DynoClient, url: char_p::Ref<'_>) -> Response {
    match request(client, url.to_str()) {
        Ok((status_code, text)) => Response {
            error: DynoError::Ok,
            status_code: status_code as c_short,
            text: text.try_into().unwrap(),
        },
        Err(e) => Response {
            error: e,
            status_code: 0,
            text: "".to_string().try_into().unwrap(),
        },
    }
}

fn request(client: &mut DynoClient, url: &str) -> Result<(u16, String), DynoError> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| DynoError::RuntimeFailed)?;

    rt.block_on(async {
        let resp = client
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| DynoError::FailedToConnect)?;

        let status_code = resp.status().as_u16();
        let text = resp
            .text()
            .await
            .map_err(|_| DynoError::FailedToReceiveBody)?;

        Ok((status_code, text))
    })
}

#[ffi_export]
pub fn DYNO_free_response(response: Response) {
    drop(response)
}

#[derive_ReprC]
#[repr(u32)]
pub enum DynoError {
    Ok = 0,
    RuntimeFailed = 1,
    FailedToConnect = 2,
    FailedToReceiveBody = 3,
}

#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("include/dynomath.h")?
        .generate()
}
