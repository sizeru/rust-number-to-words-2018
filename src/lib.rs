/*
   Rust implementation program to convert f64 number to a string of words

   Copyright (c) NexPro 2022

   Based on Original C# implementation by Jonathan Wood
   Copyright (c) 2019-2020 Jonathan Wood (www.softcircuits.com)

   Licensed under the MIT license.
*/

static ONES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

static TEENS: [&str; 10] = [
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

static TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
// US
static THOUSANDS: [&str; 5] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion"
];

const ASCII_ZERO_OFFSET: u8 = 48;
const LARGEST_ALLOWABLE_INPUT_VALUE:f64 = 9_999_999_999_999.99;

pub fn number_to_words<T: std::convert::Into<f64>>(
    number: T,
    should_capitalise_first_word: bool,
) -> String {

    // Convert to f64 and ensure number is positive value
    let number = num::abs(number.into());
    if  number > LARGEST_ALLOWABLE_INPUT_VALUE {
        return "Number too large".to_owned();
    }
    let formatted_num:String = round_and_format_number(number);
    let split_number = split_on_decimal_point(formatted_num);
    let mantissa = split_number[0].clone();
    let mut cents = split_number[1].clone();
    let mut all_zeros = true;
    let mut should_skip_next_iteration = false;
    let mut result = String::from("");
    let mut temp: String;

    // Convert integer portion of value to string
    let mut mantissa_as_bytes = mantissa.into_bytes();

    // Convert digits to bytes so we can simply compare ints
    for _digit in mantissa_as_bytes.iter_mut() {
        *_digit -= ASCII_ZERO_OFFSET;
    }
    // Reverse iterate over digits in order to build our output string
    for i in (0..mantissa_as_bytes.len()).rev() {
        if should_skip_next_iteration {
            should_skip_next_iteration = false;
            continue;
        }
        let next_digit = mantissa_as_bytes[i];
        let column = mantissa_as_bytes.len() - (i + 1);

        // Determine if digit is in the ones, tens or hundreds column
        match column % 3 {
            0 => {
                // Ones
                let mut show_thousands = true;
                if i == 0 {
                    temp = ONES[next_digit as usize].to_string() + " ";
                } else if mantissa_as_bytes[i - 1] == 1 {
                    // This digit is part of "teen" value
                    temp = TEENS[next_digit as usize].to_owned() + " ";
                    // Skip tens position
                    should_skip_next_iteration = true;
                } else if next_digit != 0 {
                    // Any non-zero digit
                    temp = ONES[next_digit as usize].to_owned() + " ";
                } else {
                    // This digit is zero. If digits in tens and hundreds
                    // column are also zero, don't show "thousands"
                    temp = "".to_owned();
                    show_thousands =
                        mantissa_as_bytes[i - 1] != 0 || (i > 1 && mantissa_as_bytes[i - 2] != 0);
                }
                // Show "thousands" if non-zero in grouping
                if show_thousands {
                    if column > 0 {
                        temp = temp
                            + &(THOUSANDS[column / 3].to_owned()
                                + if all_zeros { " " } else { ", " });
                    }
                    // Non-zero digit found
                    all_zeros = false;
                }
                result = (temp.clone() + &result).to_owned();
            }
            1 => {
                // Tens
                if next_digit > 0 {
                    temp = TENS[next_digit as usize].to_owned()
                        + (if mantissa_as_bytes[i + 1] != 0 {
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
                    temp = ONES[next_digit as usize].to_owned() + " hundred ";
                    result = temp + &result;
                }
            }
            _ => {
                // Default case. Do nothing?
            }
        }
    }
    
    if should_capitalise_first_word {
        result = capitalise_first_letter(result);
    }
    // Remove leading zero from cents if present
    if cents.starts_with('0') {
        cents.remove(0);
    }
    // Append cents
    result + "and " + &cents + "/100"
}

fn round_and_format_number(num: f64) -> String {
    format!("{:.2}", f64::round(num * 100.0) / 100.0)
}

fn split_on_decimal_point(number: String) -> [String; 2] {
    let mut v: [String; 2] = [String::new(), String::new()];
    number
        .split('.')
        .into_iter()
        .enumerate()
        .for_each(|(idx, n)| v[idx] = n.to_owned());
    v
}

fn capitalise_first_letter(mut word: String) -> String {
    if word.is_empty() {
        return "".to_owned();
    }
    word.remove(0).to_uppercase().to_string() + &word
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    // Tests for round_and_format_number()
    #[rstest]
    #[case(123.456, "123.46")] // Case1
    #[case(123.4567, "123.46")]
    #[case(123.4, "123.40")] // Case 3
    #[case(123.056, "123.06")]
    #[case(123.006, "123.01")] // Case 5
    #[case(123.005, "123.01")]
    #[case(123.004, "123.00")] // Case 7
    #[case(123.9999, "124.00")]
    #[case(9_999_999_999_999.99999, "10000000000000.00")]
    fn test_round_and_format_number(#[case] input: f64, #[case] expected: &str) {
        assert_eq!(round_and_format_number(input), expected);
    }

    #[rstest]
    #[case(0.099, true, "Zero and 10/100")] // Case1
    #[case(1.0, true, "One and 0/100")] // Case 2
    #[case(15.04, true, "Fifteen and 4/100")] // Case 3
    #[case(99988389.123, true, // Case 4
        "Ninety-nine million, \
         nine hundred eighty-eight thousand, \
         three hundred eighty-nine and 12/100"
        )]
    #[case(9308120381241.876, true,  // Case 5
        "Nine trillion, \
        three hundred eight billion, \
        one hundred twenty million, \
        three hundred eighty-one thousand, \
        two hundred forty-one and 88/100"
    )]
    #[case(9890984381241.55, true, // Case 6
        "Nine trillion, \
        eight hundred ninety billion, \
        nine hundred eighty-four million, \
        three hundred eighty-one thousand, \
        two hundred forty-one and 55/100"
    )]
    #[case(9_999_999_999_999.0100, // Case 7
        true,
        "Nine trillion, \
        nine hundred ninety-nine billion, \
        nine hundred ninety-nine million, \
        nine hundred ninety-nine thousand, \
        nine hundred ninety-nine and 1/100"
    )]
    #[case(999_999_999_999.9999, true, "One trillion and 0/100")] // Case 8
    #[case(9_999_999_999_999.09999, true, "Nine trillion, nine hundred ninety-nine billion, nine hundred ninety-nine million, nine hundred ninety-nine thousand, nine hundred ninety-nine and 10/100")] // Case 9
    #[case(9_999_999_999_999.989, true, "Nine trillion, nine hundred ninety-nine billion, nine hundred ninety-nine million, nine hundred ninety-nine thousand, nine hundred ninety-nine and 99/100")] // Case 10
    #[case(9_999_999_999_999.99, true, // Case 11
        "Nine trillion, \
        nine hundred ninety-nine billion, \
        nine hundred ninety-nine million, \
        nine hundred ninety-nine thousand, \
        nine hundred ninety-nine and 99/100"
    )]
    fn test_float_inputs(#[case] input: f64, #[case] capitalise: bool, #[case] expected: &str) {
        assert_eq!(number_to_words(input, capitalise), expected);
    }

    #[rstest]
    #[case(1, false, "one and 0/100")]
    #[case(15, false, "fifteen and 0/100")]
    #[case(1266, false, "one thousand, two hundred sixty-six and 0/100")]
    #[case(
        1230812,
        false,
        "one million, two hundred thirty thousand, eight hundred twelve and 0/100"
    )]
    #[case(99988389,
        false,
        "ninety-nine million, nine hundred eighty-eight thousand, three hundred eighty-nine and 0/100"
    )]
    fn test_signed_integer_inputs(
        #[case] input: i32,
        #[case] capitalise: bool,
        #[case] expected: &str,
    ) {
        assert_eq!(number_to_words(input, capitalise), expected);
    }

    #[rstest]
    #[case(1, true, "One and 0/100")]
    #[case(15, true, "Fifteen and 0/100"
    )]
    #[case(1266, true, 
        "One thousand, \
        two hundred sixty-six and 0/100"
    )]
    #[case(
        1230812,
        true,
        "One million, \
        two hundred thirty thousand, \
        eight hundred twelve and 0/100"
    )]
    #[case(99988389,
        true,
        "Ninety-nine million, \
        nine hundred eighty-eight thousand, \
        three hundred eighty-nine and 0/100"
    )]
    fn test_unsigned_integer_inputs(
        #[case] input: u32,
        #[case] capitalise: bool,
        #[case] expected: &str,
    ) {
        assert_eq!(number_to_words(input, capitalise), expected);
    }

    // Tests for split_on_decimal_point()
    #[rstest]
    #[case("0.0", ["0", "0"])]
    #[case("0.00", ["0", "00"])]
    #[case("1.0", ["1", "0"])]
    #[case("1.1", ["1", "1"])]
    #[case("99.999", ["99", "999"])] // Case 5
    #[case("000.0", ["000", "0"])]
    #[case("9999999999.99", ["9999999999", "99"])]
    #[case("1.", ["1", ""])]
    #[case(".", ["", ""])]
    #[case("", ["", ""])] // Case 10
    fn splitting_test(#[case] input: String, #[case] expected: [&str; 2]) {
        assert_eq!(split_on_decimal_point(input), expected);
    }

    // Tests for capitalise_first_word()
    #[rstest]
    #[case("one and", "One and")]
    #[case("fifteen and 4/100", "Fifteen and 4/100")]
    #[case("ninety", "Ninety")]
    #[case("12345", "12345")]
    #[case("tWELVE", "TWELVE")]
    #[case("$banana", "$banana")]
    #[case("", "")]

    fn test_capitalisation(#[case] input: String, #[case] expected: String) {
        assert_eq!(capitalise_first_letter(input), expected);
    }
}
    