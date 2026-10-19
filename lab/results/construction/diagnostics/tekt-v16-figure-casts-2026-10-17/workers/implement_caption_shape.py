from pathlib import Path
import json,datetime
from rust_matches import block
out=Path(__file__).resolve().parents[1]
assert json.load(open(out/'red-caption-shape-all-resources.json'))['exit']==101
log=(out/'red-caption-shape-all.log').read_text();assert '0 passed; 4 failed;' in log and '0 passed; 1 failed;' in log
assert json.load(open(out/'red-caption-casts-resources.json'))['exit']==101;assert '0 passed; 1 failed;' in (out/'red-caption-casts.log').read_text()
p=Path('01_core/src/entities/elements/figure_caption.rs');s=p.read_text();pos=s.index('    fn plain_text(&self)');s=s[:pos]+'''    fn get_field(&self, field: &str) -> Option<crate::entities::value::Value> {
        use crate::entities::value::Value;
        use crate::entities::layout_types::{Align2D,VAlign};
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "separator" => self.separator.as_ref().map(|body| Value::Content(body.clone())),
            "position" => self.position.map(|position| Value::Align(Align2D {h:None,v:Some(match position {
                CaptionPosition::Top => VAlign::Top,
                CaptionPosition::Bottom => VAlign::Bottom,
            })})),
            _ => None,
        }
    }

'''+s[pos:];p.write_text(s)
p=Path('01_core/src/entities/content/fields.rs');s=p.read_text().replace('(Content::Figure(e), f) => e.get_field(f),','(Content::Figure(e), f) => e.get_field(f),\n            (Content::FigureCaption(e), f) => e.get_field(f),');p.write_text(s)
p=Path('01_core/src/compiler/eval/bindings/field_access/content_methods.rs');s=p.read_text();a=s.index('    let candidates:');s=s[:a]+s[a:].replace('        Content::MathMatrix(_) => {','        Content::FigureCaption(_) => &["position","separator","body"],\n        Content::MathMatrix(_) => {',1);s=s.replace('Content::Figure(_) => crate::compiler::stdlib::native_figure,','Content::Figure(_) => crate::compiler::stdlib::native_figure,\n        Content::FigureCaption(_) => crate::compiler::stdlib::native_figure_caption,');p.write_text(s)
p=Path('01_core/src/compiler/eval/bindings/value_methods/element_where.rs');s=p.read_text();pos=s.index('    element!(\n        native_text,');s=s[:pos]+'''    element!(
        native_figure_caption,
        "caption",
        true,
        ["position","separator","body","kind","supplement","numbering","counter"]
    );
'''+s[pos:];p.write_text(s)
p=Path('01_core/src/compiler/stdlib/loading/encoding.rs');s=p.read_text().replace('        Content::Empty => "sequence",','        Content::Empty => "sequence",\n        Content::FigureCaption(_) => "caption",',1);p.write_text(s)
p=Path('02_shell/src/cli.rs');s=p.read_text();a=s.index('fn content_to_semantic(');pos=s.index('        Content::Raw(_) => {',a);s=s[:pos]+'''        Content::FigureCaption(_) => {
            for name in ["position","separator","body"] {
                if let Some(value)=content.get_field(name) {
                    fields.push((name.into(),value_to_semantic(&value)));
                }
            }
        }
'''+s[pos:];a=s.index('fn content_semantic_func_name(');pos=s.index('        Content::SmartQuote(_) =>',a);s=s[:pos]+'        Content::FigureCaption(_) => "caption",\n'+s[pos:];p.write_text(s)
# Coercions share the same causal span lookup as the already tested Figure path.
p=Path('01_core/src/compiler/stdlib/figure_image.rs');s=p.read_text();a,z,body=block(s,'    let value_span = |name: Option<&str>|');closure=body;helper='''fn figure_argument_value_span(args: &Args, name: Option<&str>) -> Span {'''+closure+'''\n}\n\n''';s=s[:a]+s[a:] if False else s
start=s.index('    let value_span = |name: Option<&str>|');s=s[:start]+'    let value_span = |name: Option<&str>| figure_argument_value_span(args, name);'+s[z+1:]
pos=s.index('/// Auto-detecção');s=s[:pos]+helper+'''fn caption_content_value(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::Content(body) => Ok(body.clone()),
        Value::LocatedContent(body, _) => Ok(body.content().clone()),
        Value::Str(text) => Ok(Content::text(text.as_str())),
        Value::Symbol(symbol) => Ok(Content::text(symbol.value.clone())),
        Value::None => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(span,format!("expected content, found {}",vanilla_type_name(other)))]),
    }
}

'''+s[pos:]
start=s.index('pub fn native_figure_caption(');a,z,body=block(s,'    let body = match args.items.first()',start);body_new='''
        Some(value) => caption_content_value(value,figure_argument_value_span(args,None))?,
        None => {
            return Err(vec![SourceDiagnostic::error(Span::detached(),"missing argument: body")]);
        }
    ''';s=s[:a+1]+body_new+s[z-1:]
start=s.index('pub fn native_figure_caption(');a,z,body=block(s,'    let separator = match args.named.get("separator")',start);body_new='''
        None => None,
        Some(value) => Some(caption_content_value(value,figure_argument_value_span(args,Some("separator")))?),
    ''';s=s[:a+1]+body_new+s[z-1:];p.write_text(s)
(out/'caption-shape-implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),public_api_changes=0,scope='Existing caption carrier/getters/native identity/structured projection and content coercions; no synthesis or public field added.'),indent=2)+'\n')
