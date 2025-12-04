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

pub fn brute_force_2(arr: Vec<Vec<i32>>, rows: i32, col: i32) -> i32 {
    let n = arr.len();
    let mut max_sum = 0;
    for i in 0..rows {
        for j in 0..col {
            if j == i {
                max_sum = max_sum + arr[i as usize][j as usize];
            }
            else if j == n as i32 -1-i {
                max_sum = max_sum + arr[i as usize][j as usize];
            }
        }
    }

    max_sum
}

pub fn optimize_approach(arr: Vec<Vec<i32>>, rows: i32, col: i32) -> i32 {
    let n = arr.len();
    let mut max_sum = 0;

    for i in 0..rows {
        max_sum = max_sum + arr[i as usize][i as usize];

        if i != n as i32 -1-i {
            max_sum = max_sum + arr[i as usize][n - 1 - i];
        }
    }

    max_sum

}