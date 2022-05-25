use std::cmp;

pub mod position;
pub mod simulation_step;
pub mod distance;
pub mod touching_kill_region;

fn closest_distance_to_point(from: usize, point_1: usize, point_2: usize) -> usize {
    let d1 = (from as isize - point_1 as isize).abs() as usize;
    let d2 = (from as isize - point_2 as isize).abs() as usize;
    cmp::min(d1, d2)
}
