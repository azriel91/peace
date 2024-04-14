# Equality - Rust

```rust
struct Data(u32);

# fn main() {
#
let ab = "ab".to_string();
let c = "c";
# let equality = [

// Which of the following are true / false?
"abc" == "abc",
"abc" == "ab".to_string() + "c",
"abc" == ab + c,
123 == 123,
Data(123) == Data(123),
#
# ];
#
# for equal in equality.iter() {
#     println!("{equal}");
# }
#
# }
```
