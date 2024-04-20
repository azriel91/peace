# Multiple Values

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 50%; flex: 1 0;">

```rust
fn fetch() -> String {
    "azriel".into()
}

# fn main() {
let name = fetch();
println!("{name}");
# }
```

</div>
<div style="flex-basis: 50%; flex: 1 0;">

```rust
fn fetch() -> (String, u32) {
    ("azriel".into(), 123)
}

# fn main() {
let (name, number) = fetch();
println!("{name} {number}");
# }
```

</div>
</div>
