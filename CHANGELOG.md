# Changelog

## [1.7.0] - 2026-05-27

### Added — US Full Coverage
- `us_tax_lookup` — all 50 states + DC sales tax rates with county/city estimates
- `ebt_snap_pay` — EBT/SNAP food stamp payment (auto-splits eligible items, tax-exempt)
- `tip_calculate` — tip with % suggestions (15/18/20/25%), pre-tax or post-tax

## [1.6.0] - 2026-05-27

### Added — Southeast Asia
- E-invoice: `invoicenow_sg`, `myinvois_my`, `etax_th`, `efaktur_id`, `cas_ph`, `einvoice_vn`
- Payment QR: `paynow_sg`, `duitnow_my`, `promptpay_th`, `qris_id`, `gcash_ph`, `maya_ph`, `vnpay`, `momo_vn`
- Languages: Thai, Malay, Bahasa Indonesia, Vietnamese, Filipino

## [1.5.0] - 2026-05-27

### Added — UK/EU Compliance
- E-invoice: `hmrc_mtd`, `tse_de`, `nf525_fr`, `rt_it`, `ticketbai_es`
- `receipt_sign` — SHA-256 hash chain (TSE/NF525 compliant)
- `age_verify` — block underage sales (DOB, ID scan, Challenge 25)
- `env_levy_add` — carrier bag charge, plastic tax, sugar tax
- `meal_voucher` — Ticket Restaurant, Sodexo, Edenred
- Languages: German, Italian, Spanish, Portuguese, Dutch, Polish

## [1.4.0] - 2026-05-27

### Added — South Africa, Nigeria, Egypt
- E-invoice: `sars`, `firs`, `eta`
- Payment QR: `snapscan`, `opay`, `palmpay`, `fawry`
- Languages: Zulu, Afrikaans, Yoruba, Hausa

## [1.3.0] - 2026-05-27

### Added — East Africa
- E-invoice: `ura_efris`, `tra_efd`, `erca`, `rra_ebm`
- Payment QR: `mtn_momo`, `airtel_money`, `telebirr`, `tigo_pesa`
- Languages: Amharic, Kinyarwanda

## [1.2.0] - 2026-05-27

### Added — International Compliance
- `payment_qr_generate` — UPI, WeChat, Alipay, M-Pesa, ZATCA QR
- `multi_currency_checkout` — foreign currency with FX conversion
- `tax_category_set` — HSN/SAC/HS codes
- `einvoice_generate` — India IRN, Saudi ZATCA, Kenya KRA ETR, China Fapiao
- `buyer_identify` — attach GSTIN/TRN/KRA PIN
- `receipt_bilingual` — Arabic, Chinese, Hindi, Swahili, Japanese, French

## [1.1.0] - 2026-05-27

### Added — Business Controls + Fiscal
- `cart_void` — role-gated void
- `price_override` — role-gated with reason
- `cart_suspend` / `cart_recall` — park transactions
- `split_tender` — multiple payment methods
- `tip_add` — gratuity
- `receipt_reprint` — marked as COPY
- `fiscal_config` — country, business PIN, device ID
- `role_set_limits` — per-role permissions
- `authorize_check` — verify role authorization

## [1.0.0] - 2026-05-27

### Added — Core POS
- Cart: create, add_item, remove_item, apply_discount, get
- Payments: payment_process (cash/card/mobile_money), refund
- Receipts: receipt_generate (thermal format)
- Shifts: shift_open, shift_close (cash reconciliation)
- Loyalty: loyalty_check, loyalty_redeem
- Catalog: product_register, barcode_lookup
- Reports: daily_summary
