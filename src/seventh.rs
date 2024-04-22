// Implement a function that returns the kth smallest element in a given array.


fn kth_smallest(nums: &[i32], k: usize) -> Option<i32> {
    if k > nums.len() {
        return None; 
    }

    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort();

    Some(sorted_nums[k - 1])
}

fn main() {
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    let k = 3;

    match kth_smallest(&nums, k) {
        Some(result) => println!("The {}th smallest element is: {}", k, result),
        None => println!("k is out of bounds"),
    }
}