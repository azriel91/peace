# Equality - Rust

```rust
#[derive(PartialEq)]
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

### Referential Equality

```rust
#[derive(Clone, PartialEq)]
struct Data(String);

let data_0 = Data(String::from("hello"));
let data_1 = data_0.clone();

assert!(data_0 == data_1);
assert!(std::ptr::eq(&data_0, &data_1) == false);
println!("ok!");
```
