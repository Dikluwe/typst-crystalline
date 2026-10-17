//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/support.md
//! @prompt-hash 9d88262f
//! @layer L1
//! @updated 2026-09-26

#![cfg(test)]

use super::*;

pub(super) fn p(items: Vec<Value>) -> Args {
    Args::positional(items)
}

pub(super) fn ck(s: &str) -> CounterKey {
    CounterKey::Str(s.into())
}

pub(super) fn pn(items: Vec<Value>, key: &str, val: Value) -> Args {
    let mut a = Args::positional(items);
    a.named.insert(key.into(), val);
    a
}

#[derive(Default)]
pub(super) struct NullWorld {
    pub(super) library: Library,
    pub(super) book: FontBook,
    pub(super) files: std::collections::HashMap<String, std::sync::Arc<Vec<u8>>>,
}
impl World for NullWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        Err(FileError::NotFound)
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, _: usize) -> Option<Font> {
        None
    }
    fn today(&self, _: Option<crate::entities::duration::Duration>) -> Option<Datetime> {
        None
    }
    fn read_bytes(
        &self,
        _current_file: FileId,
        path: &str,
    ) -> Result<std::sync::Arc<Vec<u8>>, String> {
        self.files
            .get(path)
            .map(std::sync::Arc::clone)
            .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
    }
    fn resolve_path(
        &self,
        _current_file: FileId,
        path: &str,
    ) -> Result<crate::entities::path::RootedPath, String> {
        let vpath = crate::entities::path::VirtualPath::new(path)
            .map_err(|e| format!("path inválido: {e:?}"))?;
        Ok(crate::entities::path::RootedPath::new(
            crate::entities::path::VirtualRoot::Project,
            vpath,
        ))
    }
    fn read_path(
        &self,
        path: &crate::entities::path::RootedPath,
    ) -> Result<std::sync::Arc<Vec<u8>>, String> {
        let key = path.vpath().get_with_slash().trim_start_matches('/');
        self.files
            .get(key)
            .map(std::sync::Arc::clone)
            .ok_or_else(|| format!("ficheiro não encontrado: {key}"))
    }
}

macro_rules! null_ctx {
    ($ctx:ident) => {
        let mut $ctx = EvalContext::new();
    };
}

pub(super) fn with_engine<R>(f: impl FnOnce(&mut Engine<'_>, &NullWorld) -> R) -> R {
    let world = null_world();
    let route = Route::root();
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards: Vec<RuleId> = Vec::new();
    let mut sink_local = Sink::new();
    let mut sink = sink_local.track_mut();
    let fixed_metrics = FixedMetrics;
    let mut engine = Engine {
        world: &world,
        font_metrics: &fixed_metrics,
        route: route.track(),
        styles: &mut styles,
        show_rules: &mut show_rules,
        active_guards: &mut active_guards,
        current_file: test_file_id(),
        sink: &mut sink,
    };
    f(&mut engine, &world)
}

pub(super) fn null_world() -> NullWorld {
    NullWorld::default()
}

pub(super) fn test_file_id() -> crate::entities::file_id::FileId {
    crate::entities::file_id::FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
}

pub(super) fn approx_float(v: Value, expected: f64) {
    match v {
        Value::Float(f) => {
            assert!((f - expected).abs() < 1e-10, "esperado {expected}, obtido {f}",)
        }
        other => panic!("esperado Value::Float, obtido {other:?}"),
    }
}

pub(super) fn approx_angle(v: Value, expected_rad: f64) {
    match v {
        Value::Angle(a) => {
            assert!(
                (a.to_rad() - expected_rad).abs() < 1e-10,
                "esperado {expected_rad} rad, obtido {} rad",
                a.to_rad(),
            )
        }
        other => panic!("esperado Value::Angle, obtido {other:?}"),
    }
}

pub(super) fn approx_float_erf(v: Value, expected: f64) {
    match v {
        Value::Float(f) => assert!(
            (f - expected).abs() < 2e-7,
            "esperado ≈{expected} (tol 2e-7), obtido {f}",
        ),
        other => panic!("esperado Value::Float, obtido {other:?}"),
    }
}

pub(super) fn world_com_png() -> NullWorld {
    let mut world = NullWorld::default();
    world.files.insert(
        "ok.png".to_string(),
        std::sync::Arc::new(b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0DIHDR".to_vec()),
    );
    world
}

pub(super) fn p804_open_path_points(
    kind: &crate::entities::geometry::ShapeKind,
) -> ((f64, f64), (f64, f64)) {
    use crate::entities::geometry::{PathItem, ShapeKind};
    match kind {
        ShapeKind::Path(items) => match items.as_slice() {
            [PathItem::MoveTo(start), PathItem::LineTo(end)] => {
                ((start.x.val(), start.y.val()), (end.x.val(), end.y.val()))
            }
            other => panic!("esperado path aberto de dois pontos: {other:?}"),
        },
        other => panic!("esperado Path, obtido {other:?}"),
    }
}

pub(super) fn p732_polygon_args_length() -> Args {
    // Triângulo do caso medido do passo: (0pt,0pt) (50pt,0pt) (25pt,40pt).
    use crate::entities::layout_types::Length;
    Args::positional(vec![
        Value::Array(vec![
            Value::Length(Length::pt(0.0)),
            Value::Length(Length::pt(0.0)),
        ]),
        Value::Array(vec![
            Value::Length(Length::pt(50.0)),
            Value::Length(Length::pt(0.0)),
        ]),
        Value::Array(vec![
            Value::Length(Length::pt(25.0)),
            Value::Length(Length::pt(40.0)),
        ]),
    ])
}

pub(super) fn p727_curve_args_minimos() -> Args {
    Args::positional(vec![
        Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("line".into()),
            Value::Array(vec![Value::Float(50.0), Value::Float(50.0)]),
        ]),
    ])
}

