#[macro_use] extern crate bart_derive;

#[test]
fn it_works() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{name}}"]
    struct Test { name: String }

    assert_eq!(
        "Hello, World",
        format!("{}", Test { name: "World".to_owned() })
    );
}

#[test]
fn it_can_borrow() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{name}}"]
    struct Test<'a> { name: &'a str }

    assert_eq!(
        "Hello, World",
        format!("{}", Test { name: "World" })
    );
}

#[test]
fn it_performs_escaping() {
    #[derive(BartDisplay)]
    #[template_string="{{txt}}"]
    struct Test<'a> { txt: &'a str }

    assert_eq!(
        "&lt;&amp;&quot;&apos;",
        format!("{}", Test { txt: "<&\"'" })
    );
}

#[test]
fn it_passes_through() {
    #[derive(BartDisplay)]
    #[template_string="{{{txt}}}"]
    struct Test<'a> { txt: &'a str }

    assert_eq!(
        "<&\"'",
        format!("{}", Test { txt: "<&\"'" })
    );
}

#[test]
fn it_can_iterate() {
    #[derive(BartDisplay)]
    #[template_string="{{#vec}}{{.}}{{/vec}}"]
    struct Test { vec: Vec<i32> }

    assert_eq!(
        "123",
        format!("{}", Test { vec: vec![1, 2, 3] })
    );
}

#[test]
fn it_can_iterate_option() {
    #[derive(BartDisplay)]
    #[template_string="{{#a}}({{.}}){{/a}}"]
    struct Test { a: Option<i32> }

    assert_eq!(
        "(1)",
        format!("{}", Test { a: Some(1) })
    );

    assert_eq!(
        "",
        format!("{}", Test { a: None })
    );
}

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

#[test]
fn it_supports_boolean_scope() {
    #[derive(BartDisplay)]
    #[template_string="{{#a?}}yes{{/a}}"]
    struct Test { a: bool }

    assert_eq!(
        "yes",
        format!("{}", Test { a: true })
    );

    assert_eq!(
        "",
        format!("{}", Test { a: false })
    );
}
