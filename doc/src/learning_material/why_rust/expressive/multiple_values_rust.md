# Multiple Values - Rust

```rust
fn fetch() -> String {
    "azriel".into()
}

# fn main() {
let name = fetch();
println!("{name}");
# }
```

---

```rust
fn fetch() -> (String, u32) {
    ("azriel".into(), 123)
}

# fn main() {
let (name, number) = fetch();
println!("{name} {number}");
# }
```
