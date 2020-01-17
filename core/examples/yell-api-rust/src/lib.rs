mod gatewasm;

use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn handler(request: i32) -> i32 {
    let request: HashMap<String, String> = gatewasm::get_request(request).unwrap();

    let mut output = request.get("input").unwrap().to_uppercase();
    output.push_str("!!1!!!!");

    let mut response: HashMap<String, String> = HashMap::new();
    response.insert(String::from("output"), output);
    gatewasm::build_response(&response).unwrap()
}
