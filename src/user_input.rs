use std::io;


pub fn get_chars() -> i32{
    println!("How many Characters?");
    let mut characters = String::new();

    io::stdin().read_line(&mut characters).expect("Failed to readline");
    if characters.ends_with('\n') {
        characters.pop();
        if characters.ends_with('\r') {
            characters.pop();
        }
    }
    characters.parse().unwrap()
}

pub fn get_iterators() -> i32{
    println!("How many Iterations?");
    let mut characters = String::new();

    io::stdin().read_line(&mut characters).expect("Failed to readline");
    if characters.ends_with('\n') {
        characters.pop();
        if characters.ends_with('\r') {
            characters.pop();
        }
    }
    characters.parse().unwrap()
}


