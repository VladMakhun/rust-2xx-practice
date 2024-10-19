// Тест 22
#[test]
fn test22() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

// Тест 23
#[test]
fn test23() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[0..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

// Тест 24
#[test]
fn test_24() {
    let s = "你好，世界";
    let substring: String = s.chars().take(2).collect();
    assert_eq!(substring, "你好");
    println!("Success!");
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}

// Тест 26
#[test]
fn test26() {
    let _t0: (u8,i16) = (0, -1);
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

// Тест 27
#[test]
fn test27() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "am");

    println!("Success!");
}

// Тест 28
#[test]
fn test28() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple length: {}", std::mem::size_of_val(&too_long_tuple)); // Виправлено
}

// Тест 29
#[test]
fn test29() {
    let tup = (1, "hello", 4.5);
    let (x, y, z) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 4.5);
}

// Тест 30
#[test]
fn test30() {
    let mut counter = 0;

    loop {
        if counter == 10 {
            break;
        }
        counter += 1;
    }

    assert_eq!(counter, 10);
}

// Тест 31
#[test]
fn test31() {
    let mut sum = 0;
    let mut n = 1;

    while n <= 10 {
        sum += n;
        n += 1;
    }

    assert_eq!(sum, 55);
}

// Тест 32
#[test]
fn test32() {
    for i in 1..=10 {
        println!("{}", i);
    }

    assert!(true);
}

// Тест 33
#[test]
fn test33() {
    let arr = [1, 2, 3];
    for elem in arr.iter() {
        println!("{}", elem);
    }

    assert!(true);
}

// Тест 34
#[test]
fn test34() {
    let arr = [1, 2, 3];
    let sum: i32 = arr.iter().sum();

    assert_eq!(sum, 6);
}

// Тест 35
#[test]
fn test35() {
    let sum = (1..=10).sum::<i32>();
    assert_eq!(sum, 55);
}

// Тест 36
#[test]
fn test36() {
    let s = "hello, world";
    let length = s.len();

    assert_eq!(length, 12);
}

// Тест 37
#[test]
fn test37() {
    let s = "hello, world";
    let first_char = s.chars().next().unwrap();

    assert_eq!(first_char, 'h');
}

// Тест 38
#[test]
fn test38() {
    let s = "hello, world";
    let last_char = s.chars().last().unwrap();

    assert_eq!(last_char, 'd');
}

// Тест 39
#[test]
fn test39() {
    let s = "hello, world";
    let is_empty = s.is_empty();

    assert_eq!(is_empty, false);
}

// Тест 40
#[test]
fn test40() {
    let mut s = String::from("hello");
    s.push_str(", world");

    assert_eq!(s, "hello, world");
}

// Тест 41
#[test]
fn test41() {
    let s = String::from("hello");
    let len = s.len();

    assert_eq!(len, 5);
}

// Тест 42
#[test]
fn test42() {
    let s = String::from("hello");
    let char_at_index = s.chars().nth(1).unwrap();

    assert_eq!(char_at_index, 'e');
}

// Тест 43
#[test]
fn test43() {
    let s = String::from("hello");
    let slice = &s[0..2];

    assert_eq!(slice, "he");
}

// Тест 44
#[test]
fn test44() {
    let s = String::from("hello, world");
    let words: Vec<&str> = s.split_whitespace().collect();

    assert_eq!(words, vec!["hello,", "world"]);
}

// Тест 45
#[test]
fn test45() {
    let s = String::from("hello");
    let replaced = s.replace("e", "a");

    assert_eq!(replaced, "hallo");
}
