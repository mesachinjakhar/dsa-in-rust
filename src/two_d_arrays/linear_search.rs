pub fn optimize_approach(arr: Vec<Vec<i32>>, rows: i32, col: i32, find: i32) -> Option<(i32, i32)>  {
    for i in 0..rows {
        for j in 0..col {
            if arr[i as usize][j as usize] == find {
                return Some((i,j));
            }
        }
    }

    return None;
}