pub fn brute_force(s: String) -> String {
    let mut chars:Vec<char> = s.chars().collect();

    let mut st = 0;
    let mut end = chars.len() - 1;

    while st < end {
        chars.swap(st, end);
        st += 1;
        end -= 1;
    }

    let mut s1 = String::new();

    for c in 0..chars.len() {
        s1.push(chars[c]);
    }
    s1
}