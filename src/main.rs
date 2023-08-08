// Hilangkan warning bagian kode tidak terpakai
#![allow(dead_code,unused_imports)]

mod test_io;
mod test_loop;
mod test_oop;

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

    // ========== OOP

    let mut car_dealer = test_oop::CarDealer::new();
    let car1 = test_oop::Car {
        model: String::from("Car BG-301"),
        speed_kmh: 180,
        price: 60000000,
        stock: 5
    };
    let car2 = test_oop::Bus {
        model: String::from("Bus AC-112"),
        speed_kmh: 120,
        price: 125000000,
        stock: 3,
        length_m: 15.5,
        capacity: 40
    };

    car_dealer.list_cars();
    car_dealer.add_new_car(Box::new(car1));
    car_dealer.add_new_car(Box::new(car2));
    car_dealer.list_cars();

    // Akan gagal dieksekusi karena "ref_car" akan menjadi null setelah dihapus
    // Rust bersifat type safe sehingga tidak boleh ada null exception
    // https://stackoverflow.com/q/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable
    // let ref_car = car_dealer.find_car_by_model(
    //     String::from("Bus AC-112")).expect("Car not found!");
    // car_dealer.delete_car_by_ref(ref_car);

    // Alternatifnya menggabungkan kedua fungsi menjadi 1
    car_dealer.delete_car_by_model(String::from("Bus AC-112"));
    car_dealer.list_cars();
}