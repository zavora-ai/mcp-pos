use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::types::*;

#[derive(Clone)]
pub struct Store {
    pub products: Arc<Mutex<HashMap<String, Product>>>,
    pub carts: Arc<Mutex<HashMap<String, Cart>>>,
    pub payments: Arc<Mutex<Vec<Payment>>>,
    pub shifts: Arc<Mutex<HashMap<String, Shift>>>,
    pub loyalty: Arc<Mutex<HashMap<String, LoyaltyAccount>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(HashMap::new())),
            carts: Arc::new(Mutex::new(HashMap::new())),
            payments: Arc::new(Mutex::new(Vec::new())),
            shifts: Arc::new(Mutex::new(HashMap::new())),
            loyalty: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn lookup_barcode(&self, barcode: &str) -> Option<Product> {
        self.products.lock().unwrap().values().find(|p| p.barcode == barcode || p.sku == barcode).cloned()
    }
}
