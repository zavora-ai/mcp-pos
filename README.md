# Point of Sale MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-pos.svg)](https://crates.io/crates/mcp-pos)
[![Docs.rs](https://docs.rs/mcp-pos/badge.svg)](https://docs.rs/mcp-pos)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://enterprise.adk-rust.com)

Point of Sale engine for [ADK-Rust Enterprise](https://enterprise.adk-rust.com) agents. Provides 15 MCP tools covering the full retail transaction lifecycle — cart management, multi-method payments, thermal receipt generation, shift reconciliation, loyalty programs, and barcode scanning. **Zero configuration, no external dependencies.**

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     mcp-pos (15 tools)                        │
├───────────┬───────────┬───────────┬───────────┬─────────────┤
│   Cart    │ Payments  │  Shifts   │  Loyalty  │  Receipts   │
├───────────┼───────────┼───────────┼───────────┼─────────────┤
│ Create    │ Cash      │ Open      │ Check     │ Thermal     │
│ Add item  │ Card      │ Close     │ Redeem    │ format      │
│ Remove    │ M-Pesa    │ Reconcile │ Points    │             │
│ Discount  │ Refund    │ Summary   │ Tiers     │             │
│ Barcode   │ Split     │           │           │             │
└───────────┴───────────┴───────────┴───────────┴─────────────┘
         │                                           │
         ▼                                           ▼
   mcp-inventory                              mcp-pricing
   (stock_issue on sale)                 (dynamic pricing)
```

## Key Principles

- **Transaction-first** — optimized for the checkout flow: scan → total → pay → receipt.
- **Multi-payment** — cash (with change), card, mobile money (M-Pesa/Flutterwave), loyalty points.
- **Shift management** — open/close registers with cash reconciliation and variance detection.
- **Thermal receipts** — generates formatted text ready for 80mm thermal printers.
- **Loyalty built-in** — points accrual, tier management, redemption as payment.
- **Zero configuration** — starts immediately with no API keys or external services.

## Tools (15)

### Product Catalog

| Tool | Description | Risk Class |
|------|-------------|:----------:|
| `product_register` | Register product (SKU, barcode, name, price, tax rate) | write |
| `barcode_lookup` | Look up product by barcode or SKU | read-only |

### Cart Management

| Tool | Description | Risk Class |
|------|-------------|:----------:|
| `cart_create` | Start a new sale/transaction | write |
| `cart_add_item` | Scan barcode to add item (auto price + tax) | write |
| `cart_remove_item` | Remove item from cart | write |
| `cart_apply_discount` | Apply percentage or fixed discount | write |
| `cart_get` | View cart contents and totals | read-only |

### Payments

| Tool | Description | Risk Class |
|------|-------------|:----------:|
| `payment_process` | Process payment (cash/card/mobile_money) | financial |
| `refund` | Process return/refund (requires approval) | financial |

### Receipts

| Tool | Description | Risk Class |
|------|-------------|:----------:|
| `receipt_generate` | Generate thermal printer receipt | read-only |

### Shift Management

| Tool | Description | Risk Class |
|------|-------------|:----------:|
| `shift_open` | Open register with opening float | write |
| `shift_close` | Close shift, reconcile cash, detect variance | write |

### Loyalty

| Tool | Description | Risk Class |
|------|-------------|:----------:|
| `loyalty_check` | Check customer points and tier | read-only |
| `loyalty_redeem` | Redeem points as payment on cart | write |

### Reports

| Tool | Description | Risk Class |
|------|-------------|:----------:|
| `daily_summary` | Sales totals, payment breakdown, avg transaction | read-only |

## Installation

### From crates.io

```bash
cargo install mcp-pos
```

### Build from source

```bash
git clone https://github.com/zavora-ai/mcp-pos
cd mcp-pos
cargo build --release
```

### Claude Desktop

```json
{
  "mcpServers": {
    "pos": { "command": "mcp-pos" }
  }
}
```

### Kiro

Add to `.kiro/settings/mcp.json`:

```json
{
  "mcpServers": {
    "pos": { "command": "mcp-pos" }
  }
}
```

### Cursor / Windsurf / Codex

```json
{
  "mcpServers": {
    "pos": { "command": "mcp-pos" }
  }
}
```

## Quick Start

### 1. Register products

```json
{"name": "product_register", "arguments": {"sku": "MILK-1L", "barcode": "5901234123457", "name": "Fresh Milk 1L", "price": 120, "tax_rate": 16}}
{"name": "product_register", "arguments": {"sku": "BREAD-WH", "barcode": "6001234567890", "name": "White Bread", "price": 65, "tax_rate": 0}}
```

### 2. Open shift

```json
{"name": "shift_open", "arguments": {"cashier": "mary", "register_id": "REG-01", "opening_float": 5000}}
```

### 3. Create cart and scan items

```json
{"name": "cart_create", "arguments": {"cashier": "mary", "customer_id": "CUST-001"}}
{"name": "cart_add_item", "arguments": {"cart_id": "cart_abc123", "barcode": "5901234123457", "quantity": 2}}
{"name": "cart_add_item", "arguments": {"cart_id": "cart_abc123", "barcode": "6001234567890"}}
```

### 4. Apply discount

```json
{"name": "cart_apply_discount", "arguments": {"cart_id": "cart_abc123", "discount_type": "percentage", "value": 10}}
```

### 5. Process payment

```json
{"name": "payment_process", "arguments": {"cart_id": "cart_abc123", "method": "cash", "tendered": 400}}
```

**Response:**
```json
{"status": "completed", "payment_id": "pay_abc", "method": "cash", "amount": 343.40, "tendered": 400, "change": 56.60}
```

### 6. Generate receipt

```json
{"name": "receipt_generate", "arguments": {"cart_id": "cart_abc123"}}
```

**Output:**
```
================================
         SALES RECEIPT          
================================
Date: 2026-05-27
Cashier: mary
--------------------------------
Fresh Milk 1L            x2
  KES 278.40
White Bread              x1
  KES 65.00
--------------------------------
Subtotal:    KES 305.00
Tax:         KES 38.40
TOTAL:       KES 343.40
--------------------------------
CASH: KES 343.40
Change: KES 56.60
================================
      Thank you! Come again     
================================
```

### 7. Close shift

```json
{"name": "shift_close", "arguments": {"shift_id": "shift_abc", "counted_cash": 5340}}
```

**Response:**
```json
{"status": "closed", "transactions": 12, "cash_sales": 4200, "card_sales": 1800, "mobile_sales": 950, "total_sales": 6950, "expected_cash": 5343.40, "counted_cash": 5340, "variance": -3.40}
```

## Payment Methods

| Method | How it works |
|--------|-------------|
| `cash` | Tendered amount → calculates change |
| `card` | Full amount charged, no change |
| `mobile_money` | M-Pesa/Flutterwave, reference stored |
| `refund` | Negative amount, requires approval gate |

## Transaction Flow

```
product_register (setup)
        │
shift_open (start of day)
        │
cart_create ──→ cart_add_item ──→ cart_apply_discount
        │                                    │
        │              cart_get (review) ◄────┘
        │                    │
        ▼                    ▼
payment_process ──→ receipt_generate
        │
        ├── loyalty points accrued
        │
        └── shift totals updated
        
shift_close (end of day, reconcile cash)
daily_summary (reporting)
```

## Loyalty Tiers

| Tier | Points Required | Benefits |
|------|:-:|---|
| Bronze | 0 | 1 point per KES 100 spent |
| Silver | 500 | 1.5x points multiplier |
| Gold | 2000 | 2x points, priority service |
| Platinum | 5000 | 3x points, exclusive offers |

Points redeem at 1 point = 1 currency unit.

## Integration with Other MCP Servers

| Server | Integration |
|--------|-------------|
| `mcp-inventory` | `stock_issue` on sale, `stock_receive` on return |
| `mcp-pricing` | Dynamic pricing, promotions, surge |
| `mcp-messaging` | Send receipt via SMS/push |
| `mcp-analytics` | Sales dashboards, trends |

## Configuration

### Environment Variables

| Variable | Required | Purpose |
|----------|:--------:|---------|
| `RUST_LOG` | No | Log level (default: `info`) |

No API keys needed. All functionality is self-contained.

### MCP Server Manifest

```toml
server_id = "mcp_pos"
display_name = "Point of Sale"
version = "1.0.0"
domain = "retail"
risk_level = "medium"
writes_allowed = "gated"
governance_gates = ["payment_audit"]
```

## Error Codes

| Code | Meaning |
|------|---------|
| `PRODUCT_NOT_FOUND` | Barcode/SKU not in catalog |
| `CART_NOT_FOUND` | Cart ID doesn't exist |
| `CART_NOT_OPEN` | Cart already checked out or voided |
| `SHIFT_NOT_FOUND` | Shift ID doesn't exist |
| `INSUFFICIENT_POINTS` | Not enough loyalty points to redeem |

## Documentation

| Document | Description |
|----------|-------------|
| [mcp-server.toml](mcp-server.toml) | ADK-Rust Enterprise registry manifest |
| [Rust Docs](https://docs.rs/mcp-pos) | Generated API documentation |

## Contributing

Contributions welcome. Priority areas:
- Split payment (multiple methods per transaction)
- Hold/park transactions
- Offline mode with sync
- Kitchen display system (KDS) integration
- Table management (restaurants)
- Weighing scale integration

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
- **Payment audit** — all financial transactions logged with actor and timestamp
