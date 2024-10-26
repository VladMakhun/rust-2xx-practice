fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let n = ((n % len) + len) % len; // Це забезпечує правильну обробку від'ємних значен
    let split_index = len - n;

    let (left, right) = s.split_at(split_index as usize);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];
        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.clone(), *n), exp.to_string())
        });
    }
}

fn main() {
    let s = "abcdefgh".to_string();
    let rotated = rotate(s, 2);
    println!("Rotated: {}", rotated);
}
