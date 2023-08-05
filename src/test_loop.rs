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

pub fn reverse_word(word: String) -> String {
    let mut rev_word = String::new();

    // Loop kata dari belakang (reverse)
    for s in word.chars().rev() {
        rev_word.push(s);
    }

    rev_word
}