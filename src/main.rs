mod stack;

fn main() {
    println!("result {}", stack::valid_parentheses::valid_parentheses("{{{}}".to_string()));
}
