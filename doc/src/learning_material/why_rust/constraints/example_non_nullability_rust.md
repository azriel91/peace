# Example: Non-nullability - Rust

```rust
fn do_something(s: String) {
    // s can *never!* be `null`
    println!("{s}");
    println!("{}", s.len());
}
```

```rust
# fn do_something(s: String) {
#     println!("{s}");
#     println!("{}", s.len());
# }
#
#
#
# fn main() {
do_something("hello".into());
# }
```

```rust
# fn do_something(s: String) {
#     println!("{s}");
#     println!("{}", s.len());
# }
#
#
#
# fn main() {
do_something(None);
# }
```

```rust
# fn do_something(s: String) {
#     println!("{s}");
#     println!("{}", s.len());
# }
#
#
#
# fn main() {
do_something();
# }
```

## Accepting "nothing"

```rust
fn do_something(s_opt: Option<String>) {
    match s_opt {
        None => println!("it's nothing!"),
        Some(s) => {
            println!("{s}");
            println!("{}", s.len());
        }
    }
}

fn main() {
    do_something(Some("hello".into()));
    do_something(None);
}
```
