#[cfg(test)]
mod tests {
    use struct_mapping::{StructMapping, ToStructMappingField};

    #[test]
    fn basic_field_get() {
        #[derive(StructMapping)]
        pub struct TestStruct {
            pub key_a: i32,
            pub key_b: u32,
        }

        impl Default for TestStruct {
            fn default() -> Self {
                Self { key_a: 0, key_b: 1 }
            }
        }

        let ex = TestStruct::default();
        assert!(matches!(ex.field_get("key_a"), Ok(a) if a == "0"));
    }

    #[test]
    fn basic_field_set() {
        #[derive(StructMapping, Default)]
        pub struct TestStruct {
            pub key_a: i32,
            pub key_b: u32,
        }

        let mut ex = TestStruct::default();
        ex.field_set("key_a", "128".to_string()).unwrap();
        assert!(matches!(ex.field_get("key_a"), Ok(a) if a == "128"));
    }

    #[test]
    fn basic_field_get_bool() {
        #[derive(StructMapping)]
        pub struct TestStruct {
            pub key_a: bool,
        }

        let ex = TestStruct { key_a: true };
        assert!(matches!(ex.field_get("key_a"), Ok(a) if a == "true"));
    }

    #[test]
    fn basic_field_get_char() {
        #[derive(StructMapping)]
        pub struct TestStruct {
            pub key_a: char,
        }

        let ex = TestStruct { key_a: 'a' as char };
        assert!(matches!(ex.field_get("key_a"), Ok(a) if a == "a"));
    }

    #[test]
    fn basic_skip() {
        #[derive(Default)]
        pub struct DeepTestStruct {
            pub some: i32,
        }

        #[derive(StructMapping, Default)]
        pub struct TestStruct {
            pub key_a: i32,
            #[struct_mapping(skip)]
            pub deep: DeepTestStruct,
        }

        let ex = TestStruct::default();
        assert!(matches!(ex.field_get("key_a"), Ok(a) if a == "0"));
    }

    #[test]
    fn basic_renamed() {
        #[derive(StructMapping, Default)]
        pub struct TestStruct {
            #[struct_mapping(rename = "key_renamed")]
            pub key_a: i32,
        }

        let ex = TestStruct::default();
        assert!(matches!(ex.field_get("key_renamed"), Ok(a) if a == "0"));
    }

    #[test]
    fn basic_alias() {
        #[derive(StructMapping, Default)]
        pub struct TestStruct {
            #[struct_mapping(rename = "key_renamed", alias = "alias")]
            pub key_a: i32,
        }

        let ex = TestStruct::default();
        assert!(matches!(ex.field_get("alias"), Ok(a) if a == "0"));
    }
}
