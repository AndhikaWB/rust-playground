use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

pub fn ask_name() {
    print!("Masukkan nama Anda: ");
    // Langsung tampilkan output print sebelum ke stdin (tanpa tunggu newline)
    // https://doc.rust-lang.org/std/macro.print.html
    io::stdout().flush().expect("Flush failed!");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Readline failed!");
    // Hilangkan newline dari stdin
    name = name.trim_end().to_string();

    println!("Halo {name}!");
}

pub fn odd_even() {
    print!("Masukkan angka: ");
    io::stdout().flush().expect("Flush failed!");

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Readline failed!");
    // Konversi ke unsigned int dan tampilkan error bila bukan angka
    // https://doc.rust-lang.org/rust-by-example/primitives.html
    let num: u32 = num.trim_end().parse().expect("Not a number!");

    if num % 2 == 0 { println!("{num} merupakan bilangan genap") }
    else { println!("{num} merupakan bilangan ganjil") }
}

pub fn guess_number() {
    // Acak angka dengan rentang tertentu
    let secret_num = rand::thread_rng().gen_range(1..=20);
    println!("Tebak nomor dari 1-20! (jawaban: {secret_num})");

    loop {
        print!("Masukkan tebakan Anda: ");
        io::stdout().flush().expect("Flush failed!");

        let mut guess_num = String::new();
        io::stdin().read_line(&mut guess_num).expect("Readline failed!");
        // Konversi ke unsigned integer dan input ulang bila error
        let guess_num: u32 = match guess_num.trim_end().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // Bisa juga menggunakan if else biasa
        match guess_num.cmp(&secret_num) {
            Ordering::Greater => println!("Terlalu besar!"),
            Ordering::Less => println!("Terlalu kecil!"),
            Ordering::Equal => {
                println!("Anda benar!");
                break;
            }
        }
    }
}