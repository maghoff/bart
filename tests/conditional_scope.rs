#[macro_use] extern crate bart_derive;

#[test]
fn it_supports_conditional_scope_with_boolean() {
    #[derive(BartDisplay)]
    #[template_string="{{#a?}}yes{{/a}}"]
    struct Test { a: bool }

    assert_eq!(
        "yes",
        Test { a: true }.to_string()
    );

    assert_eq!(
        "",
        Test { a: false }.to_string()
    );
}

#[test]
fn it_supports_negative_conditional_scope_with_boolean() {
    #[derive(BartDisplay)]
    #[template_string="{{^a?}}no{{/a}}"]
    struct Test { a: bool }

    assert_eq!(
        "",
        Test { a: true }.to_string()
    );

    assert_eq!(
        "no",
        Test { a: false }.to_string()
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
        Test { cond: TestBool { name: "Joe" } }.to_string()
    );

    assert_eq!(
        "No: ",
        Test { cond: TestBool { name: "No" } }.to_string()
    );
}

#[test]
fn it_supports_conditional_scope_with_vec() {
    #[derive(BartDisplay)]
    #[template_string="{{#a?}}yes{{/a}}"]
    struct Test { a: Vec<i32> }

    assert_eq!(
        "yes",
        Test { a: vec![1] }.to_string()
    );

    assert_eq!(
        "",
        Test { a: vec![] }.to_string()
    );
}

#[test]
fn it_supports_conditional_scope_with_borrowed_vec() {
    #[derive(BartDisplay)]
    #[template_string="{{#a?}}yes{{/a}}"]
    struct Test<'a> { a: &'a Vec<i32> }

    assert_eq!(
        "yes",
        Test { a: &vec![1, 2, 3] }.to_string()
    );

    assert_eq!(
        "",
        Test { a: &vec![] }.to_string()
    );
}

#[test]
fn it_supports_conditional_scope_with_slice() {
    #[derive(BartDisplay)]
    #[template_string="{{#a?}}yes{{/a}}"]
    struct Test<'a> { a: &'a [i32] }

    assert_eq!(
        "yes",
        Test { a: &[1] }.to_string()
    );

    assert_eq!(
        "",
        Test { a: &[] }.to_string()
    );
}
