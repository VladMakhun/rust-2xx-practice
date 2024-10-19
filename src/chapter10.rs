//10.1 Generics


//1. 
struct A;          
struct S(A);       
struct SGen<T>(T); 

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));          
    gen_spec_t(SGen(A));   
    gen_spec_i32(SGen(42)); 

    generic::<char>(SGen('a'));

    generic(SGen('b'));

    println!("Success!");
}

//2. 🌟🌟 A function call with explicitly specified type parameters looks like: fun::<A, B, ...>().

use std::ops::Add;

fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    assert_eq!(5, sum(2i8, 3i8));       
    assert_eq!(50, sum(20, 30));        
    assert_eq!(2.46, sum(1.23, 1.23));   

    println!("Success!");
}

//3.
// Define a generic struct Point
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };      
    let float = Point { x: 1.0, y: 4.0 };     

    println!("Success!");
}

//4. // Modify the struct to have two generic types
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p = Point { x: 5, y: "hello".to_string() };

    println!("Success!");
}

//5. 
// Add generic for Val to make the code work
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = Val { val: "hello".to_string() };
    println!("{}, {}", x.value(), y.value());
}

//6. 
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中' };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}

//7. 
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5.0, y: 10.0 };
    println!("{}", p.distance_from_origin());
}


//10.2 Const Generics


//1. 🌟🌟 <T, const N: usize> is part of the struct type, it means Array<i32, 3> and Array<i32, 4> are different types.
struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let arrays = (
        Array {
            data: [1, 2, 3], // Array<i32, 3>
        },
        Array {
            data: [1.0, 2.0, 3.0], // Array<f64, 3>
        },
        Array {
            data: [1, 2], // Array<i32, 2>
        },
    );

    println!("Success!");
}

//2.  Fill in the blanks to make it work.

fn print_array<T, const N: usize>(arr: [T; N])
where
    T: std::fmt::Debug,
{
    println!("{:?}", arr);
}

fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}

//3. 🌟🌟🌟 Sometimes we want to limit the size of a variable, e.g when using in embedding environments, then const expressions will fit your needs.

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// Fix the errors in main.
fn main() {
    check_size([0u8; 767]); // Total size: 767 bytes
    check_size([0i32; 191]); // Total size: 191 * 4 bytes = 764 bytes

    // A &str is a reference, and its size is fixed (typically 16 bytes on 64-bit systems).
    check_size(["hello你好"; 47]); // Size of &str: 16 * 47 = 752 bytes

    // `String` size involves a heap allocation, but as a stack-allocated value, it's usually 24 bytes.
    check_size([(); 31].map(|_| "hello你好".to_string()));  // Size of String: 31 * 24 = 744 bytes

    // A `char` in Rust is 4 bytes (UTF-32 representation).
    check_size(['中'; 191]); // Size of char: 4 * 191 = 764 bytes

    println!("Success!");
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}


//10.3 Traits

//1. Fill in the two impl blocks to make the code work.
DON'T modify the code in `main`.

trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")

    fn say_something(&self) -> String;
}

struct Student {}

impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}

struct Teacher {}

impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }

    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}

fn main() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}


//2. The compiler is capable of providing basic implementations for some traits via the #[derive] attribute. For more info, please visit here.


#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = (_one_second == _one_second);
    let _this_is_false = (_one_second > _one_second);

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}


//3.  use std::ops::Mul;

fn multiply<T>(a: T, b: T) -> T
where
    T: Mul<Output = T>,
{
    a * b
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");
}


//4. 
use std::ops;

struct Foo;
struct Bar;

#[derive(PartialEq)] 
struct FooBar;

#[derive(PartialEq)] 
struct BarFoo;


impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Foo> for Bar {
    type Output = BarFoo;

    fn sub(self, _rhs: Foo) -> BarFoo {
        BarFoo
    }
}

fn main() {
    
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success!");
}

//5. 
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post '{}' is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo: {}", self.username, self.content)
    }
}

fn summary<T: Summary>(item: T) {
    println!("{}", item.summarize());
}

fn main() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(post);
    summary(weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}


//6. 
struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}


//7. use std::ops::Add;

fn sum<T>(x: T, y: T) -> T 
where
    T: Add<Output = T>, 
{
    x + y
}

fn sum_with_associated<T>(x: T, y: T) -> T 
where
    T: Add<Output = T>, 
{
    x + y
}

fn main() {
    assert_eq!(sum(1, 2), 3); 
    assert_eq!(sum(1.5, 2.5), 4.0); 
    assert_eq!(sum_with_associated(3, 4), 7); 
    assert_eq!(sum_with_associated(2.5, 3.5), 6.0); 
}


