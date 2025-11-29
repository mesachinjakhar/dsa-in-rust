pub fn linear_search(arr: Vec<i32>, target: i32) -> Option<usize> {

    for i in 0..arr.len() {
        if arr[i] == target {
            return Some(i) 
        }
    }
    None
}