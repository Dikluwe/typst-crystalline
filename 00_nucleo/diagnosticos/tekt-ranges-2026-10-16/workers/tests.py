from pathlib import Path
import json,datetime
out=Path('00_nucleo/diagnosticos/tekt-ranges-2026-10-16');record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before={},after={})
spans=r'''
    #[test]
    fn v18_float_anchors_cardinality_and_precedence() {
        let capture = |has_spread| FloatIsNanCallSpans {
            call: span(1),
            positional: vec![span(2), span(3), span(4), span(5)],
            named: IndexMap::default(),
            has_spread,
        };
        for (count, bound, spread, expected) in [
            (0, false, false, 1), (0, false, true, 1),
            (0, true, false, 9), (0, true, true, 9),
            (1, false, false, 2), (1, false, true, 9),
            (1, true, false, 2), (1, true, true, 9),
            (2, false, false, 3), (2, false, true, 9),
            (2, true, false, 2), (2, true, true, 9),
            (4, false, false, 3), (4, false, true, 9),
            (4, true, false, 2), (4, true, true, 9),
        ] {
            let args = Args::from_parts(vec![Value::Int(0); count], IndexMap::default(), span(9));
            assert_eq!(capture(spread).anchor(&args, bound), span(expected));
        }
        for bound in [false, true] {
            for spread in [false, true] {
                let mut captured = capture(spread);
                captured.named.insert("value".into(), span(6));
                let mut named = IndexMap::default();
                named.insert("value".into(), Value::Int(0));
                let args = Args::from_parts(vec![], named, span(9));
                assert_eq!(captured.anchor(&args, bound), span(6));
            }
        }
        let mut missing = capture(false);
        missing.positional.clear();
        for count in [1, 2] {
            let args = Args::from_parts(vec![Value::Int(0); count], IndexMap::default(), span(9));
            assert_eq!(missing.anchor(&args, false), span(9));
        }
    }

    #[test]
    fn v18_math_anchors_cardinality_and_precedence() {
        let capture = |has_spread| P1293MathCallSpans {
            call: span(1),
            positional: vec![span(2), span(3), span(4), span(5)],
            named: IndexMap::default(),
            has_spread,
        };
        for identity in [P1293MathNative::Attach, P1293MathNative::Mono, P1293MathNative::Script] {
            for (count, expected) in [(0, 1), (1, 9), (2, 3), (4, 3)] {
                let args = Args::from_parts(vec![Value::Str("x".into()); count], IndexMap::default(), span(9));
                assert_eq!(capture(false).anchor(identity, &args), span(expected));
                assert_eq!(capture(true).anchor(identity, &args), span(9));
            }
            let args = Args::from_parts(vec![Value::Int(1)], IndexMap::default(), span(9));
            assert_eq!(capture(false).anchor(identity, &args), span(2));
        }
        for (identity, count, name, value, expected) in [
            (P1293MathNative::Attach, 0, "unknown", Value::Int(1), 1),
            (P1293MathNative::Attach, 2, "unknown", Value::Int(1), 3),
            (P1293MathNative::Attach, 1, "unknown", Value::Int(1), 6),
            (P1293MathNative::Attach, 1, "t", Value::Int(1), 7),
            (P1293MathNative::Mono, 0, "unknown", Value::Int(1), 6),
            (P1293MathNative::Mono, 2, "unknown", Value::Int(1), 6),
            (P1293MathNative::Script, 0, "unknown", Value::Int(1), 6),
            (P1293MathNative::Script, 2, "unknown", Value::Int(1), 6),
            (P1293MathNative::Script, 0, "cramped", Value::Int(1), 7),
            (P1293MathNative::Script, 2, "cramped", Value::Int(1), 7),
            (P1293MathNative::Script, 0, "cramped", Value::Bool(true), 1),
            (P1293MathNative::Script, 2, "cramped", Value::Bool(true), 3),
        ] {
            for spread in [false, true] {
                let mut captured = capture(spread);
                captured.named.insert(name.into(), P1293NamedSpan { full: span(6), value: span(7) });
                let mut named = IndexMap::default();
                named.insert(name.into(), value.clone());
                let args = Args::from_parts(vec![Value::Str("x".into()); count], named, span(9));
                assert_eq!(captured.anchor(identity, &args), span(if spread { 9 } else { expected }));
            }
        }
    }
'''
watch=r'''
    #[test]
    fn v18_percent_octets_preserve_ascii_and_utf8_rejection() {
        for octet in 0..=255u8 {
            let upper = format!("/left%{octet:02X}right");
            let lower = format!("/left%{octet:02x}right");
            let expected = if octet.is_ascii() && !matches!(octet, 0 | b'\\') {
                Some(format!("/left{}right", char::from(octet)))
            } else {
                None
            };
            assert_eq!(decode_url_path(&upper), expected, "octet={octet}");
            assert_eq!(decode_url_path(&lower), expected, "octet={octet}");
        }
    }

    #[test]
    fn v18_percent_routes_preserve_normalization_and_rejection() {
        for (input, expected) in [
            ("/", Some("/")), ("/caf%C3%A9", Some("/café")),
            ("/a%2Fb", Some("/a/b")), ("/a/b/../c?d=1", Some("/a/c")),
            ("/%2e%2e/private", None), ("/x/%2E%2e/y", Some("/y")),
            ("/%", None), ("/%A", None), ("/%GG", None),
            ("/%00", None), ("/%5c", None), ("/%C0%AF", None),
            ("/%FF", None), ("/a/", Some("/a/")), ("", None),
            ("relative", None), ("/a?next=../..", Some("/a")),
            ("/%252e%252e/file", Some("/%2e%2e/file")),
        ] {
            assert_eq!(decode_url_path(input).as_deref(), expected, "input={input:?}");
        }
    }
'''
for p,test in [('01_core/src/compiler/eval/call_dispatch/call_spans.rs',spans),('03_infra/src/watch.rs',watch)]:
 t=Path(p).read_text();record['before'][p]=t;assert t.count('mod tests {')==1;t=t.replace('mod tests {','mod tests {'+test,1);Path(p).write_text(t);record['after'][p]=t
(out/'tests-materialization.json').write_text(json.dumps(record,indent=2)+'\n');print('Four semantic controls added before implementation; expected baseline passes.')
