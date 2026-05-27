# Point of Sale MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-pos.svg)](https://crates.io/crates/mcp-pos)
[![Docs.rs](https://docs.rs/mcp-pos/badge.svg)](https://docs.rs/mcp-pos)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://enterprise.adk-rust.com)

Global Point of Sale engine for [ADK-Rust Enterprise](https://enterprise.adk-rust.com) agents. Provides 39 MCP tools covering the full retail transaction lifecycle across **23 countries** — cart management, 20+ payment methods, fiscal e-invoicing for 22 tax authorities, bilingual receipts in 24 languages, role-based access controls, and compliance features for every major market.

## Architecture

```
┌──────────────────────────────────────────────────────────────────────────┐
│                          mcp-pos (39 tools)                               │
├──────────┬──────────┬──────────┬──────────┬──────────┬───────────────────┤
│   Cart   │ Payments │  Fiscal  │ Controls │ Loyalty  │   Compliance      │
├──────────┼──────────┼──────────┼──────────┼──────────┼───────────────────┤
│ Create   │ Cash     │ 22 e-inv │ Void     │ Points   │ Age verify        │
│ Add/Rmv  │ Card     │ standards│ Override │ Tiers    │ Env levy          │
│ Discount │ M-Pesa   │ Receipt  │ Suspend  │ Redeem   │ Meal voucher      │
│ Get      │ UPI      │ sign     │ Recall   │          │ EBT/SNAP          │
│          │ WeChat   │ Bilingual│ Roles    │          │ US state tax      │
│          │ QRIS     │ 24 langs │ Auth     │          │ Tip calc          │
│          │ PayNow   │          │          │          │                   │
│          │ EBT/SNAP │          │          │          │                   │
└──────────┴──────────┴──────────┴──────────┴──────────┴───────────────────┘
```

## Key Principles

- **23-country compliance** — native e-invoicing for every major tax authority.
- **20+ payment methods** — cash, card, mobile money, QR payments, EBT/SNAP, meal vouchers.
- **Role-based controls** — cashiers can't void; managers can. Configurable per-role limits.
- **Fiscal integrity** — SHA-256 hash chain on receipts (TSE/NF525 compliant).
- **24 receipt languages** — bilingual output for any market.
- **Zero configuration** — starts immediately with no API keys or external services.

## Tools (39)

### Cart Management (7)

| Tool | Description |
|------|-------------|
| `product_register` | Register product (SKU, barcode, price, tax rate) |
| `cart_create` | Start a new sale |
| `cart_add_item` | Scan barcode → auto price + tax |
| `cart_remove_item` | Remove item |
| `cart_apply_discount` | Percentage or fixed (per-item or whole cart) |
| `cart_get` | View cart contents and totals |
| `barcode_lookup` | Look up product by barcode/SKU |

### Payments (6)

| Tool | Description |
|------|-------------|
| `payment_process` | Cash (with change), card, mobile money |
| `payment_qr_generate` | Generate QR for customer scan (20+ providers) |
| `split_tender` | Multiple payment methods per transaction |
| `multi_currency_checkout` | Tourist pays foreign currency, shows FX |
| `ebt_snap_pay` | US food stamps (tax-exempt items) |
| `meal_voucher` | French Ticket Restaurant, Sodexo, Edenred |

### Fiscal & E-Invoicing (5)

| Tool | Description |
|------|-------------|
| `fiscal_config` | Set country, business PIN, device ID, currency |
| `einvoice_generate` | Generate e-invoice (22 standards) |
| `receipt_generate` | Thermal printer format |
| `receipt_sign` | SHA-256 hash chain (TSE/NF525) |
| `receipt_bilingual` | Dual-language receipt (24 languages) |

### Business Controls (9)

| Tool | Description |
|------|-------------|
| `cart_void` | Cancel transaction (role-gated) |
| `price_override` | Change price (role-gated + reason) |
| `cart_suspend` | Park a transaction |
| `cart_recall` | Recall parked transaction |
| `suspended_list` | List parked carts |
| `receipt_reprint` | Reprint marked as COPY |
| `role_set_limits` | Configure per-role permissions |
| `authorize_check` | Check if role can perform action |
| `age_verify` | Block underage sales |

### Shifts & Reports (3)

| Tool | Description |
|------|-------------|
| `shift_open` | Open register with float |
| `shift_close` | Close + cash reconciliation + variance |
| `daily_summary` | Sales totals, payment breakdown |

### Loyalty (2)

| Tool | Description |
|------|-------------|
| `loyalty_check` | Check points and tier |
| `loyalty_redeem` | Redeem points as payment |

### Regional (7)

| Tool | Description |
|------|-------------|
| `us_tax_lookup` | All 50 US states + DC sales tax |
| `tip_calculate` | Tip % suggestions (15/18/20/25) |
| `env_levy_add` | UK carrier bag, plastic tax |
| `tax_category_set` | HSN/SAC codes (India) |
| `buyer_identify` | Attach GSTIN/TRN/PIN to B2B |
| `refund` | Process return (approval-gated) |
| `tip_add` | Add gratuity |

## Installation

```bash
cargo install mcp-pos
```

### Client Configuration

```json
{
  "mcpServers": {
    "pos": { "command": "mcp-pos" }
  }
}
```

Works with Claude Desktop, Kiro, Cursor, Windsurf, Codex, and any MCP client.

## Global Coverage

### E-Invoice Standards (22)

| Region | Country | Standard | Tax Authority | VAT/GST |
|--------|---------|----------|---------------|:-------:|
| 🇬🇧 | UK | `hmrc_mtd` | HMRC | 20% |
| 🇩🇪 | Germany | `tse_de` | KassenSichV | 19%/7% |
| 🇫🇷 | France | `nf525_fr` | DGFiP | 20%/10%/5.5% |
| 🇮🇹 | Italy | `rt_it` | Agenzia Entrate | 22% |
| 🇪🇸 | Spain | `ticketbai_es` | Hacienda | 21%/10%/4% |
| 🇰🇪 | Kenya | `kra_etr` | KRA | 16% |
| 🇺🇬 | Uganda | `ura_efris` | URA | 18% |
| 🇹🇿 | Tanzania | `tra_efd` | TRA | 18% |
| 🇪🇹 | Ethiopia | `erca` | ERCA | 15% |
| 🇷🇼 | Rwanda | `rra_ebm` | RRA | 18% |
| 🇿🇦 | South Africa | `sars` | SARS | 15% |
| 🇳🇬 | Nigeria | `firs` | FIRS | 7.5% |
| 🇪🇬 | Egypt | `eta` | ETA | 14% |
| 🇮🇳 | India | `india_irn` | NIC/GSTN | 5-28% |
| 🇸🇦 | Saudi Arabia | `zatca` | ZATCA | 5% |
| 🇨🇳 | China | `fapiao` | Golden Tax | 13% |
| 🇸🇬 | Singapore | `invoicenow_sg` | IRAS (Peppol) | 9% |
| 🇲🇾 | Malaysia | `myinvois_my` | LHDN | 6% SST |
| 🇹🇭 | Thailand | `etax_th` | Revenue Dept | 7% |
| 🇮🇩 | Indonesia | `efaktur_id` | DJP | 11% |
| 🇵🇭 | Philippines | `cas_ph` | BIR | 12% |
| 🇻🇳 | Vietnam | `einvoice_vn` | GDT | 10% |

### Payment QR Codes (20)

| Provider | QR Type | Countries |
|----------|---------|-----------|
| M-Pesa | `mpesa` | 🇰🇪🇹🇿 |
| MTN MoMo | `mtn_momo` | 🇺🇬🇷🇼🇬🇭 |
| Airtel Money | `airtel_money` | 🇺🇬🇰🇪🇹🇿🇷🇼 |
| Telebirr | `telebirr` | 🇪🇹 |
| Tigo Pesa | `tigo_pesa` | 🇹🇿 |
| SnapScan | `snapscan` | 🇿🇦 |
| OPay | `opay` | 🇳🇬 |
| PalmPay | `palmpay` | 🇳🇬 |
| Fawry | `fawry` | 🇪🇬 |
| UPI | `upi` | 🇮🇳 |
| WeChat Pay | `wechat` | 🇨🇳 |
| Alipay | `alipay` | 🇨🇳 |
| PayNow | `paynow_sg` | 🇸🇬 |
| DuitNow | `duitnow_my` | 🇲🇾 |
| PromptPay | `promptpay_th` | 🇹🇭 |
| QRIS | `qris_id` | 🇮🇩 (GoPay, OVO, Dana, ShopeePay) |
| GCash | `gcash_ph` | 🇵🇭 |
| Maya | `maya_ph` | 🇵🇭 |
| VNPay | `vnpay` | 🇻🇳 |
| MoMo VN | `momo_vn` | 🇻🇳 |
| ZATCA QR | `zatca` | 🇸🇦 |

### Bilingual Receipts (24 languages)

| Code | Language | Region |
|:----:|----------|--------|
| `en` | English | Global |
| `sw` | Swahili | 🇰🇪🇹🇿 |
| `am` | Amharic | 🇪🇹 |
| `rw` | Kinyarwanda | 🇷🇼 |
| `zu` | Zulu | 🇿🇦 |
| `af` | Afrikaans | 🇿🇦 |
| `yo` | Yoruba | 🇳🇬 |
| `ha` | Hausa | 🇳🇬 |
| `ar` | Arabic | 🇸🇦🇦🇪🇪🇬 |
| `hi` | Hindi | 🇮🇳 |
| `zh` | Chinese | 🇨🇳 |
| `ja` | Japanese | 🇯🇵 |
| `fr` | French | 🇫🇷🇷🇼 |
| `de` | German | 🇩🇪🇦🇹🇨🇭 |
| `it` | Italian | 🇮🇹 |
| `es` | Spanish | 🇪🇸 |
| `pt` | Portuguese | 🇵🇹🇧🇷 |
| `nl` | Dutch | 🇳🇱🇧🇪 |
| `pl` | Polish | 🇵🇱 |
| `th` | Thai | 🇹🇭 |
| `ms` | Malay | 🇲🇾 |
| `id` | Bahasa Indonesia | 🇮🇩 |
| `vi` | Vietnamese | 🇻🇳 |
| `tl` | Filipino | 🇵🇭 |

### US Sales Tax (50 states + DC)

| Rate Range | States |
|:----------:|--------|
| 0% | OR, NH, MT, DE, AK |
| 2.9–4.5% | CO, MO, AL, GA, HI, WY, SD, OK, LA, NM, NY |
| 5–6.25% | WI, ND, ME, NE, VA, AZ, OH, PA, MD, MI, IA, KY, SC, WV, VT, DC, FL, ID, IL, MA, TX, CT |
| 6.5–7.25% | NJ, NV, MN, KS, WA, IN, MS, RI, TN, AR, CA |

## Transaction Flow

```
shift_open
    │
    ├── cart_create
    │       │
    │       ├── cart_add_item (barcode scan)
    │       ├── cart_add_item (repeat)
    │       ├── cart_apply_discount (coupon/loyalty)
    │       │
    │       ├── [age_verify if restricted items]
    │       ├── [env_levy_add if bags needed]
    │       ├── [buyer_identify if B2B]
    │       │
    │       ├── payment_process / split_tender / payment_qr_generate
    │       │
    │       ├── receipt_generate + receipt_sign
    │       ├── einvoice_generate (fiscal submission)
    │       │
    │       └── [loyalty points accrued]
    │
    ├── [cart_suspend → cart_recall] (parked transactions)
    ├── [cart_void] (requires manager auth)
    ├── [refund] (requires approval)
    │
    └── shift_close (reconcile cash, detect variance)
            │
            └── daily_summary
```

## Role-Based Access Control

| Role | Max Discount | Void | Refund | Price Override | Configurable |
|------|:---:|:---:|:---:|:---:|:---:|
| Cashier | 5% | ❌ | ❌ | ❌ | ✅ |
| Supervisor | 20% | ✅ | ✅ | ✅ | ✅ |
| Manager | 100% | ✅ | ✅ | ✅ | ✅ |

Use `role_set_limits` to customize per-role permissions. Use `authorize_check` to verify before performing restricted actions.

## Quick Start

### 1. Configure for your country

```json
{"name": "fiscal_config", "arguments": {"country": "KE", "business_name": "Duka Mart", "business_pin": "P051234567A", "device_id": "ETR-CU-001"}}
```

### 2. Register products

```json
{"name": "product_register", "arguments": {"sku": "MILK-1L", "barcode": "5901234123457", "name": "Fresh Milk 1L", "price": 120, "tax_rate": 16}}
```

### 3. Process a sale

```json
{"name": "cart_create", "arguments": {"cashier": "mary"}}
{"name": "cart_add_item", "arguments": {"cart_id": "cart_abc", "barcode": "5901234123457", "quantity": 2}}
{"name": "payment_qr_generate", "arguments": {"cart_id": "cart_abc", "qr_type": "mpesa", "merchant_id": "123456"}}
{"name": "payment_process", "arguments": {"cart_id": "cart_abc", "method": "mobile_money"}}
{"name": "receipt_generate", "arguments": {"cart_id": "cart_abc"}}
{"name": "einvoice_generate", "arguments": {"cart_id": "cart_abc", "standard": "kra_etr"}}
```

### 4. US restaurant with tip

```json
{"name": "tip_calculate", "arguments": {"cart_id": "cart_abc", "tip_pct": 20, "on_pretax": true}}
{"name": "ebt_snap_pay", "arguments": {"cart_id": "cart_abc", "card_last4": "4567"}}
```

## Error Codes

| Code | Meaning |
|------|---------|
| `PRODUCT_NOT_FOUND` | Barcode/SKU not in catalog |
| `CART_NOT_FOUND` | Cart ID doesn't exist |
| `CART_NOT_OPEN` | Already checked out or voided |
| `UNAUTHORIZED` | Role lacks permission for action |
| `INSUFFICIENT_POINTS` | Not enough loyalty points |
| `INSUFFICIENT_PAYMENT` | Split tender total < cart total |
| `SHIFT_NOT_FOUND` | Shift ID doesn't exist |
| `SUSPENDED_CART_NOT_FOUND` | Parked cart not found |
| `ITEM_NOT_IN_CART` | SKU not in current cart |

## Configuration

| Variable | Required | Purpose |
|----------|:--------:|---------|
| `RUST_LOG` | No | Log level |

No API keys needed. All functionality is self-contained.

## Integration

| Server | How it connects |
|--------|----------------|
| `mcp-inventory` | `stock_issue` on sale, `stock_receive` on return |
| `mcp-pricing` | Dynamic pricing, CEL rules, promotions |
| `mcp-messaging` | Send receipt via SMS/push/WhatsApp |
| `mcp-analytics` | Sales dashboards, trends, forecasting |
| `mcp-workflow` | Approval workflows for voids/refunds |

## Documentation

| Document | Description |
|----------|-------------|
| [mcp-server.toml](mcp-server.toml) | ADK-Rust Enterprise registry manifest |
| [CHANGELOG.md](CHANGELOG.md) | Version history |
| [Rust Docs](https://docs.rs/mcp-pos) | Generated API documentation |

## Contributing

Contributions welcome. Priority areas:
- Persistent storage (SQLite/PostgreSQL)
- Kitchen Display System (KDS) for restaurants
- Table management
- Offline mode with sync
- Barcode image generation
- Integration with physical hardware (receipt printers, cash drawers, scales)

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)

## Registry Compliance

This server implements the [ADK MCP SDK](https://crates.io/crates/adk-mcp-sdk) contract:

- **HealthCheck** — async health probe for registry monitoring
- **mcp-server.toml** — manifest declaring tools, risk classes, and credentials
- **Structured tracing** — `RUST_LOG` env-filter for observability
- **Fiscal audit** — hash-chained receipts, role-gated financial actions
