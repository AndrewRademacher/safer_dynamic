use rcalc::{Client, DYNO_add, DynoError_DYNO_ERROR_OK};

fn main() {
    unsafe { run() };
}

unsafe fn run() {
    let sum = DYNO_add(235, 84);
    println!("Sum: {sum}");

    let client = Client::default();
    let response = client.request("https://httpbin.org/json");
    if response.error() != DynoError_DYNO_ERROR_OK {
        println!("Error: {}", response.error());
    } else {
        println!("Status: {}", response.status_code());
        println!("{}", response.text());
    }
}
