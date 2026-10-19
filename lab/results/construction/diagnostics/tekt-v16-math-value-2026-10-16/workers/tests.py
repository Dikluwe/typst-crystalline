from pathlib import Path
import json,datetime,hashlib
out=Path('00_nucleo/diagnosticos/tekt-v16-math-value-2026-10-16');scope=json.load(open(out/'scope.json'))
tests={scope['sources'][0]:r'''

#[cfg(test)]
mod v16_math_value_tests {
    use super::*;
    use crate::entities::symbol::Symbol;

    #[test]
    fn v16_math_value_delimiter_pairs_and_array_fallback_preserve_morphology() {
        for (value, expected) in [
            (Value::None, Some(('\0', '\0'))),
            (Value::Str("floor".into()), Some(('⌊', '⌋'))),
            (Value::Str("⟦⟧".into()), Some(('⟦', '⟧'))),
            (Value::Symbol(Symbol::new("⟦⟧", "test")), Some(('⟦', '⟧'))),
            (Value::Array(vec![]), Some(('\0', '\0'))),
            (Value::Array(vec![Value::Int(2), Value::Str("]".into())]), Some(('\0', ']'))),
            (Value::Array(vec![Value::Str("[".into()), Value::Bool(false)]), Some(('[', '['))),
            (Value::Array(vec![Value::Str("[".into()), Value::None, Value::Str("ignored".into())]), Some(('[', '\0'))),
            (Value::Auto, None),
            (Value::Int(1), None),
            (Value::Bool(false), None),
        ] {
            assert_eq!(parse_delim_val(&value), expected, "{value:?}");
        }
    }

    #[test]
    fn v16_math_value_delimiter_char_keeps_first_unicode_scalar_and_rejections() {
        for (value, expected) in [
            (Value::None, Some('\0')),
            (Value::Str("".into()), None),
            (Value::Str("⟦⟧".into()), Some('⟦')),
            (Value::Symbol(Symbol::new("←→", "arrow")), Some('←')),
            (Value::Array(vec![]), None),
            (Value::Bool(false), None),
            (Value::Float(f64::NAN), None),
            (Value::Auto, None),
        ] {
            assert_eq!(parse_delim_char(&value), expected, "{value:?}");
        }
    }
}
''',scope['sources'][1]:r'''

#[cfg(test)]
mod v16_math_value_tests {
    use super::*;
    use crate::entities::symbol::Symbol;

    #[test]
    fn v16_math_value_vector_char_keeps_none_unicode_and_neutral_types() {
        for (value, expected) in [
            (Value::None, Some('\0')),
            (Value::Str("".into()), None),
            (Value::Str("⟦⟧".into()), Some('⟦')),
            (Value::Symbol(Symbol::new("←→", "arrow")), Some('←')),
            (Value::Array(vec![]), None),
            (Value::Bool(false), None),
            (Value::Int(1), None),
            (Value::Auto, None),
        ] {
            assert_eq!(vec_delim_char(&value), expected, "{value:?}");
        }
    }

    #[test]
    fn v16_math_value_vector_array_fallback_and_top_level_error_are_preserved() {
        for (values, expected) in [
            (vec![], ('\0', '\0')),
            (vec![Value::Str("[".into())], ('[', '[')),
            (vec![Value::Int(2), Value::Str("]".into())], ('\0', ']')),
            (vec![Value::Str("[".into()), Value::Bool(false)], ('[', '[')),
            (vec![Value::Str("[".into()), Value::None], ('[', '\0')),
        ] {
            assert_eq!(math_vec_delim(&Value::Array(values), Span::detached()).unwrap(), expected);
        }
        let errors = math_vec_delim(&Value::Int(1), Span::detached()).unwrap_err();
        assert_eq!(errors[0].message, "expected array, none, symbol, or string, found integer");
        assert_eq!(errors[0].span, Span::detached());
    }
}
''',scope['sources'][2]:r'''

#[cfg(test)]
mod v16_math_value_tests {
    use super::*;
    use crate::entities::geometry::{LineCap, LineJoin, Stroke, StrokeFields};
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    use std::sync::Arc;

    #[test]
    fn v16_math_value_matrix_color_shortcut_marks_only_paint() {
        let color = Color::rgb(10, 20, 30);
        let dict = [("stroke".into(), Value::Color(color.clone()))].into_iter().collect();
        let stroke = math_matrix_augment(&Value::Dict(dict), Span::detached())
            .unwrap().unwrap().stroke.unwrap();
        assert!(stroke.specified.paint);
        assert!(!stroke.specified.thickness);
        assert_eq!(stroke.thickness, 1.0);
        assert_eq!(stroke.paint, Paint::Solid(color));
    }

    #[test]
    fn v16_math_value_matrix_existing_stroke_preserves_payload_and_presence() {
        let original = Arc::new(Stroke {
            thickness: 3.0,
            cap: LineCap::Round,
            join: LineJoin::Bevel,
            overhang: true,
            specified: StrokeFields { cap: true, join: true, miter_limit: true, ..StrokeFields::default() },
            ..Stroke::default()
        });
        let dict = [("stroke".into(), Value::Stroke(original.clone()))].into_iter().collect();
        let stroke = math_matrix_augment(&Value::Dict(dict), Span::detached())
            .unwrap().unwrap().stroke.unwrap();
        assert_eq!(stroke, *original);
        assert!(!stroke.specified.paint);
        assert!(!stroke.specified.thickness);
    }

    #[test]
    fn v16_math_value_matrix_invalid_stroke_is_rejected_before_presence() {
        for (value, kind) in [(Value::Bool(false), "bool"), (Value::None, "none")] {
            let dict = [("stroke".into(), value)].into_iter().collect();
            let errors = math_matrix_augment(&Value::Dict(dict), Span::detached()).unwrap_err();
            assert_eq!(errors[0].message, format!("math.mat(augment.stroke): espera Length / Color / Stroke, recebeu {kind}"));
            assert_eq!(errors[0].span, Span::detached());
        }
    }
}
'''}
receipt={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sources':{},'tests':7,'implementation_written':False}
for source,addition in tests.items():
 p=Path(source);text=p.read_text();assert 'mod v16_math_value_tests' not in text
 p.write_text(text+addition)
 receipt['sources'][source]={'before':text,'addition':addition,'after_sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
(out/'tests-before-code.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n')
