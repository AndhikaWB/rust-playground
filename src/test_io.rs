use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

// Fungsi non-public, hanya bisa dipakai di file yang sama
fn asker(question: &str) -> String {
    print!("{question}");
    // Langsung tampilkan output print sebelum ke stdin (tanpa tunggu newline)
    // https://doc.rust-lang.org/std/macro.print.html
    io::stdout().flush().expect("Flush failed!");

    let mut answer = String::new();
    // Hilangkan newline dari stdin
    io::stdin().read_line(&mut answer).expect("Readline failed!");

    // Return string (bisa juga menggunakan String::from)
    answer.trim_end().to_string()
}

pub fn ask_name() {
    let name = asker("Masukkan nama Anda: ");
    println!("Halo {name}!");
}

pub fn odd_even() {
    // Konversi ke unsigned int dan tampilkan error bila bukan angka
    // https://doc.rust-lang.org/rust-by-example/primitives.html
    let num: u32 = asker("Masukkan angka: ").parse().expect("Not a number!");

    if num % 2 == 0 {
        println!("{num} merupakan bilangan genap");
    } else {
        println!("{num} merupakan bilangan ganjil");
    }
}

pub fn guess_number() {
    // Acak angka dengan rentang tertentu
    let secret_num = rand::thread_rng().gen_range(1..=20);
    println!("Tebak nomor dari 1-20! (jawaban: {secret_num})");

    // Loop tak terbatas
    loop {
        // Konversi ke unsigned int dan input ulang bila error
        let guess_num: u32 = match asker("Masukkan tebakan Anda: ").parse() {
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

pub fn calculator() {
    let num1: i32 = asker("Masukkan angka pertama: ").parse().expect("Not a number!");
    let num2: i32 = asker("Masukkan angka kedua: ").parse().expect("Not a number!");

    let oper = asker("Masukkan operator (+ - * /): ");
    match oper.trim_end() {
        "+" => println!("{num1} ditambah {num2} adalah {}", num1 + num2),
        "-" => println!("{num1} dikurang {num2} adalah {}", num1 - num2),
        "*" => println!("{num1} dikali {num2} adalah {}", num1 * num2),
        "/" => println!("{num1} dibagi {num2} adalah {}", num1 as f32 / num2 as f32),
        _ => println!("Operator tidak dikenali!")
    }
}