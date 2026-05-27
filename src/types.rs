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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FiscalConfig {
    pub country: String,
    pub tax_regime: String, // vat, gst, sales_tax
    pub device_id: Option<String>, // ETR control unit, TSE ID
    pub business_name: String,
    pub business_pin: Option<String>, // KRA PIN, GSTIN, VAT number
    pub currency: String,
    pub smallest_denomination: f64, // rounding (e.g. 1.0 for KES, 0.01 for USD)
    pub receipt_prefix: String,
    pub next_receipt_number: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuspendedCart {
    pub cart: Cart,
    pub suspended_at: String,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleLimit {
    pub role: String,
    pub max_discount_pct: f64,
    pub can_void: bool,
    pub can_refund: bool,
    pub can_price_override: bool,
    pub max_refund_amount: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FiscalReceipt {
    pub receipt_number: u64,
    pub cart_id: String,
    pub hash: String, // SHA-256 chain
    pub previous_hash: String,
    pub timestamp: String,
    pub device_id: Option<String>,
    pub is_copy: bool,
}
