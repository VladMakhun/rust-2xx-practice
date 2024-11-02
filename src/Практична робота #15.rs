fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }
    if n == 1 {
        return vec!["0".to_string(), "1".to_string()];
    }

    let prev_gray = gray(n - 1);
    let mut result = Vec::new();

    for code in &prev_gray {
        result.push(format!("0{}", code));
    }
    for code in prev_gray.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray() {
        let test_data = [
            (0, vec![""]),
            (1, vec!["0", "1"]),
            (2, vec!["00", "01", "11", "10"]),
            (3, vec!["000", "001", "011", "010",
                     "110", "111", "101", "100"]),
            (4, vec!["0000", "0001", "0011", "0010",
                     "0110", "0111", "0101", "0100",
                     "1100", "1101", "1111", "1110",
                     "1010", "1011", "1001", "1000"]),
        ];

        for (n, expected) in test_data.iter() {
            assert_eq!(gray(*n), *expected);
        }
    }
}

fn main() {
    println!("{:?}", gray(3));
}
