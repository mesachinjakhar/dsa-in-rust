pub fn optimize_approach(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let mut idx: usize = 0;
        let mut i: usize = 0;

        while i < n  {
            let ch = chars[i];
            let mut count = 0;

            while i < n && chars[i] == ch {
                count = count + 1;
                i += 1;
            }

            chars[idx] = ch;
            idx += 1;

            if count > 1 {
                let count_str = count.to_string();
                for dig in count_str.chars() {
                    chars[idx] = dig;
                    idx += 1;
                }
            }

        }

        idx as i32
    }