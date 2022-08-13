# Resources

In Peace, [`Resources`][`Resources`] refers to an *any-map* &ndash; a map that can store one value of different types &ndash; whose borrowing rules are checked per entry at runtime, instead of compile time.

Example of an any-map:

| Key                    | Value               |
| ---------------------- | ------------------- |
| `TypeId::of::<u32>()`  | `1u32`              |
| `TypeId::of::<bool>()` | `true`              |
| `TypeId::of::<A>()`    | `A { value: 1u32 }` |


```rust ,edition2021,ignore
# use peace::resources::Resources;
#
let mut resources = Resources::new();
resources.insert(1u32);
resources.insert(2u64);

// Change `resources` to be immutable.
let resources = resources;

// We can validly have two mutable borrows
// from the map!
let mut a = resources.borrow_mut::<u32>();
let mut b = resources.borrow_mut::<u64>();
*a = 2;
*b = 3;

// Accessing the same value while it is already
// mutably borrowed returns an error.
assert!(resources.try_borrow::<u32>().is_err())
```

For more information about the underlying type, see [`resman`][`resman`].


## Java Equivalent

The conceptual Java equivalent looks like this:

```java
var resources = new MagicMap(); // new HashMap<Class<?>, Object>();
resources.insert((Integer) 1);  // innerMap.put(Integer.class, 1);
resources.insert((Long)    2);  // innerMap.put(Long.class,    2L);

var anInt = (Integer) resources.get(Integer.class);
var aLong = (Long)    resources.get(Long.class);

// Can mutate both.
anInt = 2;
aLong = 3L;
```

Rust's does not allow access to multiple mutable entries at the same time with the built in `HashMap`, so `Resources` is an implementation to bypass the compilation strictness.


[`Resources`]: https://docs.rs/peace_resources/latest/peace_resources/struct.Resources.html
[`resman`]: https://github.com/azriel91/resman
