#[macro_use] extern crate bart_derive;

#[test]
fn it_skips_some_option() {
    #[derive(BartDisplay)]
    #[template_string="[{{^x}}it{{/x}}]"]
    struct Test { x: Option<i32> }

    assert_eq!(
        "[]",
        Test { x: Some(42) }.to_string()
    );
}

#[test]
fn it_includes_none_option() {
    #[derive(BartDisplay)]
    #[template_string="[{{^x}}it{{/x}}]"]
    struct Test { x: Option<i32> }

    assert_eq!(
        "[it]",
        Test { x: None }.to_string()
    );
}

#[test]
fn it_supports_borrowed_option() {
    #[derive(BartDisplay)]
    #[template_string="[{{^x}}it{{/x}}]"]
    struct Test<'a> { x: &'a Option<i32> }

    assert_eq!(
        "[it]",
        Test { x: &None }.to_string()
    );
}

#[test]
fn it_supports_multiply_borrowed_option() {
    #[derive(BartDisplay)]
    #[template_string="[{{^x}}it{{/x}}]"]
    struct Test<'a> { x: &'a &'a &'a Option<i32> }

    assert_eq!(
        "[it]",
        Test { x: &&&None }.to_string()
    );
}

#[test]
fn it_skips_ok_result() {
    #[derive(BartDisplay)]
    #[template_string="[{{^x}}{{.}}{{/x}}]"]
    struct Test<'a> { x: &'a Result<i32, i32> }

    assert_eq!(
        "[]",
        Test { x: &Ok(42) }.to_string()
    );
}

#[test]
fn it_yields_err_result() {
    #[derive(BartDisplay)]
    #[template_string="[{{^x}}{{.}}{{/x}}]"]
    struct Test<'a> { x: &'a Result<i32, i32> }

    assert_eq!(
        "[42]",
        Test { x: &Err(42) }.to_string()
    );
}
