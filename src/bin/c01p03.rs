fn urltify_a(url: &str) -> String {
    url.trim().replace(" ", "%20")
}

fn urltify_b(url: &str) -> String {
    let placeholder = "%20";
    url.split_whitespace().fold(String::new(), |acc, s| {
        placeholder + s
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urltify_a() {
        assert_eq!(urltify_a("Mr John Smith   "), "Mr%20John%20Smith")
    }

    #[test]
    fn test_urltify_b() {
        assert_eq!(urltify_b("Mr John Smith   "), "Mr%20John%20Smith")
    }
}

fn main() {
    println!("Chapter 1 | Arrays and Strings");
}
