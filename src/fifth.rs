// Given a sorted array of integers, implement a function that returns the median of the array.

fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        return 0.0; // Handle empty array case
    }
    let mid = len / 2;
    if len % 2 == 0 {
        // Even number of elements
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        // Odd number of elements
        arr[mid] as f64
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];

    println!("Median of arr1: {}", find_median(&arr1));
    println!("Median of arr2: {}", find_median(&arr2));
}
