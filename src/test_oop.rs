// Konsep OOP di Rust/Go beda dengan C++ atau Python
// https://doc.rust-lang.org/book/ch17-01-what-is-oo.html
// https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html
// https://users.rust-lang.org/t/why-doesn-t-rust-allow-fields-in-trait-ii/84410
// https://stackoverflow.com/q/32552593/one-struct-to-extend-an-existing-struct

use duplicate::duplicate_item;
use std::ptr::eq;

pub struct Car {
    pub model: String,
    pub speed_kmh: u32,
    pub price: u32,
    pub stock: i32
}

pub struct Bus {
    pub model: String,
    pub speed_kmh: u32,
    pub price: u32,
    pub stock: i32,

    pub length_m: f32,
    pub capacity: u32,
}

// Field/variabel pada struct tidak bisa diakses langsung saat deklarasi trait
// Oleh karena itu harus diimplementasi kembali (lihat bagian selanjutnya)

pub trait CommonCarTrait {
    fn get_model(&self) -> String;
    fn set_model(&mut self, model: String);

    fn get_speed(&self) -> u32;

    fn get_price(&self) -> u32;

    fn get_stock(&self) -> i32;
    fn add_stock(&mut self, num: i32);

    fn honk(&self);

    fn print_info(&self) {
        println!("{} mempunyai harga Rp {} (stok saat ini: {})",
            self.get_model(), self.get_price(), self.get_stock()
        )
    }
}

// Workaround untuk implementasi trait di banyak struct sekaligus
// https://stackoverflow.com/q/39150216/implementing-a-trait-for-multiple-types-at-once

#[duplicate_item(
    struct_name horn_sound;
    [Car] ["Beep Beep!"];
    [Bus] ["Hoot Hoot!"];
)]
impl CommonCarTrait for struct_name {
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

    fn honk(&self) {
        println!("{}", horn_sound);
    }
}

// Implementasi fungsi yang hanya ada di bus

impl Bus {
    fn get_length(&self) -> f32 {
        self.length_m
    }

    fn get_capacity(&self) -> u32 {
        self.capacity
    }
}

// Membuat list untuk objek-objek dengan trait yang sama
// https://doc.rust-lang.org/book/ch17-02-trait-objects.html
// https://stackoverflow.com/q/26212397/specify-a-struct-field-that-implement-a-trait
// https://stackoverflow.com/q/25818082/vector-of-objects-belonging-to-a-trait

pub struct CarDealer {
    list: Vec<Box<dyn CommonCarTrait>>
}

impl CarDealer {
    // Konstruktor
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add_new_car(&mut self, car: Box<dyn CommonCarTrait>) {
        println!("Car added!");
        self.list.push(car);
    }

    pub fn list_cars(&self) {
        // i = index, val = value
        for (i, val) in self.list.iter().enumerate() {
            print!("{}. ", i + 1);
            val.print_info();
        }
    }

    pub fn find_car_by_model(&self, model: String) -> Option<&Box<dyn CommonCarTrait>> {
        // None = bila tidak ketemu, Some = bila ketemu
        let mut car = None;
        for i in self.list.iter() {
            if i.get_model() == model {
                println!("Car found!");
                car = Some(i);
                break;
            }
        }

        car
    }

    pub fn delete_car_by_ref(&mut self, ref_car: &Box<dyn CommonCarTrait>) {
        if let Some(pos) = self.list.iter().position(|x| eq(x, ref_car)) {
            println!("Car removed!");
            self.list.remove(pos);
        }
    }

    // Workaround untuk kedua fungsi di atas
    pub fn delete_car_by_model(&mut self, model: String) {
        if let Some(pos) = self.list.iter().position(|x| x.get_model() == model) {
            println!("Car removed!");
            self.list.remove(pos);
        }
    }
}