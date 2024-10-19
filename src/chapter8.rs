//8.1 match, matches! and if let

//1. // Fill the blanks

// Fill the blanks
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}

//2. 🌟🌟 Match is an expression, so we can use it in assignments.

fn main() {
    let boolean = true;

    
    let binary = match boolean {
        true => 1,   
        false => 0,  
    };

    assert_eq!(binary, 1);

    println!("Success!");
}

//3.🌟🌟 Using match to get the data an enum variant holds.

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    println!("Success!");
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => { 
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255); 
            assert_eq!(b, 0);   
        },
        _ => println!("no data in these variants"),
    }
}

//4.     // Fill the blank with `matches!` to make the code work

fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z')); 
    }

    println!("Success!");
}

//5. 
#[derive(PartialEq)] // Derive PartialEq for MyEnum
enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if e == MyEnum::Foo {
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

//6.  
fn main() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}

//7.  Fill in the blank

enum Foo {
    Bar(u8),
}

fn main() {
    let a = Foo::Bar(1);

    match a { 
        Foo::Bar(i) => { 
            println!("foobar holds the value: {}", i);

            println!("Success!");
        }
    }
}

//8. 
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    if let Foo::Bar = a {
        println!("foo::bar")
    } else if let Foo::Baz = a {
        println!("foo::baz")
    } else {
        println!("others")
    }
}

//9. Fix the errors in-place

fn main() {
    let age = Some(30);
    if let Some(age) = age { 
        assert_eq!(age, 30); 
    } 
    
    match age {
        Some(age) => println!("age is a new variable, it's value is {}", age),
        _ => (),
    }
}

//8.2 Patterns

//1. 🌟🌟 Use | to match several values, use ..= to match an inclusive range.

fn main() {
    match_number(3); 
}

fn match_number(n: i32) {
    match n {
        1 => println!("One!"),
        2 | 3 | 4 | 5 => println!("match 2 -> 5"), 
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

//2. 🌟🌟🌟 The @ operator lets us create a variable that holds a value, at the same time we are testing that value to see whether it matches a pattern.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Fill in the blank to let p match the second arm
    let p = Point { x: 3, y: 20 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

//3. Fix the errors

enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id } if (3..=7).contains(&id) => { 
            println!("Found an id in range [3, 7]: {}", id);
        },
        Message::Hello { id } if id == 10 || id == 11 || id == 12 => { 
            println!("Found an id in another range [10, 12]: {}", id);
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}


//4. 🌟🌟 A match guard is an additional if condition specified after the pattern in a match arm that must also match, along with the pattern matching, for that arm to be chosen.


// Fill in the blank to make the code work, `split` MUST be used
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

//5. 🌟🌟 Ignoring remaining parts of the value with ..


// Fill the blank to make the code work
fn main() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, _, _, _, _, _, _, _, _, _, last) => {
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}
//6. 🌟🌟 Using pattern &mut V to match a mutable reference requires you to be very careful, due to V being a value after matching.

fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!") // Remove `&mut` from the pattern
    }
}
