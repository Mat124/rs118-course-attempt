fn main() {
    //panic!("Hello, world!"); //manually throw an error with custom message
    let v1 = vec!["1", "2"];
    //let i = v1[2]; //also creates a panic (error)
    
    // Results
    // Used to handle errors at runtime, either Ok or Err - used with match
    use std::{fs, io};
    let f1: Result<fs::File, io::Error> = fs::File::open("Cargo.toml"); //existant file
    println!("{f1:?}");
    let f2: Result<fs::File, io::Error> = fs::File::open("foo.bar"); //nonexistant file
    println!("{f2:?}");
    //can also use match
    println!("Using match:");
    match f1 {
        Ok(file) => println!("File 1 opened successfully"),
        Err(_) => println!("Could not open file 1")
    }
    match f2 {
        Ok(file) => println!("File 2 opened successfully"),
        Err(_) => println!("Could not open file 2")
    }
    //unwrap/expect would also work, but panics if Result is Err. Useful for prototyping

    // Generics
    // Exist similar to java's generics, very easy to use. Put in <> when initialising the variable
    // already seen many examples: Result, Option, etc
    // See the Point struct below

    // Traits
    // Define shared behaviour - basically interefaces - states what functions something needs to have to have that trait

    // Lifetimes
    // Force inputs to a function to have the same lifetime so that it is known how long the output is valid for
    let s1 = String::from("This is a long string");
    let s2 = String::from("Short string");
    let s3 = longest(s1.as_str(), s2.as_str());//needs as_str() as expects string literal, not String struct
    // lifetimes also have to be marked for structs containing references, and ties the lifetime of the struct to that of the reference
    // e.g. Word is a reference to the first word in a string and stops existing when the string does
    let fst: Word;
    {//sets a new lifetime
        let sentence = String::from("one two three");
        fst = Word::new(&sentence.as_str());
        println!("first word is: {}", fst.0); //fst is valid here as sentence is valid
    }
    // println!("first word is: {}", fst.0); //fst is no longer valid, wouldn't compile

    // Closures
    // anonymous functions that are passed to higher level functions for use in them
    // e.g. the .map of a vec applies the anonymous function given to it to each item in the vec
    let square = |r| r * r; //returns the square of r
    let nums = [1, 2, 3, 4];
    println!("{nums:?}"); //as vec, requires formatting as no default, :? indicates debugging format
    let squared_nums = nums.map(square);
    println!("{squared_nums:?}");
}

struct Word<'a>(&'a str);

impl<'a> Word<'a> {
    pub fn new(input: &'a str) -> Self {
        let first_word = input.split_ascii_whitespace().next().unwrap();
        Self(first_word)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { //has lifetime 'a, means that inputs have the same lifetime or panics
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Point<T> { //x and y are of type T
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self { //creates new Point with values of x and y of type T
        Self {x, y}
    }
}