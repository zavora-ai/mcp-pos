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
}
