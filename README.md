# StructMapping

__StructMapping is a library for generating string-based accessors/mutators Rust data structure.__


## Work In Progress

- [ ] Clean-Up
- [ ] Pipeline
- [ ] Publish Crate/Cargo

---

## StructMapping in action

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]

# The core APIs
struct_mapping = { version = "1.0", features = ["derive"] }
```

</details>
<p></p>

```rust
use struct_mapping::{StructMapping, ToStructMappingField};

fn main() {
    #[derive(Default)]
    struct DeepTestStruct {}

    #[derive(StructMapping, Default)]
    struct TestStruct {
        #[struct_mapping(rename = "jimmy", alias = "jian_yang")]
        jian: u32,
        #[struct_mapping(skip)]
        #[allow(dead_code)]
        deep: DeepTestStruct,
    }

    let mut ex = TestStruct::default();

     // print "0"
    println!("{}", ex.sm_get("jimmy").unwrap());

    ex.sm_set("jimmy", "128").unwrap();

    // print "128"
    println!("{}", ex.sm_get("jimmy").unwrap());

    // print ["jimmy", "jian_yang"]
    println!("{:?}", TestStruct::sm_list());
}
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in struct_mapping by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>