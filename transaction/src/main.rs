#[macro_use]
extern crate dotenv_codegen;

mod signature;

extern crate dotenv;

fn main() {
    println!("Hello, world!");
    signature::create_tx();
}
