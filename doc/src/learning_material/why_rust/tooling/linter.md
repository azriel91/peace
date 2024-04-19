# Linter

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 45%;">

```rust
# fn main() {
# let maybe_n = Some(123);

match maybe_n {
    Some(n) => println!("{n}"),
    _ => (),
}
# }
```

</div>
<div
    style="
        flex-basis: 10%;
        text-align: center;
        display: flex;
        flex-direction: column;
        justify-content: center;
    ">
<div>📎&nbsp;Clippy</div>
<div style="font-size: 30px;">➡️</div>
</div>
<div style="flex-basis: 45%;">

```rust
# fn main() {
# let maybe_n = Some(123);

if let Some(n) = maybe_n {
    println!("{n}")
}
# }
```

</div>
</div>

> warning: you seem to be trying to use `match` for destructuring a single pattern.<br />
> Consider using `if let`

```patch
# fn main() {
# let maybe_n = Some(123);

+#[allow(clippy::single_match)] // deliberately allow this once
 match maybe_n {
     Some(n) => println!("{n}"),
     _ => (),
 }
# }
```

[Clippy lints index](https://rust-lang.github.io/rust-clippy/master/index.html)

<div class="hidden">

1. Rust ships with a linter
2. It's called clippy, also a very original name.
3. If you write the code on the left, it will tell you to write code on the right.
4. This is really nice because it's teaching you how to write cleaner code.

5. Clippy has over 9000 lints.

</div>
