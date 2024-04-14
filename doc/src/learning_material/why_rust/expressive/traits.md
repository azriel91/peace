# Traits

```rust
# use std::time::Duration;
# fn main() {
let duration =
    Duration::from_secs(3 * 60)
    + Duration::from_secs(15)
    + Duration::from_secs(47);

println!("{} seconds", duration.as_secs());
# }
```

---

```rust
# fn main() {
let duration = 3.minutes() + 15.seconds() + 47.seconds();

println!("{}", duration.for_humans());
# }
#
# trait IntoDuration {
#     fn seconds(self) -> std::time::Duration;
#     fn minutes(self) -> std::time::Duration;
# }
#
# impl IntoDuration for u64 {
#     fn seconds(self) -> std::time::Duration {
#         std::time::Duration::from_secs(self)
#     }
#     fn minutes(self) -> std::time::Duration {
#         std::time::Duration::from_secs(self * 60)
#     }
# }
#
# trait ReadableExt {
#     fn for_humans(&self) -> String;
# }
#
# impl ReadableExt for std::time::Duration {
#     fn for_humans(&self) -> String {
#         use std::fmt::Write;
#         let mut buffer = String::with_capacity(128);
#         let total = self.as_secs();
#         let mins = total.div_euclid(60);
#         let seconds = total.rem_euclid(60);
#         match mins {
#             1 => write!(&mut buffer, "{mins} minute").unwrap(),
#             _ => write!(&mut buffer, "{mins} minutes").unwrap(),
#         }
#         match seconds {
#             1 => write!(&mut buffer, " {seconds} second").unwrap(),
#             _ => write!(&mut buffer, " {seconds} seconds").unwrap(),
#         }
#         buffer
#     }
# }
```

<!--
* In Rust, we can write code, that is very readable, *and* still retain strong type checking.
* Over here we have 3 minutes, plus 15 seconds, plus 47 seconds.
* and when we run it, we get 4 minutes 2 seconds.
* We have this bit `for_humans()` here because it's easier to understand 4 minutes and 2 seconds, compared to 242,000 milliseconds.
* Because imagine, if one of you asked, "how long will the talk be?"
* and I replied, "2,700,000 milliseconds."
-->
