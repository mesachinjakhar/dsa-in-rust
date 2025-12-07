pub fn optimize_approach(arr: &Vec<i32>, ans: &mut Vec<i32>, i : usize) { 
    if i == arr.len() {
        println!("{:?}", ans);
        return;
    }

    ans.push(arr[i]);
    optimize_approach(arr, ans, i + 1);

    ans.pop();
    optimize_approach(arr, ans, i + 1);

}  

