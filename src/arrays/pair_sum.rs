pub fn brute_force(arr: Vec<i32>, target: i32) -> Vec<i32>{
    let mut ans: Vec<i32> = Vec::new();
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] + arr[j] == target {
                ans.push(arr[i]);
                ans.push(arr[j]);
            }
        }
    }
    ans
}

pub fn optimize_approach(arr: Vec<i32>, target: i32) -> Vec<i32> {
    let mut start = 0 as usize;
    let mut end = arr.len() - 1; 
    let mut ans: Vec<i32> = Vec::new();

    while start < end {
        if arr[start] + arr[end] == target {
            ans.push(arr[start]);
            ans.push(arr[end]);
        }
        if arr[start] + arr[end] > target {
            end = end - 1; 
        } else {
            start = start + 1;
        }
    }

    ans

}