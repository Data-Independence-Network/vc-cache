extern crate alloc;
extern crate core;
extern crate std;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_core;
extern crate tokio_io;


pub mod data;
pub mod server;

fn main() {
    println!("Hello, world!");

    let test: Vec<u32> = Vec::with_capacity(4);
}
