pub fn optimize_approach(arr: Vec<i32>) -> i32 {
    let mut slow = 0 as i32; 
    let mut faster = 0 as i32;

    loop {
        slow = arr[slow as usize];
        faster = arr[arr[faster as usize] as usize];

        if slow == faster {
            break;
        }
    }

    slow = 0;

    while slow != faster {
        slow = arr[slow as usize];
        faster = arr[faster as usize];
    }

    return slow as i32

}