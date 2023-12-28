# Reborrowing

Reborrowing is used in scopes to so that the `View` structs don't hold onto the borrow for the length of `'ctx`, but only for the `'view` lifetime.

```rust
#[derive(Debug)]
struct Life<'life> {
    value: &'life mut u8,
}

impl<'life> Life<'life> {
    pub fn reborrow<'short>(&'short mut self) -> Life<'short> {
        Life {
            value: &mut self.value,
        }
    }
}

// Used in
struct Scope<'life> {
    life: Life<'life>,
    other: u16,
}

struct ScopeView<'view> {
    life: Life<'view>,
    other: &'view mut u16,
}

impl<'life> Scope<'life> {
    pub fn view(&mut self) -> ScopeView<'_> {
        let Scope { life, other } = self;

        // Needed to shorten the lifetime of `'life`.
        let life = life.reborrow();

        ScopeView { life, other }
    }
}

fn main() {
    let mut value = 123;

    let life = Life { value: &mut value };
    let mut many_data = Scope { life, other: 456 };

    let ScopeView {
        life: _life,
        other: _other,
    } = many_data.view();
}
```
