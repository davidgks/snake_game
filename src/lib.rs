use wasm_bindgen::prelude::*;

#[wasm_bindgen] // this is necessary to export it to the wasm
pub fn greet(name: &str) {
    println!("Hi there {}", name);
}

// we use the command "wasm-pack build --target web" to target the web (i.e. JS)