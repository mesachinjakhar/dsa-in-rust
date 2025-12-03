mod stack;
mod arrays;
mod binary_search;
mod sorting;
fn main() {

    // println!("result {}", stack::valid_parentheses::valid_parentheses("{{{}}".to_string()));
    // let arr = vec![100,80,60,70,60,75,85];
    // println!("result {:?}", stack::stock_span::stock_span(arr));

    // // linear search on arr/vec
    // let arr = vec![1,4,6,8,35,673,74];
    // println!("result: {:?}", arrays::linear_search::linear_search(arr, 8));

    // // smallest in array
    // let arr = vec![-15,4,6,8,35,673,74];
    // println!("result: {:?}", arrays::smallest_in_array::smallest_in_array(arr));

    // // largest in array 
    // let arr = vec![-15,4,6,8,35,673,74];
    // println!("result: {:?}", arrays::smallest_in_array::largets_in_array(arr));

    // // change array
    // let mut arr = vec![1,2,3,4,5];
    // arrays::smallest_in_array::change_array(&mut arr);
    // println!("{:?}", arr);

    // // reverse array
    // let mut arr = vec![1,2,3,4,5];
    // arrays::reverse_array::reverse_array(&mut arr);
    // println!("{:?}", arr);

    // // single number brute force
    // let arr = vec![4,1,2,1,2];
    // println!("result: {:?}", arrays::single_number::brute_force(arr));


    // // single number optimize approach
    // let arr = vec![4,1,2,1,2];
    // println!("result: {:?}", arrays::single_number::optimize_approach(arr));

    // // max subarray brute force
    // let arr = vec![3,-4,5,4,-1,7,-8];
    // println!("result: {:?}", arrays::maximum_subarray::brute_force(arr));

    // // pair sum brute force
    // let arr = vec![2,7,11,15];
    // println!("result: {:?}", arrays::pair_sum::brute_force(arr, 9));

    // // pair sum optimize approach
    // let arr = vec![2,7,11,15];
    // println!("result: {:?}", arrays::pair_sum::optimize_approach(arr, 9));

    // // majority element brute force
    // let arr = vec![1,2,1,1];
    // println!("result: {:?}", arrays::majority_element::brute_force(arr));

    // // majority element better approach
    // let mut arr = vec![1,2,1,1];
    // println!("result: {:?}", arrays::majority_element::better_approach(&mut arr));

    // // majority element optimize approach Moores algo
    // let mut arr = vec![1,2,1,1];
    // println!("result: {:?}", arrays::majority_element::optimize_approach(arr));

    // let mut arr = vec![7,1,5,3,6,4];
    // println!("result: {:?}", arrays::buy_sell_stock::optimize_approach(arr));
    //  let mut arr = vec![7,1,5,3,6,4];
    // println!("result: {:?}", arrays::buy_sell_stock::brute_force(arr));

    // let mut arr = vec![1,8,6,2,5,4,8,3,7];
    // println!("result: {:?}", arrays::container_with_most_water::brute_force(arr));
    //     let mut arr = vec![1,8,6,2,5,4,8,3,7];
    // println!("result: {:?}", arrays::container_with_most_water::optimize_approach(arr));

    // let mut arr = vec![1,2,3,4];
    // println!("result: {:?}", arrays::product_of_array_except_self::brute_force(arr));


    // let mut arr = vec![1,2,3,4];
    // println!("result: {:?}", arrays::product_of_array_except_self::optimize_approach(arr));



    // let mut arr = vec![1,2,3,4];
    // println!("result: {:?}", binary_search::binary_search::binary_search(arr, 3));

    //     let mut arr = vec![1,2,3,4];
    // println!("result: {:?}", binary_search::binary_search::recursion(&arr, 3, 0, 3));

    let mut arr = vec![1,2,3,4];
    println!("result: {:?}", binary_search::search_in_roated_sorted_array::optimize_approach(arr, 3));

        let mut arr = vec![1,2,5,4];
    println!("result: {:?}", binary_search::peak_index_in_montain_array::brute_force(arr));
            let mut arr = vec![1,2,5,4];
    println!("result: {:?}", binary_search::peak_index_in_montain_array::optimize_approach(arr));

    let mut arr = vec![2,2,4,4,5,6,6];
    println!("result: {:?}", binary_search::single_element_in_sorted_array::brute_force(arr));

        let mut arr = vec![2,2,4,4,5,6,6];
    println!("result: {:?}", binary_search::single_element_in_sorted_array::optimize_approach(arr));

    let mut arr = vec![2,1,3,4];
    println!("result: {:?}", binary_search::book_allocation::optimize_approach(arr, 2));

    let mut arr = vec![40,30,10,20];
    println!("result: {:?}", binary_search::painters_partition::optimize_approach(arr, 2));

    let mut arr = vec![1,2,8,4,9];
    println!("result: {:?}", binary_search::aggresive_cow::optimize_approach(&mut arr, 3));

    let mut arr = vec![1,2,8,4,9];
    println!("result: {:?}", sorting::bubble_sort::optimize_approach(&mut arr));






}
