use rand::Rng;
use std::io;

fn main() {
    println!("Password length:");

    let mut password_length = String::new();
    io::stdin().read_line(&mut password_length).unwrap();

    let password_length_to_int: i32 = password_length
        .trim()
        .parse()
        .expect("Input not an integer");

    println!("Do you want to have special characters?: y/n");
    let mut has_special_characters = String::new();
    io::stdin().read_line(&mut has_special_characters).unwrap();

    let mut charset: String = "".to_owned();

    if has_special_characters.to_lowercase().trim() == "y" {
        charset.push_str("!@#$%^&*()_+[]{}|;:,.<>?");
    }

    println!("Do you want to have letters?: y/n");
    let mut has_letters = String::new();
    io::stdin().read_line(&mut has_letters).unwrap();

    if has_letters.to_lowercase().trim() == "y" {
        charset.push_str("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    println!("Do you want to have numbers?: y/n");
    let mut has_numbers = String::new();
    io::stdin().read_line(&mut has_numbers).unwrap();

    if has_numbers.to_lowercase().trim() == "y" {
        charset.push_str("0123456789");
    }

    let mut password: String = "".to_owned();

    let mut rng = rand::thread_rng();

    if charset.is_empty() {
        println!("No character set selected. Exiting.");
        return;
    }

    let charset_chars: Vec<char> = charset.chars().collect();

    for _ in 0..password_length_to_int {
        let index = rng.gen_range(0..charset_chars.len());
        password.push(charset_chars[index]);
    }

    println!("Generated password: {}", password);
}
