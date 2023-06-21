use bart_derive::BartDisplay;

#[test]
fn it_works() {
    #[derive(BartDisplay)]
    #[template = "tests/templates/partials/it_works.html"]
    struct Test {
        name: String,
    }

    assert_eq!(
        "(Hello, World)",
        Test {
            name: "World".to_owned()
        }
        .to_string()
    );
}

#[test]
fn it_defaults_to_the_current_dynamic_scope() {
    struct A {
        name: String,
    }

    #[derive(BartDisplay)]
    #[template = "tests/templates/partials/it_defaults_to_the_current_dynamic_scope.html"]
    struct Test {
        a: A,
    }

    assert_eq!(
        "Hello, World",
        Test {
            a: A {
                name: "World".to_owned()
            }
        }
        .to_string()
    );
}

#[test]
fn it_can_nest_within_iterator() {
    #[derive(BartDisplay)]
    #[template = "tests/templates/partials/it_can_nest_within_iterator.html"]
    struct Test<'a> {
        items: &'a [i32],
    }

    assert_eq!("(1)\n(2)\n(3)\n", Test { items: &[1, 2, 3] }.to_string());
}

#[test]
fn it_allows_named_root_scope() {
    struct Person {
        name: String,
    }

    #[derive(BartDisplay)]
    #[template = "tests/templates/partials/it_allows_named_root_scope.html"]
    struct Test {
        person: Person,
    }

    assert_eq!(
        "(Hello, World)",
        Test {
            person: Person {
                name: "World".to_owned()
            }
        }
        .to_string()
    );
}

#[test]
fn it_finds_partials_relative_to_crate_root() {
    #[derive(BartDisplay)]
    #[template = "tests/templates/partials/it_finds_partials_relative_to_crate_root.html"]
    struct Test {
        name: String,
    }

    assert_eq!(
        "(Hello, World)",
        Test {
            name: "World".to_owned()
        }
        .to_string()
    );
}
