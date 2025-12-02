pub fn brute_force(arr: Vec<i32>) -> i32 {
    for i in 1..arr.len() {
        if arr[i - 1 ] < arr[i] && arr[i + 1] < arr[i] {
            return i as i32;
        }
    }

    -1
}

pub fn optimize_approach(arr: Vec<i32>) -> i32 {

    let mut st = 1 as usize ;
    let mut end = arr.len() - 2;

    while st <= end {
        let mid = st + (end - st)/ 2;

        // check if we find peak index;
        if arr[mid] > arr[mid - 1] && arr[mid] > arr[mid + 1] {
            return mid as i32;
        }
        // which part to search next; 
        if arr[mid] > arr[mid - 1] && arr[mid] < arr[mid + 1] { // we are at left part
            st = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    -1
}