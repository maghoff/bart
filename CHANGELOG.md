Changelog
=========
This is a high-level overview of the changes that made it into a given version.

Next version
------------
New in this version:

 * Specify the root element for template rendering with the `template_root` attribute:

        struct Nested<'a> { name: &'a str }

        #[derive(BartDisplay)]
        #[template_string="Hello, {{name}}"]
        #[template_root="0"]
        struct Test<'a>(Nested<'a>);

        assert_eq!(
            "Hello, World",
            format!("{}", Test(Nested { name: "World" }))
        );

Version 0.1.1
-------------
New in this version:

 * Name parsing now accepts numerical indexes, so tuple structs can be accessed:

        #[derive(BartDisplay)]
        #[template_string="Hello, {{0}}"]
        struct Test<'a>(&'a str);

        assert_eq!(
            "Hello, World",
            format!("{}", Test("World"))
        );

 * Basic support for negative iteration, which allows accessing the error in a `Result`:

        #[derive(BartDisplay)]
        #[template_string="[{{^x}}{{.}}{{/x}}]"]
        struct Test<'a> { x: &'a Result<i32, i32> }

        assert_eq!(
            "[42]",
            format!("{}", Test { x: &Err(42) })
        );

 * Support conditional scoping for `Vec` and slices:

        #[derive(BartDisplay)]
        #[template_string="{{#a?}}yes{{/a}}"]
        struct Test<'a> { a: &'a Vec<i32> }

        assert_eq!(
            "yes",
            format!("{}", Test { a: &vec![1, 2, 3] })
        );

        assert_eq!(
            "",
            format!("{}", Test { a: &vec![] })
        );

This is the first changelog entry. The changelog will not mention changes prior to this.
