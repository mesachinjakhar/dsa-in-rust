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

    // max subarray brute force
    let arr = vec![3,-4,5,4,-1,7,-8];
    println!("result: {:?}", arrays::maximum_subarray::brute_force(arr));

    // pair sum brute force
    let arr = vec![2,7,11,15];
    println!("result: {:?}", arrays::pair_sum::brute_force(arr, 9));

    // pair sum optimize approach
    let arr = vec![2,7,11,15];
    println!("result: {:?}", arrays::pair_sum::optimize_approach(arr, 9));

    // majority element brute force
    let arr = vec![1,2,1,1];
    println!("result: {:?}", arrays::majority_element::brute_force(arr));

    // majority element better approach
    let mut arr = vec![1,2,1,1];
    println!("result: {:?}", arrays::majority_element::better_approach(&mut arr));


}
