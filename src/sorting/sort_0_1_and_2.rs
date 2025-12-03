pub fn optimize_approach(arr: &mut Vec<i32>) -> &Vec<i32> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut mid  = 0;


    while mid <= high {
        if arr[mid] == 0 {
            arr.swap(mid, low);
            mid = mid + 1;
            low = low + 1;
        } 
        else if arr[mid] == 1 {
            mid = mid + 1;
            
        }
        else {
            arr.swap(mid, high);
            high = high - 1;
        }

    }

    arr
}