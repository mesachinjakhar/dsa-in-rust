pub fn optimize_approach(arr: &mut Vec<i32>) {
    let mut pivot = 0 as usize;

    for i in (0..arr.len()).rev() {
        if i == 0 {
            break;
        }
        if arr[i - 1] < arr[i] {
            pivot = i - 1;
            break;
        }
    }

    if pivot == 0 {
         arr.reverse();
         return
    }

    // swap with just bigger number
    for i in (pivot+1..arr.len()).rev() {
        if arr[i as usize] > arr[pivot as usize] {
            arr.swap(i as usize, pivot as usize);
            break;
        }
    }

    // reverse 
    let i = pivot + 1;
    let j = arr.len() - 1;

    for e in (i as usize..j).rev() {
        arr.swap(e as usize, j as usize);
    }


}