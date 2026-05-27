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
    pub suspended: Arc<Mutex<HashMap<String, SuspendedCart>>>,
    pub fiscal: Arc<Mutex<FiscalConfig>>,
    pub fiscal_receipts: Arc<Mutex<Vec<FiscalReceipt>>>,
    pub roles: Arc<Mutex<Vec<RoleLimit>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            products: Arc::new(Mutex::new(HashMap::new())),
            carts: Arc::new(Mutex::new(HashMap::new())),
            payments: Arc::new(Mutex::new(Vec::new())),
            shifts: Arc::new(Mutex::new(HashMap::new())),
            loyalty: Arc::new(Mutex::new(HashMap::new())),
            suspended: Arc::new(Mutex::new(HashMap::new())),
            fiscal: Arc::new(Mutex::new(FiscalConfig {
                country: "KE".into(), tax_regime: "vat".into(), device_id: None,
                business_name: "My Store".into(), business_pin: None, currency: "KES".into(),
                smallest_denomination: 1.0, receipt_prefix: "RCP".into(), next_receipt_number: 1,
            })),
            fiscal_receipts: Arc::new(Mutex::new(Vec::new())),
            roles: Arc::new(Mutex::new(vec![
                RoleLimit { role: "cashier".into(), max_discount_pct: 5.0, can_void: false, can_refund: false, can_price_override: false, max_refund_amount: None },
                RoleLimit { role: "supervisor".into(), max_discount_pct: 20.0, can_void: true, can_refund: true, can_price_override: true, max_refund_amount: Some(50000.0) },
                RoleLimit { role: "manager".into(), max_discount_pct: 100.0, can_void: true, can_refund: true, can_price_override: true, max_refund_amount: None },
            ])),
        }
    }

    pub fn lookup_barcode(&self, barcode: &str) -> Option<Product> {
        self.products.lock().unwrap().values().find(|p| p.barcode == barcode || p.sku == barcode).cloned()
    }
}
