pub fn is_freq_same(freq1: [i32; 26], freq2: [i32; 26]) -> bool {
for i in 0..freq1.len() {
        if freq1[i] != freq2[i] {
            return false;
        }
    }

    return true;
}

pub fn optimize_approach(s1: String, s2: String) -> bool {
     let mut freq = [0i32; 26];

    let s1b = s1.as_bytes();
    let s2b = s2.as_bytes();

    for &b in s1b {
        freq[(b - b'a') as usize] +=1;
    }

    let window_size = s1.len();

    for i in 0..s2.len() {
        let mut window_idx = 0; 
        let mut idx = i;

        let mut window_freq = [0i32; 26];

        while window_idx < window_size && idx < s2.len() {
            window_freq[(s2b[idx] - b'a') as usize] += 1;
            window_idx += 1;
            idx += 1;
        }

        if is_freq_same(freq, window_freq) {
            return true;
        }

    }

    return false;
}