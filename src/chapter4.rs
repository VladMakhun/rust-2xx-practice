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

///Char, Bool and Unit

// first task 

fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4); 

    println!("Success!");
}

// second task 

fn main() {
    let c1 = "中";
    
    print_char(c1.chars().next().unwrap()); 
}

fn print_char(c: char) {
    println!("{}", c);
}


// third task 

fn main() {
    let _f: bool = false;

    let t = true;
    if !t {
        println!("Success!");
    } else { 
        println!("Condition was false, so we reach here!");
    }
} 

// four task 

fn main() {
    let f = false; 
    let t = true && false; 
    assert_eq!(t, f); 

    println!("Success!");
}

//five task 

fn main() {
    let _v: () = (); 

    let v = implicitly_ret_unit();
    assert_eq!(v, ()); 
    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// six task 

use std::mem::size_of_val;

fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0); 

    println!("Success!");
}


//4.3 Statements and Expressions

// first task 

fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x 
    };

    assert_eq!(v, 3);

    println!("Success!");
}


// second task

fn main() {
    let v = {
        let x = 3; 
        x 
    };

    assert!(v == 3);

    println!("Success!");
}

// third task 

fn main() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y 
}

/// 4.4 Functions

// first task

fn main() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 { 
    x + y 
}

// second task 

fn main() {
    print();
}

// Change return type to unit type ()
fn print() -> () {
    println!("Success!");
}

// third task 

fn main() {
    never_return();

    println!("Failed!"); // This line will never be reached
}

fn never_return() -> ! {
    loop {} // Infinite loop
}


// five task 

fn main() {
    let b = true; 

    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
