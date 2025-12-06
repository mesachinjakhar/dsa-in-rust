pub fn optimize_approach(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } if n == 1 {
        return 1;
    }
    return optimize_approach(n-1) + optimize_approach(n - 2);
}