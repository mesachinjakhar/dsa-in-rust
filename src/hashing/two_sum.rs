use std::collections::HashMap;

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

pub fn optimize_approach(arr: &mut Vec<i32>, tar: i32) -> Option<(usize, usize)> {
    let mut m: HashMap<i32, usize> = HashMap::new();

    for i in 0..arr.len() {
        let first = arr[i];
        let second = tar - first;
        if let Some(v) = m.get(&second) {
            return Some((i, *v));
        }
        m.insert(arr[i], i);
    }

    return None;
}