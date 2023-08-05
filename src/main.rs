// Hilangkan warning bagian kode tidak terpakai
#![allow(dead_code,unused_imports)]

mod test_io;
mod test_loop;

fn main() {
    // test_io::ask_name();
    // test_io::odd_even();
    // test_io::guess_number();

    test_loop::prime_numbers(100);
    println!("{}", test_loop::square_root(64));
    println!("{}", test_loop::decimal_to_binary(138));
    println!("{}", test_loop::reverse_word("saya makan ikan".to_string()));
}