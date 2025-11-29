// brute force 
pub fn brute_force(arr: Vec<i32>) -> i32 {
    let n = arr.len(); 

    for i in 0..n {
        let mut count = 0;
        for j in 0..n {
            if arr[j] == arr[i] {
                count = count + 1;
            }
        }
        if count == 1 {
            return arr[i];
        }
    }

    -1
}

pub fn optimize_approach(nums: Vec<i32>) -> i32 {
    let mut result = 0; 
    for i in 0..nums.len() {
        result ^= nums[i];
    }
    result
}