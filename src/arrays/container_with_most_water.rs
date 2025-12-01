use std::cmp::{max, min};

pub fn brute_force(heights: Vec<i32>) -> i32 {
    let mut max_water: i32 = 0;

    for i in 0..heights.len() {
        for j in i+1..heights.len() {
            let width = j - i ;
            let height = min(heights[i], heights[j]);
            let water = width * height as usize;

            max_water = max(water as i32, max_water);

        }
    }

    max_water
}