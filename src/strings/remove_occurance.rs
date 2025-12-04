 pub fn remove_occurrences(mut s: String, part: String) -> String {

        if part.is_empty() {
            return s;
        }

        while let Some(starting_idx) = s.find(&part) {
            let end = starting_idx + part.len();
            s.replace_range(starting_idx..end, "");
        }

        return s;
    }