pub(super) fn measure_default(content: &Content) -> (f64, f64) {
    use crate::compiler::layout::measure_content_real;
    use crate::entities::style_chain::StyleChain;
    measure_content_real(content, &StyleChain::default_chain(), &FixedMetrics)
}

pub(super) fn make_bib_dict(key: &str, author: &str, title: &str, year: i64) -> Value {
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("key".into(), Value::Str(key.into()));
    d.insert("author".into(), Value::Str(author.into()));
    d.insert("title".into(), Value::Str(title.into()));
    d.insert("year".into(), Value::Int(year));
    Value::Dict(d)
}

#[expect(
    clippy::too_many_arguments,
    reason = "C121: Factory cfg(test) constrói um único dicionário bibliográfico com os oito campos fornecidos, sem política de produção"
)]
pub(super) fn make_bib_dict_full(
    key: &str,
    author: &str,
    title: &str,
    year: i64,
    volume: &str,
    pages: &str,
    journal: &str,
    publisher: &str,
) -> Value {
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("key".into(), Value::Str(key.into()));
    d.insert("author".into(), Value::Str(author.into()));
    d.insert("title".into(), Value::Str(title.into()));
    d.insert("year".into(), Value::Int(year));
    d.insert("volume".into(), Value::Str(volume.into()));
    d.insert("pages".into(), Value::Str(pages.into()));
    d.insert("journal".into(), Value::Str(journal.into()));
    d.insert("publisher".into(), Value::Str(publisher.into()));
    Value::Dict(d)
}

pub(super) fn make_bib_dict_with_url_doi(
    key: &str,
    author: &str,
    title: &str,
    year: i64,
    url: &str,
    doi: &str,
) -> Value {
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("key".into(), Value::Str(key.into()));
    d.insert("author".into(), Value::Str(author.into()));
    d.insert("title".into(), Value::Str(title.into()));
    d.insert("year".into(), Value::Int(year));
    d.insert("url".into(), Value::Str(url.into()));
    d.insert("doi".into(), Value::Str(doi.into()));
    Value::Dict(d)
}

#[expect(
    clippy::too_many_arguments,
    reason = "C122: Factory cfg(test) da extensão bibliográfica mantém dez campos explícitos e valores literais dos testes"
)]
pub(super) fn make_bib_dict_full_p159g(
    key: &str,
    author: &str,
    title: &str,
    year: i64,
    editor: &str,
    series: &str,
    note: &str,
    isbn: &str,
    location: &str,
    organization: &str,
) -> Value {
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("key".into(), Value::Str(key.into()));
    d.insert("author".into(), Value::Str(author.into()));
    d.insert("title".into(), Value::Str(title.into()));
    d.insert("year".into(), Value::Int(year));
    d.insert("editor".into(), Value::Str(editor.into()));
    d.insert("series".into(), Value::Str(series.into()));
    d.insert("note".into(), Value::Str(note.into()));
    d.insert("isbn".into(), Value::Str(isbn.into()));
    d.insert("location".into(), Value::Str(location.into()));
    d.insert("organization".into(), Value::Str(organization.into()));
    Value::Dict(d)
}

pub(super) fn lookup_math(name: &str) -> Option<Value> {
    use super::make_math_module;
    // P731 — `math` passa a ser `Value::Module` (era `Value::Dict`).
    if let Value::Module(m) = make_math_module() {
        m.scope().get(name).cloned()
    } else {
        None
    }
}

use crate::entities::math_style::MathStyleKind;

pub(super) fn call_math_style(
    f: fn(&mut EvalContext, &Args, &dyn World, FileId) -> SourceResult<Value>,
    items: Vec<Value>,
) -> SourceResult<Value> {
    null_ctx!(ctx);
    f(&mut ctx, &p(items), &null_world(), test_file_id())
}

pub(super) fn assert_styled(
    v: SourceResult<Value>,
    exp_kind: Option<MathStyleKind>,
    exp_bold: Option<bool>,
    exp_italic: Option<bool>,
    exp_cramped: Option<bool>,
) {
    match v.unwrap() {
        Value::Content(Content::MathStyled(m)) => {
            assert_eq!(m.kind, exp_kind, "kind mismatch");
            assert_eq!(m.bold, exp_bold, "bold mismatch");
            assert_eq!(m.italic, exp_italic, "italic mismatch");
            assert_eq!(m.cramped, exp_cramped, "cramped mismatch");
        }
        other => panic!("esperado Content::MathStyled, obteve {other:?}"),
    }
}

pub(super) fn run_eval(source: &str) -> SourceResult<Value> {
    with_engine(|engine, world| {
        let mut ctx = EvalContext::new();
        let mut scopes = Scopes::new(None);
        native_eval(
            &mut ctx,
            &p(vec![Value::Str(source.into())]),
            world,
            test_file_id(),
            &mut scopes,
            engine,
        )
    })
}

pub(super) fn run_eval_with_var(
    source: &str,
    name: &str,
    value: Value,
) -> SourceResult<Value> {
    with_engine(|engine, world| {
        let mut ctx = EvalContext::new();
        let mut scopes = Scopes::new(None);
        scopes.define(name, value);
        native_eval(
            &mut ctx,
            &p(vec![Value::Str(source.into())]),
            world,
            test_file_id(),
            &mut scopes,
            engine,
        )
    })
}
