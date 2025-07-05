// Rules
#![allow(dead_code)]

// Imports
use std::io;

mod end_exponent;
mod dictionnary_pages;
mod multiplicative_persistence;
mod palindrome_mutlibasis;
mod constants;

const MOD_LIST: [&str; 4] = ["end_exponent", "dictionnary_pages", "multiplicative_persistence", "palindrome_mutlibasis"];


// Series of exercice from the book "100 enigmes mathematiques résolues avec Python", using Rust.
// This project is focused the chapter 3, "Digits" ("Chiffres"), p61.
// Each problem will have its own file, imported here.

fn main() {
    // User inputs
    println!("# **ENIGMES**");
    println!("# *Chapter 3 - Digits (p.61)*");
    println!();
    println!("Choose from the following: ");
    for module in MOD_LIST {
        println!("- {}", module);
    }
    /*
    let mut user_mode: String = String::new();
    io::stdin()
        .read_line(&mut user_mode)
        .expect("(!) - Failed to read the line");
    */

    /*
    // Palindrome in multiple codexes
    println!("## Palindromes");
    let is_palindrome = palindrome_mutlibasis::number_palindrome(12121);
    println!("{}", is_palindrome);
    */

    /*
    // Multiplicative persistence
    println!("## Multiplicative persistence");
    let a: u32 = 10u32.pow(9u32);
    let max_pers = multiplicative_persistence::max_persistence(a);
    println!("Max persistence for {} = {}, with {}", a, max_pers.0, max_pers.1);
    */

    
    /*
    // Pages of a dictionnary
    println!("## Dictionnary pages");
    println!("Digits: {}", dictionnary_pages::count_digits(24u32));
    println!("Pages: {}", dictionnary_pages::count_pages(4989u32));
    */

    /*
    // End exponents
    println!("## End exponents");
    let a: u128 = 8;
    let solutions: Vec<usize> = end_exponent::try_for_a(a, 1usize, 10000usize);
    println!("Solutions for {}: {:?}", a, solutions);
    for n in solutions.iter() {
        println!("Ψ:{} -> {}", (*n) as u32, a.pow((*n) as u32));
    }
    */

    // Constant
    println!("## Constants");
    println!("Precision: {}; iterations: {}", constants::PRECISION, constants::ITERATIONS);
    println!("Euler's number");
    let approx_euler_number1: Vec<u8> = constants::approximation_euler_number(constants::ITERATIONS);
    println!("- Found: {:?}", approx_euler_number1)

}

