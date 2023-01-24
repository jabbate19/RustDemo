use std::io::{self, Read, Write};

fn main() {
    // This code is automatically provided to you when you "cargo init"
    // Easy first Hello World!
    println!("Hello, world!");

    // Now what if we want to add our name to the output?
    // We will need a String to take from stdin
    // Note how we add "mut" to make it mutable
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // print!, println!, and format! all allow for format parameters via {}
    println!("Hello, {}!", input.trim());

    // // Note how doing this would break the above println
    // let input2 = input;
    // // Do this to fix it
    // // let input2 = &input;
    // println!("Hello, {}!", input);
    // // Since we gave ownership of input to input2, it can no longer be used!
}
