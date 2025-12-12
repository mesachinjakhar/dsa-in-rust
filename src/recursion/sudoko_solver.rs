pub fn optimize_approach(arr: &mut Vec<Vec<char>>, row: usize, col: usize) {
    let n = arr.len();

    // base case
    if row == n && col == n {
        return;
    }

    // check row 
    let mut nextr_row = row;
    let mut next_col = col + 1;

    if next_col == n {
        nextr_row = nextr_row + 1;
        next_col = 0;
    }

    if arr[row][col] != '.' {
        optimize_approach(arr, row, col + 1);
    }

    for i in 0..n {
        if is_safe(arr, nextr_row, next_col, num) {
            arr[row][col] == i;
            optimize_approach(arr, nextr_row, next_col);
            arr[row][col] = '.';
        }
    }
}