use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub sku: String,
    pub barcode: String,
    pub name: String,
    pub price: f64,
    pub tax_rate: f64,
    pub category: String,
    pub currency: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CartItem {
    pub sku: String,
    pub name: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub discount: f64,
    pub tax: f64,
    pub line_total: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cart {
    pub id: String,
    pub status: String, // open, checked_out, voided
    pub items: Vec<CartItem>,
    pub subtotal: f64,
    pub total_discount: f64,
    pub total_tax: f64,
    pub total: f64,
    pub currency: String,
    pub customer_id: Option<String>,
    pub cashier: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub cart_id: String,
    pub method: String, // cash, card, mobile_money, split
    pub amount: f64,
    pub tendered: f64,
    pub change: f64,
    pub reference: Option<String>,
    pub status: String, // completed, failed, refunded
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shift {
    pub id: String,
    pub cashier: String,
    pub register_id: String,
    pub status: String, // open, closed
    pub opening_float: f64,
    pub cash_sales: f64,
    pub card_sales: f64,
    pub mobile_sales: f64,
    pub refunds: f64,
    pub transactions: u32,
    pub opened_at: String,
    pub closed_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoyaltyAccount {
    pub customer_id: String,
    pub name: String,
    pub points: u64,
    pub tier: String, // bronze, silver, gold, platinum
    pub total_spend: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub cart_id: String,
    pub lines: Vec<String>,
    pub generated_at: String,
}
