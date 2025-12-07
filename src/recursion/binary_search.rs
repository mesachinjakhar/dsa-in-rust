pub fn optimize_approach(arr: &Vec<i32>, st: i32, end: i32, tar: i32) -> i32 {
    if st > end {
        return -1 ;
    }
    
    let mid = st + (end - st)/ 2; 

    if arr[mid as usize] == tar {
        return mid;
    }

    if arr[mid as usize] < tar {
        return optimize_approach(arr, mid + 1, end, tar)
    } 
    else {
        return optimize_approach(arr, st, mid - 1, tar)
    }

}
