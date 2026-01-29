use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let message = "Hello from Ferris!";
    let width = message.len();

    let stdout = stdout();
    let out = BufWriter::new(stdout.lock());

    say(message, width, out).unwrap();

    // Declaring whole number
    let mut x = -43;
    x = 10;
    println!("The value of x is: {}", x);

    // Declaring datatypes
    let x: i32 = 2147483647; //i32 is signed 32bit variable ;
    let y: u64 =  9223372036854775809; //u64 is unsigned 64bit variable
    println!("x is {}", x); 
    println!("y is {}", y);

    //Combination of placeholders
    let my_num: i32 = 5; // integer
    let my_double: f64 = 5.99; //float
    let my_letter: char = 'D'; //character
    let my_bool: bool = true; //boolean
    let my_text: &str = "Hello"; // string slice

    println!("my_num = {}", my_num);
    println!("my_double = {}", my_double);
    println!("my_letter = {}", my_letter);
    println!("my_bool = {}", my_bool);
    println!("my_text = {}", my_text);

    //Creating a constant
    const BIRTHYEAR: i32 = 1980;
    println!("What is your birth year? {}", BIRTHYEAR);

    // Operators
    let add = 5+10;
    let sub = 10-4;
    let mul = 6*2;
    let div = 12/3;
    let rem = 10%3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);


    //Assignment operators
    let mut x = 10; 
    println!("Start: {}", x); 
    x += 5; 
    println!("After += 5: {}", x); 
    x -= 2; 
    println!("After -= 2: {}", x); 
    x *= 2; 
    println!("After *= 2: {}", x); 
    x /= 3; 
    println!("After /= 3: {}", x); 
    x %= 4; 
    println!("After %= 4: {}", x); 

    //Comparison Operators
    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b); 
    println!("5 != 10: {}", a != b); 
    println!("5 < 10: {}", a < b); 
    println!("5 >= 10: {}", a >= b);     

    // Logical Operators
    let logged_in = true; 
    let is_admin = false; 

    println!("Is regular user: {}", logged_in && !is_admin); 
    println!("Has any access: {}", logged_in || is_admin); 
    println!("Not logged in: {}", !logged_in); 

    // Conditions or if-else statements
    let score = 85; 

    if score >= 90 { 
    println!("Grade: A"); 
    } else if score >= 80 { 
    println!("Grade: B"); 
    } else if score >= 70 { 
    println!("Grade: C"); 
    } else { 
    println!("Grade: F"); 
    } 

    // Expression
    let time = 20; 
    let greeting = if time < 18 { 
    "Good day." 
    } else { 
    "Good evening." 
    }; 
    println!("{}", greeting);

    //Match
    let day = 4; 
    let result = match day { 
    1 => "Monday", 
    2 => "Tuesday", 
    3 => "Wednesday", 
    4 => "Thursday", 
    5 => "Friday", 
    6 => "Saturday", 
    7 => "Sunday", 
    _ => "Invalid day.", 
    }; 
    println!("{}", result);

    //Loop 
    let mut count = 1; 
    let result = loop { 
    println!("Hello!"); 
    if count == 3 { 
    break count;  
    } 
    count += 1; 
    }; 
    println!("The loop stopped at: {}", result); 

    //While Loop
    let mut num = 1; 
    while num <= 10 { 
    if num == 6 { 
    num += 1; 
    continue; 
    } 
    println!("Number: {}", num); 
    num += 1; 
    }

    // For Loop
    for i in 1..=10 { 
    if i == 3 { 
    continue; // skip 3 
    } 
    if i == 5 { 
    break; // stop before printing 5 
    }   
    println!("i is: {}", i); 
    }


    // Function
    fn add(a: i32, b: i32) -> i32 { 
        a + b 
    } 
    let sum = add(3, 4); 
    println!("Sum is: {}", sum); 

    //Scope
    let x = 5; 
    { 
        let x = 10; 
        println!("Inside block: {}", x); 
    } 
    println!("Outside block: {}", x); 

    //String
    let text1 = "Hello World".to_string(); 
    println!("{}", text1); 
    let text2 = String::from("Hello World"); 
    println!("{}", text2); 

    //Ownership
    let a = String::from("Hello"); 
    let b = a; 
    // println!("{}", a); Error: a no longer owns the value 
    println!("{}", b); // Ok: b now owns the value\

    //Clone
    let a = String::from("Hello"); 
    let b = a.clone(); // Now both have the same value 
    println!("a = {}", a); 
    println!("b = {}", b);
    }

    //Borrowing and References
    let a = String::from("Hello"); 
    let b = &a; 
    println!("a = {}", a); 
    println!("b = {}", b); 

    //Mutable References
    let mut name = String::from("John"); 
    let name_ref = &mut name; 
    name_ref.push_str(" Doe"); 
    println!("{}", name_ref);

    // Data Structures
    // Arrays
    let fruits = ["apple", "banana", "orange"]; 
    println!("Last fruit: {}", fruits[2]); 

    // Vectors
    let mut fruits = vec!["apple", "banana"]; 
    fruits.push("cherry"); 
    println!("Last fruit: {}", fruits[2]);

    // Tuples
    let person = ("John", 30, true); 
    println!("Name: {}", person.0); 
    println!("Age: {}", person.1); 
    println!("Is active: {}", person.2);

    // HashMaps
    use std::collections::HashMap;
    let mut capitalCities = HashMap::new(); 
    capitalCities.insert("France", "Paris"); 
    capitalCities.insert("Japan", "Tokyo"); 
    println!("Capital of Japan is {}", capitalCities["Japan"]); 

    // Structs
    // Create a Struct called Person 
    struct Person { 
    name: String, 
    age: u32, 
    can_vote: bool, 
    } 
    // Create a Person object 
    let user = Person { 
    name: String::from("John"), 
    age: 35, 
    can_vote: true, 
    }; 

    // Access and print the values 
    println!("Name: {}", user.name); 
    println!("Age: {}", user.age); 
    println!("Can vote? {}", user.can_vote); 

    // Enums
    enum Direction { 
        Up, 
        Down, 
        Left, 
        Right, 
    } 
    let my_direction = Direction::Left; 
    match my_direction { 
    Direction::Up => println!("Going up"), 
    Direction::Down => println!("Going down"), 
    Direction::Left => println!("Going left"), 
    Direction::Right => println!("Going right"), 
    } 
