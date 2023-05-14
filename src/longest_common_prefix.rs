fn longest_common_prefix(strs: &[&str]) -> String {

    let mut prefix = String::new();

    if strs.len() == 0 {
        return prefix;
    }

    let mut i = 0;
    let mut j = 0;

    for c in strs[0].chars() {
        for s in strs {

            if s.len() <= i {
                return prefix;
            }

            if s.chars().nth(i).unwrap() != c {
                return prefix;
            }


        }

    }







}





pub fn main() {
    let test_cases = vec![["flower", "flow", "flight"], vec!["dog", "racecar", "car"]];
    for strs in &test_cases {
        let result = longest_common_prefix(strs);
        println!("Longest common prefix: {:?}: {}", strs, result);

    }

}
