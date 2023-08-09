// Konsep OOP di Rust/Go beda dengan C++ atau Python
// https://doc.rust-lang.org/book/ch17-01-what-is-oo.html
// https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html
// https://users.rust-lang.org/t/why-doesn-t-rust-allow-fields-in-trait-ii/84410
// https://stackoverflow.com/q/32552593/is-it-possible-for-one-struct-to-extend-an-existing-struct

use duplicate::duplicate_item;

struct CommonCar {
    model: String,
    speed_kmh: u32,
    price: u32,
    stock: i32
}

struct Bus {
    car: CommonCar,
    length_m: f32,
    capacity: i32
}

trait CarTrait {
    fn get_model(&self) -> String;
    fn set_model(&mut self, model: String);

    fn get_speed(&self) -> u32;

    fn get_price(&self) -> u32;

    fn get_stock(&self) -> i32;
    fn add_stock(&mut self, num: i32);

    fn print_info(&self) {
        println!("{} mempunyai harga Rp {} dan kecepatan {} km/jam (stok saat ini: {})",
            self.get_model(), self.get_price(), self.get_speed(), self.get_stock()
        )
    }
}

impl CarTrait for Car {
    fn get_model(&self) -> String {
        self.model.to_string()
    }

    fn set_model(&mut self, model: String) {
        self.model = model;
    }

    fn get_speed(&self) -> u32 {
        self.speed_kmh
    }

    fn get_price(&self) -> u32 {
        self.price
    }

    fn get_stock(&self) -> i32 {
        self.stock
    }

    fn add_stock(&mut self, num: i32) {
        if self.stock - num >= 0 {
            self.stock = self.stock + num;
        } else {
            panic!("Negative stock!");
        }
    }
}

fn main() {

}