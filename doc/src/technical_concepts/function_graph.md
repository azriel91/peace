# Function Graph

A [`fn_graph`][`fn_graph`] is a data structure that holds functions, tracks their dependencies, and streams the functions in dependency order.

This is the basis of a framework that enables:

* Consumer provided logic and data.
* Framework controlled invocation of logic.

When consumers can plug in their logic and data to the framework, control now resides with the framework on invoking that logic and presenting the data in a desired form.


## Scenario

Consider the following program:

1. Initialize two inputs, `a` and `b`, and two outputs `c` and `d`.
2. Add `a` to `c`.
3. Add `b` to `c`.
4. Add `c` to `d`.
5. Print the values of `a`, `b`, and `c`.


### Static Representation

The above program can be represented by the following code:

```rust
// Define data to work on.
let mut a = -1;
let mut b = -1;
let mut c = -1;
let mut d = -1;

# type Fn1 = fn(&mut i32, &mut i32, &mut i32, &mut i32);
# type Fn2 = fn(&i32, &mut i32);
# type Fn3 = fn(&i32, &mut i32);
# type Fn4 = fn(&i32, &mut i32);
# type Fn5 = fn(&i32, &i32, &i32);
#
// Define logic.
let fn1: Fn1 = |a, b, c, d| { *a = 1; *b = 2; *c = 0; *d = 0; };
let fn2: Fn2 = |a,    c   | *c += *a;
let fn3: Fn3 = |   b, c   | *c += *b;
let fn4: Fn4 = |      c, d| *d += *c;
let fn5: Fn5 = |a, b, c   | println!("{a} + {b} = {c}");

// Invoke logic over data.
fn1(&mut a, &mut b, &mut c, &mut d);
fn2(&a, &mut c);
fn3(&b, &mut c);
fn4(&c, &mut d);
fn5(&a, &b, &c);
```


### Dynamic Representation

The following shows the complete logic implemented using [`fn_graph`][`fn_graph`]:

```rust ,ignore
# // [dependencies]
# // fn_graph = { version = "0.5.4", features = ["fn_meta", "resman"] }
# // futures = "0.3.21"
# // resman = { version = "0.15.0", features = ["fn_meta", "fn_res"] }
# // tokio = { version = "1.20.0", features = ["rt", "macros", "time"] }
# use std::{
#     fmt,
#     ops::{AddAssign, Deref, DerefMut},
# };
#
# use fn_graph::FnGraphBuilder;
# use futures::stream::StreamExt;
# use resman::{IntoFnRes, Resources};
#
// Define newtypes for each parameter.
#[derive(Debug)] struct A(u32);
#[derive(Debug)] struct B(u32);
#[derive(Debug)] struct C(u32);
#[derive(Debug)] struct D(u32);

# fn main() {
// Initialize data.
let mut resources = Resources::new();
resources.insert(A(0));
resources.insert(B(0));
resources.insert(C(0));
resources.insert(D(0));
let resources = &resources; // Now the map is compile time immutable.

# type Fn1 = fn(&mut A, &mut B, &mut C, &mut D);
# type Fn2 = fn(&A, &mut C);
# type Fn3 = fn(&B, &mut C);
# type Fn4 = fn(&C, &mut D);
# type Fn5 = fn(&A, &B, &C);
#
// Define logic and insert them into graph structure.
let fn_graph = {
    let fn1: Fn1 = |a, b, c, d| { a.0 = 1; b.0 = 2; c.0 = 0; d.0 = 0; };
    let fn2: Fn2 = |a,    c   | *c += a.0;
    let fn3: Fn3 = |   b, c   | *c += b.0;
    let fn4: Fn4 = |      c, d| *d += c.0;
    let fn5: Fn5 = |a, b, c   | println!("{a} + {b} = {c}");

    let mut fn_graph_builder = FnGraphBuilder::new();

    // Store functions in graph.
    let [fn_id1, fn_id2, fn_id3, fn_id4, fn_id5] = fn_graph_builder.add_fns([
        fn1.into_fn_res(),
        fn2.into_fn_res(),
        fn3.into_fn_res(),
        fn4.into_fn_res(),
        fn5.into_fn_res(),
    ]);

    // Define dependencies to control ordering.
    fn_graph_builder
        .add_edges([
            (fn_id1, fn_id2),
            (fn_id1, fn_id3),
            (fn_id2, fn_id4),
            (fn_id2, fn_id5),
            (fn_id3, fn_id4),
            (fn_id3, fn_id5),
        ])
        .unwrap();

    fn_graph_builder.build()
};

// Invoke logic over data.
//
// For synchronous sequential execution, you may uncomment the next line:
// fn_graph.iter().for_each(|fun| fun.call(resources));
//
// For asynchronous concurrent execution, use the following:
let rt = tokio::runtime::Builder::new_current_thread()
    .enable_time()
    .build()
    .unwrap();
rt.block_on(async move {
    fn_graph
        .stream()
        .for_each_concurrent(None, |fun| async move { fun.call(resources); })
        .await;
});

// prints:
// 1 + 2 = 3
# }
#
# macro_rules! u32_newtype {
#     ($name:ident) => {
#         impl fmt::Display for $name {
#             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
#                 self.0.fmt(f)
#             }
#         }
#         impl Deref for $name {
#             type Target = u32;
#             fn deref(&self) -> &Self::Target {
#                 &self.0
#             }
#         }
#         impl DerefMut for $name {
#             fn deref_mut(&mut self) -> &mut Self::Target {
#                 &mut self.0
#             }
#         }
#         impl AddAssign<u32> for $name {
#             fn add_assign(&mut self, other: u32) {
#                 *self = Self(self.0 + other);
#             }
#         }
#     };
# }
# u32_newtype!(A);
# u32_newtype!(B);
# u32_newtype!(C);
# u32_newtype!(D);
```

Note that the knowledge of types is only needed at the point of definition for both data and logic. At the point of invocation, it's *dynamic*:

```rust ,ignore
let resources: &Resources = /* .. */;

fn_graph
    .stream()
    .for_each_concurrent(None, |fun| async move { fun.call(resources); })
    .await;
```


## How It Works

The remainder of this section shows how [`fn_graph`][`fn_graph`] is implemented.

Generic data storage and dynamic access is already solved by [`Resources`][`Resources`] from the previous page. The remaining parts to solve all relate to the logic:

* **Abstraction:** Abstraction over different logic types to retrieve parameters and invoke the logic.
* **Ordering:** Sort logic by their dependencies and how they access data.
* **Streaming:** Concurrently stream logic once they are available without violating data access rules.

[`Resources`]: resources.html
[`fn_graph`]: https://github.com/azriel91/fn_graph
