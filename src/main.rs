//extern crate rand;
//#[macro_use]
//extern crate text_io;
//
mod character;
//mod computer;

use character::Character;
use character::Player;
//use std::io;

fn main() {
//    println!("----- Andrew's Rogue-like Game Attempt 1 -----");
//
//    println!("Please choose a class.");
//
//    for (i, elem) in character::CLASSES.iter().enumerate() {
//        println!("{}. {}", i + 1, &elem);
//    }
//
//    let mut char_ind: usize = 100;
//    while char_ind > character::CLASSES.len() {
//        char_ind = read!();
//    }
//
//    let mut input_name = String::new();
//    println!("Please choose a name.");
//    io::stdin().read_line(&mut input_name).expect("Failed to read line");
//    let char_name = input_name.trim();

    let nathan = Character::new("Nathan".to_string(), character::CLASSES[0]);

    println!("{}", nathan.info());
    print!("memory size {}", std::mem::size_of::<Character>());

}

