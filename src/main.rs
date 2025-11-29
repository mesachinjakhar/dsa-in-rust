mod stack;
mod arrays;

fn main() {

    println!("result {}", stack::valid_parentheses::valid_parentheses("{{{}}".to_string()));
    let arr = vec![100,80,60,70,60,75,85];
    println!("result {:?}", stack::stock_span::stock_span(arr));

    // linear search on arr/vec
    let arr = vec![1,4,6,8,35,673,74];
    println!("result: {:?}", arrays::linear_search::linear_search(arr, 8));

    // smallest in array
    let arr = vec![-15,4,6,8,35,673,74];
    println!("result: {:?}", arrays::smallest_in_array::smallest_in_array(arr));

    // largest in array 
    let arr = vec![-15,4,6,8,35,673,74];
    println!("result: {:?}", arrays::smallest_in_array::largets_in_array(arr));

    // change array
    let mut arr = vec![1,2,3,4,5];
    arrays::smallest_in_array::change_array(&mut arr);
    println!("{:?}", arr);

    // reverse array
    let mut arr = vec![1,2,3,4,5];
    arrays::reverse_array::reverse_array(&mut arr);
    println!("{:?}", arr);

    // single number brute force
    let arr = vec![4,1,2,1,2];
    println!("result: {:?}", arrays::single_number::brute_force(arr));


    // single number optimize approach
    let arr = vec![4,1,2,1,2];
    println!("result: {:?}", arrays::single_number::optimize_approach(arr));


}
