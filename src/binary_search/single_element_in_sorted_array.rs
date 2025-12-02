pub fn brute_force(arr:Vec<i32>) -> i32 {
    
    for i in 0..arr.len() {
        if arr[i] != arr[i + 1] && arr[i] != arr[i - 1] {
            return arr[i];
        }
    }
    -1
}

pub fn optimize_approach(arr:Vec<i32>) -> i32 {

    let mut st = 0;
    let mut end = arr.len() - 1; 

    let mut n = arr.len();

    while st <=end {
        let mid = st + (end - st) / 2; 

        // edge cases
        if mid == 0 && arr[mid + 1] != arr[mid] {
            return arr[mid]
        }
        if mid == n - 1 && arr[mid - 1] != arr[mid] {
            return arr[mid];
        }


        // check if current mid is ans
        if arr[mid] != arr[mid - 1] && arr[mid] != arr[mid + 1] {
            return arr[mid];
        }

        if mid % 2 == 0 {
        // check which part to search now
        if arr[mid] == arr[mid - 1 ] { // search left part
            end = mid - 1;
        } else {
            st = mid + 1; 
        }
        } else {
            // check which part to search now
            if arr[mid] == arr[mid - 1] {
                st = mid + 1;
            } else {
                end = mid - 1;
            }
        }
    } 

    -1
}