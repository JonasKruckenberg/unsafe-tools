# `mimic`

Mimic generates "type-erased" struct definitions of _equal size_ and _equal alignment_.
This is useful in FFI scenarios where you don't want to or cannot communicate the exact layout and type of fields to the other language,
but still want to communicate it's size and alignment e.g. to be able to stack-allocate the type.

```rust
use unsafe_tools_mimic::impl_mimic;

#[derive(Clone)]
pub struct Foo {
    a: u64,
}
impl_mimic!(Foo);

let ty = Foo { a: 42 };
let mimic = ty.clone().into_mimic();

assert_eq!(size_of_val(&ty), size_of_val(&mimic));
assert_eq!(align_of_val(&ty), align_of_val(&mimic));
```
