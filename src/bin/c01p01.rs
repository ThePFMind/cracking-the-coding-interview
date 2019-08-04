use std::collections::HashSet;

fn is_all_unique_chars_a(s: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    for c in s.chars() {
        if chars.contains(&c) {
            return false;
        }
        chars.insert(c);
    }
    true;
}

fn is_all_unique_chars_b(s: &str) -> bool {
    let mut bitfield: i64 = 0;
    let int_of_a: i16 = 'a' as i16;

    for c in s.chars() {
        let mut int_of: i16 = c as i16;

        int_of -= int_of_a;

        if (1 << int_of) & bitfield != 0 {
            return false;
        }

        bitfield |= 1 << int_of;
    }
     true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_all_unique_chars_a() {
        assert_eq!(is_all_unique_chars_a(&String::from("#44")), false);
        assert_eq!(is_all_unique_chars_a(&String::from("#117")), false);
        assert_eq!(is_all_unique_chars_a(&String::from("#123")), true);
    }
    #[test]
    fn test_is_all_unique_chars_b() {
        assert_eq!(is_all_unique_chars_b(&String::from("aa")), false);
        assert_eq!(is_all_unique_chars_b(&String::from("ab")), true);
    }
}

fn main() {
    println!("Chapter 1 | Arrays and Strings");
    is_all_unique_chars_a(&String::from("#123"));
    is_all_unique_chars_b(&String::from("ab"));
}
