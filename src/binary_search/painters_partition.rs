fn is_valid(arr: &Vec<i32>, mid: i32, m: i32) -> bool {
    let mut worker = 1;
    let mut total = 0;
    for i in 0..arr.len() {
        if arr[i] + total <= mid {
            total = total + arr[i];
        } else {
            worker += 1;
            total = arr[i];
        }
    }

    if worker <= m {
        return true;
    } else {
        return false;
    }
}


pub fn optimize_approach(arr: Vec<i32>, m: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut max = 0;

    // cal sum and max
    for i in 0..arr.len() {
        sum = sum + arr[i];
        if arr[i] > max {
            max = arr[i]
        }
    }

    let mut st = max;
    let mut end = sum;
    let mut ans= 0;

    while st <= end {
        let mut mid = st + (end - st) / 2;

        if is_valid(&arr, mid, m ) {
            ans = mid;
            end = mid - 1;
        } else {
            st = mid + 1;
        }

    }

    ans


}