fn is_one_edit_way(s1: &str, s2: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to() {
        assert_eq!(is_one_edit_way("pale", "ple"), true)
    }
}
fn main() {
    println!("chapter 1 | Arrays and Strings");
}
