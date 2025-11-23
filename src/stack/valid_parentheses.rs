pub fn valid_parentheses (s: String) -> bool {
    let mut stack: Vec<char> = Vec::new(); 

    for char in s.chars() {
        if char == '(' || char == '[' ||  char == '{' {
            stack.push(char)
        } else {
            // string started with ending parentheses directly
            if stack.is_empty() {
                return false; 
            }
            // check if the current char is == to last pushed char
            if let Some(top) = stack.last() {
                if (char == ')' && *top != '(') ||
                (char == ']' && *top != '[') || 
                (char == '}' && *top != '{') {
                    return false
                }
                stack.pop();
            }
        }
    }
    return stack.is_empty();
}