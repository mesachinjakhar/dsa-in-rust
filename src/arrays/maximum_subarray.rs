pub fn brute_force(nums: Vec<i32>) -> i32 {
    let mut max = i32::MIN;

    for i in 0..nums.len() {
        let mut sum = nums[i];
        for j in i+1..nums.len() {
            sum = sum + nums[j];
            if sum > max {
                max = sum;
            }
        }
    }
    return max;
}

