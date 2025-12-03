pub fn optimize_approach(arr: &mut Vec<i32>) -> &Vec<i32> {
    for i in 0..arr.len() - 1 {
        let mut smallest_index = i;
        for j in i+1..arr.len() {
            if arr[j] < arr[smallest_index] {
                smallest_index = j;
            }
        }
        arr.swap(i, smallest_index);
    }

    arr

}