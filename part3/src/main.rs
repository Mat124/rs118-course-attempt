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

    // Generics
    // Exist similar to java's generics, very easy to use. Put in <> when initialising the variable

}