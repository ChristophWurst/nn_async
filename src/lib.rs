extern crate rand;

mod nn;

use rand::Rng;
use std::vec::Vec;

pub fn find_nn() {
    let mut rand = rand::thread_rng();

    let points: Vec<nn::Point> = (0..50000)
        .map(|_| {
            nn::Point {
                x: rand.next_f32(),
                y: rand.next_f32(),
                z: rand.next_f32(),
            }
        })
        .collect();
    let indices: Vec<usize> = (0..points.len())
        .map(|idx| nn::find_closest(&points, idx))
        .collect();
    indices.iter().take(10).for_each(|idx| println!("{}", idx));
}
