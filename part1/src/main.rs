#![allow(unused)]

struct Professor {
    name: String,
    id: String,
    subject: String,
    parking_space: bool,
    age: u32,
    bald: bool,
    hair: HairColour
}

enum HairColour {
    Red,
    Blonde,
    Brown,
    Black
}

enum Option<T> {
    None,
    Some(T)
}

fn getHair(x: Professor) -> Option<String> {

}

fn main() {
    let mut x = '7';
    println!("Hello, world! The value of x is {}", x);
    x = 'd';
    println!("The value of x is now {}", x);

    let tup_one = (1, 2);
    println!("The first value in tup_one is: {}\nthe second value is: {}", tup_one.0, tup_one.1);

    let matt = Professor {
        name: String::from("Matt"),
        id: String::from("1234567"),
        subject: String::from("Computer Science"),
        parking_space: true,
        age: 30,
        bald: true,
        hair: None
    };

    println!("The new professor is called {} and they are {} years old!", matt.name, matt.age);
}