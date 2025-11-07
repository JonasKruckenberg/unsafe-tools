# `canary`

`canary` wraps your types and checks for uninitialized memory. There is no reason to use this type in safe Rust,
but unsafe Rust or Rust that interacts with other languages over FFI may find this useful to find problematic behaviour.

Just wrap the type in question in `Canary` and it will automatically check for correct initialization on each `Deref` or `DerefMut` (when debug assertions are enabled).

```rust
use unsafe_tools_canary::Canary;

const CANARY: u64 = u64::from_le_bytes(*b"testmagc");

let mut my_val = Canary::<_, CANARY>::new(String::new());

assert_eq!(*my_val, "");

my_val.push_str("foo");

assert_eq!(*my_val, "foo");
```

When you attempt to access an incorrectly uninitialized type, it will panic:

```rust,should_panic
use unsafe_tools_canary::Canary;
use std::mem::MaybeUninit;

const CANARY: u64 = u64::from_le_bytes(*b"testmagc");

let b: Canary<u64, CANARY> = unsafe { MaybeUninit::uninit().assume_init() };

// THIS WILL PANIC!
assert_eq!(*b, 42);
```
