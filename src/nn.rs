use std::f32;
use std::vec::Vec;

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn norm(a: &Point, b: &Point) -> f32 {
    (a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)
}

pub fn find_closest(p_points: &Vec<Point>, idx: usize) -> usize {
    let mut index: usize = 0;
    let p_point = &p_points[idx];

    let mut min_so_far = f32::MAX;

    (0..p_points.len()).for_each(|i| {
        if i == idx {
            return;
        }

        let p = &p_points[i];
        let dist = norm(&p_point, &p);
        if dist < min_so_far {
            min_so_far = dist;
            index = i as usize;
        }
    });

    index
}
