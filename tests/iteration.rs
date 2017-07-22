#[macro_use] extern crate bart_derive;

#[test]
fn it_can_iterate() {
    #[derive(BartDisplay)]
    #[template_string="{{#vec}}{{.}}{{/vec}}"]
    struct Test { vec: Vec<i32> }

    assert_eq!(
        "123",
        Test { vec: vec![1, 2, 3] }.to_string()
    );
}

#[test]
fn it_can_iterate_option() {
    #[derive(BartDisplay)]
    #[template_string="{{#a}}({{.}}){{/a}}"]
    struct Test { a: Option<i32> }

    assert_eq!(
        "(1)",
        Test { a: Some(1) }.to_string()
    );

    assert_eq!(
        "",
        Test { a: None }.to_string()
    );
}

#[test]
fn it_can_iterate_borrowed_slice() {
    #[derive(BartDisplay)]
    #[template_string="{{#slice}}{{.}}{{/slice}}"]
    struct Test<'a> { slice: &'a [i32] }

    assert_eq!(
        "123",
        Test { slice: &[1, 2, 3] }.to_string()
    );
}

#[test]
fn it_can_iterate_function() {
    #[derive(BartDisplay)]
    #[template_string="{{#as_vec()}}{{.}}{{/as_vec()}}"]
    struct Test {
        a: i32,
        b: i32,
        c: i32,
    }

    impl Test {
        pub fn as_vec(&self) -> Vec<i32> {
            vec![self.a, self.b, self.c]
        }
    }

    assert_eq!(
        "123",
        Test { a: 1, b: 2, c: 3 }.to_string()
    );
}

#[test]
fn it_can_iterate_dot() {
    #[derive(BartDisplay)]
    #[template_string="{{#opt}}{{#.}}{{.}}{{/.}}{{/opt}}"]
    struct Test { opt: Option<Vec<i32>> }
    assert_eq!(
        "123",
        Test { opt: Some(vec![1, 2, 3]) }.to_string()
    );
}
