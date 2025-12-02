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