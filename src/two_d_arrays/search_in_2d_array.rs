pub fn optimize_approach(arr: Vec<Vec<i32>>, tar: i32) -> Option<(usize, usize)> {
    // handle empty input
    let rows = arr.len() as isize;
    if rows == 0 {
        return None;
    }
    let cols = arr[0].len() as isize;
    if cols == 0 {
        return None;
    }

    // binary search over rows using signed indices to avoid underflow
    let mut st: isize = 0;
    let mut end: isize = rows - 1;

    while st <= end {
        let mid = st + (end - st)/ 2;
        let mid_usize = mid as usize;

        let first = arr[mid_usize][0];
        let last = arr[mid_usize][(cols - 1) as usize];



        if tar >= first && tar <= last {
                        
            // binary search inside row
                // binary search inside this row
                let mut l: usize = 0;
                let mut r: usize = (cols - 1) as usize;


           while l <= r {
                let m = l + (r - l) / 2;
                let v = arr[mid_usize][m];
                if v == tar {
                    return Some((mid_usize, m as usize));
                } else if  v < tar {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
                
            }
        }
        else if arr[mid_usize][0] > tar {
            end = mid -1;
        } else {
            st = mid + 1;
        }
    }

    return None;

}