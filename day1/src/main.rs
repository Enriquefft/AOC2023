use std::fs;
use lazy_static::lazy_static;
use std::collections::HashMap;


fn get_digit <T: Iterator<Item=char>> (mut iter: T) -> Option<u32> {


    while let Some(c) = iter.next() {

        if c.is_digit(10) {

            let resoponse = c.to_digit(10);
            println!("Found digit: {}", c);
        }

        if let Some(digit) = eval_string(c, &mut iter){
            println!("Found digit: {}", digit);
            return Some(digit);
        }

    }
    None

}

lazy_static! {

    static ref CHAR_MAP: HashMap<char, Vec<(String, u8)>> = {

    let mut m: HashMap<char, Vec<(String, u8)>> = HashMap::new();

    // one
    m.insert('o', vec![(String::from("ne"), 1)]);

    // two, three
    m.insert('t', vec![(String::from("wo"), 2), (String::from("hree"), 3)]);

    // four, five
    m.insert('f', vec![(String::from("our"), 4), (String::from("ive"), 5)]);
    // six, seven
    m.insert('s', vec![(String::from("ix"), 6), (String::from("even"), 7)]);
    // eight
    m.insert('e', vec![(String::from("ight"), 8)]);
    // nine
    m.insert('n', vec![(String::from("ine"), 9)]);

        m
    };


}

fn eval_string <T: Iterator<Item=char>> (c: char, iter: & mut T) -> Option<u32> {

    let number_info = CHAR_MAP.get(&c);

    if number_info.is_none() {
        return None;
    }

    return scan_it_for(iter, number_info.unwrap());

}

/// Scan the iterator for the given string, and return the number_to_scan
/// if the string is found.  Otherwise, return None.
/// If a number is found, it will be returned instead.
///
/// # Arguments
/// * `iter` - The iterator to scan
/// * `number_str` - The string to scan for
/// * `number_to_scan` - The number to return if the string is found
fn scan_it_for<T: Iterator<Item=char>>(iter: &mut T, number_to_scan: &Vec<(String, u8)>) -> Option<u32> {


    for (c, number_char) in iter.zip(number_str.chars()) {

        if c.is_digit(10) {
            return c.to_digit(10);
        }

        if c != number_char {
            return None;
        }
    }
    return Some(number_to_scan);
}

fn get_calibration(contents: String)-> u32{
    let mut calibration_value: u32 = 0;

    contents.lines().for_each(|line| {

        let first_digit = get_digit(line.chars());
        let last_digit = get_digit(line.chars().rev());

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
           let digit = first * 10 + last;
            println!("Found final digit: {}", digit);
           calibration_value += digit;
       }

    });
    return calibration_value;
}

fn main() {

    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let calibration_value = get_calibration(contents);

    println!("Calibration value 1: {}", calibration_value);
}
