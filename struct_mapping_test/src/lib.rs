#[cfg(test)]
mod tests {
    use struct_mapping::{StructMapping, ToStructMappingField};

    #[test]
    fn basic_sm_get() {
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
        assert!(matches!(ex.sm_get("key_a"), Ok(a) if a == "0"));
    }

    #[test]
    fn basic_sm_set() {
        #[derive(StructMapping, Default)]
        pub struct TestStruct {
            pub key_a: i32,
            pub key_b: u32,
        }

        let mut ex = TestStruct::default();
        ex.sm_set("key_a", "128".to_string()).unwrap();
        assert!(matches!(ex.sm_get("key_a"), Ok(a) if a == "128"));
    }

    #[test]
    fn basic_sm_get_bool() {
        #[derive(StructMapping)]
        pub struct TestStruct {
            pub key_a: bool,
        }

        let ex = TestStruct { key_a: true };
        assert!(matches!(ex.sm_get("key_a"), Ok(a) if a == "true"));
    }

    #[test]
    fn basic_sm_get_char() {
        #[derive(StructMapping)]
        pub struct TestStruct {
            pub key_a: char,
        }

        let ex = TestStruct { key_a: 'a' as char };
        assert!(matches!(ex.sm_get("key_a"), Ok(a) if a == "a"));
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
        assert!(matches!(ex.sm_get("key_a"), Ok(a) if a == "0"));
    }

    #[test]
    fn basic_renamed() {
        #[derive(StructMapping, Default)]
        pub struct TestStruct {
            #[struct_mapping(rename = "key_renamed")]
            pub key_a: i32,
        }

        let ex = TestStruct::default();
        assert!(matches!(ex.sm_get("key_renamed"), Ok(a) if a == "0"));
    }

    #[test]
    fn basic_alias() {
        #[derive(StructMapping, Default)]
        pub struct TestStruct {
            #[struct_mapping(rename = "key_renamed", alias = "alias")]
            pub key_a: i32,
        }

        let ex = TestStruct::default();
        assert!(matches!(ex.sm_get("alias"), Ok(a) if a == "0"));
    }

    #[test]
    fn basic_sm_list() {
        #[derive(StructMapping, Default)]
        pub struct TestStruct {
            pub key_a: i32,
        }

        assert_eq!(TestStruct::sm_list(), vec!["key_a".to_string()]);
    }

    #[test]
    fn basic_readme_example() {
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
        assert!(matches!(ex.sm_get("jimmy"), Ok(value) if value == "0"));

        ex.sm_set("jimmy", "128".to_string()).unwrap();

        // print "128"
        assert!(matches!(ex.sm_get("jimmy"), Ok(value) if value == "128"));

        // print ["jimmy", "jian_yang"]
        assert_eq!(
            TestStruct::sm_list(),
            vec!["jimmy".to_string(), "jian_yang".to_string()]
        );
    }
}
