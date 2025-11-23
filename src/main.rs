mod stack;

fn main() {

    println!("result {}", stack::valid_parentheses::valid_parentheses("{{{}}".to_string()));
    let arr = vec![100,80,60,70,60,75,85];
    println!("result {:?}", stack::stock_span::stock_span(arr))
}
