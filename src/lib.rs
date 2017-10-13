extern crate futures;
extern crate futures_cpupool;
extern crate num_cpus;
extern crate rand;

mod nn;

use futures::Future;
use futures_cpupool::CpuPool;
use rand::Rng;
use std::sync::Arc;
use std::vec::Vec;

const NUM_POINTS: usize = 50000;

fn init_points() -> Vec<nn::Point> {
    let mut rand = rand::thread_rng();

    (0..NUM_POINTS)
        .map(|_| {
            nn::Point {
                x: rand.next_f32(),
                y: rand.next_f32(),
                z: rand.next_f32(),
            }
        })
        .collect()
}

pub fn find_nn() {
    let points = init_points();
    let indices: Vec<usize> = (0..NUM_POINTS)
        .map(|idx| nn::find_closest(&points, idx))
        .collect();
    indices.iter().take(10).for_each(|idx| println!("{}", idx));
}

pub fn find_nn_async() {
    let pool = CpuPool::new_num_cpus();
    let points = init_points();

    let mut futures: Vec<futures_cpupool::CpuFuture<usize, ()>> = Vec::with_capacity(NUM_POINTS);
    let points = Arc::new(points);
    for idx in 0..NUM_POINTS {
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

pub fn find_nn_chunks() {
    let pool = CpuPool::new_num_cpus();
    let points = init_points();
    let num_cpus = num_cpus::get();
    println!("Running on {} logica CPUs", num_cpus);
    let indices = (0..NUM_POINTS).collect::<Vec<usize>>();
    let chunks = indices.chunks(num_cpus);

    let points = Arc::new(points);
    let mut futures: Vec<futures_cpupool::CpuFuture<Vec<usize>, ()>> = Vec::with_capacity(num_cpus);
    for chunk in chunks {
        let pz = points.clone();
        let c: Vec<usize> = chunk.iter().map(|x| *x).collect::<Vec<usize>>();
        futures.push(pool.spawn_fn(move || {
            let mut idxs = vec![];
            for idx in c {
                idxs.push(nn::find_closest(&pz, idx));
            }
            futures::future::ok(idxs)
        }));
    }
    futures::future::join_all(futures)
        .map(|results| {
            let mut all = vec![];
            for result in results {
                for r in result {
                    all.push(r);
                }
            }
            all.iter().take(10).for_each(|idx| println!("{}", idx));
        })
        .wait()
        .expect("not to fail");
}
