pub fn brute_force(arr: Vec<i32>) -> i32 {
    for i in 0..arr.len() {
        let mut count = 0;
        for j in 0..arr.len() {
            if arr[j] == arr[i] {
                count = count + 1;
            }
        }
        if count > arr.len() / 2 {
            return arr[i];
        }
    }

    0
}