//8. // Define the Pair struct with generic type T
struct Pair<T> {
    x: T,
    y: T,
}

// Implement methods for Pair
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Implement methods for Pair when T implements Debug and PartialOrd
impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

// Implement Debug and PartialOrd for the Unit struct
#[derive(Debug, PartialOrd, PartialEq)]
struct Unit(i32);

fn main() {
    // Create a Pair of Unit instances
    let pair = Pair {
        x: Unit(1),
        y: Unit(3),
    };

    // Call the cmp_display method
    pair.cmp_display();
}

//9. fn example1() {
    // `T: Trait` is the commonly used way.
    // `T: Fn(u32) -> u32` specifies that we can only pass a closure to `T`.
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(10), 11); // The first call with 10 returns 11 (10 + 1)
    assert_eq!(cacher.value(15), 11); // The second call with 15 returns the cached value, which is 11
}

fn example2() {
    // We can also use `where` to construct `T`
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(20), 21); // The first call with 20 returns 21 (20 + 1)
    assert_eq!(cacher.value(25), 21); // The second call with 25 returns the cached value, which is 21
}

fn main() {
    example1();
    example2();

    println!("Success!");
}


//(10.4 Trait Object)

//1. trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn hatch_a_bird(number: u32) -> Box<dyn Bird> {
    if number == 1 {
        Box::new(Swan)
    } else {
        Box::new(Duck)
    }
}

fn main() {
    // FILL in the blank.
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim(); // Uncommenting this line will cause a compilation error
    // But it can quack.
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly(); // Uncommenting this line will cause a compilation error
    // But it can quack too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}


//2. trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn main() {
    // FILL in the blank to make the code work.
    let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];

    for bird in birds {
        bird.quack();
        // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
        // So, the code below will cause an error.
        // bird.fly(); // Uncommenting this line will cause a compilation error
    }
}


//3. trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn main() {
    let x = 1.1f64;
    let y = 8u8;

    // Draw x.
    draw_with_box(Box::new(x)); // Fill in the blank here

    // Draw y.
    draw_with_ref(&y); // Fill in the blank here

    println!("Success!");
}

fn draw_with_box(x: Box<dyn Draw>) {
    println!("{}", x.draw()); // Added to print the result of draw
}

fn draw_with_ref(x: &dyn Draw) { // Fill in the blank here
    println!("{}", x.draw()); // Added to print the result of draw
}

//4. 
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// Implement static dispatch using generics
fn static_dispatch<T: Foo>(item: T) {
    println!("{}", item.method());
}

// Implement dynamic dispatch using trait objects
fn dynamic_dispatch(item: &dyn Foo) {
    println!("{}", item.method());
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);       // Call the function using static dispatch
    dynamic_dispatch(&y);     // Call the function using dynamic dispatch

    println!("Success!");
}

//5. trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self { 42 }
}

impl MyTrait for String {
    fn f(&self) -> Self { self.clone() }
}

fn my_function(x: Box<dyn MyTrait>)  {
    x.f()
}

fn main() {
    my_function(Box::new(13_u32)); // Using a Box<u32>
    my_function(Box::new(String::from("abc"))); // Using a Box<String>

    println!("Success!");
}


//10.5 Advance Traits

//1. struct Container(i32, i32);

// Using associated types to re-implement trait Contains.
trait Contains {
    type A; // Associated type for the first parameter
    type B; // Associated type for the second parameter

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool; // Use associated types
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32; // Specify that A is i32
    type B = i32; // Specify that B is i32

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    
    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}


//2. use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// 1. Implementation with a default generic parameter for T.
impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// 2. Another implementation using a different default generic parameter (with f64).
impl Sub for Point<f64> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// 3. Implementation without using default parameters directly (using a concrete type).
impl Sub<Point<i32>> for Point<i32> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 });

    println!("Success!");
}


//3. 
trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

fn main() {
    let person = Human;

    assert_eq!(person.fly(), "This is your captain speaking."); // Calls Pilot's fly method
    assert_eq!(<Human as Wizard>::fly(&person), "Up!"); // Calls Wizard's fly method

    assert_eq!(person.fly(), "*waving arms furiously*"); // Calls Human's fly method

    println!("Success!");
}

//4 . trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also implement Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to implement both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

// Implement the necessary traits for CSStudent to make the code work
impl Person for CSStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for CSStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for CSStudent {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn main() {
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string()
    };

    // Fill in the blank
    println!("{}", comp_sci_student_greeting(&student));
}

//5. 
use std::fmt;

// Define a newtype `Pretty` here
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
