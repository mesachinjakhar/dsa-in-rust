use std::vec;

pub fn optimize_approach(arr: &mut Vec<i32>, tar: i32) -> Vec<Vec<i32>> {
    arr.sort();
    let mut ans: Vec<Vec<i32>> = Vec::new();

    let n = arr.len(); 

    for i in 0..n - 3 {
        if i > 0 && arr[i] == arr[i - 1] {
            continue;
        }
        for j in i+1..n - 2 {

            if j > 1 && arr[j] == arr[j - 1] {
                continue;
            }

            let mut p = j+ 1;
            let mut q = n - 1;

            while p < q {
                let sum = arr[i] + arr[j] + arr[p] + arr[q];


                if sum < tar {
                    p += 1;
                } else if sum > tar {
                    q -= 1;
                } else {
                    ans.push(vec![arr[i], arr[j], arr[p], arr[q]]);
                    p += 1;
                    q -= 1;

                    while p < q && arr[p] == arr[p - 1] {
                        p += 1;
                    }
                }
            }

        }
    }


    ans
}