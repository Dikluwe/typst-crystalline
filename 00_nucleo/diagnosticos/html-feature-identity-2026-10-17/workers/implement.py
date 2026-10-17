from pathlib import Path
import gzip,json,re,datetime
out=Path(__file__).resolve().parents[1];assert json.load(open(out/'red-resources.json'))['exit']==101
assert json.load(open(out/'red-result.json'))['functional_red']
import hashlib
red=json.load(gzip.open(out/'red-state.json.gz'));assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in red['source_hashes'].items())
p=Path('01_core/src/entities/module.rs');s=p.read_text();s=s.replace('#[derive(Debug)]\nstruct ModuleInner {','#[derive(Debug, Clone, Copy, PartialEq, Eq)]\nenum ModuleKind {\n    Ordinary,\n    Std,\n    Format,\n}\n\n#[derive(Debug)]\nstruct ModuleInner {\n    kind: ModuleKind,',1)
a='''    pub fn new(name: impl Into<String>, scope: Scope) -> Self {
        Self(Arc::new(ModuleInner {
            name: name.into(),''';z='''    pub fn new(name: impl Into<String>, scope: Scope) -> Self {
        Self::with_kind(name.into(), scope, ModuleKind::Ordinary)
    }

    pub(crate) fn stdlib(scope: Scope) -> Self {
        Self::with_kind("global".into(), scope, ModuleKind::Std)
    }

    pub(crate) fn format(scope: Scope) -> Self {
        Self::with_kind("format".into(), scope, ModuleKind::Format)
    }

    pub(crate) fn is_std(&self) -> bool {
        self.0.kind == ModuleKind::Std
    }

    pub(crate) fn is_format(&self) -> bool {
        self.0.kind == ModuleKind::Format
    }

    fn with_kind(name: String, scope: Scope, kind: ModuleKind) -> Self {
        Self(Arc::new(ModuleInner {
            kind,
            name,''';assert s.count(a)==1;s=s.replace(a,z);p.write_text(s)
for f,count in [('01_core/src/compiler/eval/mod.rs',2),('01_core/src/compiler/eval/modules.rs',1)]:
 p=Path(f);s=p.read_text();a='Module::new("global", stdlib.clone())';assert s.count(a)==count;p.write_text(s.replace(a,'Module::stdlib(stdlib.clone())'))
p=Path('01_core/src/compiler/stdlib/format.rs');s=p.read_text();a='Module::new("format", scope)';assert s.count(a)==1;p.write_text(s.replace(a,'Module::format(scope)'))
p=Path('01_core/src/compiler/eval/bindings/field_access/identity.rs');s=p.read_text();start=s.index('pub(super) fn is_effective_format_module(');end=s.index('/// Compara por identidade Arc',start);s=s[:start]+'''pub(super) fn is_effective_format_module(module: &crate::entities::module::Module) -> bool {
    module.is_format()
}

pub(super) fn is_effective_std_module(module: &crate::entities::module::Module) -> bool {
    module.is_std()
}
'''+s[end:];p.write_text(s)
p=Path('01_core/src/compiler/eval/bindings/field_access/fields.rs');s=p.read_text();a='''if field == "html"
        && matches!(&target, Value::Module(module) if is_effective_format_module(module))''';z='''if field == "html"
        && matches!(&target, Value::Module(module) if is_effective_std_module(module) || is_effective_format_module(module))''';assert s.count(a)==1;s=s.replace(a,z);s=s.replace('// P1465 — `format.html` existe no módulo efetivo em todos os perfis, mas','// `std.html` e `format.html` existem internamente em todos os perfis, mas');p.write_text(s)
(out/'implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources=json.load(open(out/'scope.json'))['sources']),indent=2)+'\n');print('Private module identity implemented')
