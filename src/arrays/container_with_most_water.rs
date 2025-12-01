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

pub fn optimize_approach(heights: Vec<i32>) -> i32 {
    let mut left = 0 as usize;
    let mut right = heights.len() - 1; 

    let mut max_water = 0;

    while left < right {
        let width = right - left;
        let height = min(heights[left], heights[right]);

        let curr_water = width * height as usize;

        max_water = max(curr_water, max_water);

        if heights[left] < heights[right] {
            left += 1;
        } else {
            right -= 1;
        }

    }

    max_water as i32

}