fn smaller_element(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut stack: Vec<i32> = Vec::with_capacity(n);
    let mut ans: Vec<i32> = Vec::new();

    for i in 0..n {
        while !stack.is_empty() && nums[i] > *stack.last().unwrap() {
            stack.pop();
        }

        if stack.is_empty() {
            ans.push(-1);
        } else {
            ans.push(*stack.last().unwrap());
        }

        stack.push(nums[i]);
    }
    ans
}