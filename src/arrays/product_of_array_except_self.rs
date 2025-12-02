pub fn brute_force(arr: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();

    for i in 0..arr.len() {
        let mut product = 1;
        for j in 0..arr.len() {
            if i != j {
                product = product * arr[j];
            }
        }
        ans.push(product);
    }

    ans
}

pub fn optimize_approach(arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    let mut ans = vec![1; n];

    // prefix
    for i in 1..n {
        ans[i] = ans[i - 1] * arr[i -1];
    }

    // suffix 
    let mut suffix = 1;
    for i in (0..n).rev() {
        ans[i] = ans[i] * suffix;
        suffix = suffix * arr[i];
    }

    ans

}