    #[test]
    fn v16_c_selector_preserves_literal_metacharacters_and_unicode() {
        let mut literals: Vec<String> = "\\.+*?()|[]{}^$#&-~"
            .chars()
            .map(|character| character.to_string())
            .collect();
        literals.extend(
            [
                r"\d+", r"\n", r"[a-z&&[^q]]", r"(?x)a # note", r"a{2,3}|b?",
                "é中🙂", "e\u{301}", "á.中[🙂]", " \t\n\r", "\0", "plain text",
            ]
            .into_iter()
            .map(str::to_string),
        );
        for literal in literals {
            let Some(Selector::Regex(regex)) =
                value_to_query_selector(&Value::Str(literal.clone().into()))
            else {
                panic!("nonempty literal was refused: {literal:?}");
            };
            assert!(!regex.is_match(""), "{literal:?}");
            let haystack = format!("PREFIX{literal}SUFFIX");
            let matched = regex.captures_first(&haystack).expect("literal substring");
            assert_eq!(matched.text, literal, "{literal:?}");
            assert!(matched.captures.is_empty(), "{literal:?}");
            let anchored = crate::entities::regex::Regex::new(
                &format!(r"\A(?:{})\z", regex.pattern()),
            ).unwrap();
            assert!(anchored.is_match(&literal), "{literal:?}");
            assert!(!anchored.is_match(&format!("{literal}x")), "{literal:?}");
        }
        assert!(value_to_query_selector(&Value::Str("".into())).is_none());
        for (literal, nonliteral) in [
            ("a.b", "axb"), (r"\d+", "123"), ("a|b", "a"),
            ("[a-z]", "q"), ("(?i)a", "A"), ("a{2}", "aa"),
        ] {
            let Some(Selector::Regex(regex)) =
                value_to_query_selector(&Value::Str(literal.into()))
            else { panic!("literal refused: {literal:?}"); };
            assert!(!regex.is_match(nonliteral), "{literal:?}");
        }
    }

    #[test]
    fn v16_c_selector_preserves_literal_pattern_identity() {
        for (literal, expected) in [
            (r"\.+*?()|[]{}^$#&-~", r"\\\.\+\*\?\(\)\|\[\]\{\}\^\$\#\&\-\~"),
            ("á.中[🙂]", r"á\.中\[🙂\]"),
            (r"\d+", r"\\d\+"),
            ("ordinary/!%=:", "ordinary/!%=:")
        ] {
            let Some(Selector::Regex(regex)) =
                value_to_query_selector(&Value::Str(literal.into()))
            else { panic!("literal refused: {literal:?}"); };
            assert_eq!(regex.pattern(), expected);
        }
    }
