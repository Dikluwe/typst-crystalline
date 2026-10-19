from pathlib import Path
import json,gzip,hashlib,re,datetime
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));red=json.load(gzip.open(out/'red-input.json.gz'));assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in red['source_hashes'].items());log=(out/'red.log').read_text();assert '16 failed' in log and '2 passed' in log and 'registered format element missing' in log;assert json.load(open(out/'red-resources.json'))['exit']==101
# Closed representation remains internal; public NativeFunc/Func fields stay intact.
p=Path('01_core/src/entities/func.rs');s=p.read_text();anchor='pub(crate) enum FuncRepr {';assert s.count(anchor)==1;s=s.replace(anchor,'''#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FormatElementKind {
    Svg,
    Png,
    Pdf,
    Html,
    Bundle,
}

'''+anchor);s=s.replace('    Native(NativeFunc),','    Native(NativeFunc),\n    Format(FormatElementKind, NativeFunc),',1);anchor='impl Func {\n';assert s.count(anchor)==1;s=s.replace(anchor,anchor+'''    pub(crate) fn native_format(kind: FormatElementKind, native: NativeFunc) -> Self {
        Self(Arc::new(FuncRepr::Format(kind, native)), Span::detached())
    }

''')
s=s.replace('FuncRepr::Native(n) => Some(n.name),','FuncRepr::Native(n) | FuncRepr::Format(_, n) => Some(n.name),');s=s.replace('FuncRepr::Native(n) => Some(n.call),','FuncRepr::Native(n) | FuncRepr::Format(_, n) => Some(n.call),');s=s.replace('FuncRepr::Native(n) => n.namespace.as_deref(),','FuncRepr::Native(n) | FuncRepr::Format(_, n) => n.namespace.as_deref(),')
# All closed fallback axes name the new representation.
s=s.replace('| FuncRepr::Native(_)\n', '| FuncRepr::Native(_)\n                | FuncRepr::Format(_, _)\n')
# element_name uses one arm per representation.
anchor='            FuncRepr::Native(_) => None,'
if anchor in s:s=s.replace(anchor,anchor+'\n            FuncRepr::Format(_, _) => None,',1)
s=s.replace('(FuncRepr::Native(a), FuncRepr::Native(b)) => a.name == b.name,','(FuncRepr::Native(a) | FuncRepr::Format(_, a), FuncRepr::Native(b) | FuncRepr::Format(_, b)) => a.name == b.name,');s=s.replace('            FuncRepr::Native(native) => {\n                0u8.hash(state);','            FuncRepr::Native(native) | FuncRepr::Format(_, native) => {\n                0u8.hash(state);');s=s.replace('Retorna `Some(fn_ptr)` apenas para `FuncRepr::Native`. Closures','Retorna `Some(fn_ptr)` para Native e Format com o mesmo NativeFunc. Closures');p.write_text(s)
# Static dispatch and native public projections preserve their payload operations.
p=Path('01_core/src/compiler/eval/call_dispatch/apply.rs');s=p.read_text();assert s.count('        FuncRepr::Native(native) => {')==1;s=s.replace('        FuncRepr::Native(native) => {','        FuncRepr::Native(native) | FuncRepr::Format(_, native) => {');p.write_text(s)
p=Path('01_core/src/compiler/eval/bindings/field_access/identity.rs');s=p.read_text();assert s.count('            FuncRepr::Native(native) => {')==2;s=s.replace('            FuncRepr::Native(native) => {','            FuncRepr::Native(native) | FuncRepr::Format(_, native) => {');s=s.replace('            FuncRepr::Native(_)\n','            FuncRepr::Native(_)\n            | FuncRepr::Format(_, _)\n');p.write_text(s)
p=Path('01_core/src/compiler/eval/observation/value.rs');s=p.read_text();anchor='            (FuncRepr::NativeWithEngine(a), FuncRepr::NativeWithEngine(b)) => {';assert s.count(anchor)==1;s=s.replace(anchor,'''            (FuncRepr::Format(ak, a), FuncRepr::Format(bk, b)) => {
                let crate::entities::func::NativeFunc { name: _, call: _, namespace: _ } = a;
                ObservationRelation::exact(ak == bk)
                    .and(ObservationRelation::exact(std::ptr::fn_addr_eq(a.call, b.call)))
                    .and(a.name.observation_relation(b.name))
                    .and(a.namespace.observation_relation(&b.namespace))
            }
'''+anchor);# precise Func fallback only
start=s.index('impl ObservationEq for Func {');end=s.index('\n#[cfg(test)]',start);part=s[start:end];assert part.count('| FuncRepr::Native(_)')==1;part=part.replace('| FuncRepr::Native(_)','| FuncRepr::Native(_)\n                | FuncRepr::Format(_, _)');s=s[:start]+part+s[end:];p.write_text(s)
# Factories retain identical NativeFunc payloads, replacing only carrier construction.
for source in ['01_core/src/compiler/stdlib/format.rs','01_core/src/compiler/stdlib/pdf.rs','01_core/src/compiler/stdlib/html/namespace.rs']:
 p=Path(source);s=p.read_text()
 if source.endswith('/namespace.rs'):s=s.replace('use super::*;','use super::*;\nuse crate::entities::func::{FormatElementKind, NativeFunc};',1)
 else:s=s.replace('use crate::entities::func::Func;','use crate::entities::func::{FormatElementKind, Func, NativeFunc};',1)
 specs=[('svg','Svg','native_format','Arc::new(svg_namespace)'),('png','Png','native_format','Arc::new(png_namespace)'),('pdf','Pdf','native_format','Arc::new(pdf_namespace)'),('html','Html','native_format','Arc::new(html_namespace)'),('bundle','Bundle','native_format','Arc::new(bundle_namespace)')] if source.endswith('/format.rs') else ([('pdf','Pdf','native_pdf_root','std::sync::Arc::new(scope)')] if source.endswith('/pdf.rs') else [('html','Html','native_html_root','Arc::new(scope)')])
 for name,kind,callback,namespace in specs:
  pattern=r'Func::native_with_namespace\(\s*"'+name+r'",\s*'+callback+r',\s*'+re.escape(namespace)+r',?\s*\)';replacement='Func::native_format(FormatElementKind::'+kind+', NativeFunc { name: "'+name+'", call: '+callback+', namespace: Some('+namespace+') })';s,n=re.subn(pattern,lambda _:replacement,s);assert n==1,(source,name,n)
 p.write_text(s)
