from pathlib import Path
import json,datetime,hashlib
from rust_matches import block,variants
out=Path(__file__).resolve().parents[1];p=Path('01_core/src/compiler/stdlib/figure_image.rs');s=p.read_text();a=s.index('    let _ = ctx;',s.index('pub fn native_figure('));z=s.index('    // F-5a de-bake',a)
s=s[:a]+'''    let _ = ctx;
    let value_span = |name: Option<&str>| {
        let occurrence = args.occurrences.as_ref().and_then(|occurrences| {
            if let Some(name) = name {
                occurrences.iter().rev().find(|arg| arg.name.as_deref() == Some(name))
            } else {
                occurrences.iter().find(|arg| arg.name.is_none())
            }
        });
        occurrence.map_or(args.span, |arg| arg.value_span.or(arg.span).or(args.span))
    };
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::LocatedContent(c, _)) => c.content().clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(Value::None) => Content::Empty,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            value_span(None),
            format!("expected content, found {}", vanilla_type_name(other)),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(), "missing argument: body",
        )]),
    };

    // Keep the existing caption body extraction; reject invalid casts explicitly.
    let caption = match args.named.get("caption") {
        None | Some(Value::None) => None,
        Some(Value::Content(Content::FigureCaption(fc))) => Some(fc.body.clone()),
        Some(Value::Content(c)) => Some(c.clone()),
        Some(Value::LocatedContent(c, _)) => Some(c.content().clone()),
        Some(Value::Str(s)) => Some(Content::text(s.as_str())),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            value_span(Some("caption")),
            format!("expected content or none, found {}", vanilla_type_name(other)),
        )]),
    };

    let kind = match args.named.get("kind") {
        Some(Value::Str(s)) => Some(s.to_string()),
        None => infer_kind_from_body(&body),
        Some(Value::Auto) => None,
        Some(Value::Func(function)) => {
            let is_element = function.element_name().is_some()
                || crate::compiler::eval::bindings::native_element_fields(function).is_some();
            if !is_element {
                return Err(vec![SourceDiagnostic::error(
                    value_span(Some("kind")), "expected element",
                )]);
            }
            // Typed element identity needs the separate public FigureKind contract.
            // Preserve the historical path until that contract is approved.
            infer_kind_from_body(&body)
        }
        Some(other) => return Err(vec![SourceDiagnostic::error(
            value_span(Some("kind")),
            format!("expected function, string, or auto, found {}", vanilla_type_name(other)),
        )]),
    };

'''+s[z:];p.write_text(s)
p=Path('01_core/src/compiler/stdlib/layout/casting.rs');s=p.read_text();definition=variants(Path('01_core/src/entities/value.rs').read_text(),'Value');positive={'Length','Float','Int','Relative','Ratio'};negative='\n        | '.join('Value::'+n+('(..)' if a else '') for n,a in definition if n not in positive)+' => None,'
for marker in ['pub(crate) fn extract_length(', 'pub(crate) fn extract_block_width(']:
 a,z,body=block(s,marker);assert body.count('_ => None,')==1;body=body.replace('_ => None,',negative).replace(' // neutro: N16[β] — Value não-dimensionável retorna None no fallback de largura','');s=s[:a+1]+body+s[z-1:]
p.write_text(s)
(out/'implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),api_changes=0,negative_cast_variants=len(definition)-len(positive),pending=['Typed figure kind and synthesis','Relative measure carriers and cast policies']),indent=2)+'\n')
