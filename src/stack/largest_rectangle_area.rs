pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut right: Vec<i32> = Vec::new();
        let mut left: Vec<i32> = Vec::new();
        let mut stack: Vec<usize> = Vec::new();

        let mut max_area = 0;

        // right side: next smaller element to the right
        for i in (0..n).rev() {
            while !stack.is_empty() && heights[*stack.last().unwrap()] >= heights[i] {
                stack.pop();
            }

            if stack.is_empty() {
                right.push(n as i32);
            } else {
                right.push(*stack.last().unwrap() as i32);
            }

            stack.push(i);
        }

        right.reverse(); // because we pushed in reverse order

        stack.clear();

        // left side: next smaller element to the left
        for i in 0..n {
            while !stack.is_empty() && heights[*stack.last().unwrap()] >= heights[i] {
                stack.pop();
            }

            if stack.is_empty() {
                left.push(-1);
            } else {
                left.push(*stack.last().unwrap() as i32);
            }

            stack.push(i);
        }

        // compute areas
        for i in 0..n {
            let width: i32 = right[i] - left[i] - 1;
            let area = width * heights[i];
            if area > max_area {
                max_area = area;
            }
        }

        max_area
    }