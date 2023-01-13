# Feature Gated Incremental Functionality

Tried the following parameter type for `EnsureOpSpec::exec`:

```rust ,ignore
use std::marker::PhantomData;

/// Parameters to `EnsureOpSpec::exec`.
#[derive(Debug)]
pub struct EnsureExecParams<
    'params,
    Data,
    #[cfg(feature = "state_current")] StateCurrent,
    #[cfg(feature = "state_desired")] StateDesired,
    #[cfg(feature = "state_diff")] StateDiff,
> {
    /// Data accessed by the ensure op spec.
    pub data: Data,
    /// Current state of the item.
    #[cfg(feature = "state_current")]
    pub state_current: &'params StateCurrent,
    /// Desired state of the item.
    #[cfg(feature = "state_desired")]
    pub state_desired: &'params StateDesiredl,
    /// Diff between current and desired states.
    #[cfg(feature = "state_diff")]
    pub diff: &'params StateDiff,
    /// Marker.
    marker: PhantomData<&'params ()>,
}
```

But the following produces a compile error when used:

```rust ,ignore
async fn exec(
    ensure_exec_params: EnsureExecParams<
        '_,
        Self::Data<'_>,
        #[cfg(feature = "state_current")]
        Self::State,
        #[cfg(feature = "state_desired")]
        Self::StateLogical,
        #[cfg(feature = "state_diff")]
        Self::StateDiff,
    >,
) -> Result<Self::StatePhysical, Self::Error>;
```

The `#[cfg(..)]` attributes are not supposed in function parameter type parameters: See the [Attributes Galore] RFC and [rfc#2602].

Perhaps it is possible to define the type separately, but we probably need to define this in a separate trait:

```rust ,ignore
#[cfg(all(
    not(feature = "state_current"),
    not(feature = "state_desired"),
    not(feature = "state_diff"),
))]
pub type EnsureExecParams<'params> = EnsureExecParams<
    'params,
    Self::Data<'params>,
>

#[cfg(all(
    feature = "state_current",
    not(feature = "state_desired"),
    not(feature = "state_diff"),
))]
pub type EnsureExecParams<'params> = EnsureExecParams<
    'params,
    Self::Data<'params>,
    Self::State,
>

#[cfg(all(
    feature = "state_current",
    feature = "state_desired",
    not(feature = "state_diff"),
))]
pub type EnsureExecParams<'params> = EnsureExecParams<
    'params,
    Self::Data<'params>,
    Self::State,
    Self::StateLogical,
>

#[cfg(all(
    feature = "state_current",
    feature = "state_desired",
    feature = "state_diff",
))]
pub type EnsureExecParams<'params> = EnsureExecParams<
    'params,
    Self::Data<'params>,
    Self::State,
    Self::StateLogical,
    Self::StateDiff,
>
```


[Attributes Galore]: https://github.com/Centril/rfcs/blob/rfc/attributes-galore/text/0000-attributes-galore.md
[rfc#2602]: https://github.com/rust-lang/rfcs/pull/2602
