# ref_mut_n

This defines a nested mutable reference.

```toml
[dependencies]
ref-mut-n = "1"
```

```rs
use ref_mut_n::ref_mut_n;

fn main() {
    let mut data = [[0u8; 3]; 4];
    let _ref_data: &mut [&mut [u8]] = ref_mut_n!(data, 4);
}
```

work as

```rs
let _ref_data: &mut [&mut [u8]] = {
    let [ref mut a0, ref mut a1, ref mut a2, ref mut a3] = data;
    &mut[a0, a1, a2, a3]
}
```
