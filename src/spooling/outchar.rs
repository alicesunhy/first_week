pub fn print_char(a: char, b: char) {
    for value in a..=b {
        println!("{value}");
    }
}

pub fn print_char_rev(a: char, b: char) {
    for value in (b..=a).rev() {
        println!("{value}");
    }
}
