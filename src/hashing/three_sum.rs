pub fn brute_force(arr: Vec<i32>) -> Option<(i32, i32, i32)>{

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


pub fn optimize_approach(arr: &mut Vec<i32>) -> Vec<(i32, i32, i32)> {

    arr.sort();
    let mut ans: Vec<(i32, i32, i32)> = Vec::new();

    for i in 0..arr.len() {

        if i > 0 {
            if arr[i] == arr[i - 1] {
                continue;
            }
        }


        let mut j = i + 1;
        let mut k = arr.len() - 1;

        while j < k {
            let sum = arr[i] + arr[j] + arr[k];

            if sum < 0 {
                j += 1;

            } else if sum > 0 {
                k -= 1;
            } else {
                ans.push((arr[i], arr[j], arr[k]));

                j += 1;
                k -= 1;

                while j < k && arr[j] == arr [j - 1]{
                    j += 1; 
                }
            }
        }

        }

    
    return ans;


}



