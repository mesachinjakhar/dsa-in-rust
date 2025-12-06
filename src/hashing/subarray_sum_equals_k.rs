use std::collections::HashMap;

pub fn brute_force(arr: Vec<i32>, tar: i32) -> i32 {

    let mut count = 0;

    for i in 0..arr.len() {
        let mut sum = 0;
        for j in i..arr.len() {
            sum += arr[j];
            if sum == tar {
                count += 1;
            }
        }
    }

    return count
}

pub fn optimize_approach(arr: Vec<i32>, k:i32) -> i32 {
    let n = arr.len();
    let mut count = 0 ;

    let mut prefix_sum = vec![0i32; n];

    prefix_sum[0] = arr[0];

    for i in 1..n {
        prefix_sum[i] = prefix_sum[i -1] + arr[i];
    }

    let mut map: HashMap<i32, i32> = HashMap::new();

    for j in 0..n {
        if prefix_sum[j] == k {
            count += 1;
        }

        let val = prefix_sum[j] - k; 

        if let Some(freq) = map.get(&val) {
            count = count + *freq;
        }

        map.entry(prefix_sum[j])
        .and_modify(|freq| *freq += 1)
        .or_insert(1);

    }

    return count;

}