pub fn optimize_approach(arr: &mut Vec<i32>, idx: usize) {
    if idx == arr.len() {
        println!("{:?}", arr);
        return;
    }

    for i in idx..arr.len() {
        arr.swap(idx, i);
        optimize_approach(arr, idx + 1);
        arr.swap(idx, i);
    }

}