fn main () {
    let numbers = [123, 121, 1221, 12321];
    for &number in &numbers {
        if is_palindrome(number) {
            println!("{} is a palindrome.", number);
        } else {
            println!("{} is not a palindrome.", number);
        }
    }
}


fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];
        data.iter().for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
    }
}
