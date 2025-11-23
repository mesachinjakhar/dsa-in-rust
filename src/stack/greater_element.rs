pub fn greater_element(arr: Vec<i32> ) -> Vec<i32> {
    let n = arr.len(); 
    let mut stack: Vec<i32> = Vec::with_capacity(n);
    let mut ans: Vec<i32> = Vec::with_capacity(n);

    for i in (0..n).rev() {
        while !stack.is_empty() && arr[i] > *stack.last().unwrap() {
            stack.pop();
        }

        if stack.is_empty() {
            ans.push(-1);
        } else {
            ans.push(*stack.last().unwrap());
        }

        stack.push(arr[i]);
    }

    ans.reverse();
    ans
}