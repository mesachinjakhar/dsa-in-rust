pub fn optimize_approach(nums1: &mut Vec<i32>, nums2: Vec<i32>, m: i32, n: i32) {
    let mut i = m -1; 
        let mut j = n - 1;
        let mut idx = m + n - 1;

        while j as isize >= 0 && i as isize >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[idx as usize] = nums1[i as usize];
                idx -= 1;
                i -= 1;
            } else {
                nums1[idx as usize] = nums2[j as usize]; 
                idx -= 1;
                j -= 1;
            }
        }

        if j as isize >= 0 {
            while j as isize >= 0 {
                nums1[idx as usize] = nums2[j as usize];
                idx -= 1;
                j -= 1;
            }
        }
    }