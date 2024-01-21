#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! chrono = "0.4.31"
//! ```

fn main() {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    println!("Hello, world! at {}", date);
}
