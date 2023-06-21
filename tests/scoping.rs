#[macro_use]
extern crate bart_derive;

#[test]
fn it_can_access_nested_fields() {
    struct Nested {
        a: i32,
    }

    #[derive(BartDisplay)]
    #[template_string = "{{nested.a}}"]
    struct Test {
        nested: Nested,
    }

    assert_eq!(
        "42",
        Test {
            nested: Nested { a: 42 }
        }
        .to_string()
    );
}

#[test]
fn it_can_scope_into_nested_values() {
    struct Nested {
        a: i32,
    }

    #[derive(BartDisplay)]
    #[template_string = "{{#nested.}}{{.a}}{{/nested}}"]
    struct Test {
        nested: Nested,
    }

    assert_eq!(
        "42",
        Test {
            nested: Nested { a: 42 }
        }
        .to_string()
    );
}

#[test]
fn it_can_scope_into_function_values() {
    struct Nested {
        a: i32,
    }

    #[derive(BartDisplay)]
    #[template_string = "{{#as_nested().}}{{.a}}{{/as_nested()}}"]
    struct Test {
        a: i32,
    }

    impl Test {
        pub fn as_nested(&self) -> Nested {
            Nested { a: self.a }
        }
    }

    assert_eq!("42", Test { a: 42 }.to_string());
}