# Internal callback transport only; no externally public new function.
p=Path('01_core/src/compiler/stdlib/html/frame.rs');s=p.read_text();assert 'pub(super) fn native_html_frame' in s;s=s.replace('pub(super) fn native_html_frame','pub(in crate::compiler) fn native_html_frame');p.write_text(s)
p=Path('01_core/src/compiler/stdlib/html.rs');s=p.read_text();anchor='pub use self::namespace::make_html_module;';assert s.count(anchor)==1;s=s.replace(anchor,anchor+'\npub(in crate::compiler) use self::frame::native_html_frame;');assert 'frame::*,' in s;s=s.replace('frame::*,','');p.write_text(s)
p=Path('01_core/src/compiler/stdlib/mod.rs');s=p.read_text();anchor='#[cfg(test)]\nmod tests;';assert s.count(anchor)==1;s=s.replace(anchor,'''pub(in crate::compiler) use html::native_html_frame;
pub(in crate::compiler) use pdf::{native_pdf_attach, native_pdf_artifact};

'''+anchor);p.write_text(s)
# Single owner holds public language metadata.
p=Path('01_core/src/compiler/eval/bindings/value_methods/element_where.rs');s=p.read_text();anchor='    use crate::compiler::stdlib::*;\n';assert s.count(anchor)==1;branch='''    use crate::entities::func::FormatElementKind;
    if let crate::entities::func::FuncRepr::Format(kind, _) = function.repr() {
        return match kind {
            FormatElementKind::Svg => Some(("svg", &["pretty"], false)),
            FormatElementKind::Png => Some(("png", &["ppi"], false)),
            FormatElementKind::Pdf => Some(("pdf", &["pages", "standard", "tagged", "pretty"], false)),
            FormatElementKind::Html => Some(("html", &["pretty"], false)),
            FormatElementKind::Bundle => Some(("bundle", &[], false)),
        };
    }
''';s=s.replace(anchor,anchor+branch);schema=json.load(open(out/'batch-schema.json'))['cases'];unique={c['callback']:c for c in schema if c['callback']};rows=['    element!('+name+', '+json.dumps(c['name'])+', '+str(c['locatable']).lower()+', ['+', '.join(map(json.dumps,c['fields']))+']);' for name,c in unique.items()];anchor='    element!(native_accent,';assert s.count(anchor)==1;s=s.replace(anchor,'\n'.join(rows)+'\n'+anchor);p.write_text(s)
(out/'implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in scope['sources']},red_failures=16,red_controls=2),indent=2)+'\n')
