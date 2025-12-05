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

pub fn better_approach(arr: &mut Vec<i32>, tar: i32) -> Option<(usize, usize)> {
    arr.sort();
    let mut  st = 0;
    let mut end = arr.len() - 1;

    while st < end {
        if arr[st] + arr[end] == tar {
            return Some((st,end));
        }
        else if arr[st] + arr[end] > tar {
            end = end - 1;
        } else {
            st = st + 1;
        }
    }

    return None;
}