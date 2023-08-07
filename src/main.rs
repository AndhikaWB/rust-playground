// Hilangkan warning bagian kode tidak terpakai
#![allow(dead_code,unused_imports)]

mod test_io;
mod test_loop;
// mod test_oop;

fn main() {
    // test_io::ask_name();
    // test_io::odd_even();
    // test_io::guess_number();
    // test_io::calculator();

    test_loop::prime_numbers(100);
    println!("{}", test_loop::square_root(64));
    println!("{}", test_loop::decimal_to_binary(138));
    println!("{}", test_loop::reverse_word("saya makan ikan"));

    // Tes pass by reference (bukan by value)
    let mut num_arr = [3,1,7,5,2,8,10,11,6];
    test_loop::bubble_sort(&mut num_arr);
    // https://doc.rust-lang.org/std/fmt/#formatting-traits
    println!("{:?}", num_arr);
}