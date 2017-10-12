extern crate futures;
extern crate futures_cpupool;
extern crate rand;

mod nn;

use futures::Future;
use futures_cpupool::CpuPool;
use rand::Rng;
use std::sync::Arc;
use std::vec::Vec;

pub fn find_nn() {
    let pool = CpuPool::new_num_cpus();
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

    let mut futures: Vec<futures_cpupool::CpuFuture<usize, ()>> = Vec::with_capacity(points.len());
    let points = Arc::new(points);
    for idx in 0..points.len() {
        let pz = points.clone();
        let x = idx.clone();
        let fut = pool.spawn_fn(move || futures::future::ok(nn::find_closest(&pz, x)));
        futures.push(fut);
    }

    futures::future::join_all(futures)
        .map(|results| {
            results.iter().take(10).for_each(|idx| println!("{}", idx));
        })
        .wait()
        .expect("not to fail");
}
