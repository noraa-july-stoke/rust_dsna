fn longest_common_prefix(strs: &[&str]) -> String {


}





pub fn main() {
    let test_cases = vec![["flower", "flow", "flight"], vec!["dog", "racecar", "car"]];
    for strs in &test_cases {
        let result = longest_common_prefix(strs);
        println!("Longest common prefix: {:?}: {}", strs, result);

    }

}
