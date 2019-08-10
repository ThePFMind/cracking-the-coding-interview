use std::collections::HashMap;

fn is_palindrome_permuation(s: &str) -> bool {
    let mut char_map: HashMap<char, i32> = HashMap::new();

    for c in s.to_lowercase().chars() {
        if c != ' ' {
            let x = char_map.entry(c).or_insert(0);
            *x += 1;
        }
    }

    let mut num_of_single = 0;

    for v in char_map.values() {
        if v % 2 != 0 {
            num_of_single += 1;
        }
    }

    if num_of_single > 1 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_permutation() {
        assert_eq!(is_palindrome_permuation("MADAM"), true);
        assert_eq!(is_palindrome_permuation("Taco Coa"), true);
    }

}

fn main() {
    println!("Chapter 1 | Arrays and Strings");
    is_palindrome_permuation("MADAM");
}
