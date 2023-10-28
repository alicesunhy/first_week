pub fn print_char(a: char, b: char) {
    let a_value = a as u32;
    let b_value = b as u32;
    let max = a_value.max(b_value);
    let min = a_value.min(b_value);

    for n in min..=max {
        let value: Option<char> = char::from_u32(n);
        match value {
            Some(p) => println!("char value : {p}"),
            None => println!("char no value"),
        }
    }
}
