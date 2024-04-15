# Linter

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 45%;">

```rust
# let maybe_n = Some(123);

match maybe_n {
    Some(n) => println!("{n}"),
    _ => (),
}
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
# let maybe_n = Some(123);

if let Some(n) = maybe_n {
    println!("{n}")
}
```

</div>
</div>

> warning:
>
> you seem to be trying to use `match` for destructuring a single pattern.
>
> Consider using `if let`

[Clippy lints index](https://rust-lang.github.io/rust-clippy/master/index.html)
