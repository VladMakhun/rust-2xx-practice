//16.1 println! and format

//1. 
fn main() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
}

//2. 
fn main() {
   /* Fill in the blanks to make it print:
   Hello world, I am 
   Sunface!
   */
   print!("Hello world, ");
   println!("I am" );
   println!("Sunface!");
}


// 16.2 Debug and Display

//1. 
use std::fmt;

struct Structure(i32);

// Implement the Debug trait for the Structure
impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Structure({})", self.0)
    }
}

fn main() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("{} months in a year.", 12); // Fill in the blank here

    println!("Now {:?} will print!", Structure(3)); // Fill in the blank here
}

//2. 🌟🌟 So fmt::Debug definitely makes one type printable, but sacrifices some elegance. Maybe we can get more elegant by replacing {:?} with something else( but not {} !)

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Implement the Display trait for Person
use std::fmt;

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person {{\n    name: \"{}\",\n    age: {},\n}}", self.name, self.age)
    }
}

fn main() {
    let person = Person { name: "Sunface".to_string(), age: 18 };

    // Use Display trait for a nicer output
    println!("{}", person);
}

//3. 🌟🌟 We can also manually implement Debug trait for our types

use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

// Implement fmt::Debug for Deep to customize output
impl fmt::Debug for Deep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Access the inner Structure and format its value
        write!(f, "{}", self.0 .0) // Prints the inner i32 value directly
    }
}

fn main() {    
    // This will now print the value directly
    println!("Now {:?} will print!", Deep(Structure(7)));
}


//4. 
use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

// Implement the Display trait for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y) // Format as required for Display
    }
}

// Implement the Debug trait for Point2D
impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y) // Format as required for Debug
    }
}

fn main() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}", point), "Debug: Complex { real: 3.3, imag: 7.2 }");
    
    println!("Success!");
}

//5. 
use std::fmt; 

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?; // Format as "count: value"
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]"); // Check expected output
    println!("Success!");
}


// 16.3 Formatting

//1. fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
    assert_eq!(format!("{1}{0}", 1, 2), "21"); // Fill this blank
    assert_eq!(format!("{1}{0}{1}{0}", 1, 2), "2112"); // Fill this blank
    println!("Success!");
}

//2. 
fn main() {
    println!("{argument}", argument = "test"); // => "test"

    /* Fill in the blanks */
    assert_eq!(format!("{name}{}", 1, name = "2"), "21"); // Fill this blank
    assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b"); // Fill this blank
    
    /* Fix the error */
    // Named argument must be placed after other arguments
    println!("{abc} {0}", abc = "def", 2); // Fix the error here

    println!("Success!");
}

//3.🌟🌟 By default, you can pad string with spaces
fn main() {
    // The following two are padding with 5 spaces
    println!("Hello {:5}!", "x"); // =>  "Hello x    !"  
    println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

    /* Fill in the blanks */
    assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x    !"); // Fill this blank
    assert_eq!(format!("Hello {:1$}!", "x", 5), "Hello x    !"); // Fill this blank

    println!("Success!");
}

//4. 🌟🌟🌟 Left align, right align, pad with specified characters.

fn main() {
    // Left align
    println!("Hello {:<5}!", "x"); // => Hello x    !
    // Right align
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!"); // Right-align with padding
    // Center align
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !"); // Center-align with padding

    // Left align, pad with '&'
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&!"); // Fill this blank

    println!("Success!");
}

//5. fn main() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    /* Fill in the blank */
    assert!(format!("{number:0>width$}", number=1, width=6) == "000001"); // Fill in with "000001"
    
    println!("Success!");
}

//6.🌟🌟 Floating point precision
fn main() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); // same as {:.4} => 3.1416 

    assert_eq!(format!("{:.2}", v), "3.14"); // Fill in with "{:.2}"
    assert_eq!(format!("{:+.2}", v), "+3.14"); // Fill in with "{:+.2}"
    assert_eq!(format!("{:.0}", v), "3"); // Fill in with "{:.0}"

    println!("Success!");
}

//7.🌟🌟🌟 String length
fn main() {
    let s = "Hello, world!";

    println!("{0:.5}", s); // => Hello

    assert_eq!(format!("Hello {1:.3}!", 3, "abcdefg"), "Hello abc!"); // Fill in with "{1:.3}"

    println!("Success!");
}

//8. fn main() {
    assert_eq!(format!("{:b}", 27), "11011"); // Fill in with "{:b}"
    assert_eq!(format!("{:o}", 27), "33");    // Fill in with "{:o}"
    assert_eq!(format!("{:x}", 27), "1b");    // Fill in with "{:x}"
    assert_eq!(format!("{:X}", 27), "1B");    // Fill in with "{:X}"

    println!("{:x}!", 27); // Hex with no prefix => 1b

    println!("{:#010b}", 27); // Pad binary with 0, width = 10,  => 0b00011011

    println!("Success!");
}

//9. 
fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}

fn main() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];
    
    /* Make it print:
    sunface: 99.1
    jack: 60.3
    */
    for (name, score) in scores {
        // Using width and precision to format the score
        println!("{name}: {:width.*}", score, precision);
    }
}
