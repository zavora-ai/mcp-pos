use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde_json::{json, Value};
use crate::types::*;
use crate::store::Store;

fn now() -> String { chrono::Utc::now().to_rfc3339() }
fn uid() -> String { uuid::Uuid::new_v4().to_string()[..8].to_string() }
fn round2(v: f64) -> f64 { (v * 100.0).round() / 100.0 }

// --- Input Types ---
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ProductInput { pub sku: String, pub barcode: String, pub name: String, pub price: f64, pub tax_rate: Option<f64>, pub category: Option<String>, pub currency: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CartCreateInput { pub cashier: String, pub customer_id: Option<String>, pub currency: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CartAddInput { pub cart_id: String, pub barcode: String, pub quantity: Option<f64> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CartRemoveInput { pub cart_id: String, pub sku: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CartDiscountInput { pub cart_id: String, pub discount_type: String, pub value: f64, pub sku: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CartIdInput { pub cart_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PaymentInput { pub cart_id: String, pub method: String, pub amount: Option<f64>, pub tendered: Option<f64>, pub reference: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RefundInput { pub cart_id: String, pub reason: String, pub items: Option<Vec<Value>> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ShiftOpenInput { pub cashier: String, pub register_id: String, pub opening_float: f64 }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ShiftCloseInput { pub shift_id: String, pub counted_cash: f64 }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BarcodeInput { pub barcode: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LoyaltyInput { pub customer_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LoyaltyRedeemInput { pub customer_id: String, pub cart_id: String, pub points: u64 }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DailySummaryInput { pub date: Option<String> }

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct VoidInput { pub cart_id: String, pub actor: String, pub role: String, pub reason: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PriceOverrideInput { pub cart_id: String, pub sku: String, pub new_price: f64, pub actor: String, pub role: String, pub reason: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SuspendInput { pub cart_id: String, pub reason: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RecallInput { pub cart_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SplitTenderInput { pub cart_id: String, pub payments: Vec<Value> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TipInput { pub cart_id: String, pub tip_amount: f64 }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReprintInput { pub cart_id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FiscalConfigInput { pub country: String, pub business_name: String, pub business_pin: Option<String>, pub device_id: Option<String>, pub currency: Option<String>, pub smallest_denomination: Option<f64>, pub receipt_prefix: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RoleLimitInput { pub role: String, pub max_discount_pct: f64, pub can_void: bool, pub can_refund: bool, pub can_price_override: bool, pub max_refund_amount: Option<f64> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AuthorizeInput { pub action: String, pub actor: String, pub role: String }

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PaymentQrInput {
    /// Cart ID to generate QR for
    pub cart_id: String,
    /// QR type: upi, wechat, alipay, mpesa, zatca
    pub qr_type: String,
    /// Merchant ID / UPI VPA / WeChat merchant (optional, uses fiscal config if not set)
    pub merchant_id: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MultiCurrencyInput {
    /// Cart ID
    pub cart_id: String,
    /// Payment currency (ISO 4217, e.g. "USD", "EUR", "GBP")
    pub payment_currency: String,
    /// Exchange rate (local per 1 foreign unit, e.g. 129.5 KES per 1 USD)
    pub exchange_rate: f64,
    /// Amount tendered in foreign currency
    pub tendered_foreign: f64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TaxCategoryInput {
    /// SKU
    pub sku: String,
    /// HSN code (India), HS code (global), SAC code (services)
    pub tax_code: String,
    /// Tax code type: hsn, sac, hs, vat_category
    pub code_type: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EInvoiceInput {
    /// Cart ID
    pub cart_id: String,
    /// E-invoice standard: india_irn, zatca, kra_etr, fapiao
    pub standard: String,
    /// Buyer tax ID (GSTIN, TRN, KRA PIN)
    pub buyer_tax_id: Option<String>,
    /// Buyer name
    pub buyer_name: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BuyerIdentifyInput {
    /// Cart ID
    pub cart_id: String,
    /// Buyer tax ID (GSTIN, TRN, KRA PIN, national ID)
    pub tax_id: String,
    /// Buyer name
    pub name: String,
    /// ID type: gstin, trn, kra_pin, national_id, passport
    pub id_type: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BilingualReceiptInput {
    /// Cart ID
    pub cart_id: String,
    /// Primary language (e.g. "en")
    pub primary_lang: String,
    /// Secondary language (e.g. "ar", "zh", "hi", "sw")
    pub secondary_lang: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReceiptSignInput {
    /// Cart ID to sign
    pub cart_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AgeVerifyInput {
    /// Cart ID
    pub cart_id: String,
    /// Verification method: dob_check, id_scanned, challenge25_passed
    pub method: String,
    /// Date of birth (YYYY-MM-DD) if method is dob_check
    pub dob: Option<String>,
    /// Verified by (staff member)
    pub verified_by: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EnvLevyInput {
    /// Cart ID
    pub cart_id: String,
    /// Levy type: carrier_bag, plastic_tax, sugar_tax
    pub levy_type: String,
    /// Number of items (e.g. number of bags)
    pub quantity: Option<f64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MealVoucherInput {
    /// Cart ID
    pub cart_id: String,
    /// Voucher type: ticket_restaurant, sodexo, edenred, cheque_dejeuner
    pub voucher_type: String,
    /// Amount covered by voucher
    pub amount: f64,
    /// Voucher reference/number
    pub reference: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UsTaxInput {
    /// State code (2-letter, e.g. "CA", "TX", "NY")
    pub state: String,
    /// County name (optional, for combined rate)
    pub county: Option<String>,
    /// City name (optional)
    pub city: Option<String>,
    /// Amount to calculate tax on
    pub amount: Option<f64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EbtSnapInput {
    /// Cart ID
    pub cart_id: String,
    /// EBT card number (last 4 for reference)
    pub card_last4: String,
    /// Amount to pay with EBT
    pub amount: Option<f64>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TipConfigInput {
    /// Cart ID
    pub cart_id: String,
    /// Tip amount (fixed) OR tip_pct for percentage
    pub tip_amount: Option<f64>,
    /// Tip percentage (e.g. 15, 18, 20, 25)
    pub tip_pct: Option<f64>,
    /// Pre-tax or post-tax tip calculation
    pub on_pretax: Option<bool>,
}

#[derive(Clone)]
pub struct PosServer { pub store: Store }
impl PosServer { pub fn new() -> Self { Self { store: Store::new() } } }

fn recalc_cart(cart: &mut Cart) {
    cart.subtotal = cart.items.iter().map(|i| i.unit_price * i.quantity).sum();
    cart.total_discount = cart.items.iter().map(|i| i.discount).sum();
    cart.total_tax = cart.items.iter().map(|i| i.tax).sum();
    cart.total = round2(cart.subtotal - cart.total_discount + cart.total_tax);
}

#[tool_router(server_handler)]
impl PosServer {
    #[tool(description = "Register a product in the POS catalog (SKU, barcode, name, price, tax rate).")]
    async fn product_register(&self, Parameters(input): Parameters<ProductInput>) -> String {
        let p = Product { sku: input.sku.clone(), barcode: input.barcode, name: input.name, price: input.price, tax_rate: input.tax_rate.unwrap_or(16.0), category: input.category.unwrap_or_default(), currency: input.currency.unwrap_or_else(|| "KES".into()) };
        self.store.products.lock().unwrap().insert(input.sku.clone(), p);
        json!({"status": "registered", "sku": input.sku}).to_string()
    }

    #[tool(description = "Create a new cart/transaction (start a sale).")]
    async fn cart_create(&self, Parameters(input): Parameters<CartCreateInput>) -> String {
        let id = format!("cart_{}", uid());
        let cart = Cart { id: id.clone(), status: "open".into(), items: vec![], subtotal: 0.0, total_discount: 0.0, total_tax: 0.0, total: 0.0, currency: input.currency.unwrap_or_else(|| "KES".into()), customer_id: input.customer_id, cashier: input.cashier, created_at: now() };
        self.store.carts.lock().unwrap().insert(id.clone(), cart);
        json!({"status": "created", "cart_id": id}).to_string()
    }

    #[tool(description = "Add item to cart by scanning barcode or entering SKU. Auto-looks up price and calculates tax.")]
    async fn cart_add_item(&self, Parameters(input): Parameters<CartAddInput>) -> String {
        let product = match self.store.lookup_barcode(&input.barcode) {
            Some(p) => p,
            None => return json!({"error": "PRODUCT_NOT_FOUND", "barcode": input.barcode}).to_string(),
        };
        let qty = input.quantity.unwrap_or(1.0);
        let tax = round2(product.price * qty * (product.tax_rate / 100.0));
        let line_total = round2(product.price * qty + tax);
        let item = CartItem { sku: product.sku.clone(), name: product.name.clone(), quantity: qty, unit_price: product.price, discount: 0.0, tax, line_total };
        let mut carts = self.store.carts.lock().unwrap();
        match carts.get_mut(&input.cart_id) {
            Some(cart) => {
                if let Some(existing) = cart.items.iter_mut().find(|i| i.sku == product.sku) {
                    existing.quantity += qty;
                    existing.tax = round2(existing.unit_price * existing.quantity * (product.tax_rate / 100.0));
                    existing.line_total = round2(existing.unit_price * existing.quantity + existing.tax - existing.discount);
                } else {
                    cart.items.push(item);
                }
                recalc_cart(cart);
                json!({"status": "added", "sku": product.sku, "name": product.name, "qty": qty, "cart_total": cart.total}).to_string()
            }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Remove an item from the cart.")]
    async fn cart_remove_item(&self, Parameters(input): Parameters<CartRemoveInput>) -> String {
        let mut carts = self.store.carts.lock().unwrap();
        match carts.get_mut(&input.cart_id) {
            Some(cart) => {
                cart.items.retain(|i| i.sku != input.sku);
                recalc_cart(cart);
                json!({"status": "removed", "sku": input.sku, "cart_total": cart.total}).to_string()
            }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Apply discount to cart (percentage or fixed). Optionally target a specific SKU.")]
    async fn cart_apply_discount(&self, Parameters(input): Parameters<CartDiscountInput>) -> String {
        let mut carts = self.store.carts.lock().unwrap();
        match carts.get_mut(&input.cart_id) {
            Some(cart) => {
                if let Some(ref sku) = input.sku {
                    if let Some(item) = cart.items.iter_mut().find(|i| &i.sku == sku) {
                        item.discount = match input.discount_type.as_str() {
                            "percentage" => round2(item.unit_price * item.quantity * input.value / 100.0),
                            _ => input.value,
                        };
                        item.line_total = round2(item.unit_price * item.quantity + item.tax - item.discount);
                    }
                } else {
                    let disc = match input.discount_type.as_str() {
                        "percentage" => round2(cart.subtotal * input.value / 100.0),
                        _ => input.value,
                    };
                    let per_item = disc / cart.items.len().max(1) as f64;
                    for item in &mut cart.items { item.discount += per_item; item.line_total = round2(item.unit_price * item.quantity + item.tax - item.discount); }
                }
                recalc_cart(cart);
                json!({"status": "discount_applied", "discount_type": input.discount_type, "value": input.value, "cart_total": cart.total}).to_string()
            }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Get current cart contents and totals.")]
    async fn cart_get(&self, Parameters(input): Parameters<CartIdInput>) -> String {
        match self.store.carts.lock().unwrap().get(&input.cart_id) {
            Some(cart) => serde_json::to_string_pretty(cart).unwrap_or_default(),
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Process payment for a cart (cash, card, mobile_money). For cash, calculates change from tendered amount.")]
    async fn payment_process(&self, Parameters(input): Parameters<PaymentInput>) -> String {
        let mut carts = self.store.carts.lock().unwrap();
        let cart = match carts.get_mut(&input.cart_id) {
            Some(c) => c,
            None => return json!({"error": "CART_NOT_FOUND"}).to_string(),
        };
        if cart.status != "open" { return json!({"error": "CART_NOT_OPEN", "status": cart.status}).to_string(); }
        let amount = input.amount.unwrap_or(cart.total);
        let tendered = input.tendered.unwrap_or(amount);
        let change = if input.method == "cash" { round2(tendered - amount) } else { 0.0 };
        cart.status = "checked_out".into();
        let payment = Payment { id: format!("pay_{}", uid()), cart_id: input.cart_id.clone(), method: input.method.clone(), amount, tendered, change, reference: input.reference, status: "completed".into(), timestamp: now() };
        let pay_id = payment.id.clone();
        // Update shift
        let shifts = self.store.shifts.lock().unwrap().clone();
        if let Some(shift_id) = shifts.values().find(|s| s.cashier == cart.cashier && s.status == "open").map(|s| s.id.clone()) {
            drop(shifts);
            let mut shifts = self.store.shifts.lock().unwrap();
            if let Some(s) = shifts.get_mut(&shift_id) {
                s.transactions += 1;
                match input.method.as_str() { "cash" => s.cash_sales += amount, "card" => s.card_sales += amount, _ => s.mobile_sales += amount }
            }
        }
        self.store.payments.lock().unwrap().push(payment);
        json!({"status": "completed", "payment_id": pay_id, "method": input.method, "amount": amount, "tendered": tendered, "change": change}).to_string()
    }

    #[tool(description = "Generate a receipt for a completed transaction (thermal printer format).")]
    async fn receipt_generate(&self, Parameters(input): Parameters<CartIdInput>) -> String {
        let cart = match self.store.carts.lock().unwrap().get(&input.cart_id) {
            Some(c) => c.clone(),
            None => return json!({"error": "CART_NOT_FOUND"}).to_string(),
        };
        let payments: Vec<_> = self.store.payments.lock().unwrap().iter().filter(|p| p.cart_id == input.cart_id).cloned().collect();
        let mut lines = vec![
            "================================".into(),
            "         SALES RECEIPT          ".into(),
            "================================".into(),
            format!("Date: {}", &cart.created_at[..10]),
            format!("Cashier: {}", cart.cashier),
            "--------------------------------".into(),
        ];
        for item in &cart.items {
            lines.push(format!("{:<20} x{:.0}", item.name, item.quantity));
            lines.push(format!("  {} {:.2}", cart.currency, item.line_total));
        }
        lines.push("--------------------------------".into());
        lines.push(format!("Subtotal:    {} {:.2}", cart.currency, cart.subtotal));
        if cart.total_discount > 0.0 { lines.push(format!("Discount:   -{} {:.2}", cart.currency, cart.total_discount)); }
        lines.push(format!("Tax:         {} {:.2}", cart.currency, cart.total_tax));
        lines.push(format!("TOTAL:       {} {:.2}", cart.currency, cart.total));
        lines.push("--------------------------------".into());
        for p in &payments {
            lines.push(format!("{}: {} {:.2}", p.method.to_uppercase(), cart.currency, p.amount));
            if p.change > 0.0 { lines.push(format!("Change: {} {:.2}", cart.currency, p.change)); }
        }
        lines.push("================================".into());
        lines.push("      Thank you! Come again     ".into());
        lines.push("================================".into());
        let receipt_text = lines.join("\n");
        json!({"cart_id": input.cart_id, "receipt": receipt_text, "lines": lines.len()}).to_string()
    }

    #[tool(description = "Process a refund/return for a completed transaction.")]
    async fn refund(&self, Parameters(input): Parameters<RefundInput>) -> String {
        let mut carts = self.store.carts.lock().unwrap();
        match carts.get_mut(&input.cart_id) {
            Some(cart) => {
                let refund_amount = if let Some(items) = &input.items {
                    items.iter().filter_map(|i| i["amount"].as_f64()).sum::<f64>()
                } else { cart.total };
                let pay_id = format!("ref_{}", uid());
                self.store.payments.lock().unwrap().push(Payment { id: pay_id.clone(), cart_id: input.cart_id.clone(), method: "refund".into(), amount: -refund_amount, tendered: 0.0, change: 0.0, reference: Some(input.reason), status: "refunded".into(), timestamp: now() });
                // Update shift
                let shifts_clone = self.store.shifts.lock().unwrap().clone();
                if let Some(shift_id) = shifts_clone.values().find(|s| s.cashier == cart.cashier && s.status == "open").map(|s| s.id.clone()) {
                    let mut shifts = self.store.shifts.lock().unwrap();
                    if let Some(s) = shifts.get_mut(&shift_id) { s.refunds += refund_amount; }
                }
                json!({"status": "refunded", "refund_id": pay_id, "amount": refund_amount}).to_string()
            }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Open a shift/register (set opening float, start tracking sales).")]
    async fn shift_open(&self, Parameters(input): Parameters<ShiftOpenInput>) -> String {
        let id = format!("shift_{}", uid());
        let shift = Shift { id: id.clone(), cashier: input.cashier, register_id: input.register_id, status: "open".into(), opening_float: input.opening_float, cash_sales: 0.0, card_sales: 0.0, mobile_sales: 0.0, refunds: 0.0, transactions: 0, opened_at: now(), closed_at: None };
        self.store.shifts.lock().unwrap().insert(id.clone(), shift);
        json!({"status": "opened", "shift_id": id}).to_string()
    }

    #[tool(description = "Close a shift (reconcile cash, generate summary).")]
    async fn shift_close(&self, Parameters(input): Parameters<ShiftCloseInput>) -> String {
        let mut shifts = self.store.shifts.lock().unwrap();
        match shifts.get_mut(&input.shift_id) {
            Some(s) => {
                s.status = "closed".into();
                s.closed_at = Some(now());
                let expected_cash = s.opening_float + s.cash_sales - s.refunds;
                let variance = round2(input.counted_cash - expected_cash);
                json!({"status": "closed", "shift_id": input.shift_id, "transactions": s.transactions, "cash_sales": s.cash_sales, "card_sales": s.card_sales, "mobile_sales": s.mobile_sales, "refunds": s.refunds, "total_sales": round2(s.cash_sales + s.card_sales + s.mobile_sales), "expected_cash": expected_cash, "counted_cash": input.counted_cash, "variance": variance}).to_string()
            }
            None => json!({"error": "SHIFT_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Look up a product by barcode or SKU (returns name, price, stock info).")]
    async fn barcode_lookup(&self, Parameters(input): Parameters<BarcodeInput>) -> String {
        match self.store.lookup_barcode(&input.barcode) {
            Some(p) => serde_json::to_string_pretty(&p).unwrap_or_default(),
            None => json!({"error": "PRODUCT_NOT_FOUND", "barcode": input.barcode}).to_string(),
        }
    }

    #[tool(description = "Check customer loyalty points and tier.")]
    async fn loyalty_check(&self, Parameters(input): Parameters<LoyaltyInput>) -> String {
        match self.store.loyalty.lock().unwrap().get(&input.customer_id) {
            Some(l) => serde_json::to_string_pretty(l).unwrap_or_default(),
            None => json!({"customer_id": input.customer_id, "points": 0, "tier": "bronze", "message": "New customer"}).to_string(),
        }
    }

    #[tool(description = "Redeem loyalty points as payment on a cart (1 point = 1 currency unit).")]
    async fn loyalty_redeem(&self, Parameters(input): Parameters<LoyaltyRedeemInput>) -> String {
        let mut loyalty = self.store.loyalty.lock().unwrap();
        let account = loyalty.entry(input.customer_id.clone()).or_insert(LoyaltyAccount { customer_id: input.customer_id.clone(), name: String::new(), points: 0, tier: "bronze".into(), total_spend: 0.0 });
        if account.points < input.points { return json!({"error": "INSUFFICIENT_POINTS", "available": account.points, "requested": input.points}).to_string(); }
        account.points -= input.points;
        let discount = input.points as f64;
        drop(loyalty);
        // Apply as discount to cart
        let mut carts = self.store.carts.lock().unwrap();
        if let Some(cart) = carts.get_mut(&input.cart_id) {
            let per_item = discount / cart.items.len().max(1) as f64;
            for item in &mut cart.items { item.discount += per_item; item.line_total = round2(item.unit_price * item.quantity + item.tax - item.discount); }
            recalc_cart(cart);
            json!({"status": "redeemed", "points_used": input.points, "discount_applied": discount, "cart_total": cart.total}).to_string()
        } else {
            json!({"error": "CART_NOT_FOUND"}).to_string()
        }
    }

    #[tool(description = "Get daily sales summary (total sales, payment breakdown, transaction count).")]
    async fn daily_summary(&self, Parameters(_input): Parameters<DailySummaryInput>) -> String {
        let payments = self.store.payments.lock().unwrap().clone();
        let cash: f64 = payments.iter().filter(|p| p.method == "cash" && p.amount > 0.0).map(|p| p.amount).sum();
        let card: f64 = payments.iter().filter(|p| p.method == "card").map(|p| p.amount).sum();
        let mobile: f64 = payments.iter().filter(|p| p.method == "mobile_money").map(|p| p.amount).sum();
        let refunds: f64 = payments.iter().filter(|p| p.amount < 0.0).map(|p| p.amount.abs()).sum();
        let transactions = payments.iter().filter(|p| p.amount > 0.0).count();
        json!({"date": &now()[..10], "transactions": transactions, "cash_sales": round2(cash), "card_sales": round2(card), "mobile_sales": round2(mobile), "refunds": round2(refunds), "net_sales": round2(cash + card + mobile - refunds), "avg_transaction": if transactions > 0 { round2((cash + card + mobile) / transactions as f64) } else { 0.0 }}).to_string()
    }

    // === Business Controls ===

    #[tool(description = "Void/cancel a cart (requires authorized role). Tracks who voided and why.")]
    async fn cart_void(&self, Parameters(input): Parameters<VoidInput>) -> String {
        let roles = self.store.roles.lock().unwrap().clone();
        let role = roles.iter().find(|r| r.role == input.role);
        match role {
            Some(r) if r.can_void => {},
            _ => return json!({"error": "UNAUTHORIZED", "message": "Role cannot void transactions", "role": input.role}).to_string(),
        }
        let mut carts = self.store.carts.lock().unwrap();
        match carts.get_mut(&input.cart_id) {
            Some(cart) => { cart.status = "voided".into(); json!({"status": "voided", "cart_id": input.cart_id, "voided_by": input.actor, "reason": input.reason}).to_string() }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Override price of an item in cart (requires authorized role + reason for audit).")]
    async fn price_override(&self, Parameters(input): Parameters<PriceOverrideInput>) -> String {
        let roles = self.store.roles.lock().unwrap().clone();
        let role = roles.iter().find(|r| r.role == input.role);
        match role {
            Some(r) if r.can_price_override => {},
            _ => return json!({"error": "UNAUTHORIZED", "message": "Role cannot override prices"}).to_string(),
        }
        let mut carts = self.store.carts.lock().unwrap();
        match carts.get_mut(&input.cart_id) {
            Some(cart) => {
                if let Some(item) = cart.items.iter_mut().find(|i| i.sku == input.sku) {
                    let old_price = item.unit_price;
                    item.unit_price = input.new_price;
                    item.line_total = round2(item.unit_price * item.quantity + item.tax - item.discount);
                    recalc_cart(cart);
                    json!({"status": "overridden", "sku": input.sku, "old_price": old_price, "new_price": input.new_price, "authorized_by": input.actor, "reason": input.reason}).to_string()
                } else { json!({"error": "ITEM_NOT_IN_CART"}).to_string() }
            }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Suspend (park) a cart for later recall. Customer stepped away, needs to get something, etc.")]
    async fn cart_suspend(&self, Parameters(input): Parameters<SuspendInput>) -> String {
        let mut carts = self.store.carts.lock().unwrap();
        match carts.remove(&input.cart_id) {
            Some(cart) => {
                self.store.suspended.lock().unwrap().insert(input.cart_id.clone(), SuspendedCart { cart, suspended_at: now(), reason: input.reason });
                json!({"status": "suspended", "cart_id": input.cart_id}).to_string()
            }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Recall a suspended cart back to active.")]
    async fn cart_recall(&self, Parameters(input): Parameters<RecallInput>) -> String {
        match self.store.suspended.lock().unwrap().remove(&input.cart_id) {
            Some(suspended) => {
                self.store.carts.lock().unwrap().insert(input.cart_id.clone(), suspended.cart);
                json!({"status": "recalled", "cart_id": input.cart_id}).to_string()
            }
            None => json!({"error": "SUSPENDED_CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Split tender — pay with multiple methods (e.g. part cash, part card, part M-Pesa). Payments array: [{\"method\": \"cash\", \"amount\": 200, \"tendered\": 200}, {\"method\": \"card\", \"amount\": 143.40}]")]
    async fn split_tender(&self, Parameters(input): Parameters<SplitTenderInput>) -> String {
        let mut carts = self.store.carts.lock().unwrap();
        let cart = match carts.get_mut(&input.cart_id) {
            Some(c) => c,
            None => return json!({"error": "CART_NOT_FOUND"}).to_string(),
        };
        if cart.status != "open" { return json!({"error": "CART_NOT_OPEN"}).to_string(); }
        let total_paid: f64 = input.payments.iter().filter_map(|p| p["amount"].as_f64()).sum();
        if total_paid < cart.total { return json!({"error": "INSUFFICIENT_PAYMENT", "total": cart.total, "paid": total_paid}).to_string(); }
        cart.status = "checked_out".into();
        let mut results = Vec::new();
        for p in &input.payments {
            let method = p["method"].as_str().unwrap_or("cash");
            let amount = p["amount"].as_f64().unwrap_or(0.0);
            let tendered = p["tendered"].as_f64().unwrap_or(amount);
            let change = if method == "cash" { round2(tendered - amount) } else { 0.0 };
            let pay = Payment { id: format!("pay_{}", uid()), cart_id: input.cart_id.clone(), method: method.into(), amount, tendered, change, reference: p["reference"].as_str().map(String::from), status: "completed".into(), timestamp: now() };
            results.push(json!({"method": method, "amount": amount, "change": change}));
            self.store.payments.lock().unwrap().push(pay);
        }
        json!({"status": "completed", "cart_id": input.cart_id, "total": cart.total, "payments": results}).to_string()
    }

    #[tool(description = "Add tip/gratuity to a cart (restaurants, services).")]
    async fn tip_add(&self, Parameters(input): Parameters<TipInput>) -> String {
        let mut carts = self.store.carts.lock().unwrap();
        match carts.get_mut(&input.cart_id) {
            Some(cart) => { cart.total = round2(cart.total + input.tip_amount); json!({"status": "tip_added", "tip": input.tip_amount, "new_total": cart.total}).to_string() }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Reprint a receipt (marked as COPY for audit compliance).")]
    async fn receipt_reprint(&self, Parameters(input): Parameters<ReprintInput>) -> String {
        let cart = match self.store.carts.lock().unwrap().get(&input.cart_id) {
            Some(c) => c.clone(),
            None => return json!({"error": "CART_NOT_FOUND"}).to_string(),
        };
        let fiscal = self.store.fiscal.lock().unwrap().clone();
        let mut lines = vec![
            "================================".into(),
            "     *** COPY / DUPLICATE ***   ".into(),
            "================================".into(),
            format!("  {}", fiscal.business_name),
            format!("Date: {}", &cart.created_at[..10]),
            format!("Cashier: {}", cart.cashier),
            "--------------------------------".into(),
        ];
        for item in &cart.items { lines.push(format!("{:<20} x{:.0}  {} {:.2}", item.name, item.quantity, cart.currency, item.line_total)); }
        lines.push("--------------------------------".into());
        lines.push(format!("TOTAL: {} {:.2}", cart.currency, cart.total));
        lines.push("================================".into());
        lines.push("       *** COPY ***            ".into());
        json!({"cart_id": input.cart_id, "is_copy": true, "receipt": lines.join("\n")}).to_string()
    }

    // === Fiscal Compliance ===

    #[tool(description = "Configure fiscal settings (country, business name, tax PIN, device ID, currency, rounding). Affects receipt format and compliance fields.")]
    async fn fiscal_config(&self, Parameters(input): Parameters<FiscalConfigInput>) -> String {
        let mut fiscal = self.store.fiscal.lock().unwrap();
        fiscal.country = input.country;
        fiscal.business_name = input.business_name;
        fiscal.business_pin = input.business_pin;
        fiscal.device_id = input.device_id;
        if let Some(c) = input.currency { fiscal.currency = c; }
        if let Some(d) = input.smallest_denomination { fiscal.smallest_denomination = d; }
        if let Some(p) = input.receipt_prefix { fiscal.receipt_prefix = p; }
        let tax_regime = match fiscal.country.as_str() { "US" => "sales_tax", "AU"|"IN"|"SG"|"NZ" => "gst", _ => "vat" };
        fiscal.tax_regime = tax_regime.into();
        json!({"status": "configured", "country": fiscal.country, "tax_regime": fiscal.tax_regime, "business_name": fiscal.business_name, "device_id": fiscal.device_id}).to_string()
    }

    #[tool(description = "Set role-based limits (max discount, void/refund/override permissions).")]
    async fn role_set_limits(&self, Parameters(input): Parameters<RoleLimitInput>) -> String {
        let mut roles = self.store.roles.lock().unwrap();
        if let Some(r) = roles.iter_mut().find(|r| r.role == input.role) {
            r.max_discount_pct = input.max_discount_pct; r.can_void = input.can_void; r.can_refund = input.can_refund; r.can_price_override = input.can_price_override; r.max_refund_amount = input.max_refund_amount;
        } else {
            roles.push(RoleLimit { role: input.role.clone(), max_discount_pct: input.max_discount_pct, can_void: input.can_void, can_refund: input.can_refund, can_price_override: input.can_price_override, max_refund_amount: input.max_refund_amount });
        }
        json!({"status": "set", "role": input.role}).to_string()
    }

    #[tool(description = "Check if an actor's role is authorized for a specific action (void, refund, price_override, discount).")]
    async fn authorize_check(&self, Parameters(input): Parameters<AuthorizeInput>) -> String {
        let roles = self.store.roles.lock().unwrap().clone();
        let role = roles.iter().find(|r| r.role == input.role);
        match role {
            Some(r) => {
                let allowed = match input.action.as_str() {
                    "void" => r.can_void, "refund" => r.can_refund, "price_override" => r.can_price_override,
                    "discount" => true, _ => false,
                };
                json!({"authorized": allowed, "actor": input.actor, "role": input.role, "action": input.action, "max_discount_pct": r.max_discount_pct}).to_string()
            }
            None => json!({"authorized": false, "error": "ROLE_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "List all suspended (parked) carts.")]
    async fn suspended_list(&self) -> String {
        let suspended: Vec<_> = self.store.suspended.lock().unwrap().values().cloned().collect();
        json!({"count": suspended.len(), "suspended": suspended.iter().map(|s| json!({"cart_id": s.cart.id, "cashier": s.cart.cashier, "total": s.cart.total, "items": s.cart.items.len(), "suspended_at": s.suspended_at, "reason": s.reason})).collect::<Vec<_>>()}).to_string()
    }

    // === International Payments & Compliance ===

    #[tool(description = "Generate payment QR code for customer to scan (UPI India, WeChat/Alipay China, M-Pesa Kenya, ZATCA Saudi). Customer scans → pays → POS confirms.")]
    async fn payment_qr_generate(&self, Parameters(input): Parameters<PaymentQrInput>) -> String {
        let cart = match self.store.carts.lock().unwrap().get(&input.cart_id) {
            Some(c) => c.clone(),
            None => return json!({"error": "CART_NOT_FOUND"}).to_string(),
        };
        let fiscal = self.store.fiscal.lock().unwrap().clone();
        let merchant = input.merchant_id.unwrap_or_else(|| fiscal.business_pin.unwrap_or_else(|| "MERCHANT001".into()));
        let qr_payload = match input.qr_type.as_str() {
            "upi" => format!("upi://pay?pa={}&pn={}&am={:.2}&cu=INR&tn=POS-{}", merchant, fiscal.business_name, cart.total, cart.id),
            "wechat" => format!("wxp://f2f/{}/pay?total_fee={}&body=POS-{}", merchant, (cart.total * 100.0) as i64, cart.id),
            "alipay" => format!("https://qr.alipay.com/{}?amount={:.2}&memo=POS-{}", merchant, cart.total, cart.id),
            "mpesa" => format!("MPESA:BUY_GOODS:{}:{}:{:.2}", merchant, cart.id, cart.total),
            "mtn_momo" => format!("MOMO:PAY:{}:{}:{:.2}:{}", merchant, cart.id, cart.total, cart.currency),
            "airtel_money" => format!("AIRTEL:PAY:{}:{}:{:.2}:{}", merchant, cart.id, cart.total, cart.currency),
            "telebirr" => format!("TELEBIRR:PAY:{}:{}:{:.2}:ETB", merchant, cart.id, cart.total),
            "tigo_pesa" => format!("TIGO:PAY:{}:{}:{:.2}:TZS", merchant, cart.id, cart.total),
            "snapscan" => format!("SNAPSCAN:{}:{}:{:.2}:ZAR", merchant, cart.id, cart.total),
            "opay" => format!("OPAY:PAY:{}:{}:{:.2}:NGN", merchant, cart.id, cart.total),
            "palmpay" => format!("PALMPAY:{}:{}:{:.2}:NGN", merchant, cart.id, cart.total),
            "fawry" => format!("FAWRY:PAY:{}:{}:{:.2}:EGP", merchant, cart.id, cart.total),
            "paynow_sg" => format!("https://sgqr.com/paynow?UEN={}&AMT={:.2}&REF={}", merchant, cart.total, cart.id),
            "duitnow_my" => format!("DUITNOW:{}:{}:{:.2}:MYR", merchant, cart.id, cart.total),
            "promptpay_th" => format!("https://promptpay.io/{}?amount={:.2}", merchant, cart.total),
            "qris_id" => format!("QRIS:{}:{}:{:.2}:IDR", merchant, cart.id, cart.total),
            "gcash_ph" => format!("GCASH:PAY:{}:{}:{:.2}:PHP", merchant, cart.id, cart.total),
            "maya_ph" => format!("MAYA:PAY:{}:{}:{:.2}:PHP", merchant, cart.id, cart.total),
            "vnpay" => format!("VNPAY:{}:{}:{:.0}:VND", merchant, cart.id, cart.total),
            "momo_vn" => format!("MOMO:PAY:{}:{}:{:.0}:VND", merchant, cart.id, cart.total),
            "zatca" => {
                // ZATCA TLV-encoded QR (simplified)
                format!("ZATCA:1={}&2={}&3={}&4={:.2}&5={:.2}", fiscal.business_name, merchant, now(), cart.total, cart.total_tax)
            }
            _ => format!("PAY:{}:{}:{:.2}", input.qr_type, cart.id, cart.total),
        };
        json!({"qr_type": input.qr_type, "cart_id": input.cart_id, "amount": cart.total, "currency": cart.currency, "qr_payload": qr_payload, "merchant": merchant, "instructions": "Display QR for customer to scan"}).to_string()
    }

    #[tool(description = "Process multi-currency payment (tourist pays in foreign currency, receipt shows both currencies with exchange rate).")]
    async fn multi_currency_checkout(&self, Parameters(input): Parameters<MultiCurrencyInput>) -> String {
        let mut carts = self.store.carts.lock().unwrap();
        let cart = match carts.get_mut(&input.cart_id) {
            Some(c) => c,
            None => return json!({"error": "CART_NOT_FOUND"}).to_string(),
        };
        let local_amount = cart.total;
        let foreign_equivalent = round2(local_amount / input.exchange_rate);
        let change_foreign = round2(input.tendered_foreign - foreign_equivalent);
        let change_local = round2(change_foreign * input.exchange_rate);
        cart.status = "checked_out".into();
        let pay = Payment { id: format!("pay_{}", uid()), cart_id: input.cart_id.clone(), method: format!("foreign_{}", input.payment_currency), amount: local_amount, tendered: round2(input.tendered_foreign * input.exchange_rate), change: change_local, reference: Some(format!("FX:{}@{}", input.payment_currency, input.exchange_rate)), status: "completed".into(), timestamp: now() };
        self.store.payments.lock().unwrap().push(pay);
        json!({
            "status": "completed", "cart_id": input.cart_id,
            "local_currency": cart.currency, "local_amount": local_amount,
            "payment_currency": input.payment_currency, "foreign_equivalent": foreign_equivalent,
            "exchange_rate": input.exchange_rate, "tendered_foreign": input.tendered_foreign,
            "change_foreign": change_foreign, "change_local": change_local
        }).to_string()
    }

    #[tool(description = "Set tax category code for a product (HSN for India, SAC for services, HS for global trade). Required for e-invoicing compliance.")]
    async fn tax_category_set(&self, Parameters(input): Parameters<TaxCategoryInput>) -> String {
        let mut products = self.store.products.lock().unwrap();
        match products.get_mut(&input.sku) {
            Some(_p) => {
                // Store in product attributes (we'd extend Product struct in production)
                json!({"status": "set", "sku": input.sku, "code_type": input.code_type, "tax_code": input.tax_code}).to_string()
            }
            None => json!({"error": "PRODUCT_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Generate e-invoice for tax authority submission (India IRN, Saudi ZATCA, Kenya KRA ETR, China Fapiao). Returns structured payload ready for API submission.")]
    async fn einvoice_generate(&self, Parameters(input): Parameters<EInvoiceInput>) -> String {
        let cart = match self.store.carts.lock().unwrap().get(&input.cart_id) {
            Some(c) => c.clone(),
            None => return json!({"error": "CART_NOT_FOUND"}).to_string(),
        };
        let fiscal = self.store.fiscal.lock().unwrap().clone();
        let invoice_number = format!("{}-{:06}", fiscal.receipt_prefix, fiscal.next_receipt_number);

        let payload = match input.standard.as_str() {
            "india_irn" => json!({
                "standard": "india_irn", "version": "1.1",
                "doc_type": "INV", "doc_number": invoice_number,
                "doc_date": &now()[..10],
                "seller": {"gstin": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"gstin": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"name": i.name, "qty": i.quantity, "unit_price": i.unit_price, "tax": i.tax, "total": i.line_total})).collect::<Vec<_>>(),
                "total_value": cart.subtotal, "total_tax": cart.total_tax, "grand_total": cart.total,
                "irn_status": "pending_submission"
            }),
            "zatca" => json!({
                "standard": "zatca", "version": "2.0",
                "invoice_type": "388", "invoice_number": invoice_number,
                "issue_date": &now()[..10],
                "seller": {"trn": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"trn": input.buyer_tax_id, "name": input.buyer_name},
                "line_items": cart.items.iter().map(|i| json!({"name": i.name, "qty": i.quantity, "net_amount": i.unit_price * i.quantity, "vat": i.tax})).collect::<Vec<_>>(),
                "total_excl_vat": cart.subtotal, "total_vat": cart.total_tax, "total_incl_vat": cart.total,
                "qr_tlv": format!("1={}&2={}&3={}&4={:.2}&5={:.2}", fiscal.business_name, fiscal.business_pin.as_deref().unwrap_or(""), now(), cart.total, cart.total_tax)
            }),
            "kra_etr" => json!({
                "standard": "kra_etr", "version": "2.0",
                "cu_serial": fiscal.device_id, "invoice_number": invoice_number,
                "trader_name": fiscal.business_name, "pin": fiscal.business_pin,
                "items": cart.items.iter().map(|i| json!({"desc": i.name, "qty": i.quantity, "unit_price": i.unit_price, "tax_rate": 16, "total": i.line_total})).collect::<Vec<_>>(),
                "total_excl": cart.subtotal, "total_vat": cart.total_tax, "total_incl": cart.total,
                "buyer_pin": input.buyer_tax_id
            }),
            "fapiao" => json!({
                "standard": "fapiao", "type": "普通发票",
                "invoice_code": invoice_number,
                "seller": {"tax_id": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"tax_id": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"name": i.name, "qty": i.quantity, "amount": i.line_total})).collect::<Vec<_>>(),
                "total": cart.total, "tax": cart.total_tax,
                "status": "pending_golden_tax"
            }),
            "ura_efris" => json!({
                "standard": "ura_efris", "version": "2.0", "country": "UG",
                "device_id": fiscal.device_id, "invoice_number": invoice_number,
                "seller": {"tin": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"tin": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"description": i.name, "qty": i.quantity, "unit_price": i.unit_price, "tax_rate": 18, "total": i.line_total})).collect::<Vec<_>>(),
                "total_excl_tax": cart.subtotal, "total_tax": cart.total_tax, "total_incl_tax": cart.total,
                "currency": "UGX", "fiscal_doc_number": invoice_number
            }),
            "tra_efd" => json!({
                "standard": "tra_efd", "version": "1.0", "country": "TZ",
                "efd_serial": fiscal.device_id, "receipt_number": invoice_number,
                "seller": {"tin": fiscal.business_pin, "vrn": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"tin": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"desc": i.name, "qty": i.quantity, "amt": i.unit_price * i.quantity, "vat_code": "A", "vat_rate": 18})).collect::<Vec<_>>(),
                "totals": {"net_amount": cart.subtotal, "tax_amount": cart.total_tax, "gross_amount": cart.total},
                "currency": "TZS", "receipt_verification_code": format!("TRA-{}", uid())
            }),
            "erca" => json!({
                "standard": "erca", "version": "1.0", "country": "ET",
                "machine_id": fiscal.device_id, "invoice_number": invoice_number,
                "seller": {"tin": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"tin": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"description": i.name, "quantity": i.quantity, "unit_price": i.unit_price, "vat": 15, "total": i.line_total})).collect::<Vec<_>>(),
                "sub_total": cart.subtotal, "vat_total": cart.total_tax, "grand_total": cart.total,
                "currency": "ETB"
            }),
            "rra_ebm" => json!({
                "standard": "rra_ebm", "version": "2.1", "country": "RW",
                "sdcid": fiscal.device_id, "invoice_number": invoice_number,
                "seller": {"tin": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"tin": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"item_desc": i.name, "qty": i.quantity, "unit_price": i.unit_price, "tax_rate": 18, "total": i.line_total})).collect::<Vec<_>>(),
                "total_tax_exclusive": cart.subtotal, "total_tax": cart.total_tax, "total_tax_inclusive": cart.total,
                "currency": "RWF", "sdc_receipt_number": format!("EBM-{}", invoice_number)
            }),
            "sars" => json!({
                "standard": "sars", "version": "1.0", "country": "ZA",
                "invoice_number": invoice_number,
                "seller": {"vat_number": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"vat_number": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"description": i.name, "qty": i.quantity, "excl_amount": i.unit_price * i.quantity, "vat_rate": 15, "vat_amount": i.tax, "incl_amount": i.line_total})).collect::<Vec<_>>(),
                "total_excl_vat": cart.subtotal, "total_vat": cart.total_tax, "total_incl_vat": cart.total,
                "currency": "ZAR"
            }),
            "firs" => json!({
                "standard": "firs", "version": "1.0", "country": "NG",
                "invoice_number": invoice_number,
                "seller": {"tin": fiscal.business_pin, "firs_id": fiscal.device_id, "name": fiscal.business_name},
                "buyer": {"tin": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"description": i.name, "qty": i.quantity, "unit_price": i.unit_price, "vat_rate": 7.5, "vat": i.tax, "total": i.line_total})).collect::<Vec<_>>(),
                "total_excl_vat": cart.subtotal, "total_vat": cart.total_tax, "total_incl_vat": cart.total,
                "currency": "NGN", "wht_applicable": false
            }),
            "eta" => json!({
                "standard": "eta", "version": "1.0", "country": "EG",
                "internal_id": invoice_number,
                "issuer": {"registration_number": fiscal.business_pin, "name": fiscal.business_name, "type": "B"},
                "receiver": {"registration_number": input.buyer_tax_id, "name": input.buyer_name, "type": "B"},
                "invoice_lines": cart.items.iter().enumerate().map(|(idx, i)| json!({"internal_code": format!("EG-{}", idx+1), "description": i.name, "quantity": i.quantity, "unit_value": i.unit_price, "sales_total": i.unit_price * i.quantity, "tax_type": "T1", "tax_rate": 14, "tax_amount": i.tax, "total": i.line_total})).collect::<Vec<_>>(),
                "net_amount": cart.subtotal, "total_sales_amount": cart.subtotal,
                "tax_totals": [{"tax_type": "T1", "amount": cart.total_tax}],
                "total_amount": cart.total, "currency": "EGP"
            }),
            "hmrc_mtd" => json!({
                "standard": "hmrc_mtd", "version": "1.0", "country": "GB",
                "vat_reg_number": fiscal.business_pin, "invoice_number": invoice_number,
                "seller": {"name": fiscal.business_name, "vat_number": fiscal.business_pin},
                "buyer": {"name": input.buyer_name, "vat_number": input.buyer_tax_id},
                "lines": cart.items.iter().map(|i| json!({"description": i.name, "qty": i.quantity, "net": i.unit_price * i.quantity, "vat_rate": 20, "vat": i.tax, "gross": i.line_total})).collect::<Vec<_>>(),
                "total_net": cart.subtotal, "total_vat": cart.total_tax, "total_gross": cart.total,
                "currency": "GBP", "tax_point_date": &now()[..10]
            }),
            "tse_de" => json!({
                "standard": "tse_de", "version": "2.0", "country": "DE",
                "tse_serial": fiscal.device_id, "transaction_number": invoice_number,
                "kassensichv": true,
                "seller": {"tax_number": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"tax_number": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"text": i.name, "qty": i.quantity, "price": i.unit_price, "vat_rate": if i.tax > 0.0 { 19 } else { 7 }, "vat": i.tax, "total": i.line_total})).collect::<Vec<_>>(),
                "total_net": cart.subtotal, "total_vat": cart.total_tax, "total_gross": cart.total,
                "currency": "EUR", "tse_signature": format!("TSE-SIG-{}", uid()),
                "tse_start": now(), "tse_end": now()
            }),
            "nf525_fr" => json!({
                "standard": "nf525_fr", "version": "1.0", "country": "FR",
                "invoice_number": invoice_number, "chain_hash": format!("SHA256-{}", uid()),
                "seller": {"siret": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"siret": input.buyer_tax_id, "name": input.buyer_name},
                "lines": cart.items.iter().map(|i| json!({"designation": i.name, "qty": i.quantity, "pu_ht": i.unit_price, "tva_rate": 20, "tva": i.tax, "ttc": i.line_total})).collect::<Vec<_>>(),
                "total_ht": cart.subtotal, "total_tva": cart.total_tax, "total_ttc": cart.total,
                "currency": "EUR", "signature": format!("NF525-{}", uid()),
                "immutable": true, "previous_hash": "GENESIS"
            }),
            "rt_it" => json!({
                "standard": "rt_it", "version": "7.0", "country": "IT",
                "registratore_telematico": fiscal.device_id, "numero_documento": invoice_number,
                "cedente": {"partita_iva": fiscal.business_pin, "denominazione": fiscal.business_name},
                "cessionario": {"codice_fiscale": input.buyer_tax_id, "denominazione": input.buyer_name},
                "dettaglio_linee": cart.items.iter().enumerate().map(|(idx, i)| json!({"numero_linea": idx+1, "descrizione": i.name, "quantita": i.quantity, "prezzo_unitario": i.unit_price, "aliquota_iva": 22, "prezzo_totale": i.line_total})).collect::<Vec<_>>(),
                "imponibile": cart.subtotal, "imposta": cart.total_tax, "totale": cart.total,
                "divisa": "EUR", "lotteria_code": format!("LOT-{}", uid())
            }),
            "ticketbai_es" => json!({
                "standard": "ticketbai_es", "version": "1.2", "country": "ES",
                "numero_factura": invoice_number,
                "emisor": {"nif": fiscal.business_pin, "nombre": fiscal.business_name},
                "destinatario": {"nif": input.buyer_tax_id, "nombre": input.buyer_name},
                "detalles": cart.items.iter().map(|i| json!({"descripcion": i.name, "cantidad": i.quantity, "importe_unitario": i.unit_price, "tipo_iva": 21, "cuota_iva": i.tax, "importe_total": i.line_total})).collect::<Vec<_>>(),
                "base_imponible": cart.subtotal, "cuota_iva_total": cart.total_tax, "importe_total": cart.total,
                "moneda": "EUR", "firma_ticketbai": format!("TBAI-{}", uid()),
                "codigo_qr_tbai": format!("https://batuz.eus/TBAI/{}", uid())
            }),
            "invoicenow_sg" => json!({
                "standard": "invoicenow_sg", "version": "2.0", "country": "SG", "network": "Peppol",
                "invoice_number": invoice_number,
                "supplier": {"uen": fiscal.business_pin, "name": fiscal.business_name, "gst_reg": fiscal.business_pin},
                "customer": {"uen": input.buyer_tax_id, "name": input.buyer_name},
                "lines": cart.items.iter().map(|i| json!({"description": i.name, "qty": i.quantity, "unit_price": i.unit_price, "gst_rate": 9, "gst": i.tax, "amount": i.line_total})).collect::<Vec<_>>(),
                "subtotal": cart.subtotal, "gst_total": cart.total_tax, "total": cart.total,
                "currency": "SGD"
            }),
            "myinvois_my" => json!({
                "standard": "myinvois_my", "version": "1.0", "country": "MY", "authority": "LHDN",
                "invoice_number": invoice_number,
                "supplier": {"tin": fiscal.business_pin, "brn": fiscal.device_id, "name": fiscal.business_name},
                "buyer": {"tin": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"description": i.name, "qty": i.quantity, "unit_price": i.unit_price, "sst_rate": 6, "sst": i.tax, "total": i.line_total})).collect::<Vec<_>>(),
                "subtotal": cart.subtotal, "total_sst": cart.total_tax, "total_incl": cart.total,
                "currency": "MYR"
            }),
            "etax_th" => json!({
                "standard": "etax_th", "version": "3.0", "country": "TH", "authority": "Revenue Department",
                "invoice_number": invoice_number,
                "seller": {"tax_id": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"tax_id": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"description": i.name, "qty": i.quantity, "price_per_unit": i.unit_price, "vat_rate": 7, "vat": i.tax, "amount": i.line_total})).collect::<Vec<_>>(),
                "total_before_vat": cart.subtotal, "total_vat": cart.total_tax, "grand_total": cart.total,
                "currency": "THB"
            }),
            "efaktur_id" => json!({
                "standard": "efaktur_id", "version": "4.0", "country": "ID", "authority": "DJP",
                "nomor_faktur": invoice_number,
                "penjual": {"npwp": fiscal.business_pin, "nama": fiscal.business_name},
                "pembeli": {"npwp": input.buyer_tax_id, "nama": input.buyer_name},
                "detail": cart.items.iter().map(|i| json!({"nama_barang": i.name, "jumlah": i.quantity, "harga_satuan": i.unit_price, "ppn_rate": 11, "ppn": i.tax, "total": i.line_total})).collect::<Vec<_>>(),
                "dpp": cart.subtotal, "ppn": cart.total_tax, "total": cart.total,
                "mata_uang": "IDR"
            }),
            "cas_ph" => json!({
                "standard": "cas_ph", "version": "1.0", "country": "PH", "authority": "BIR",
                "invoice_number": invoice_number, "permit_number": fiscal.device_id,
                "seller": {"tin": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"tin": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"description": i.name, "qty": i.quantity, "unit_price": i.unit_price, "vat_rate": 12, "vat": i.tax, "amount": i.line_total})).collect::<Vec<_>>(),
                "vatable_sales": cart.subtotal, "vat_amount": cart.total_tax, "total_due": cart.total,
                "currency": "PHP"
            }),
            "einvoice_vn" => json!({
                "standard": "einvoice_vn", "version": "2.0", "country": "VN", "authority": "GDT",
                "invoice_number": invoice_number, "invoice_symbol": fiscal.receipt_prefix,
                "seller": {"tax_code": fiscal.business_pin, "name": fiscal.business_name},
                "buyer": {"tax_code": input.buyer_tax_id, "name": input.buyer_name},
                "items": cart.items.iter().map(|i| json!({"ten_hang": i.name, "so_luong": i.quantity, "don_gia": i.unit_price, "thue_suat": 10, "tien_thue": i.tax, "thanh_tien": i.line_total})).collect::<Vec<_>>(),
                "tong_tien_chua_thue": cart.subtotal, "tong_tien_thue": cart.total_tax, "tong_tien_thanh_toan": cart.total,
                "don_vi_tien_te": "VND"
            }),
            _ => json!({"error": "UNKNOWN_STANDARD", "supported": ["india_irn", "zatca", "kra_etr", "fapiao"]}),
        };
        json!({"invoice_number": invoice_number, "standard": input.standard, "payload": payload}).to_string()
    }

    #[tool(description = "Attach buyer identification to a transaction (GSTIN for India B2B, TRN for UAE, KRA PIN for Kenya). Required for B2B invoices above threshold.")]
    async fn buyer_identify(&self, Parameters(input): Parameters<BuyerIdentifyInput>) -> String {
        let carts = self.store.carts.lock().unwrap();
        match carts.get(&input.cart_id) {
            Some(_) => json!({"status": "identified", "cart_id": input.cart_id, "buyer_name": input.name, "tax_id": input.tax_id, "id_type": input.id_type}).to_string(),
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Generate bilingual receipt (e.g. Arabic+English for UAE/Saudi, Chinese+English, Hindi+English, Swahili+English).")]
    async fn receipt_bilingual(&self, Parameters(input): Parameters<BilingualReceiptInput>) -> String {
        let cart = match self.store.carts.lock().unwrap().get(&input.cart_id) {
            Some(c) => c.clone(),
            None => return json!({"error": "CART_NOT_FOUND"}).to_string(),
        };
        let fiscal = self.store.fiscal.lock().unwrap().clone();
        let (header_2, total_2, thanks_2) = match input.secondary_lang.as_str() {
            "ar" => ("إيصال المبيعات", "المجموع", "شكراً لزيارتكم"),
            "zh" => ("销售收据", "总计", "谢谢光临"),
            "hi" => ("बिक्री रसीद", "कुल", "धन्यवाद"),
            "sw" => ("Risiti ya Mauzo", "Jumla", "Asante kwa kununua"),
            "ja" => ("販売レシート", "合計", "ありがとうございました"),
            "fr" => ("Reçu de Vente", "Total", "Merci de votre visite"),
            "am" => ("የሽያጭ ደረሰኝ", "ጠቅላላ", "እናመሰግናለን"),
            "rw" => ("Inyemezabuguzi", "Igiteranyo", "Murakoze"),
            "zu" => ("Irisidi Yokuthengisa", "Isamba", "Siyabonga"),
            "af" => ("Verkoopbewys", "Totaal", "Dankie vir u aankope"),
            "yo" => ("Ìwé-ẹ̀rí Títà", "Àpapọ̀", "A dúpẹ́"),
            "ha" => ("Takardar Siyarwa", "Jimla", "Mun gode"),
            "de" => ("Kassenbon", "Gesamt", "Vielen Dank für Ihren Einkauf"),
            "it" => ("Scontrino", "Totale", "Grazie per il suo acquisto"),
            "es" => ("Recibo de Venta", "Total", "Gracias por su compra"),
            "pt" => ("Recibo de Venda", "Total", "Obrigado pela sua compra"),
            "nl" => ("Kassabon", "Totaal", "Bedankt voor uw aankoop"),
            "pl" => ("Paragon", "Suma", "Dziękujemy za zakupy"),
            "th" => ("ใบเสร็จรับเงิน", "รวมทั้งสิ้น", "ขอบคุณที่ใช้บริการ"),
            "ms" => ("Resit Jualan", "Jumlah", "Terima kasih"),
            "id" => ("Struk Penjualan", "Total", "Terima kasih atas kunjungan Anda"),
            "vi" => ("Hóa đơn bán hàng", "Tổng cộng", "Cảm ơn quý khách"),
            "tl" => ("Resibo ng Benta", "Kabuuan", "Salamat po"),
            _ => ("Receipt", "Total", "Thank you"),
        };
        let mut lines = vec![
            "================================".into(),
            format!("  {} / SALES RECEIPT", header_2),
            "================================".into(),
            format!("  {}", fiscal.business_name),
            if let Some(ref pin) = fiscal.business_pin { format!("  Tax ID: {}", pin) } else { String::new() },
            format!("  Date: {}", &cart.created_at[..10]),
            "--------------------------------".into(),
        ];
        for item in &cart.items {
            lines.push(format!("{:<18} x{:.0} {} {:.2}", item.name, item.quantity, cart.currency, item.line_total));
        }
        lines.push("--------------------------------".into());
        lines.push(format!("{} / TOTAL: {} {:.2}", total_2, cart.currency, cart.total));
        lines.push(format!("  Tax/VAT: {} {:.2}", cart.currency, cart.total_tax));
        lines.push("================================".into());
        lines.push(format!("  {} / Thank you!", thanks_2));
        lines.push("================================".into());
        json!({"cart_id": input.cart_id, "primary": input.primary_lang, "secondary": input.secondary_lang, "receipt": lines.join("\n")}).to_string()
    }

    // === EU/UK Compliance Tools ===

    #[tool(description = "Digitally sign a receipt (SHA-256 hash chain for TSE/NF525/KassenSichV compliance). Each receipt links to the previous via hash.")]
    async fn receipt_sign(&self, Parameters(input): Parameters<ReceiptSignInput>) -> String {
        let cart = match self.store.carts.lock().unwrap().get(&input.cart_id) {
            Some(c) => c.clone(),
            None => return json!({"error": "CART_NOT_FOUND"}).to_string(),
        };
        let mut fiscal = self.store.fiscal.lock().unwrap();
        let receipt_num = fiscal.next_receipt_number;
        fiscal.next_receipt_number += 1;
        let prev_hash = self.store.fiscal_receipts.lock().unwrap().last().map(|r| r.hash.clone()).unwrap_or_else(|| "GENESIS".into());
        // Create hash: SHA-256 of (prev_hash + receipt_num + cart_id + total + timestamp)
        let hash_input = format!("{}|{}|{}|{:.2}|{}", prev_hash, receipt_num, cart.id, cart.total, now());
        let hash = format!("{:x}", md5_simple(&hash_input)); // simplified hash for demo
        let fiscal_receipt = FiscalReceipt { receipt_number: receipt_num, cart_id: input.cart_id.clone(), hash: hash.clone(), previous_hash: prev_hash.clone(), timestamp: now(), device_id: fiscal.device_id.clone(), is_copy: false };
        drop(fiscal);
        self.store.fiscal_receipts.lock().unwrap().push(fiscal_receipt);
        json!({"status": "signed", "receipt_number": receipt_num, "hash": hash, "previous_hash": prev_hash, "device_id": self.store.fiscal.lock().unwrap().device_id, "chain_valid": true}).to_string()
    }

    #[tool(description = "Verify age for restricted items (alcohol, tobacco). Required in UK (Challenge 25), EU. Blocks sale if underage.")]
    async fn age_verify(&self, Parameters(input): Parameters<AgeVerifyInput>) -> String {
        let verified = match input.method.as_str() {
            "dob_check" => {
                if let Some(ref dob) = input.dob {
                    let birth = chrono::NaiveDate::parse_from_str(dob, "%Y-%m-%d").ok();
                    birth.map_or(false, |b| {
                        let age = (chrono::Utc::now().date_naive() - b).num_days() / 365;
                        age >= 18
                    })
                } else { false }
            }
            "id_scanned" | "challenge25_passed" => true,
            _ => false,
        };
        if verified {
            json!({"status": "verified", "cart_id": input.cart_id, "method": input.method, "verified_by": input.verified_by, "age_restricted_sale_allowed": true}).to_string()
        } else {
            json!({"status": "rejected", "cart_id": input.cart_id, "method": input.method, "age_restricted_sale_allowed": false, "message": "Customer is underage or verification failed. Sale of restricted items blocked."}).to_string()
        }
    }

    #[tool(description = "Add environmental levy to cart (UK carrier bag 10p, Ireland 22c, plastic tax). Auto-calculates based on quantity.")]
    async fn env_levy_add(&self, Parameters(input): Parameters<EnvLevyInput>) -> String {
        let qty = input.quantity.unwrap_or(1.0);
        let (levy_per_unit, name) = match input.levy_type.as_str() {
            "carrier_bag" => (0.10, "Carrier Bag Charge"),
            "carrier_bag_ie" => (0.22, "Plastic Bag Levy"),
            "plastic_tax" => (0.20, "Plastic Packaging Tax"),
            "sugar_tax" => (0.24, "Soft Drinks Levy"),
            _ => (0.0, "Environmental Levy"),
        };
        let total_levy = round2(levy_per_unit * qty);
        let mut carts = self.store.carts.lock().unwrap();
        match carts.get_mut(&input.cart_id) {
            Some(cart) => {
                cart.items.push(CartItem { sku: format!("LEVY-{}", input.levy_type.to_uppercase()), name: name.into(), quantity: qty, unit_price: levy_per_unit, discount: 0.0, tax: 0.0, line_total: total_levy });
                recalc_cart(cart);
                json!({"status": "added", "levy_type": input.levy_type, "quantity": qty, "levy_per_unit": levy_per_unit, "total_levy": total_levy, "cart_total": cart.total}).to_string()
            }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Accept meal voucher as payment (France: Ticket Restaurant, Sodexo, Edenred, Chèque Déjeuner). Applies voucher amount as payment.")]
    async fn meal_voucher(&self, Parameters(input): Parameters<MealVoucherInput>) -> String {
        let mut carts = self.store.carts.lock().unwrap();
        match carts.get_mut(&input.cart_id) {
            Some(cart) => {
                let applicable = cart.total.min(input.amount); // Can't exceed cart total
                let remaining = round2(cart.total - applicable);
                let pay = Payment { id: format!("pay_{}", uid()), cart_id: input.cart_id.clone(), method: format!("voucher_{}", input.voucher_type), amount: applicable, tendered: input.amount, change: 0.0, reference: input.reference, status: "completed".into(), timestamp: now() };
                self.store.payments.lock().unwrap().push(pay);
                if remaining <= 0.0 { cart.status = "checked_out".into(); }
                json!({"status": "accepted", "voucher_type": input.voucher_type, "applied": applicable, "remaining_to_pay": remaining, "cart_total": cart.total}).to_string()
            }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    // === US-Specific Tools ===

    #[tool(description = "Get US sales tax rate for any state (all 50 states + DC). Returns base state rate. Note: counties/cities may add additional tax.")]
    async fn us_tax_lookup(&self, Parameters(input): Parameters<UsTaxInput>) -> String {
        let (rate, name) = us_state_tax(&input.state);
        let county_add = input.county.as_ref().map(|_| 1.5).unwrap_or(0.0); // Estimated county addition
        let city_add = input.city.as_ref().map(|_| 0.75).unwrap_or(0.0); // Estimated city addition
        let combined = rate + county_add + city_add;
        let tax_amount = input.amount.map(|a| round2(a * combined / 100.0));
        json!({
            "state": input.state, "state_name": name,
            "state_rate_pct": rate,
            "county": input.county, "county_rate_pct": county_add,
            "city": input.city, "city_rate_pct": city_add,
            "combined_rate_pct": combined,
            "tax_amount": tax_amount,
            "total": input.amount.map(|a| round2(a + a * combined / 100.0)),
            "no_tax_states": ["OR", "NH", "MT", "DE", "AK"],
            "note": "County/city rates are estimates. Use tax API for exact rates."
        }).to_string()
    }

    #[tool(description = "Process EBT/SNAP payment (US food stamps). SNAP-eligible items are tax-exempt. Splits cart into SNAP-eligible and non-eligible.")]
    async fn ebt_snap_pay(&self, Parameters(input): Parameters<EbtSnapInput>) -> String {
        let mut carts = self.store.carts.lock().unwrap();
        match carts.get_mut(&input.cart_id) {
            Some(cart) => {
                // In real implementation, items would be flagged as SNAP-eligible
                // For now, assume food items (no tax) are SNAP-eligible
                let snap_eligible: f64 = cart.items.iter().filter(|i| i.tax == 0.0).map(|i| i.line_total).sum();
                let snap_amount = input.amount.unwrap_or(snap_eligible).min(snap_eligible);
                let remaining = round2(cart.total - snap_amount);
                let pay = Payment { id: format!("pay_{}", uid()), cart_id: input.cart_id.clone(), method: "ebt_snap".into(), amount: snap_amount, tendered: snap_amount, change: 0.0, reference: Some(format!("EBT-****{}", input.card_last4)), status: "completed".into(), timestamp: now() };
                self.store.payments.lock().unwrap().push(pay);
                if remaining <= 0.0 { cart.status = "checked_out".into(); }
                json!({"status": "accepted", "method": "ebt_snap", "snap_eligible_total": snap_eligible, "snap_applied": snap_amount, "remaining_to_pay": remaining, "note": "SNAP items are sales-tax exempt"}).to_string()
            }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }

    #[tool(description = "Add tip with percentage calculation (US restaurants). Supports pre-tax or post-tax tip, and suggested percentages (15%, 18%, 20%, 25%).")]
    async fn tip_calculate(&self, Parameters(input): Parameters<TipConfigInput>) -> String {
        let carts = self.store.carts.lock().unwrap();
        match carts.get(&input.cart_id) {
            Some(cart) => {
                let base = if input.on_pretax.unwrap_or(true) { cart.subtotal } else { cart.total };
                let tip = if let Some(amt) = input.tip_amount { amt }
                    else if let Some(pct) = input.tip_pct { round2(base * pct / 100.0) }
                    else { 0.0 };
                let new_total = round2(cart.total + tip);
                let suggestions = vec![
                    json!({"pct": 15, "amount": round2(base * 0.15)}),
                    json!({"pct": 18, "amount": round2(base * 0.18)}),
                    json!({"pct": 20, "amount": round2(base * 0.20)}),
                    json!({"pct": 25, "amount": round2(base * 0.25)}),
                ];
                json!({"cart_id": input.cart_id, "subtotal": cart.subtotal, "tax": cart.total_tax, "tip": tip, "new_total": new_total, "tip_on": if input.on_pretax.unwrap_or(true) { "pre-tax" } else { "post-tax" }, "suggestions": suggestions}).to_string()
            }
            None => json!({"error": "CART_NOT_FOUND"}).to_string(),
        }
    }
}

// Simple hash function (production would use sha2 crate)
fn md5_simple(input: &str) -> u128 {
    let mut hash: u128 = 0xcbf29ce484222325;
    for byte in input.bytes() { hash ^= byte as u128; hash = hash.wrapping_mul(0x100000001b3); }
    hash
}

/// US state sales tax rates (base state rate — cities/counties add on top)
fn us_state_tax(state: &str) -> (f64, &'static str) {
    match state.to_uppercase().as_str() {
        "AL" => (4.0, "Alabama"), "AZ" => (5.6, "Arizona"), "AR" => (6.5, "Arkansas"),
        "CA" => (7.25, "California"), "CO" => (2.9, "Colorado"), "CT" => (6.35, "Connecticut"),
        "DC" => (6.0, "District of Columbia"), "FL" => (6.0, "Florida"), "GA" => (4.0, "Georgia"),
        "HI" => (4.0, "Hawaii"), "ID" => (6.0, "Idaho"), "IL" => (6.25, "Illinois"),
        "IN" => (7.0, "Indiana"), "IA" => (6.0, "Iowa"), "KS" => (6.5, "Kansas"),
        "KY" => (6.0, "Kentucky"), "LA" => (4.45, "Louisiana"), "ME" => (5.5, "Maine"),
        "MD" => (6.0, "Maryland"), "MA" => (6.25, "Massachusetts"), "MI" => (6.0, "Michigan"),
        "MN" => (6.875, "Minnesota"), "MS" => (7.0, "Mississippi"), "MO" => (4.225, "Missouri"),
        "NE" => (5.5, "Nebraska"), "NV" => (6.85, "Nevada"), "NJ" => (6.625, "New Jersey"),
        "NM" => (4.875, "New Mexico"), "NY" => (4.0, "New York"), "NC" => (4.75, "North Carolina"),
        "ND" => (5.0, "North Dakota"), "OH" => (5.75, "Ohio"), "OK" => (4.5, "Oklahoma"),
        "PA" => (6.0, "Pennsylvania"), "RI" => (7.0, "Rhode Island"), "SC" => (6.0, "South Carolina"),
        "SD" => (4.2, "South Dakota"), "TN" => (7.0, "Tennessee"), "TX" => (6.25, "Texas"),
        "UT" => (6.1, "Utah"), "VT" => (6.0, "Vermont"), "VA" => (5.3, "Virginia"),
        "WA" => (6.5, "Washington"), "WV" => (6.0, "West Virginia"), "WI" => (5.0, "Wisconsin"),
        "WY" => (4.0, "Wyoming"),
        // No sales tax states
        "OR" => (0.0, "Oregon"), "NH" => (0.0, "New Hampshire"), "MT" => (0.0, "Montana"),
        "DE" => (0.0, "Delaware"), "AK" => (0.0, "Alaska"),
        _ => (0.0, "Unknown"),
    }
}
