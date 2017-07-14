#[macro_use] extern crate bart_derive;

#[test]
fn it_works() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{name}}"]
    struct Test { name: String }

    assert_eq!(
        "Hello, World",
        Test { name: "World".to_owned() }.to_string()
    );
}

#[test]
fn it_finds_template_files() {
    #[derive(BartDisplay)]
    #[template="tests/templates/basic/it_finds_template_files.html"]
    struct Test { name: String }

    assert_eq!(
        "Hello, World",
        Test { name: "World".to_owned() }.to_string()
    );
}

#[test]
fn it_handles_names_with_underscore() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{your_name}}"]
    struct Test { your_name: String }

    assert_eq!(
        "Hello, World",
        Test { your_name: "World".to_owned() }.to_string()
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
        Test { name: "World".to_owned() }.to_string()
    );
}

#[test]
fn it_can_borrow() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{name}}"]
    struct Test<'a> { name: &'a str }

    assert_eq!(
        "Hello, World",
        Test { name: "World" }.to_string()
    );
}

#[test]
fn it_performs_escaping() {
    #[derive(BartDisplay)]
    #[template_string="{{txt}}"]
    struct Test<'a> { txt: &'a str }

    assert_eq!(
        "&lt;&amp;&quot;&apos;",
        Test { txt: "<&\"'" }.to_string()
    );
}

#[test]
fn it_passes_through() {
    #[derive(BartDisplay)]
    #[template_string="{{{txt}}}"]
    struct Test<'a> { txt: &'a str }

    assert_eq!(
        "<&\"'",
        Test { txt: "<&\"'" }.to_string()
    );
}

#[test]
fn template_root_element() {
    struct Nested<'a> { name: &'a str }

    #[derive(BartDisplay)]
    #[template_string="Hello, {{name}}"]
    #[template_root="0"]
    struct Test<'a>(Nested<'a>);

    assert_eq!(
        "Hello, World",
        Test(Nested { name: "World" }).to_string()
    );
}

#[test]
fn function_call() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{name()}}"]
    struct Test;

    impl Test {
        fn name(&self) -> &'static str {
            "World"
        }
    }

    assert_eq!(
        "Hello, World",
        Test.to_string()
    );
}
