# Swedihs Postal Codes

Validate Swedish postal codes in Rust. The project comes bundled with a list of
valid postal codes. None of them will be revoked manually but it's quite
uncommon for those to get removed. It's however more common for new postal codes
to be added. To support this, the library has an optional fallback API that will
check for valid codes if not found in the CSV.

**Note** by turning this feature on, every invalid request will perform a new
HTTP request.

## Usage

```toml
[dependencies]
swedish-postal-codes = "0.1.0"
```

```rust
use swedish_postal_codes::PostalCode;

fn main() {
    let fallback = true;
    let pc = PostalCode::new(fallback);

    let from_integer = 11220;
    println!("{}: {}", from_integer, pc.valid(from_integer));

    let from_string = "11120";
    println!("{}: {}", from_string, pc.valid(from_string));

    let invalid = 55555i64;
    println!("{}: {}", invalid, pc.valid(invalid));
}
```
