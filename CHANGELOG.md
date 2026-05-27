# Changelog

## [1.0.0] - 2026-05-27

### Added
- `product_register` — register products with SKU, barcode, price, tax rate
- `barcode_lookup` — scan barcode to get product info and price
- `cart_create` — start a new sale/transaction
- `cart_add_item` — add item by barcode scan (auto price + tax calculation)
- `cart_remove_item` — remove item from cart
- `cart_apply_discount` — percentage or fixed discount (per-item or whole cart)
- `cart_get` — view cart contents with running totals
- `payment_process` — process payment (cash with change, card, mobile money)
- `receipt_generate` — thermal printer formatted receipt
- `refund` — process returns (full or partial, requires approval)
- `shift_open` — open register with opening float
- `shift_close` — close shift with cash reconciliation and variance detection
- `barcode_lookup` — product lookup by barcode or SKU
- `loyalty_check` — check customer points and tier
- `loyalty_redeem` — redeem points as payment discount
- `daily_summary` — sales totals, payment breakdown, average transaction
- Multi-payment support: cash, card, mobile_money
- Automatic tax calculation per line item
- Shift-level sales tracking (cash/card/mobile breakdown)
- Loyalty tier system (bronze/silver/gold/platinum)
