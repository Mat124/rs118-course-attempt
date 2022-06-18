#![allow(unused)]

struct Professor {
    name: String,
    id: String,
    subject: String,
    parking_space: bool,
    age: u32,
    student_count: u32,
    hair: HairColour
}

enum HairColour {
    Red,
    Blonde,
    Brown,
    Black
}

fn get_student_teacher_ratio(x: Professor) -> Option<u32> {
    if (x.student_count == 0) {
        None
    } else {
        Some(x.student_count)
    }
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
        student_count: 0,
        hair: HairColour::Black
    };

    println!("The new professor is called {} and they are {} years old!", matt.name, matt.age);
    
    println!("Their student teacher ratio is {:?}", get_student_teacher_ratio(matt));

    /*
    loop {
        println!("Why did you make this infinite loop?");
    }
    */

    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    for letter in letters {
        println!("The next letter is: {}", letter);
    }

    let funky_string = String::from("This is my funky new string");

    let mut s_count = 0;

    for letter in funky_string.chars() {
        match letter {
            's' => {
                s_count = s_count + 1;
            },
            _ => {

            }
        }
    }

    println!("There are {} characters in the phrase: \"{}\"", s_count, funky_string);
}