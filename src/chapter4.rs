/// https://practice.course.rs/basic-types/numbers.html
#[test]
fn test41() {
  
// first task
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    //y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");
}
}
// second task

// Fill the blank
fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

//third task 
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x)); 

    println!("Success!");
}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
// four task

fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

// five task

fn main() {
    let v1 = 251_u16 + 8; 
    let v2 = u8::checked_add(251, 8).unwrap_or(0); 

    println!("{}, {}", v1, v2);
}

// six task

fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597); 

    println!("Success!");
}

// seven task 

fn main() {
    let x = 1_000.000_1; 
    let y: f32 = 0.12; 
    let z = 0.01_f64; 

    assert_eq!(type_of(&x), "f64".to_string()); 
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

//nine task 

fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5); 

    println!("{}", 122 - 97); 

    for c in 'a'..='z' {
        println!("{}", c);
    }
}

// ten task

use std::ops::{Range, RangeInclusive};

fn main() {
    assert_eq!((1..5), Range { start: 1, end: 5 }); // Fill with 5
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); // Fill with =5

    println!("Success!");
}

