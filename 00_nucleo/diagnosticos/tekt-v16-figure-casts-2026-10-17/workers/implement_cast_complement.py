from pathlib import Path
import json,datetime,hashlib
out=Path(__file__).resolve().parents[1];assert json.load(open(out/'red-symbol-resources.json'))['exit']==101;assert '0 passed; 2 failed;' in (out/'red-symbol.log').read_text()
p=Path('01_core/src/compiler/stdlib/structural/math/accent.rs');s=p.read_text();pos=s.index('pub(crate) fn normalize_math_accent');s=s[:pos]+'''/// Only known combining marks/aliases are callable as symbol wrappers.
pub(crate) fn is_known_math_accent(value: &str) -> bool {
    let direct = value.parse::<char>().ok();
    ACCENT_ALIASES.iter().any(|(accent, aliases)| Some(*accent) == direct || aliases.contains(&value))
}

'''+s[pos:];p.write_text(s)
p=Path('01_core/src/compiler/stdlib/structural/math/lr.rs');s=p.read_text();values=''.join(json.load(open(out/'lr-symbol-starts.json'))['starts']);s+='''
/// Classify the pinned left/right symbol wrappers without calling them.
pub(crate) fn is_callable_lr_symbol(value: &str) -> bool {
    value.chars().count() == 1 && '''+json.dumps(values,ensure_ascii=False)+'''.contains(value)
}
''';p.write_text(s)
p=Path('01_core/src/compiler/stdlib/structural/mod.rs');s=p.read_text().replace('pub(crate) use math::accent::math_accent_content;','pub(crate) use math::accent::{is_known_math_accent, math_accent_content};\npub(crate) use math::lr::is_callable_lr_symbol;');p.write_text(s)
p=Path('01_core/src/compiler/stdlib/figure_image.rs');s=p.read_text();a=s.index('    let body = match args.items.first()',s.index('pub fn native_figure('));z=s.index('    // F-5a de-bake',a);body=s[a:z];body=body.replace('Some(Value::Str(s)) => Content::text(s.as_str()),','Some(Value::Str(s)) => Content::text(s.as_str()),\n        Some(Value::Symbol(s)) => Content::text(s.value.clone()),').replace('Some(Value::Str(s)) => Some(Content::text(s.as_str())),','Some(Value::Str(s)) => Some(Content::text(s.as_str())),\n        Some(Value::Symbol(s)) => Some(Content::text(s.value.clone())),')
classes=json.load(open(out/'type-kind-classes.json'));positive='\n                | '.join('crate::entities::value::Type::'+n for n in classes['constructors']);negative='\n                | '.join('crate::entities::value::Type::'+n for n in classes['no_constructor'] if n not in ['Bool','Relative']);needle='''        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                value_span(Some("kind")),''';assert needle in body
extra='''        Some(Value::Type(ty)) => {
            let message = match ty {
                '''+positive+''' => "expected element".to_string(),
                crate::entities::value::Type::Bool => "type boolean does not have a constructor".to_string(),
                crate::entities::value::Type::Relative => "type relative length does not have a constructor".to_string(),
                '''+negative+''' => format!("type {} does not have a constructor", ty.name()),
            };
            return Err(vec![SourceDiagnostic::error(value_span(Some("kind")), message)]);
        }
        Some(Value::Symbol(symbol)) => {
            let value = symbol.value.as_str();
            let callable = crate::compiler::stdlib::structural::is_known_math_accent(value)
                || crate::compiler::stdlib::structural::is_callable_lr_symbol(value);
            let message = if callable { "expected element".to_string() }
                else { format!("symbol {value} is not callable") };
            return Err(vec![SourceDiagnostic::error(value_span(Some("kind")), message)]);
        }
'''
body=body.replace(needle,extra+needle);p.write_text(s[:a]+body+s[z:])
(out/'cast-complement-implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in json.load(open(out/'scope.json'))['sources']},public_api_changes=0),indent=2)+'\n')
