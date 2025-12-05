pub fn optimize_approach(arr: Vec<Vec<i32>>) {
    let mut srow = 0;
    let mut erow = arr.len() - 1;
    let mut scol = 0;
    let mut ecol = arr[0].len() - 1;

    while srow <= erow && scol <= ecol {
        // starting row
        for j in srow..ecol + 1 {
            println!("{}", arr[srow][j]); 
        }
        srow +=1;

        // ending column
        for j in srow..erow + 1 {
            println!("{}", arr[j][ecol]);
        }
        ecol -= 1;

        // ending row
        for j in (scol..ecol + 1).rev() {
            println!("{}", arr[erow][j])
        }

        erow -= 1; 

        // left col 
        for j in (srow..erow +1 ).rev() {
            println!("{}", arr[j][scol]);
        }
        scol += 1;
    }
}