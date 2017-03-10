#[macro_use] extern crate bart_derive;

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
