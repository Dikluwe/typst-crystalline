from pathlib import Path
import json,datetime
out=Path('00_nucleo/diagnosticos/clippy-enums-2026-10-16')
tests={
'decimal':r'''
    #[test]
    fn approved_traits_decimal_display_preserves_scale() {
        fn render<T: std::fmt::Display + ToString>(value: &T) -> (String, String) {
            (format!("{value}"), ToString::to_string(value))
        }
        for (value, expected) in [
            (Decimal::new(12300, 4), "1.2300"),
            (Decimal::new(-12300, 4), "-1.2300"),
            (Decimal::default(), "0"),
        ] {
            assert_eq!(render(&value), (expected.into(), expected.into()));
        }
        assert!(Decimal::from_str("invalid").is_none());
    }
''',
'version':r'''
    #[test]
    fn approved_traits_version_display_preserves_components() {
        fn render<T: std::fmt::Display + ToString>(value: &T) -> (String, String) {
            (format!("{value}"), ToString::to_string(value))
        }
        for (components, expected) in [(vec![], ""), (vec![1, 2, 0], "1.2.0")] {
            let value = Version::from_components(components.clone());
            assert_eq!(render(&value), (expected.into(), expected.into()));
            assert_eq!(value.components, components);
        }
        assert!(Version::from_str("1.invalid").is_none());
    }
''',
'duration':r'''
    #[test]
    fn approved_traits_duration_display_covers_signed_range() {
        fn render<T: std::fmt::Display + ToString>(value: &T) -> (String, String) {
            (format!("{value}"), ToString::to_string(value))
        }
        for (nanos, expected) in [
            (i128::MIN, "-1969226660422097589487121d13h41m55.884105728s"),
            (i128::MAX, "1969226660422097589487121d13h41m55.884105727s"),
            (i128::MIN + 1, "-1969226660422097589487121d13h41m55.884105727s"),
            (0, "0s"), (-1, "-0.000000001s"), (-1_500_000_000, "-1.5s"),
            (-60_000_000_000, "-1m"),
        ] {
            assert_eq!(render(&Duration::from_nanos(nanos)), (expected.into(), expected.into()));
        }
    }
''',
'style':r'''
    #[test]
    fn approved_traits_collect_styles_preserves_order_and_other_fields() {
        let empty: Styles = std::iter::empty::<Style>().collect();
        assert!(empty.is_empty());
        let collected: Styles = [Style::Weight(400), Style::Weight(700)].into_iter().collect();
        let mut expected = StyleDelta::empty();
        expected.weight = Some(700);
        assert_eq!(collected.delta(), &expected);
        assert_eq!(collected, Styles::from_iter([Style::Weight(400), Style::Weight(700)]));
    }
'''
}
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before={},tests=tests,after={})
for name,test in tests.items():
 p=Path(f'01_core/src/entities/{name}.rs');s=p.read_text();record['before'][str(p)]=s
 assert s.count('mod tests {')==1
 s=s.replace('mod tests {','mod tests {'+test,1);p.write_text(s);record['after'][str(p)]=s
(out/'traits-tests-materialization.json').write_text(json.dumps(record,indent=2)+'\n')
