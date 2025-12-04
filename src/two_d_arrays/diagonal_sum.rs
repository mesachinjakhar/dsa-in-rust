pub fn brute_force(arr: Vec<Vec<i32>>, rows: i32, col: i32) -> i32 {
    let mut max_sum = 0;
    for i in 0..rows {
        for j in i..i+1 {
            println!("current row is: {}, current col is: {}", i, j);
            max_sum = max_sum + arr[i as usize][j as usize];
        }
    }

    max_sum

}