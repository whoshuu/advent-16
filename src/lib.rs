pub fn find_input(s: &String, i: &String) -> bool {
    match s.find(i) {
        Some(_) => true,
        None => false,
    }
}

#[test]
fn find_input_test() {
    assert!(find_input(&"Sue 1: cars: 5, akitas: 3, goldfish: 0".to_string(), &"cars: 5".to_string()));
    assert!(!find_input(&"Sue 321: akitas: 3, goldfish: 0, children: 4".to_string(), &"cars: 5".to_string()));
}
