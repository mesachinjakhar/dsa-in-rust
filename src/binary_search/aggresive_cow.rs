fn is_valid(arr: &Vec<i32>, c: i32, mid: i32) -> bool {
    let mut curr_cow = 1;
    let mut curr_stall = arr[0];

    for i in 0..arr.len() {
        if arr[i] - curr_stall >= mid {
            curr_cow = curr_cow + 1;
            curr_stall = arr[i];
        }

        if curr_cow == c {
            return true
        }
    }

    return false;

}


pub fn optimize_approach(arr: &mut Vec<i32>, c: i32) -> i32 {

    arr.sort();
    let n = arr.len();

    let mut st = arr[0];
    let mut end = arr[n - 1] - st;

    let mut ans = 0;

    while st <=end {
        let mid = st + (end - st) / 2; 

        if is_valid(&arr, c, mid) {
            ans = mid;
            st = mid + 1;
        } else {
            end = mid -1;
        }

    }

    ans 
}