use std::collections::HashMap;
use std::collections::HashSet;

fn p01_is_all_unique_chars(s: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    for c in s.chars() {
        if chars.contains(&c) {
            return false;
        }
        chars.insert(c);
    }
    return true;
}

fn P02_is_permutation(n1: &str, n2: &str) -> bool {
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_all_unique_chars() {
        assert_eq!(p01_is_all_unique_chars(&String::from("#44")), false);
        assert_eq!(p01_is_all_unique_chars(&String::from("#117")), false);
        assert_eq!(p01_is_all_unique_chars(&String::from("#123")), true);
    }

    #[test]
    fn test_not_all_unique_chars() {
        assert_eq!(
            P02_is_permutation(&String::from(""), &String::from("")),
            false
        );
    }

}

fn main() {
    println!("Chapter 1 | Arrays and Strings");
}
