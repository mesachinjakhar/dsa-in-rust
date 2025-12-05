use std::collections::HashMap;

pub fn optimize_approach(arr: Vec<Vec<i32>>) -> i32 {
    let mut m: HashMap<i32, usize> = HashMap::new();
    let mut a = 0;

    let n = 3;
    let mut  total_sum = 0;

    for i in 0..arr.len() {
        for j in 0..n {
            if let Some(v) = m.get(&arr[i][j]) {
            a = arr[i][j];
        } else {
            m.insert(arr[i][j], j);
        }
        total_sum = total_sum + arr[i][j];
        }
    }

    let expected_sum = n * n * (n * n + 1)/2;
    let missing_value = expected_sum as i32 - (total_sum - a);

    return missing_value;

}