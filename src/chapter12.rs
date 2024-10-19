//12.1 as


//1. 
fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8; // Fill in the blank with `u8`

    let c1: char = decimal as char;
    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8);

    println!("Success!");
}

//2. fn main() {
    assert_eq!(u8::MAX, 255);

    let v = 255; 
    println!("Success!");
}

//3. 🌟🌟 When casting any value to an unsigned type T, T::MAX + 1 is added or subtracted until the value fits into the new type.

fn main() {
    assert_eq!(1000 as u16, 1000); 

    assert_eq!(1000 as u8, 244); 

    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255); 

    
    assert_eq!(300.1_f32 as u8, 255); 
    assert_eq!(-100.1_f32 as u8, 0); 
    

    
    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

//4. 🌟🌟🌟 Raw pointers can be converted to memory address (integer) and vice versa.

fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr(); 
    let first_address: usize = p1 as usize; 
    let second_address = first_address + 4; 
    let p2: *mut i32 = second_address as *mut i32; 
    unsafe {
        *p2 += 1; 
    }
    
    assert_eq!(values[1], 3); 

    println!("Success!"); 
}


//5. 
fn main() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 8 * 13); 
    }

    println!("Success!");
}

//12.2 From Into

//1. 
fn main() {
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    let i3: i32 = 'a' as u32 as i32;

    let i4: i32 = 'a' as i32;

    assert_eq!(i3, 97); 
    assert_eq!(i4, 97); 

    let s: String = String::from('a');

    let s2: String = 'a'.to_string();

    assert_eq!(s, s2);
    
    println!("Success!");
}

//2. 
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

// FILL in the blanks
fn main() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30.into();
    assert_eq!(num.value, 30);

    println!("Success!");
}

//3. 🌟🌟🌟 When performing error handling it is often useful to implement From trait for our own error type. Then we can use ? to automatically convert the underlying error type to our own error type.

use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> CliError {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> CliError {
        CliError::ParseError(error)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let contents = fs::read_to_string(&file_name)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    match open_and_parse_file("numbers.txt") {
        Ok(value) => println!("Parsed number: {}", value),
        Err(e) => match e {
            CliError::IoError(err) => eprintln!("IO Error: {}", err),
            CliError::ParseError(err) => eprintln!("Parse Error: {}", err),
        },
    }

    println!("Success!");
}


//4 .

fn main() {
    let n: i16 = 256;

    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0); 

    println!("Success!");
}

//5. 
#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // Fill in the blanks
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8))); 
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(())); 
    println!("Success!");
}


// 12.3 Others

//1. use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // Fill in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!");
}

//2. 🌟🌟🌟 We can use parse method to convert a String into a i32 number, this is because FromStr is implemented for i32 type in standard library: impl FromStr for i32

use std::str::FromStr;

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse().unwrap(); 
    let from_str = i32::from_str("20").unwrap(); 
    
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}

//3. 🌟🌟 We can also implement the FromStr trait for our custom types

use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(',')
                                 .map(|x| x.trim())
                                 .collect();

        let x_fromstr = coords[0].parse::<i32>()?;  
        let y_fromstr = coords[1].parse::<i32>()?;  

        Ok(Point { x: x_fromstr, y: y_fromstr }) 
    }
}

fn main() {
  
    let p = Point::from_str("(3, 4)");  
    
    assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

    println!("Success!");
}

