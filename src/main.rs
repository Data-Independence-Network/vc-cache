extern crate byteorder;
extern crate bytes;
extern crate smallvec;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_core;
extern crate tokio_io;


pub mod data;
pub mod logic;
pub mod server;

fn main() {
    println!("Hello, world!");

    let test: Vec<u32> = Vec::with_capacity(4);

    server::start_small_load_optimized(app, "0.0.0.0", 4321);
}
