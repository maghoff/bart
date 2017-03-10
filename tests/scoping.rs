#[macro_use] extern crate bart_derive;

#[test]
fn it_can_access_nested_fields() {
    struct Nested { a: i32 }

    #[derive(BartDisplay)]
    #[template_string="{{nested.a}}"]
    struct Test { nested: Nested }

    assert_eq!(
        "42",
        format!("{}", Test { nested: Nested { a: 42 } })
    );
}

#[test]
fn it_can_scope_into_nested_values() {
    struct Nested { a: i32 }

    #[derive(BartDisplay)]
    #[template_string="{{#nested.}}{{.a}}{{/nested}}"]
    struct Test { nested: Nested }

    assert_eq!(
        "42",
        format!("{}", Test { nested: Nested { a: 42 } })
    );
}

