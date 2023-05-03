pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

#[test]
fn test_rev() {
    println!("{}", reverse("abb"));
}
