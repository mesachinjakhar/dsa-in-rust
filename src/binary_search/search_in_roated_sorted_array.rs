pub fn optimize_approach(arr: Vec<i32>, tar: i32) -> i32 {
    let mut st: usize = 0;
    let mut end = arr.len() - 1; 


    while st <= end {
        let mid = st + (end - st)/2;

        if arr[mid] as i32 == tar  {
            return mid as i32;
        }

        // check which part of arrary is sorted
        if arr[st] > arr[mid]{ // right half is sorted
            // check if target can be in right half?
            if arr[mid] <= tar && arr[end] >= tar {
                st = mid + 1;
            } else {
                end = mid -1; 
            }
        } else {
            // check if target can be in left half
            if arr[st] <= tar && arr[mid] >= tar {
                end = mid -1;
            } else {
                st = mid + 1;
            }
        }

    }

    -1

}