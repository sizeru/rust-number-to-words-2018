use number_to_text::number_to_words;

fn main() {
    // test here
    let test = 2345355.736;
    println!("{}", number_to_words(test, true));
    println!("{}", number_to_words(test, false));
}
