use std::cmp;

pub fn square_root(num: i32) -> i32 {
    // Dari 0 sampai tak terhingga
    for i in 0.. {
        // Return di awal (pakai keyword)
        if i * i == num { return i; }
        else if i * i > num { break; }
    }
    // Return di akhir (tanpa keyword)
    -1
}

pub fn prime_numbers(max_num: i32) {
    // Dari 2 sampai max_num (max_num termasuk)
    for i in 2..=max_num {
        let mut prime = true;
        // Dari 2 sampai i (i tidak termasuk)
        for j in 2..i {
            if i % j == 0 {
                prime = false;
                break;
            }
        }
        // Lebih efisien bila langsung print
        if prime { print!("{i} "); }
    }
    println!();
}

pub fn decimal_to_binary(mut dec: u32) -> String {
    let mut bin = String::new();

    while dec > 0 {
        // Konversi angka ke string
        let temp = (dec % 2).to_string();
        bin.insert_str(0, &temp);        
        dec /= 2;
    }

    bin
}

pub fn reverse_word(word: &str) -> String {
    let mut rev_word = String::new();

    // Loop kata dari belakang (reverse)
    for s in word.chars().rev() {
        rev_word.push(s);
    }

    rev_word
}

// pub fn bubble_sort(num_arr: &mut [i32]) {
//     for i in 0..num_arr.len() {
//         for j in i..num_arr.len() {
//             // Urutkan dari terkecil ke terbesar
//             if num_arr[i] > num_arr[j] {
//                 let temp = num_arr[i];
//                 num_arr[i] = num_arr[j];
//                 num_arr[j] = temp;
//             }
//         }
//     }
// }

// T = bisa semua tipe, bukan hanya i32
// https://doc.rust-lang.org/book/ch10-00-generics.html
// https://doc.rust-lang.org/reference/types/slice.html

// <T: cmp::Ord> untuk membatasi tipe T ke tipe yang bisa dibandingkan (int/char)
// Woarkaround karena operator > tidak akan berfungsi bila tipe T tidak dibatasi

pub fn bubble_sort<T: cmp::Ord>(num_arr: &mut [T]) {
    for i in 0..num_arr.len() {
        for j in i..num_arr.len() {
            // Urutkan dari terkecil ke terbesar
            // TODO: Fix reference error tanpa swap?
            if num_arr[i] > num_arr[j] {
                num_arr.swap(i, j);
            }
        }
    }
}