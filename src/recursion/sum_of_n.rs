pub fn optimize_approach(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    return n + optimize_approach(n - 1);
}