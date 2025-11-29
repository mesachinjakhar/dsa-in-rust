pub fn reverse_array(arr: &mut Vec<i32>) {
    let mut  str = 0;
    let mut end = arr.len() - 1; 

    while str < end {
        arr.swap(str,end); // swap method takes index of array so we passed str, end , not the acutal value like arr[str], arr[end];
        str += 1;
        end -= 1;
    }
}