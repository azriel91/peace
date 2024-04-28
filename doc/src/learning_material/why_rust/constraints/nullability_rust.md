# Nullability - Rust

```rust ,ignore
fn print(s: String) {
    println!("{s}");
    println!("{}", s.len());
}
```

```rust
# fn print(s: String) {
#     println!("{s}");
#     println!("{}", s.len());
# }
# fn main() {
print();
# }
```

```rust
# fn print(s: String) {
#     println!("{s}");
#     println!("{}", s.len());
# }
#
# fn main() {
print(None);
# }
```

```rust
# fn print(s: String) {
#     println!("{s}");
#     println!("{}", s.len());
# }
#
#
# fn main() {
print("hello".into());
# }
```



## Accepting "nothing"

```rust
fn print(s_opt: Option<String>) {
    match s_opt {
        None => println!("nothing!"),
        Some(s) => {
            println!("{s}");
            println!("{}", s.len());
        }
    }
}

fn main() {
    print(None);
    print(Some("hello".into()));
}
```
