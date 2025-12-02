pub fn brute_force(arr: Vec<i32>) -> i32 {
    for i in 1..arr.len() {
        if arr[i - 1 ] < arr[i] && arr[i + 1] < arr[i] {
            return i as i32;
        }
    }

    -1
}