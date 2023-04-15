# Params Definition

For an item spec to work with different values, the values must be passed in.

Item spec implementors define these as part of the `ItemSpec` trait:

```rust ,ignore
trait ItemSpec {
    type Params: ..;
}
```

## Use Cases

The following shows a number of use cases of these params:

* **State Apply:** Param values must be known, and Peace should pass concrete values to the `ItemSpec::{state_current, state_desired, apply}` functions.
* **State Discovery (fallible):**

    Param values may be known, if predecessors have previously executed.

    - `try_state_current`: `StateDiscoverCmd::current`

        e.g. Look up file contents on a remote host:

        ```rust ,ignore
        match params_partial.dest_ip() {
            Some(dest_ip) => Some(file_hash(dest_ip, path)),
            None => None, // or `Some(FileState::None)`
        }
        ```

    - `try_state_desired`: `StateDiscoverCmd::desired`

        e.g. Look up source file contents:

        ```rust ,ignore
        match params_partial.src_path() {
            Some(src_path) => file_hash(src_path),
            None => None, // or `Some(FileState::None)`
        }
        ```


### By Item Spec Function

* `try_state_current`: Should work with `field_partial`s.
* `try_state_desired`: Should work with `field_partial`s.
* `state_current`: Needs real concrete param values.
* `state_desired`: Needs real concrete param values.
* `state_diff`: Doesn't need parameters or data; everything should be captured in `State`s.

    But for presentation, it's useful to know what a file should be (current vs desired), or difference between params (multiple profile current vs current).

* `state_clean`: Maybe always returns `ItemSpecState::None`, and doesn't need parameters or data.

    However, presenting `state_clean` with e.g. a file path, would mean the None state contains the value, which means `state_clean` needs params.

    Arguably `state_desired` will show the path that would be created.

    `StateDiff` for cleaning should also show the deletion of the path.

* `apply_check`: Doesn't need parameters or data.
* `apply_dry`: Needs concrete param values, even if they are fake.
* `apply`: Needs real concrete param values.


### Encoding: Serialization / Deserialization

Because:

* It is convenient to serialize `ItemSpec::Params::Spec` and store it, and deserialize it for use at a later time.
* It is useful to support config-based parameter specification (no compiler needed).
* It is not possible to serialize closures.

Then there must be a way to encode the same functionality that `ItemSpec::Params::Spec::field_from_map` provides, as something serializable.

Possibilities:

* `ToString` and `FromStr` impls that represent the logic
* Serialized form uses enum variants, and when deserializing, map that back to functions.
* Custom language.


## Code Implications

From the implementor's perspective, item spec trait needs to change to support the above use cases.

The following snippets are here to show the changes that include the above concepts. These are:

* non-compilable.
* just enough to show where types are changed.
* show certain trait bounds (non-exhaustive).
* do not include the encoding / decoding of `field_from_map` concept.


### Framework

```rust ,ignore
// Traits in Peace Framework
trait ItemSpec {
    type Params: Params + Serialize + Deserialize;

    fn setup(&self, resources);
    fn try_state_current(fn_ctx, params_partial, data);
    fn try_state_desired(fn_ctx, params_partial, data);
    fn state_clean      (        params_partial, data);
    fn state_current    (fn_ctx, params,         data);
    fn state_desired    (fn_ctx, params,         data);
    fn apply_dry        (fn_ctx, params,         data, state_current, state_target, diff);
    fn apply            (fn_ctx, params,         data, state_current, state_target, diff);
    fn apply_check      (                              state_current, state_target, diff);
    fn state_diff       (state_a, state_b);

    // Once more, with types:
    fn setup(&self, &mut Resources<Empty>);
    fn try_state_current(FnCtx<'_>, Self::Params<'_>::Partial, Self::Data<'_>);
    fn try_state_desired(FnCtx<'_>, Self::Params<'_>::Partial, Self::Data<'_>);
    fn state_clean      (           Self::Params<'_>::Partial, Self::Data<'_>);
    fn state_current    (FnCtx<'_>, Self::Params<'_>         , Self::Data<'_>);
    fn state_desired    (FnCtx<'_>, Self::Params<'_>         , Self::Data<'_>);
    fn apply_dry        (FnCtx<'_>, Self::Params<'_>         , Self::Data<'_>, Self::State, Self::State, Self::StateDiff);
    fn apply            (FnCtx<'_>, Self::Params<'_>         , Self::Data<'_>, Self::State, Self::State, Self::StateDiff);
    fn apply_check      (                                                      Self::State, Self::State, Self::StateDiff);
    fn state_diff       (Self::State, Self::State);
}

/// For Peace to access <ItemSpec::Params as Params>::Spec
trait Params {
    type Spec: Serialize + Deserialize;
    type SpecBuilder: SpecBuilder<Output = Self::Spec>;
    type Partial: Serialize + Deserialize;
}

enum ValueSpec<T> {
    Value(T),
    From,
    FromMap(Box<dyn Fn(&Resources) -> Option<T>>),
}
```

Also need to provide a `Params` derive macro.


### Implementor

```rust ,ignore
// Implementation
struct FileUploadItemSpec;

impl ItemSpec for FileUploadItemSpec {
    type Params = FileUploadParams;
}

#[derive(Clone, Debug, Params, Serialize, Deserialize)]
struct FileUploadParams {
    src: PathBuf,
    dest_ip: IpAddr,
    dest_path: PathBuf,
}
```

Auto generated by `Params` derive:

```rust ,ignore
impl Params for FileUploadParams {
    type Spec = FileUploadParamsSpec;
    type SpecBuilder = FileUploadParamsSpecBuilder;
    type Partial = FileUploadParamsPartial;
}

// Serialize / Deserialize not needed.
struct FileUploadParamsPartial {
    src: Option<PathBuf>,
    dest_ip: Option<IpAddr>,
    dest_path: Option<PathBuf>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FileUploadParamsSpec {
    src: ValueSpec<PathBuf>,
    dest_ip: ValueSpec<IpAddr>,
    dest_path: ValueSpec<PathBuf>,
}

#[derive(Clone, Debug)]
struct FileUploadParamsSpecBuilder {
    src: Option<ValueSpec<PathBuf>>,
    dest_ip: Option<ValueSpec<IpAddr>>,
    dest_path: Option<ValueSpec<PathBuf>>,
}
```

See:

* [`optfield`](https://github.com/roignpar/optfield)
* [`partial_derive`](https://github.com/rise0chen/partial_derive)
* [`optional-field`](https://github.com/cvpartner/optional-field)
