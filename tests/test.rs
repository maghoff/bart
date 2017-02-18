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
fn it_handles_names_with_underscore() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{your_name}}"]
    struct Test { your_name: String }

    assert_eq!(
        "Hello, World",
        format!("{}", Test { your_name: "World".to_owned() })
    );
}

#[test]
fn it_handles_tuple_struct_field_names() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{0}}"]
    struct Test<'a>(&'a str);

    assert_eq!(
        "Hello, World",
        format!("{}", Test("World"))
    );
}

#[test]
fn it_handles_some_whitespace() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{  name  }}"]
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
fn it_can_iterate_borrowed_slice() {
    #[derive(BartDisplay)]
    #[template_string="{{#slice}}{{.}}{{/slice}}"]
    struct Test<'a> { slice: &'a [i32] }

    assert_eq!(
        "123",
        format!("{}", Test { slice: &[1, 2, 3] })
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
fn it_supports_conditional_scope_with_boolean() {
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

#[test]
fn it_supports_negative_conditional_scope_with_boolean() {
    #[derive(BartDisplay)]
    #[template_string="{{^a?}}no{{/a}}"]
    struct Test { a: bool }

    assert_eq!(
        "",
        format!("{}", Test { a: true })
    );

    assert_eq!(
        "no",
        format!("{}", Test { a: false })
    );
}

#[test]
fn it_supports_conditional_scope_with_non_bool() {
    extern crate bart;

    struct TestBool<'a> {
        name: &'a str,
    }

    impl<'a> bart::Conditional for TestBool<'a> {
        fn val(&self) -> bool {
            self.name.len() > 2
        }
    }

    #[derive(BartDisplay)]
    #[template_string="{{cond.name}}: {{#cond?}}Hello {{.name}}{{/cond}}"]
    struct Test<'a> { cond: TestBool<'a> }

    assert_eq!(
        "Joe: Hello Joe",
        format!("{}", Test { cond: TestBool { name: "Joe" } })
    );

    assert_eq!(
        "No: ",
        format!("{}", Test { cond: TestBool { name: "No" } })
    );
}
