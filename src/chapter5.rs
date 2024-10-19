//5.1 Ownership

//1. Use as many approaches as you can to make it work

fn main() {
    let x = String::from("Hello world");
    let y = x.clone(); 
    println!("{}, {}", x, y); 
}

//2. Don't modify code in main!

fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String { 
    println!("{}", s);
    s 
}

//3. Only modify the code below!

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("Hello world");
    s 
}

//4. // Fix the error without removing any code

fn main() {
    let s = String::from("Hello World");

    print_str(&s); // Pass a reference to s

    println!("{}", s);
}

fn print_str(s: &String) { // Take a reference to String
    println!("{}", s)
}

//5. Don't use clone ,use copy instead

fn main() {
    let s = String::from("Hello World");

    print_str(&s); // Pass a reference to s

    println!("{}", s);
}

fn print_str(s: &String) { // Take a reference to String
    println!("{}", s)
}

//6. Mutability can be changed when ownership is transferred.

fn main() {
    let s = String::from("Hello ");
    
    let mut s1 = s; // Make s1 mutable

    s1.push_str("World!"); // Now we can modify s1

    println!("Success!");
}

//7. 

fn main() {
    let x = Box::new(5);
    
    let y = &*x; // This creates an immutable reference
    
    // *y = 4; // This would cause an error since y is immutable

    assert_eq!(*x, 5); // This assertion will pass

    println!("Success!");
}


//8.  Modify this line only, don't use `_s`

fn main() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", (&t.0, &t.1)); // Use references to the remaining elements
}

//9. Fill the blanks

fn main() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone(); // Use clone() to get owned values
    let _t = t; // Retain the original tuple

    println!("{:?}, {:?}, {:?}", s1, s2, _t); // -> "hello", "world", ("hello", "world")
}

//5.2 Reference and Borrowing

//1. Fill the blanks
fn main() {
    let x = 5;
    // Fill the blank
    let p = &x; 

    println!("the memory address of x is {:p}", p); 
}


//2. 
fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);  

    println!("Success!");
}


//3.  Fix error

fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s); 

    println!("Success!");
}

fn borrow_object(s: &String) {}


//4. 
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s); 

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world");
}


//5. 

fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s; // Create a mutable reference to s
    
    p.push_str("world"); // Modify the string through the reference

    println!("Success!");
}


//6. 
fn main() {
    let c = '中';

    let r1 = &c;
    // Fill the blank, don't change other code
    let r2 = &c; // Make r2 a reference to c

    assert_eq!(*r1, *r2);
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

//7.  Remove something to make it work.  Don't remove a whole line !

fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // Remove the mutable reference
    // let r2 = &mut s; // This line can be removed to avoid borrowing conflict.

    println!("{}", r1); // Now, only r1 is used

    println!("Success!");
}


//8. 🌟 Error: Borrow an immutable object as mutable

fn main() {
    // Fix error by modifying this line
    let mut s = String::from("hello, "); // Make `s` mutable

    borrow_object(&mut s); // Pass a mutable reference to the function

    println!("Success!");
}

fn borrow_object(s: &mut String) {}


//9. 🌟🌟 Ok: Borrow a mutable object as immutable


fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);  // Borrowing an immutable reference
    
    s.push_str("world");  // Modifying the string

    println!("Success!");
}

fn borrow_object(s: &String) {}

//10. Comment one line to make it work

fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    
    
    println!("{}", r1); 
}


//11.
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s; 
    let r2 = &mut s; 

    r1.push_str("world");
    r2.push_str("!"); 

    println!("{} {}", r1, r2); /
}
