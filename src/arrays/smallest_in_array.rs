use std::i32;

pub fn smallest_in_array(arr: Vec<i32>) -> i32 {
    let mut smallest:i32 = i32::MAX;

    for i in 0..arr.len() {
        if arr[i] < smallest {
            smallest = arr[i];
        }
    }
    smallest
}

pub fn largets_in_array(arr: Vec<i32>) -> i32 {

    let mut largest: i32 = i32::MIN;

     for i in 0..arr.len() {
        if arr[i] > largest {
            largest = arr[i];
        }
    }
    largest

}