pub fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, n: i32, m: i32, row: i32, column: i32) {
        if row >= n || row < 0 || column >= m || column < 0 {
            return; 
        }

        if grid[row as usize][column as usize] == '0' || visited[row as usize][column as usize] == true {
            return; 
        }

        visited[row as usize][column as usize] = true; 
        dfs(grid, visited, n, m, row - 1, column);
        dfs(grid, visited, n, m, row + 1, column); 
        dfs(grid, visited, n, m, row, column -1); 
        dfs(grid, visited, n, m, row, column +1);

    }
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len() ; // rows
        let m = grid[0].len(); // column 

        let mut visited = vec![vec![false; m]; n];
        let mut count = 0; 

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' && visited[i][j] != true {
                    dfs(&grid, &mut visited, n as i32 , m as i32, i as i32, j as i32 );
                    count += 1;
                }  
            }
        }

        return count;
        
    }