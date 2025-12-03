pub fn optimize_approach(arr: &mut Vec<i32>) -> &Vec<i32> {

    for i in 1..arr.len() {
        let mut curr = arr[i];
        let mut prev = i - 1 ;

        while prev >= 0 && arr[prev] > curr {
            arr[prev + 1] = arr[prev];
            prev -= 1;
        }

        arr[prev + 1] = curr;
    }

    arr
}