use std::collections::VecDeque;
use std::cmp::max;

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut visited = vec![vec![false; m];n];
        let mut ans = 0; 
        let mut queue = VecDeque::new();

        // push all 2 values in queue
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 2 {
                    queue.push_back((i as i32, j as i32, 0));
                }
            }
        }

        // bfs 
        while let Some((row,col,time)) = queue.pop_front() {
            if !visited[row as usize][col as usize] {
                ans = max(ans, time);
                visited[row as usize][col as usize] = true;
                // make child calls
                if row + 1 < n as i32 && !visited[row as usize+1][col as usize] && grid[row as usize+1][col as usize] != 0 {
                    queue.push_back((row+1, col, time+1));
                }
                if row-1 >=0 && !visited[row as usize -1][col as usize] && grid[row as usize -1][col as usize] != 0 {
                    queue.push_back((row -1, col, time +1));
                }
                if col +1 < m as i32 && !visited[row as usize][col as usize+1] && grid[row as usize][col as usize+1] != 0 {
                    queue.push_back((row, col+1, time +1));
                }
                if col - 1 >=0 && !visited[row as usize][col as usize - 1] && grid[row as usize][col as usize - 1] != 0 {
                    queue.push_back((row, col -1, time +1));
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 && visited[i][j] != true {
                    ans = -1; 
                }
            }
        }


        return ans; 

}