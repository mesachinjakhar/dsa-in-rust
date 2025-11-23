use std::collections::HashMap;


pub fn greater_element(list1: Vec<i32> ,list2: Vec<i32> ) -> Vec<i32> {
    let n = list2.len(); 
    let mut stack: Vec<i32> = Vec::with_capacity(n);
    let mut ans: Vec<i32> = Vec::with_capacity(n);
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in (0..n).rev() {
        while !stack.is_empty() && list2[i] > *stack.last().unwrap() {
            stack.pop();
        }

        if stack.is_empty() {
            map.insert(list2[i], -1);
            
        } else {
            map.insert(list2[i], *stack.last().unwrap());
        }

        stack.push(list2[i]);
    }

    for i in 0..list1.len() {
        if let Some(v) = map.get(&list1[i]) {
            ans.push(*v);
        }
    }
    ans
}