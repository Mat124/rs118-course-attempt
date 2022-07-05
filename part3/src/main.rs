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

}