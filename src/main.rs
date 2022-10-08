// Based on Original C# implementation by Jonathan Wood
// Copyright (c) 2019-2020 Jonathan Wood (www.softcircuits.com)
//
// Rust implementation
// Copyright (c) NexPro 2022
// Licensed under the MIT license.
//

extern crate num;

const MAX_DIGITS: usize = 49;

const ONES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

// US
const THOUSANDS: [&str; 10] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
];

const ASCII_ZERO_OFFSET: u8 = 48;
const NUMBER_TO_TEST: f64 = 78345929533.333;

fn main() {
    // test here
    println!("{}", number_to_words(NUMBER_TO_TEST));
}

fn number_to_words(number: f64) -> String {
    let mut all_zeros: bool = true;
    let mut should_skip_next_iteration = false;
    let mut result = String::from("");
    let mut temp: String;

    // Ensure number is positive value
    let number = num::abs(number);

    // Convert integer portion of value to string
    let rounded: f64 = num::Float::round(number);

    // Convert integer portion of value to string
    let mut digits_as_bytes = rounded.to_string().into_bytes();
    if digits_as_bytes.len() > MAX_DIGITS {
        return "* * * * * * * NUMBER TOO LARGE * * * * * * *".to_string();
    }
    for _digit in digits_as_bytes.iter_mut() {
        *_digit -= ASCII_ZERO_OFFSET;
    }
    // Reverse iterate over digits
    for mut _i in (0..digits_as_bytes.len()).rev() {
        if should_skip_next_iteration {
            should_skip_next_iteration = false;
            continue;
        }
        let next_digit = digits_as_bytes[_i];
        let column = digits_as_bytes.len() - (_i + 1);

        // Determine if ones, tens, or hundreds column
        match column % 3 {
            0 => {
                // Ones
                let mut show_thousands = true;
                if _i == 0 {
                    // First digit in number (last in loop)
                    temp = ONES[next_digit as usize].to_string() + " ";
                } else if digits_as_bytes[_i - 1] == 1 {
                    // This digit is part of "teen" value
                    temp = TEENS[next_digit as usize].to_string() + " ";
                    // Skip tens position
                    should_skip_next_iteration = true;
                } else if next_digit != 0 {
                    // Any non-zero digit
                    temp = ONES[next_digit as usize].to_string() + " ";
                } else {
                    // This digit is zero. If digits in tens and hundreds
                    // column are also zero, don't show "thousands"
                    temp = "".to_string();
                    show_thousands =
                        digits_as_bytes[_i - 1] != 0 || (_i > 1 && digits_as_bytes[_i - 2] != 0);
                }
                // Show "thousands" if non-zero in grouping
                if show_thousands {
                    if column > 0 {
                        temp = temp
                            + &(THOUSANDS[column / 3].to_string()
                                + if all_zeros { " " } else { ", " });
                    }
                    // Non-zero digit found
                    all_zeros = false;
                }
                result = (temp.clone() + &result).to_string();
            }
            1 => {
                // Tens
                if next_digit > 0 {
                    temp = TENS[next_digit as usize].to_string()
                        + (if digits_as_bytes[_i + 1] != 0 {
                            "-"
                        } else {
                            " "
                        });
                    result = temp + &result;
                }
            }
            2 => {
                // Hundreds
                if next_digit > 0 {
                    temp = ONES[next_digit as usize].to_string() + " hundred ";
                    result = temp + &result;
                }
            }
            _ => {
                // Default case. Do nothing?
            }
        }
    }
    // Append fractional portion/cents
    let cents = number - num::Float::floor(number);
    result = result + "and " + &format!("{:}/100", num::Float::floor(cents * 100.0));
    result
}
