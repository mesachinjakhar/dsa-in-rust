pub fn stock_span(arr: Vec<u32>) -> Vec<u32> {
        let n = arr.len();
        let mut stack: Vec<usize> = Vec::new();
       let mut ans: Vec<u32> = Vec::with_capacity(n);

    for i in 0..n {
        while !stack.is_empty() && arr[i] >= arr[*stack.last().unwrap()] {
            stack.pop();
        }

        if stack.is_empty() {
            ans.push((i+1) as u32 );
        } else {
            ans.push((i - stack.last().unwrap()) as u32);
        } 
        stack.push(i);
    }
    ans
}