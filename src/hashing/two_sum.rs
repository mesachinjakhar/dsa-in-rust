pub fn brute_force(arr: Vec<i32>, tar: i32) -> Option<(usize, usize)> {

    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] + arr[j] == tar {
                return Some((i, j ));
            }
        }

    }

    return None;

}