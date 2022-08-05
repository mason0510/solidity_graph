//define structs and functions for the utils module
//name

//define struct name CeloPrice
pub struct CeloPrice {
    pub price: f64,
    pub timestamp: i64,
}

//others
impl CeloPrice {
    pub fn new(price: f64, timestamp: i64) -> Self {
        CeloPrice {
            price,
            timestamp,
        }
    }
}
