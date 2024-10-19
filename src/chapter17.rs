//17.1 basic

// 1./* Annotate the lifetime of `i` and `borrow2` */

// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses 
// both `borrow1` and `borrow2`. The duration of `borrow1` compared 
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3;                                             
    {                                                    
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐
        //                                                │
        println!("borrow1: {}", borrow1); //              │
    } // `borrow1 ends. ──────────────────────────────────┘
    {                                                    
        let borrow2 = &i; 
                                                        
        println!("borrow2: {}", borrow2);               
    }                                                   
}   
//2 
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

//3 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    let string2 = String::from("short");
    
    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);
}

//4 
fn static_output() -> &'static str {
    "foo" // Returning a string literal
}

fn main() {
    let my_str = static_output();
    println!("{}", my_str); // Valid output
}

//5 

