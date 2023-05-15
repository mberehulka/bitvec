# BitVec

### Simple vector of bits aligned inside a vector of u8, u16, u32, u64 or u128

Bits are aligned from left to right;
So, in a vector of u32, the position is:
- 31 = 0b1
- 30 = 0b10
- 16 = 0b1000000000000000
- 0 = 0b10000000000000000000000000000000

### Example:
```rust
let mut v = super::BitVec::<u64>::new(); // []
v.resize(128); // [0u64, 0u64]
v.set(127, true); // [0u64, 1u32]
println!("{:#?}", v);
assert!(v.get(127) == true);
```