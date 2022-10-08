/*
   Rust implementation program to convert u64 number to a string of words

   Copyright (c) NexPro 2022

   Based on Original C# implementation by Jonathan Wood
   Copyright (c) 2019-2020 Jonathan Wood (www.softcircuits.com)

   Licensed under the MIT license.
*/

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

pub fn number_to_words<T: std::convert::Into<f64>>(number: T) -> String {
    let number: f64 = number.into();
    let mut all_zeros = true;
    let mut should_skip_next_iteration = false;
    let mut result = String::from("");
    let mut temp: String;
    // Ensure number is positive value
    let number = num::abs(number);

    // Convert integer portion of value to string
    let rounded = num::Float::round(number);

    // Convert integer portion of value to string
    let mut digits_as_bytes = rounded.to_string().into_bytes();
    if digits_as_bytes.len() > MAX_DIGITS {
        return "* * * * * * * NUMBER TOO LARGE * * * * * * *".to_string();
    }
    // Modify digits so we can simply compare ints
    for _digit in digits_as_bytes.iter_mut() {
        *_digit -= ASCII_ZERO_OFFSET;
    }
    // Reverse iterate over digits in order to build our output string
    for i in (0..digits_as_bytes.len()).rev() {
        if should_skip_next_iteration {
            should_skip_next_iteration = false;
            continue;
        }
        let next_digit = digits_as_bytes[i];
        let column = digits_as_bytes.len() - (i + 1);

        // Determine if digit is in the ones, tens or hundreds column
        match column % 3 {
            0 => {
                // Ones
                let mut show_thousands = true;
                if i == 0 {
                    // First digit in number (last in loop)
                    temp = ONES[next_digit as usize].to_string() + " ";
                } else if digits_as_bytes[i - 1] == 1 {
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
                        digits_as_bytes[i - 1] != 0 || (i > 1 && digits_as_bytes[i - 2] != 0);
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
                        + (if digits_as_bytes[i + 1] != 0 {
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
    result = result + "and " + &format!("{:}/100", num::Float::round(cents * 100.0));
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(1.0, "one and 0/100")]
    #[case(15.04, "fifteen and 4/100")]
    #[case(99988389.123, "ninety-nine million, nine hundred eighty-eight thousand, three hundred eighty-nine and 12/100")]
    #[case(12308120381241.876, "twelve trillion, three hundred eight billion, one hundred twenty million, three hundred eighty-one thousand, two hundred forty-two and 88/100")]
    #[case(1266473890984381241., "one quintillion, two hundred sixty-six quadrillion, four hundred seventy-three trillion, eight hundred ninety billion, nine hundred eighty-four million, three hundred eighty-one thousand, two hundred and 0/100")]
    fn test_float_inputs(#[case] input: f64, #[case] expected: &str) {
        assert_eq!(number_to_words(input), expected);
    }

    #[rstest]
    #[case(1, "one and 0/100")]
    #[case(15, "fifteen and 0/100")]
    #[case(1266, "one thousand, two hundred sixty-six and 0/100")]
    #[case(
        1230812,
        "one million, two hundred thirty thousand, eight hundred twelve and 0/100"
    )]
    #[case(99988389, "ninety-nine million, nine hundred eighty-eight thousand, three hundred eighty-nine and 0/100")]
    fn test_signed_integer_inputs(#[case] input: i32, #[case] expected: &str) {
        assert_eq!(number_to_words(input), expected);
    }

    #[rstest]
    #[case(1, "one and 0/100")]
    #[case(15, "fifteen and 0/100")]
    #[case(1266, "one thousand, two hundred sixty-six and 0/100")]
    #[case(
        1230812,
        "one million, two hundred thirty thousand, eight hundred twelve and 0/100"
    )]
    #[case(99988389, "ninety-nine million, nine hundred eighty-eight thousand, three hundred eighty-nine and 0/100")]
    fn test_unsigned_integer_inputs(#[case] input: u32, #[case] expected: &str) {
        assert_eq!(number_to_words(input), expected);
    }
}
