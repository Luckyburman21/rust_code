// Implement a function that checks whether a given string is a palindrome or not.

fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut left = 0;
    let mut right = bytes.len() - 1;

    while left < right {
        if bytes[left] != bytes[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

fn main() {
    let test_case = "racecar";

    println!("{}", is_palindrome(test_case));
}
