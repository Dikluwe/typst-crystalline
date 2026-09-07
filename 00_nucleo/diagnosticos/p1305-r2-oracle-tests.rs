    // P1305-r2: independent snapshots of pinned vanilla before candidate.
    mod p1305_oracles {
        use super::*;

        fn world(text: &str) -> ImportMockWorld {
            ImportMockWorld::new(
                text,
                &[
                    ("ordinary/std.typ", "#let x = 7\n"),
                    ("ordinary/global.typ", "#let x = 8\n"),
                    ("ordinary/map.typ", "#let x = 9\n"),
                    ("ordinary/unlisted-name.typ", "#let x = 10\n"),
                    ("ordinary/holder.typ", "#let saved = std\n"),
                    ("reexport/std.typ", "#import std: *\n"),
                    (
                        "routes/inner.typ",
                        "#let observation = (repr(std), { let alias = std; repr(alias) }, { import std as renamed; (repr(renamed), renamed.calc.abs(-7)) })\n",
                    ),
                ],
            )
        }

        fn observe(expression: &str, features: Features) -> Value {
            let (result, side) =
                eval_expression_with_features(&world(""), expression, features);
            assert!(
                side.is_empty(),
                "{expression}: unexpected side diagnostics {side:?}"
            );
            result.unwrap_or_else(|errors| panic!("{expression}: {errors:?}"))
        }

        #[test]
        fn p1305_array_repr_0() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(0))"###, features),
                    Value::Str(r###"()"###.into()),
                    "{profile}/array-repr-0"
                );
            }
        }

        #[test]
        fn p1305_array_repr_1() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(1))"###, features),
                    Value::Str(r###"(0,)"###.into()),
                    "{profile}/array-repr-1"
                );
            }
        }

        #[test]
        fn p1305_array_repr_39() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(39))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
)"###
                            .into()
                    ),
                    "{profile}/array-repr-39"
                );
            }
        }

        #[test]
        fn p1305_array_repr_40() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(40))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
)"###
                            .into()
                    ),
                    "{profile}/array-repr-40"
                );
            }
        }

        #[test]
        fn p1305_array_repr_41() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(41))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  .. (1 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/array-repr-41"
                );
            }
        }

        #[test]
        fn p1305_array_repr_42() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(42))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  .. (2 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/array-repr-42"
                );
            }
        }

        #[test]
        fn p1305_array_repr_81() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(81))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  .. (41 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/array-repr-81"
                );
            }
        }

        #[test]
        fn p1305_array_repr_256() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(256))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  .. (216 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/array-repr-256"
                );
            }
        }

        #[test]
        fn p1305_array_strings() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"repr(range(42).map(x => "item-" + str(x)))"###,
                        features
                    ),
                    Value::Str(
                        r###"(
  "item-0",
  "item-1",
  "item-2",
  "item-3",
  "item-4",
  "item-5",
  "item-6",
  "item-7",
  "item-8",
  "item-9",
  "item-10",
  "item-11",
  "item-12",
  "item-13",
  "item-14",
  "item-15",
  "item-16",
  "item-17",
  "item-18",
  "item-19",
  "item-20",
  "item-21",
  "item-22",
  "item-23",
  "item-24",
  "item-25",
  "item-26",
  "item-27",
  "item-28",
  "item-29",
  "item-30",
  "item-31",
  "item-32",
  "item-33",
  "item-34",
  "item-35",
  "item-36",
  "item-37",
  "item-38",
  "item-39",
  .. (2 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/array-strings"
                );
            }
        }

        #[test]
        fn p1305_array_distinct_after_boundary() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ let a = range(42); a.at(40) = -777; (repr(a), a.len(), a.at(39), a.at(40), a.at(41), a) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(
                            r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  .. (2 items omitted),
)"###
                                .into()
                        ),
                        Value::Int(42),
                        Value::Int(39),
                        Value::Int(-777),
                        Value::Int(41),
                        Value::Array(vec![
                            Value::Int(0),
                            Value::Int(1),
                            Value::Int(2),
                            Value::Int(3),
                            Value::Int(4),
                            Value::Int(5),
                            Value::Int(6),
                            Value::Int(7),
                            Value::Int(8),
                            Value::Int(9),
                            Value::Int(10),
                            Value::Int(11),
                            Value::Int(12),
                            Value::Int(13),
                            Value::Int(14),
                            Value::Int(15),
                            Value::Int(16),
                            Value::Int(17),
                            Value::Int(18),
                            Value::Int(19),
                            Value::Int(20),
                            Value::Int(21),
                            Value::Int(22),
                            Value::Int(23),
                            Value::Int(24),
                            Value::Int(25),
                            Value::Int(26),
                            Value::Int(27),
                            Value::Int(28),
                            Value::Int(29),
                            Value::Int(30),
                            Value::Int(31),
                            Value::Int(32),
                            Value::Int(33),
                            Value::Int(34),
                            Value::Int(35),
                            Value::Int(36),
                            Value::Int(37),
                            Value::Int(38),
                            Value::Int(39),
                            Value::Int(-777),
                            Value::Int(41)
                        ])
                    ]),
                    "{profile}/array-distinct-after-boundary"
                );
            }
        }

        #[test]
        fn p1305_nested_inner() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr((range(41), ("tail",)))"###, features),
                    Value::Str(
                        r###"(
  (
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32,
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    .. (1 items omitted),
  ),
  ("tail",),
)"###
                            .into()
                    ),
                    "{profile}/nested-inner"
                );
            }
        }

        #[test]
        fn p1305_nested_outer() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(81).map(x => (x, x + 1)))"###, features),
                    Value::Str(
                        r###"(
  (0, 1),
  (1, 2),
  (2, 3),
  (3, 4),
  (4, 5),
  (5, 6),
  (6, 7),
  (7, 8),
  (8, 9),
  (9, 10),
  (10, 11),
  (11, 12),
  (12, 13),
  (13, 14),
  (14, 15),
  (15, 16),
  (16, 17),
  (17, 18),
  (18, 19),
  (19, 20),
  (20, 21),
  (21, 22),
  (22, 23),
  (23, 24),
  (24, 25),
  (25, 26),
  (26, 27),
  (27, 28),
  (28, 29),
  (29, 30),
  (30, 31),
  (31, 32),
  (32, 33),
  (33, 34),
  (34, 35),
  (35, 36),
  (36, 37),
  (37, 38),
  (38, 39),
  (39, 40),
  .. (41 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/nested-outer"
                );
            }
        }

        #[test]
        fn p1305_nested_multiline() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr((range(18), range(41)))"###, features),
                    Value::Str(
                        r###"(
  (
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
  ),
  (
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32,
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    .. (1 items omitted),
  ),
)"###
                            .into()
                    ),
                    "{profile}/nested-multiline"
                );
            }
        }

        #[test]
        fn p1305_escaped_strings() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(("a\n\"b\\c", "tail"))"###, features),
                    Value::Str(r###"("a\n\"b\\c", "tail")"###.into()),
                    "{profile}/escaped-strings"
                );
            }
        }

        #[test]
        fn p1305_ascii_body_49() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"repr(("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0))"###,
                        features
                    ),
                    Value::Str(
                        r###"("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0)"###
                            .into()
                    ),
                    "{profile}/ascii-body-49"
                );
            }
        }

        #[test]
        fn p1305_ascii_body_50() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"repr(("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0))"###,
                        features
                    ),
                    Value::Str(
                        r###"("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0)"###
                            .into()
                    ),
                    "{profile}/ascii-body-50"
                );
            }
        }

        #[test]
        fn p1305_ascii_body_51() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"repr(("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0))"###,
                        features
                    ),
                    Value::Str(
                        r###"(
  "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  0,
)"###
                            .into()
                    ),
                    "{profile}/ascii-body-51"
                );
            }
        }

        #[test]
        fn p1305_content_op() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(math.op([a \# \*], limits: false))"###, features),
                    Value::Str(
                        r###"op(
  text: sequence([a], [ ], [#], [ ], [*]),
  limits: false,
)"###
                            .into()
                    ),
                    "{profile}/content-op"
                );
            }
        }

        #[test]
        fn p1305_named_modules() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"(repr(std), repr(color.map), repr(calc), repr(sym), repr(pdf))"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Str(r###"<module map>"###.into()),
                        Value::Str(r###"<module calc>"###.into()),
                        Value::Str(r###"<module sym>"###.into()),
                        Value::Str(r###"<module pdf>"###.into())
                    ]),
                    "{profile}/named-modules"
                );
            }
        }

        #[test]
        fn p1305_named_aliases() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ let a = std; let b = a; let c = color.map; (repr(a), repr(b), repr(c), a.calc.abs(-7)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Str(r###"<module global>"###.into()),
                        Value::Str(r###"<module map>"###.into()),
                        Value::Int(7)
                    ]),
                    "{profile}/named-aliases"
                );
            }
        }

        #[test]
        fn p1305_ordinary_collisions() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import "ordinary/std.typ" as a; import "ordinary/global.typ" as b; import "ordinary/map.typ" as c; import "ordinary/unlisted-name.typ" as d; let alias = a; (repr(a), repr(b), repr(c), repr(d), repr(alias), a.x, b.x, c.x, d.x) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module std>"###.into()),
                        Value::Str(r###"<module global>"###.into()),
                        Value::Str(r###"<module map>"###.into()),
                        Value::Str(r###"<module unlisted-name>"###.into()),
                        Value::Str(r###"<module std>"###.into()),
                        Value::Int(7),
                        Value::Int(8),
                        Value::Int(9),
                        Value::Int(10)
                    ]),
                    "{profile}/ordinary-collisions"
                );
            }
        }

        #[test]
        fn p1305_ordinary_bare() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import "ordinary/std.typ"; (repr(std), std.x) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module std>"###.into()),
                        Value::Int(7)
                    ]),
                    "{profile}/ordinary-bare"
                );
            }
        }

        #[test]
        fn p1305_global_bare() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import std; (repr(std), std.calc.abs(-7)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Int(7)
                    ]),
                    "{profile}/global-bare"
                );
            }
        }

        #[test]
        fn p1305_global_alias_bare() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ let renamed = std; import renamed; (repr(renamed), renamed.calc.abs(-8)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Int(8)
                    ]),
                    "{profile}/global-alias-bare"
                );
            }
        }

        #[test]
        fn p1305_global_rename() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import std as renamed; (repr(renamed), renamed.calc.abs(-9)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Int(9)
                    ]),
                    "{profile}/global-rename"
                );
            }
        }

        #[test]
        fn p1305_global_items() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import std: calc as c, rgb as color-fn; (c.abs(-10), repr(type(color-fn))) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Int(10),
                        Value::Str(r###"function"###.into())
                    ]),
                    "{profile}/global-items"
                );
            }
        }

        #[test]
        fn p1305_global_wildcard() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import std: *; (calc.abs(-11), repr(type(rgb))) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Int(11),
                        Value::Str(r###"function"###.into())
                    ]),
                    "{profile}/global-wildcard"
                );
            }
        }

        #[test]
        fn p1305_reexport_collision() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import "reexport/std.typ" as r; let alias = r; (repr(r), repr(alias), r.calc.abs(-12), repr(r.calc)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module std>"###.into()),
                        Value::Str(r###"<module std>"###.into()),
                        Value::Int(12),
                        Value::Str(r###"<module calc>"###.into())
                    ]),
                    "{profile}/reexport-collision"
                );
            }
        }

        #[test]
        fn p1305_imported_route() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import "routes/inner.typ" as r; r.observation }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Str(r###"<module global>"###.into()),
                        Value::Array(vec![
                            Value::Str(r###"<module global>"###.into()),
                            Value::Int(7)
                        ])
                    ]),
                    "{profile}/imported-route"
                );
            }
        }

        #[test]
        fn p1305_global_field_bare() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import "ordinary/holder.typ" as holder; import holder.saved; (repr(saved), saved.calc.abs(-13)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Int(13)
                    ]),
                    "{profile}/global-field-bare"
                );
            }
        }

        #[test]
        fn p1305_global_dynamic_named() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import (std) as chosen; (repr(chosen), chosen.calc.abs(-14)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Int(14)
                    ]),
                    "{profile}/global-dynamic-named"
                );
            }
        }

        #[test]
        fn p1305_global_dynamic_items() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"{ import (std): calc as c; c.abs(-15) }"###, features),
                    Value::Int(15),
                    "{profile}/global-dynamic-items"
                );
            }
        }

        #[test]
        fn p1305_global_dynamic_wildcard() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"{ import (std): *; calc.abs(-16) }"###, features),
                    Value::Int(16),
                    "{profile}/global-dynamic-wildcard"
                );
            }
        }

        #[test]
        fn p1305_full_array_values_survive_repr() {
            for (_, features) in p1300_profiles() {
                for n in [0, 1, 39, 40, 41, 42, 81, 256] {
                    let expression = format!(
                        "{{ let a = range({n}); let before = a; let ignored = repr(a); a }}"
                    );
                    let expected = Value::Array((0..n).map(Value::Int).collect());
                    assert_eq!(observe(&expression, features.clone()), expected, "n={n}");
                }
            }
        }

        #[test]
        fn p1305_document_global_constructor_all_profiles() {
            use comemo::Track;
            let world = world(
                "#let observation = (repr(std), { let alias = std; repr(alias) }, { import std as renamed; (repr(renamed), renamed.calc.abs(-7)) })\n",
            );
            for (profile, features) in p1300_profiles() {
                let routines = Routines::new();
                let traced = Traced::default();
                let mut sink = Sink::new();
                let route = Route::root();
                let registry = crate::entities::element_registry::ElementRegistry::new();
                let result = eval_with_full_error_target_and_features(
                    &routines,
                    &world,
                    traced.track(),
                    sink.track_mut(),
                    route.track(),
                    &world.main,
                    &registry,
                    false,
                    EvalTarget::Paged,
                    features,
                )
                .unwrap_or_else(|errors| panic!("{profile}: {errors:?}"));
                assert_eq!(
                    result.scope().get("observation"),
                    Some(&Value::Array(vec![
                        Value::Str("<module global>".into()),
                        Value::Str("<module global>".into()),
                        Value::Array(vec![
                            Value::Str("<module global>".into()),
                            Value::Int(7)
                        ]),
                    ])),
                    "{profile}"
                );
            }
        }

        #[test]
        fn p1305_import_binding_negatives_and_dynamic_spans() {
            let cases: [(&str, &str, std::ops::Range<usize>, &[&str]); 6] = [
                ("{ import std; global }", "unknown variable `global`", 14..20, &[]),
                (
                    "{ let renamed = std; import renamed; global }",
                    "unknown variable `global`",
                    37..43,
                    &[],
                ),
                (
                    "{ import \"ordinary/holder.typ\" as holder; import holder.saved; global }",
                    "unknown variable `global`",
                    63..69,
                    &[],
                ),
                (
                    "{ import (std); global }",
                    "dynamic import requires an explicit name",
                    9..14,
                    &["you can name the import with `as`"],
                ),
                (
                    "{ import { std }; none }",
                    "dynamic import requires an explicit name",
                    9..16,
                    &["you can name the import with `as`"],
                ),
                (
                    "{ let f() = std; import f(); none }",
                    "dynamic import requires an explicit name",
                    24..27,
                    &["you can name the import with `as`"],
                ),
            ];
            for (profile, features) in p1300_profiles() {
                for (expression, message, span, hints) in &cases {
                    let world = world("");
                    let source = Source::new_with_parser(
                        world.main(),
                        expression.to_string(),
                        crate::compiler::parse::parse_code,
                    );
                    let (result, side) = eval_expression_with_features(
                        &world,
                        expression,
                        features.clone(),
                    );
                    assert!(side.is_empty(), "{profile}/{expression}: {side:?}");
                    let diagnostics = result.expect_err(expression);
                    assert_eq!(diagnostics.len(), 1, "{profile}/{expression}");
                    assert_eq!(
                        diagnostics[0].message, *message,
                        "{profile}/{expression}"
                    );
                    assert_eq!(
                        diagnostics[0]
                            .hints
                            .iter()
                            .map(|h| h.as_str())
                            .collect::<Vec<_>>(),
                        *hints,
                        "{profile}/{expression}"
                    );
                    assert_eq!(
                        source.span_byte_range(diagnostics[0].span),
                        Some(span.clone()),
                        "{profile}/{expression}"
                    );
                }
            }
        }

        #[test]
        fn p1305_content_sequence_does_not_inherit_array_elision() {
            let children: Vec<_> =
                (0..41).map(|i| Content::text(i.to_string().as_str())).collect();
            let value = Value::Content(Content::Sequence(std::sync::Arc::from(children)));
            let expected = format!(
                "sequence(\n  {},\n)",
                (0..41).map(|i| format!("[{i}]")).collect::<Vec<_>>().join(",\n  ")
            );
            assert_eq!(repr_value_for_serialization(&value), expected);
        }

        #[test]
        fn p1305_args_fields_remain_integral() {
            let expression = "{ let f(..args) = repr(args); f(..range(41)) }";
            let expected = format!(
                "arguments({})",
                (0..41).map(|i| i.to_string()).collect::<Vec<_>>().join(", ")
            );
            assert_eq!(
                observe(expression, Features::empty()),
                Value::Str(expected.into())
            );
        }
    }
