fn is_palindrome(s: &str) -> bool {

    // 1. Convert to lowercase
    let mut s = s.to_lowercase();
    // 2. Remove non-alphanumeric characters
    s.retain(|c| c.is_alphanumeric());

    // 3. Compare the string with its reverse
    s == s.chars().rev().collect::<String>()

}


pub fn main() {
    let test_cases = ["racecar", "hello", "Madam", "A man, a plan, a canal, Panama"];

    for &test_case in & test_cases {
        let result = is_palindrome(test_case);
        println!("{}: {}", test_case, result)
    }
}
