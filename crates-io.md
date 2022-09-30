# StructMapping

__StructMapping is a library for generating string-based accessors/mutators Rust data structure.__

---

## StructMapping in action

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
