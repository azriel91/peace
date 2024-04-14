# Traits 2

Traits are *like* interfaces, that you can implement on existing types.

```rust ,no_run
fn turn_on_heater<T>(t: T)
where
    T: TemperatureExt,
{
    if t.is_cold() {
        println!("❄️ It's cold, turning on heater");
    } else {
        println!("🔥 It's not cold, not turning on heater.");
    }
}
# fn main() {}
```

```rust ,no_run
#
#
#
#
#
#
#
#
trait TemperatureExt {
    fn is_cold(self) -> bool;
}
```

```rust
# fn turn_on_heater<T>(t: T)
# where
#     T: TemperatureExt,
# {
#     if t.is_cold() {
#         println!("❄️ It's cold, turning on heater");
#     } else {
#         println!("🔥 It's not cold, not turning on heater.");
#     }
# }
# trait TemperatureExt {
#     fn is_cold(self) -> bool;
# }
fn main() {
    turn_on_heater(2);
    turn_on_heater("freezing");
    turn_on_heater("warm");
    // turn_on_heater(Vec::new()); // compile error
}

impl TemperatureExt for i32 {
    fn is_cold(self) -> bool {
        self < 20
    }
}

impl<'s> TemperatureExt for &'s str {
    fn is_cold(self) -> bool {
        self == "freezing"
    }
}
```
