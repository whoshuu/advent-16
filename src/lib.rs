pub fn find_input(s: &String, i: &String) -> bool {
    let colon = i.find(":").unwrap();
    let m = s.find(&i[..colon]);

    if m.is_none() {
        return true;
    }

    match s.find(i) {
        Some(_) => true,
        None => false,
    }
}

pub fn find_greater_than(s: &String, i: &String) -> bool {
    let colon = i.find(":").unwrap();
    let m = s.find(&i[..colon]);

    if m.is_none() {
        return true;
    }

    let f = s.find(&(i[..colon].to_string() + ": 10"));

    if f.is_some() {
        return true;
    }

    let value = i[colon + 2..].parse::<u32>().unwrap();
    let comp = s[m.unwrap() + i.len() - 1..m.unwrap() + i.len()].parse::<u32>().unwrap();

    comp > value
}

pub fn find_less_than(s: &String, i: &String) -> bool {
    let colon = i.find(":").unwrap();
    let m = s.find(&i[..colon]);

    if m.is_none() {
        return true;
    }

    let f = s.find(&(i[..colon].to_string() + ": 10"));

    if f.is_some() {
        return false;
    }

    let value = i[colon + 2..].parse::<u32>().unwrap();
    let comp = s[m.unwrap() + i.len() - 1..m.unwrap() + i.len()].parse::<u32>().unwrap();

    comp < value
}

#[test]
fn find_input_test() {
    assert!(find_input(&"Sue 1: cars: 5, akitas: 3, goldfish: 0".to_string(), &"cars: 5".to_string()));
    assert!(find_input(&"Sue 321: akitas: 3, goldfish: 0, children: 4".to_string(), &"cars: 5".to_string()));
    assert!(!find_input(&"Sue 123: cars: 3, goldfish: 0, children: 4".to_string(), &"cars: 5".to_string()));
}

#[test]
fn find_greater_than_test() {
    assert!(find_greater_than(&"Sue 1: cars: 6, akitas: 3, goldfish: 0".to_string(), &"cars: 5".to_string()));
    assert!(find_greater_than(&"Sue 1: cars: 10, akitas: 3, goldfish: 0".to_string(), &"cars: 5".to_string()));
    assert!(find_greater_than(&"Sue 321: akitas: 3, goldfish: 0, children: 4".to_string(), &"cars: 5".to_string()));
    assert!(!find_greater_than(&"Sue 123: cars: 5, goldfish: 0, children: 4".to_string(), &"cars: 5".to_string()));
}

#[test]
fn find_less_than_test() {
    assert!(find_less_than(&"Sue 1: cars: 4, akitas: 3, goldfish: 0".to_string(), &"cars: 5".to_string()));
    assert!(!find_less_than(&"Sue 1: cars: 10, akitas: 3, goldfish: 0".to_string(), &"cars: 5".to_string()));
    assert!(find_less_than(&"Sue 321: akitas: 3, goldfish: 0, children: 4".to_string(), &"cars: 5".to_string()));
    assert!(!find_less_than(&"Sue 123: cars: 5, goldfish: 0, children: 4".to_string(), &"cars: 5".to_string()));
}
