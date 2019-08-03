fn count_of(s: &str) -> i32 {
    let mut num_of: i32 = 0;
    for c in s.chars() {
        let num_of_c: i32 = c as i32;
        num_of += num_of_c;
    }
    num_of
}

fn is_permutation(s1: &str, s2: &str) -> bool {
    if count_of(s1) != count_of(s2) {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert_eq!(is_permutation("12345", "54321"), true);
        assert_eq!(is_permutation("12345", "55321"), false);
    }
}

fn main() {
    println!("Chapter 1 | Arrays and Strings");
}
