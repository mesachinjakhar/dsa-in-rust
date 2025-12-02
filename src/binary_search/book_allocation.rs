fn is_valid(arr: &Vec<i32>, mid: i32, m: i32) -> bool {
    let mut stu = 1;
    let mut pages = 0;

    for i in 0..arr.len() {
        if arr[i] > mid {
            return false;
        }
        if pages + arr[i] <= mid {
            pages = pages + arr[i];
        } else {
            stu = stu + 1;
            pages = arr[i];
        }
    }

    if stu <= m {
        return  true;
    } else {
        return false;
    }

}

pub fn optimize_approach(arr: Vec<i32>, m: i32) -> i32 {
    let mut sum = 0;

    for i in 0..arr.len() {
        sum = sum + arr[i]
    }

    let mut st = 0;
    let mut end = sum;
    let mut ans = 0;

    while st <= end {
        let mid = st + (end - st)/2;

        if is_valid(&arr, mid, m) {
            ans = mid;
            end = mid - 1;
        } else {
            st = mid + 1;
        }
    }

    ans

}