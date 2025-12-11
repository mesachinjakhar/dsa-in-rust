use dsa_in_rust::strings;

mod stack;
mod arrays;
mod binary_search;
mod sorting;
mod two_d_arrays;
mod hashing;
mod recursion;
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

    // let mut arr = vec![1,2,3,4];
    // println!("result: {:?}", binary_search::search_in_roated_sorted_array::optimize_approach(arr, 3));

    //     let mut arr = vec![1,2,5,4];
    // println!("result: {:?}", binary_search::peak_index_in_montain_array::brute_force(arr));
    //         let mut arr = vec![1,2,5,4];
    // println!("result: {:?}", binary_search::peak_index_in_montain_array::optimize_approach(arr));

    // let mut arr = vec![2,2,4,4,5,6,6];
    // println!("result: {:?}", binary_search::single_element_in_sorted_array::brute_force(arr));

    //     let mut arr = vec![2,2,4,4,5,6,6];
    // println!("result: {:?}", binary_search::single_element_in_sorted_array::optimize_approach(arr));

    // let mut arr = vec![2,1,3,4];
    // println!("result: {:?}", binary_search::book_allocation::optimize_approach(arr, 2));

    // let mut arr = vec![40,30,10,20];
    // println!("result: {:?}", binary_search::painters_partition::optimize_approach(arr, 2));

    // let mut arr = vec![1,2,8,4,9];
    // println!("result: {:?}", binary_search::aggresive_cow::optimize_approach(&mut arr, 3));

    // let mut arr = vec![1,2,8,4,9];
    // println!("result: {:?}", sorting::bubble_sort::optimize_approach(&mut arr));

    // let mut arr = vec![1,2,8,4,9];
    // println!("result: {:?}", sorting::selection_sort::optimize_approach(&mut arr));

    // let mut arr = vec![1,2,8,4,9];
    // println!("result: {:?}", sorting::insertion_sort::optimize_approach(&mut arr));

    // let mut arr = vec![1,0,0,2,2,0,1,1,2];
    // println!("result: {:?}", sorting::sort_0_1_and_2::optimize_approach(&mut arr));

    // let mut arr = vec![1,2,3,4,0,0,0];
    // let mut arr2 = vec![4,5,6];
    // println!("result: {:?}", sorting::merge_2_sorted_array::optimize_approach(&mut arr, arr2, 4,  3));
    // println!("{:?}", arr);

    // let mut arr = vec![4,5,6];
    // println!("result: {:?}", sorting::next_permutation::optimize_approach(&mut arr));
    // println!("{:?}", arr);

    // let s = "Sachin".to_string();
    // println!("{}", strings::reverse_string::brute_force(s));

    // let s = "TenE$t".to_string();
    // println!("{}", strings::is_palindrome::brute_force(s));

    // let s1 = "ab".to_string();
    // let s2 = "djasjbairkdt".to_string();

    // println!("{}", strings::permutation_in_string::optimize_approach(s1, s2));


    // let s2 = "hello world".to_string();
    // println!("{}", strings::reverse_words_in_string::optimize_approach(s2));

    // let mut arr = vec!['a','a','b','b','c','c','c'];
    // println!("{}", strings::compress_string::optimize_approach(&mut arr));

    // let mut tow_d_arr = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    // let mut rows = 3;
    // let mut column = 3;
    // println!("{:?}",two_d_arrays::linear_search::optimize_approach(tow_d_arr, rows, column, 8));

    // let mut tow_d_arr = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    // println!("{:?}",two_d_arrays::maximum_row_sum::brute_force(tow_d_arr, rows, column));

    // let mut tow_d_arr = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    // println!("{:?}",two_d_arrays::maximum_col_sum::brute_force(tow_d_arr, rows, column));

    // let mut tow_d_arr = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    // println!("{}",two_d_arrays::diagonal_sum::optimize_approach(tow_d_arr, rows, column));

    // let mut tow_d_arr = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    // println!("{:?}",two_d_arrays::search_in_2d_array::optimize_approach(tow_d_arr, 8));

    // let mut tow_d_arr = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    // println!("{:?}",two_d_arrays::spiral_matrix::optimize_approach(tow_d_arr));

    // let mut arr = vec![2,4,5,10];
    // println!("{:?}", hashing::two_sum::optimize_approach(&mut arr, 9));

    // let mut arr = vec![vec![1,2,3], vec![4,9,6], vec![7,8,9]];
    // println!("{:?}", hashing::find_missing_repeating_values::optimize_approach(arr));

    // let mut arr = vec![1,2,3,4,3,2];
    // println!("{}", hashing::find_duplicates::optimize_approach(arr));

    // let mut arr = vec![1,0,-1,2,-1,2];
    // println!("{:?}", hashing::three_sum::optimize_approach(&mut arr));

    // let mut arr = vec![-2,-1,-1,1,1,2,2];
    // println!("{:?}", hashing::four_sum::optimize_approach(&mut arr, 0));

    // let mut arr = vec![9,4,20,3,10,5];
    // println!("{:?}", hashing::subarray_sum_equals_k::optimize_approach(arr, 33));

    // recursion 

    // let n = 5;
    // recursion::print_to_n::optimize_approach(n);
    // println!("{}",recursion::factorial::optimize_approach(5));

    // println!("{}",recursion
    // ::sum_of_n::optimize_approach(5));

    // println!("{}",recursion
    // ::fibnacci::optimize_approach(4));

    // let arr = vec![1,2,3,20,5];
    // println!("{}", recursion::is_array_sorted::optimize_approach(&arr, 0));

    // let arr = vec![1,2,3,4,5];
    // println!("{}", recursion::binary_search::optimize_approach(&arr, 0, 4, 4));

    // let arr = vec![1,2,2];
    // let mut ans: Vec<i32> = Vec::new();
    // recursion::print_all_subsets::optimize_approach(&arr, &mut ans, 0);


    let arr = vec![1,2,2];
    let mut ans: Vec<i32> = Vec::new();
    recursion::print_all_subsets_2::optimize_approach(&arr, &mut ans, 0);

    let mut arr = vec![1,2,3];
    recursion::permutations::optimize_approach(&mut arr, 0);

}
