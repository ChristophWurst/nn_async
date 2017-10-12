extern crate nn_async;

use nn_async::*;

fn main() {
    println!("Started");
    find_nn();
    println!("Sync complete");
    find_nn_async();
    println!("Async complete");
}
