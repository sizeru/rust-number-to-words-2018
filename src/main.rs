use number_to_text::number_to_words;

fn main() {
    // test here
    let test = 2345355.736;
    println!("{}", number_to_words(test, true));
    println!("{}", number_to_words(test, false));
    let max_test: f64 = 9.999999999999E65;
    println!("{}", max_test);
    println!("{}", number_to_words(max_test, true));
    println!("{}", number_to_words(max_test, false));
}
