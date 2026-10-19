from pathlib import Path
p=Path('01_core/src/compiler/eval/tests.rs');s=p.read_text();i=s.rfind('\n}');s=s[:i]+r'''

    #[test]
    fn raw_presence_fields_and_explicit_defaults() {
        for source in [
            r#"#assert.eq(raw("x").fields(), (text:"x"))"#,
            r#"#assert.eq(raw("x", block:false).fields(), (text:"x", block:false))"#,
            r#"#assert.eq(raw("x", block:true, lang:none).fields(), (text:"x", block:true, lang:none))"#,
            r#"#assert.eq(raw("x", lang:"typc").fields(), (text:"x", lang:"typc"))"#,
            r#"#assert.eq(raw("x", lang:none, block:false).fields().keys(), ("text", "block", "lang"))"#,
        ] {
            let world = MockWorld::new(source);
            assert!(eval_for_test(&world, &world.source).is_ok(), "{source}");
        }
    }

    #[test]
    fn raw_presence_has_at_alias_and_source() {
        let world = MockWorld::new(r#"#let f=raw;#let a=f("x");#let b=f("x",block:false,lang:none);#assert(a.has("text"));#assert(not a.has("block"));#assert(not a.has("lang"));#assert.eq(a.at("lang",default:17),17);#assert(b.has("block"));#assert(b.has("lang"));#assert.eq(b.at("block"),false);#assert.eq(b.at("lang"),none);#assert.eq(b.text,"x")"#);
        assert!(eval_for_test(&world, &world.source).is_ok());
    }

    #[test]
    fn raw_presence_language_equality_distinguishes_omission() {
        let world = MockWorld::new(r#"#assert(raw("x") != raw("x",block:false));#assert(raw("x") != raw("x",lang:none));#assert(raw("x",block:false) == raw("x",block:false));#assert(raw("x",block:false,lang:none) != raw("x",block:false))"#);
        assert!(eval_for_test(&world, &world.source).is_ok());
    }

    #[test]
    fn raw_presence_stdlib_encoders_keep_known_fields() {
        let world = MockWorld::new(r#"#let a=raw("x",block:false,lang:none);#assert.eq(json.decode(json.encode(a)),(func:"raw",text:"x",block:false,lang:none));#assert.eq(yaml.decode(yaml.encode(a)),(func:"raw",text:"x",block:false,lang:none));#assert.eq(json.decode(json.encode(raw("x"))),(func:"raw",text:"x"))"#);
        assert!(eval_for_test(&world, &world.source).is_ok());
    }

    #[test]
    fn raw_presence_unknown_field_errors_are_specific() {
        for (field, expected) in [
            ("block", "field \"block\" in raw is not known at this point"),
            ("lang", "field \"lang\" in raw is not known at this point"),
            ("missing", "raw does not have field \"missing\""),
        ] {
            let world = MockWorld::new(&format!("#raw(\"x\").at(\"{field}\")"));
            let errors = eval_for_test(&world, &world.source).unwrap_err();
            assert!(errors.iter().any(|e| e.message == expected), "{errors:?}");
        }
    }

    #[test]
    fn raw_presence_markup_and_bool_auto_display() {
        for source in [
            "#let a=`x`;#assert.eq(a.fields(),(text:\"x\",block:false))",
            "#let a=```typc\nx\n```;#assert.eq(a.fields(),(text:\"x\",block:true,lang:\"typc\"))",
            "#show raw: it=>{assert.eq(it.block,false);assert.eq(it.lang,\"typc\");[OK]};#false #auto",
        ] {
            let world = MockWorld::new(source);
            assert!(eval_for_test(&world, &world.source).is_ok(), "{source}");
        }
    }
''' +s[i:];p.write_text(s)
p=Path('02_shell/src/cli.rs');s=p.read_text();i=s.rfind('\n}');s=s[:i]+r'''

    #[test]
    fn raw_presence_cli_constructor_block_and_snapshot() {
        for snapshot in [None, Some(Default::default())] {
            let present = snapshot.is_some();
            let raw = sourced(Content::raw("x", None, false));
            let value = Value::LocatedContent(
                IntrospectedContent::new(raw, snapshot),
                typst_core::entities::location::Location::from_raw(31),
            );
            let bytes = serialize_eval(&value, EvalFormat::Json, false).unwrap();
            let expected = if present { serde_json::json!({"func":"raw"}) }
                else { serde_json::json!({"func":"raw","text":"x","block":false}) };
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(), expected);
            let bytes = serialize_query(&[value], None, true, false).unwrap();
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(), expected);
        }
    }
''' +s[i:];p.write_text(s)
