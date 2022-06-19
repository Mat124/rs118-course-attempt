fn main() {
    println!("Hello, world! This is Part 2.");

    // string literal vs String struct
    let s = "string literal of constant contents";
    println!("{}", s);

    let mut s_mut = String::from("mutable string of variable contents and length");
    println!("{}", s_mut);
    s_mut = String::from("the same string variable with different contents");
    println!("{}", s_mut);
    s_mut.push_str(" that has now been extended"); //can't do this if didn't use String::from
    println!("{s_mut}");

    // Moving values
    let x = 5;
    let y = x; //copies the value of 5 and binds it to y

    let s1 = String::from("string 1");
    let s2 = s1; //this destroys s1 - it no longer has a value
    // String is a struct of 3 parts: length, capacity of buffer, pointer to bufer containing chars
    //println!("{s1}"); //doesnt work as s1 has no value anymore
    println!("{s2}"); //works and has the value of string 1

    //Copy and Clone
    let s3 = String::from("string 3");
    let s4 = s3.clone();

    println!("{s3}"); //s3 has been cloned, so still has a value
    println!("{s4}"); //s4 has the same value as s4, stored elsewhere

    //Ownership and Functions
    let s5 = String::from("string 5");
    let int1 = 5;
    let mut s6 = String::from("string 6");
    takes_ownership(s5);
    //can't use s5 anymore - like doing 'some_string = s5', so s5 is no longer valid
    makes_copy(int1);
    println!("{int1}");
    //can continue to use int1, as i32 is automatically copied instead of moved
    println!("{s6}");
    s6 = takes_and_returns_ownership(s6);
    println!("{s6}");

    //References and Borrowing
    let s7 = String::from("string 7");
    let s7_len = calc_length(&s7);
    //using &s7 instead of s7 means a reference to s7 has been passed (like a pointer)
    println!("{s7}");

    //Mutable References
    //this allows changes to the variables to made inside the function that are returned
    let mut s8 = String::from("string 8");
    println!("{s8}");
    add_text(&mut s8); //requires mutable reference
    println!("{s8}");
    //cannot make multiple mutable references to the same variable, or have mutable and immutable references to the same variable
    //EITHER ONE MUTABLE REFERENCE OR ANY AMOUNT OF IMMUTABLE REFERENCES

    //Slices
    //References with a length, e.g. only 5 elements of a string starting from the 2nd element
    let s9 = String::from("string 9");
    let slice = last_five(&s9);
    println!("{slice}");

    
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

fn takes_and_returns_ownership(mut some_string: String) -> String {
    some_string.push_str(".5");
    some_string //returns this value
}

fn calc_length(s: &String) -> usize {
    s.len()
}

fn add_text(s: &mut String) { //requires mutable type
    s.push_str(".5");
}

fn last_five(s: &String) -> &str {
    let i = s.len();
    &s[i-5..i]
}