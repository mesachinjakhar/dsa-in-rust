pub fn optimize_approach(n: i32) {
    if n == 1 {
        println!("{}", n);
        return;
    }

    println!("{}", n);

    optimize_approach(n - 1);

    println!(" now backtracking with n: {}", n);

}