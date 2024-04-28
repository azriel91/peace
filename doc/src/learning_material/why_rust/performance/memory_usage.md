# Memory Usage

Stack memory is used when possible, heap when required / requested.

A field in a struct does not necessarily mean another pointer dereference.

## Java

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 50%; flex: 1 0;">

```java
class Inner { int value;   }
class Outer { Inner inner; }

public static void main(String[] ag) {
    var s = "hello";

    var outer = new Outer();
    outer.inner = new Inner();
    inner.value = 123;

    System.out.println(
        "" + outer.inner.value
    );
}
```

</div>
<div style="flex-basis: 50%; flex: 1 0;">

<object
    type="image/svg+xml"
    data="memory_java.svg"
    width="150"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQAsEsFMCcENYGNwE8BcoAEXzXgA6Y456ED6A9gK4Auc6WA3gL7YlkHkDOtskAOwDmjVu1L4uggQ2ZscveEgDWxEopXkAZgNHz1tJcvIA3ROQCMe8Vg3GzscgAZroAZQAm0cgPgBbaG41O0YAIgAeACMAPgBlQxVwgHoY0PZORiwImIAJSWTU9jttXRIsv3hBAAoASjSFBPtzFyynAA8Adidu%2BttG03MrVs7ugA5ezh4%2BQREsgB1QvAAbJcoFickqOlksrHaupyQNimkd0LKLACYAZjTQaA8hQOD%2Bh2dycknefmFGAG1im8nAAaXCbb4zAC6RVegw%2Bkxo9FgmQBsMcFlBCO2sGhEgoiLg8M2p2RJD%2BWKRmOJAhkONAoFoeACL1QS2eNncXnIXi08GoS1oQWYWGgbT48EYAHIkNRYNxKLBgARKIIkVhYDQBF4PMAlkJJVh9DgHk9udBefzBaIRWKEFKZXKFUqVQIkQajX0jCVGAAyOzC7jgQjeJCUVakyXcZSod02QHNRgAKjscbRliTKbKX2mvywPs4AaDBBDYYVUqWkACsazmwJpMTnBskxJScbZQrQnAtF9Ha7NhwWkgKx4Qa57lgFSWUos3Ul-awg%2BHgfgXPAlBMsklAFYnHOygOh0sRyvvFpKDKhZKZ7v54uj8uuUpaJAN9PZ-PvpRlN4H95x5OpQAFnffc%2Bg1b9j1XddN2uED90-CDf3IJ8X2gKUdxvUCEJ-WhWTQrAPEoWh6A8ecCBXDwZila49wMIw1DKcJwiTXtaA-IsS3DRhIiWahoCbSQGJIJiWMgTs2KwjjyFDLisFQaAVkoAB3ekyhNbxWN9dTyFYj8%2BC-H9Rz-BUAKwSUADY4LKO9IOMid4CnMyMNohovSBIkKAhX55xErBE203TJODaTS1JIRYGgaABFTNy4U%2BWtsSEnBfP8x4NLEvsguLELZJ4viBPxbEPKkGkGB85i-ICjKJPgqSZLLLATEgMNoDYoA)</small>

</div>
</div>

## Rust

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 50%; flex: 1 0;">

```rust ,ignore
struct Inner { value: i32   }
struct Outer { inner: Inner }

fn main() {
    let mut s = String::new();
    s.push_str("hello");

    let outer = Outer {
        inner: Inner {
            value: 123
        }
    };

    println!("{}", outer.inner.value);
}
```

</div>
<div style="flex-basis: 50%; flex: 1 0;">

<object
    type="image/svg+xml"
    data="memory_rust.svg"
    width="150"></object>
<small>[](https://azriel.im/dot_ix/?src=LQhQAsEsFMCcENYGNwE8BcoAEXzXgA6Y456ED6AzgC6yQB2A5ulgN4C%2B2WN8SA1sRI9%2B5AGb0WHLjmF9yAN0TkADJM4lu1XnMWxyARjWhQ9APYATaOXrwAttEqDZLAEQAeAEYA%2BAMpb%2BbgD03i5cZEQ47t4AEvgEQSFcsmISGi628AwAFACUoTL%2BOkqqWC7KAB4A7Mo1%2BZraCkqGpST6AEwAzHXhVLQMzKUAOi54ADajpsOhoNDmjA5OhY16yuTkPTR0TCwA2sm6KgA0uHG9W4wAusbUePaLqKML0lhmluSWovAArqPUjmxYaDlWjwFgAciQX1glFMsGABFMDGocCwsFMX3olnMwFGjDBWHUJFm83e0E%2BPz%2BkkBwIQ4Mh0Nh8MR9GRsHxhIKDXELAAZLIAZRwIQrEhTBNYODKHxUOznvtiiwAFSyOVLA7NZWFZ4bPrbLA88ICoUEEVi2Hg0aQeyyjSWxjgai8u0O544USQcZUIVvMywDKjcH6Gpg11Yd2ewXwN7gUzyODggCsyhDGjdHtGXqjVlEpkh-zBQeTofDGcjb141EgccDwdDm1MfCsZasvv94IALLXU5o0Y3M9HY-GsGCOl3U-W%2B83yBWq9BE2ONBOm9QHnOsOZTNRkeZQwQo%2BZ%2BuCOimhIVBBo3G4lc7qHXjabxSwPKMvtBtXFzyRL9fIPbb93IxNchRUfLBUGgcZTAAd1AIA)</small>

</div>
</div>
