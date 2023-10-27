pub mod spooling;
fn main() {
    my_mod::one();
    my_mod::two_md::two();
}

mod my_mod {
    use crate::spooling::outchar;
    pub fn one() {
        println!("'a'~'Z' 之间的所有字符");
        outchar::print_char('a', 'Z');
    }

    pub mod two_md {
        use crate::spooling::outchar;
        pub fn two() {
            println!("'A'~'a' 之间的所有字符");
            outchar::print_char('A', 'z');
        }
    }
}
