pub fn optimize_approach(arr: Vec<i32>) -> Option<(i32, i32, i32)>{

    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            for k in j+1..arr.len() {
                if arr[i] + arr[j] + arr[k] == 0 {
                    return Some((arr[i],arr[j],arr[k]));
                }
             }
        }
    }

    return None;

}