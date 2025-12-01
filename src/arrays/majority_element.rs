pub fn brute_force(arr: Vec<i32>) -> i32 {
    for i in 0..arr.len() {
        let mut count = 0;
        for j in 0..arr.len() {
            if arr[j] == arr[i] {
                count = count + 1;
            }
        }
        if count > arr.len() / 2 {
            return arr[i];
        }
    }
    0
}

pub fn better_approach(arr: &mut Vec<i32>) -> i32 {
    let n = arr.len();

    arr.sort();
    let mut freq= 1;
    let mut ans = arr[0];

    for i in 1..n {
        if arr[i] == arr[i - 1] {
            freq += 1;
        } else {
            ans = arr[i];
            freq = 1;
        }

        if freq > n/2 {
            return ans
        }
    }
    ans
}

pub fn optimize_approach(arr: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut freq = 0; 

    for i in 0..arr.len() {
        if freq == 0 {
            ans = arr[i];
        }
        if arr[i] == ans {
            freq += 1;
        } else {
            freq -=1;
        }
    }

    let mut count = 0 ;
    for i in 0..arr.len() {
        if arr[i] == ans {
            count += 1;
        }
    }

    if count > arr.len()/2 {
        return ans
    } else {
        -1
    }

}