struct Car<T: CarTrait> {
    model: String,
    max_speed_kmh: u32,
    price: u32,
    stock: u32
}

trait CarTrait {
    fn add_stock(&self);
    fn check_stock(&self);
    fn sell(&self);
    fn drive_km(&self);
    fn lock(&self);
}

