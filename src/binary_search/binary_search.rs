pub fn binary_search(arr: Vec<i32>, tar: i32) -> Option<i32> {
    let mut start = 0;
    let mut end = arr.len() - 1;
    while start <= end {
        let mid = start + (end - start) / 2; 
        if arr[mid] < tar {
            start = mid + 1;
        }
        if arr[mid] > tar {
            end = mid - 1;
        }
        if arr[mid] == tar {
            return Some(mid as i32 );
        }
    }

    return None
}