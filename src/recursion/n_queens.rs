pub fn is_safe(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let n = board.len();

    // horizontal 
    for i in 0..n {
        if i != col {
            if board[row][i] == 'q' {
            return false;
        }
        }
    }
    // vertical 
    for i in 0..n{ 
        if i!= col {
            if board[i][col] == 'q' {
                return false;
            }

        }
    }

    // left diagonal
    let mut r = row as isize - 1;
    let mut c = col as isize - 1; 

    while r >=0 && c >=0 {
        if board[r as usize][c as usize] == 'q' {
            return false;
        }
        r -= 1;
        c -= 1;
    }

    let mut r = row as isize - 1;
    let mut c = col as isize + 1;

    while r >= 0 && c < n as isize {
        if board[r as usize][c as usize] == 'q' {
            return false;
        }
        r -= 1;
        c += 1;
    }

    return true;

}


pub fn optimize_approach(board: &mut Vec<Vec<char>>, ans: &mut Vec<Vec<Vec<char>>>, row: usize) {
    if row == board.len() {
        ans.push(board.to_vec());
        return;
    }

    for col in 0..board.len() {
        if is_safe(board, row, col) {
            board[row][col] = 'q';
            optimize_approach(board, ans, row+1);
            board[row][col] = '.';
        }
    }
}