// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            // Check if it's the first occurrence
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid);
            } else {
                // Continue searching on the left side for the first occurrence
                right = mid;
            }
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 3, 3, 4, 5, 6, 7];
    let target = 3;

    match first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
