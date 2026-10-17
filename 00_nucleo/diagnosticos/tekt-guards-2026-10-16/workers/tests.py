from pathlib import Path
import json,subprocess
out=Path('00_nucleo/diagnosticos/tekt-guards-2026-10-16');record=dict(before={},after={})
p=Path('01_core/src/compiler/stdlib/foundations/color.rs');old=p.read_text();anchor='    #[test]\n    fn colors_ci_single_color_targets_and_alpha() {'
test='''    #[test]
    fn v17_relative_ratio_boundaries_preserve_values_and_diagnostics() {
        use crate::entities::layout_types::{Length, Ratio};
        use crate::entities::rel::Rel;
        let relative = |rel, abs| Value::Relative(Rel { rel, abs });
        for value in [0.0, 0.5, 1.0] {
            let component = relative(value, Length::ZERO);
            assert_eq!(ratio_component(&component).unwrap(), value as f32);
            let color = colors_ci_call(native_cmyk, vec![component; 4]).unwrap();
            assert_eq!(color, Value::Color(Color::Cmyk {
                c: value as f32, m: value as f32, y: value as f32, k: value as f32,
            }));
        }
        for component in [
            relative(-0.01, Length::ZERO),
            relative(1.01, Length::ZERO),
            relative(f64::NAN, Length::ZERO),
            relative(f64::INFINITY, Length::ZERO),
            relative(0.5, Length::pt(1.0)),
        ] {
            let expected = if let Value::Relative(value) = &component {
                if value.abs.is_zero() {
                    "ratio must be between 0% and 100%"
                } else {
                    "expected ratio, found relative length"
                }
            } else {
                unreachable!("fixture is a Relative")
            };
            assert_eq!(ratio_component(&component).unwrap_err()[0].message, expected);
            let errors = colors_ci_call(native_cmyk, vec![
                component, Value::Ratio(Ratio(0.0)), Value::Ratio(Ratio(0.0)),
                Value::Ratio(Ratio(0.0)),
            ]).unwrap_err();
            assert_eq!(errors[0].message, expected);
        }
    }

'''
assert old.count(anchor)==1;p.write_text(old.replace(anchor,test+anchor));record['before'][str(p)]=old
p=Path('03_infra/tests/p1428_math_frac.rs');old=p.read_text();anchor='#[test]\nfn p1428_decoder_html_e_fail_closed() {'
test='''#[test]
fn v17_numeric_html_entities_keep_unicode_validation_and_single_decode() {
    assert_eq!(decode_text_node("&#X22;&#x1F600;&#128512;"), Ok("\\\"😀😀".into()));
    assert_eq!(decode_text_node("&amp;#X22;"), Ok("&#X22;".into()));
    for invalid in [
        "&#x;", "&#X;", "&#;", "&#xD800;", "&#1114112;", "&#x110000;",
        "&éx22;", "&#é22;", "&＃x22;", "&#x2g;", "&#X22",
    ] {
        assert_eq!(decode_text_node(invalid), Err(Unknown::HtmlEntity), "{invalid}");
    }
}

'''
assert old.count(anchor)==1;p.write_text(old.replace(anchor,test+anchor));record['before'][str(p)]=old
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*record['before']],check=True)
record['after']={p:Path(p).read_text() for p in record['before']};(out/'tests-materialization.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(dict(test_sources=list(record['before']),new_tests=2)))
