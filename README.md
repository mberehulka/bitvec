# BitVec

### Simple vector of bits aligned inside a vector of u8, u16, u32, u64 or u128.

Bits are aligned in little endian order.

### Example:
```rust
let mut v = super::BitVec::<u64>::new(); // []
v.resize(128); // [0u64, 0u64]
v.set(127, true); // [0u64, 1u32]
println!("{:#?}", v);
assert!(v.get(127) == true);
```
