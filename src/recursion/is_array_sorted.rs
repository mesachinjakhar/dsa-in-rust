pub fn optimize_approach(arr: Vec<i32>, idx: usize) -> bool {

    // cases with answers
    if arr.len() == 1 {
        return true;
    }

    // base case
    if idx == arr.len() - 1 {
        return arr[idx] > arr[idx - 1];
    }

    if idx == 0 {
        return optimize_approach(arr, idx + 1);
    } 
    if arr[idx] > arr[idx - 1] {
        return optimize_approach(arr, idx + 1);
    }

    false

}