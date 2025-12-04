pub fn optimize_approach(mut s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut i = chars.len() as i32 - 1;

        let mut ans = String::new();

        while i >=0 {
            while i>=0 && chars[i as usize] == ' ' {
                i -= 1;
            }

            if i < 0 {
                break;
            }

            let mut word = String::new();

            while i >= 0 && chars[i as usize] != ' ' {
                word.push(chars[i as usize]);
                i -= 1;
            }

            let word: String = word.chars().rev().collect();

            if !ans.is_empty()  {
                ans.push(' ');
            }
            ans.push_str(&word);
        }

        ans
    
    }