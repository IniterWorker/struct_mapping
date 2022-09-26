# StructMapping

__StructMapping is a library for create string-based accessors/mutators Rust data structure.__


## Work In Progress

- [ ] Finalize the design
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
use struct_mapping::StructMapping;

#[derive(StructMapping, Default)]
struct TestStruct {
    field_number: u32,
    #[struct_mapping(skip)]
    deep: DeepTestStruct,
}

fn main() {
    let ex = TestStruct::default();

    // print "0"
    println!("{:?}", ex.field_get("field_number"));

    ex.field_set("field_number", "128".to_string()).unwrap();
    
    // print "128"
    println!("{:?}", ex.field_get("field_number"));
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