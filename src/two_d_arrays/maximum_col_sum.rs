pub fn brute_force(arr: Vec<Vec<i32>>, rows: i32, col: i32) -> i32 {
    let mut max_sum = 0;
    for i in 0..rows {
        let mut total_sum = 0;
        for j in 0..col {
            total_sum = total_sum + arr[j as usize][i as usize];
        }
        if total_sum > max_sum {
            max_sum = total_sum;
        }
    }

    max_sum
}