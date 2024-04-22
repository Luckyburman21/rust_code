// Implement a function that finds the longest common prefix of a given set of strings.


fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let first_string = &strings[0];
    let mut longest_prefix = String::new();
    
    'outer: for (i, ch) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(c) = string.chars().nth(i) {
                if c != ch {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        longest_prefix.push(ch);
    }
    
    longest_prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    let longest_prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", longest_prefix);
}
