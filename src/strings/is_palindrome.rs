pub fn brute_force(s: String) -> bool {
    let s1 = s.to_lowercase();
    let mut chars: Vec<char> = s1.chars().collect();

    let mut st = 0;
    let mut end = chars.len() - 1; 

    while st < end {
        if chars[st as usize] != chars[end as usize] {
            return false;
        }
        st +=1;
        end -= 1;
    }

    return true;
}