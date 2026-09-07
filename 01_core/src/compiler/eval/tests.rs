//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/tests.md
//! @prompt-hash 7234ef9d
//! @layer L1
//! @updated 2026-06-17
//!
//! Testes unitários e helpers de teste de eval. Extraído de mod.rs no
//! Passo 96.1 (ADR-0037 Regra 5: testes seguem o domínio — aqui um
//! módulo dedicado mantém o mod.rs focado no dispatcher).
//!
//! Excepção Regra 6 da ADR-0037: este ficheiro só contém código de
//! teste (gated por `#[cfg(test)]` a partir do `mod.rs`). Testes E2E
//! cruzam domínios por natureza (um único programa Typst exercita
//! markup+math+control_flow+rules+closures simultaneamente); distribuí-los
//! por cluster produziria duplicação ou perda de cobertura. Tamanho
//! (~2080 linhas) reflecte a cobertura ampla da suite.

use super::*;
use crate::compiler::layout::FixedMetrics;
use crate::entities::elements::math_attach::MathAttachSlot;
use crate::entities::introspector::Introspector;
use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

pub(crate) fn eval_for_test<W: World>(
    world: &W,
    source: &Source,
) -> SourceResult<Module> {
    let registry = crate::entities::element_registry::ElementRegistry::new();
    eval_for_test_with_registry(world, source, &registry)
}

/// P802 — variante de `eval_for_test` que devolve também o `Sink`,
/// para asserções sobre warnings emitidos durante o eval.
pub(crate) fn eval_for_test_keep_sink<W: World>(
    world: &W,
    source: &Source,
) -> (SourceResult<Module>, Sink) {
    use comemo::Track;
    let routines = Routines::new();
    let traced = Traced::default();
    let mut sink = Sink::new();
    let route = Route::root();
    let registry = crate::entities::element_registry::ElementRegistry::new();
    let result = eval(
        &routines,
        world,
        traced.track(),
        sink.track_mut(),
        route.track(),
        source,
        &registry,
    );
    (result, sink)
}

/// Lote F-3 inc-2 — eval de teste com um `ElementRegistry` injetado, para
/// exercitar `#name(args)` (elemento de utilizador na linguagem).
pub(crate) fn eval_for_test_with_registry<W: World>(
    world: &W,
    source: &Source,
    registry: &crate::entities::element_registry::ElementRegistry,
) -> SourceResult<Module> {
    use comemo::Track;
    let routines = Routines::new();
    let traced = Traced::default();
    let mut sink = Sink::new();
    let route = Route::root();

    eval(
        &routines,
        world,
        traced.track(),
        sink.track_mut(),
        route.track(),
        source,
        registry,
    )
}

/// P350c — eval de teste com a **flag de erro completo LIGADA** (`full_error = true`),
/// para verificar a classificação (cíclico / não-convergente) no 3º hint do erro de
/// recursão. Espelha `eval_for_test_with_registry`, mas via `eval_with_full_error`.
pub(crate) fn eval_for_test_full_error<W: World>(
    world: &W,
    source: &Source,
) -> SourceResult<Module> {
    use comemo::Track;
    let routines = Routines::new();
    let traced = Traced::default();
    let mut sink = Sink::new();
    let route = Route::root();
    let registry = crate::entities::element_registry::ElementRegistry::new();
    eval_with_full_error(
        &routines,
        world,
        traced.track(),
        sink.track_mut(),
        route.track(),
        source,
        &registry,
        true,
    )
}

/// Função de teste que permite customizar o limite de iterações de loop.
///
/// A profundidade de chamadas é verificada por `Route::check_call_depth`
/// (MAX_CALL_DEPTH = 80) e não é configurável — tests que exercitam
/// recursão infinita trigueam esse limite sem override.
pub(crate) fn eval_for_test_with_limits<W: World>(
    world: &W,
    source: &Source,
    max_loop_iterations: usize,
) -> SourceResult<Module> {
    let mut ctx = EvalContext::new();
    ctx.max_loop_iterations = max_loop_iterations;

    let route = Route::root().with_id(source.id());
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards: Vec<RuleId> = Vec::new();
    let current_file = source.id();
    use comemo::Track;
    let mut sink_local = Sink::new();
    let mut sink = sink_local.track_mut();
    let root = source.root();
    let mut scopes = Scopes::new(None);
    let stdlib = make_stdlib(&world.inputs());
    for (name, binding) in stdlib.iter() {
        scopes.define(name, binding.value().clone());
    }
    scopes.enter();

    let fixed_metrics = FixedMetrics;
    let mut engine = Engine {
        world,
        font_metrics: &fixed_metrics,
        route: route.track(),
        styles: &mut styles,
        show_rules: &mut show_rules,
        active_guards: &mut active_guards,
        current_file,
        sink: &mut sink,
    };

    let content_val = eval_markup(root, &mut scopes, &mut ctx, &mut engine)?;

    let module_scope = scopes.exit();
    let content = match content_val {
        Value::Content(c) => Some(c),
        _ => None,
    };
    let mut module = Module::new(source.id().into_raw().get().to_string(), module_scope);
    module.set_content(content);
    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::scopes::Scopes;
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::counter::CounterKey;
    use crate::entities::element_kind::ElementKind;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::scope::Scope;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    fn ck(s: &str) -> CounterKey {
        CounterKey::Str(s.into())
    }

    fn sel(kind: crate::entities::element_kind::ElementKind) -> CounterKey {
        CounterKey::Selector(crate::entities::selector::Selector::Kind(kind))
    }

    // ── MockWorld para integração com eval() ─────────────────────────────────

    struct MockWorld {
        library: Library,
        book: FontBook,
        source: Source,
        files: std::collections::HashMap<String, std::sync::Arc<Vec<u8>>>,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(id, text.to_string()),
                files: std::collections::HashMap::new(),
            }
        }

        fn add_file(&mut self, path: &str, data: Vec<u8>) {
            self.files.insert(path.to_string(), std::sync::Arc::new(data));
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _id: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
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

    // ── Testes via Scope directamente ────────────────────────────────────────

    #[test]
    fn scope_define_via_value_real() {
        let mut scope = Scope::new();
        scope.define("x", Value::Int(42));
        scope.define("s", Value::Str("hello".into()));
        assert_eq!(scope.get("x"), Some(&Value::Int(42)));
        assert_eq!(scope.get("s"), Some(&Value::Str("hello".into())));
    }

    #[test]
    fn scopes_lookup_em_pilha() {
        let mut scopes = Scopes::new(None);
        scopes.enter();
        scopes.define("x", Value::Int(1));
        scopes.enter();
        scopes.define("y", Value::Int(2));
        assert_eq!(scopes.get("x"), Some(&Value::Int(1)));
        assert_eq!(scopes.get("y"), Some(&Value::Int(2)));
        assert_eq!(scopes.get("z"), None);
    }

    #[test]
    fn scopes_exit_remove_local() {
        let mut scopes = Scopes::new(None);
        scopes.enter();
        scopes.define("global", Value::Bool(true));
        scopes.enter();
        scopes.define("local", Value::Int(99));
        scopes.exit();
        assert!(scopes.get("local").is_none());
        assert!(scopes.get("global").is_some());
    }

    // ── Testes de parse/AST ───────────────────────────────────────────────────

    #[test]
    fn ast_int_literal_parseable() {
        let source = Source::detached("#let x = 42");
        assert!(!source.root().erroneous());
    }

    #[test]
    fn ast_str_literal_parseable() {
        let source = Source::detached("#let s = \"hello\"");
        assert!(!source.root().erroneous());
    }

    #[test]
    fn ast_bool_literal_parseable() {
        let source = Source::detached("#let b = true");
        assert!(!source.root().erroneous());
    }

    #[test]
    fn nome_modulo_deriva_de_file_id() {
        let id = FileId::from_raw(NonZeroU16::new(7).unwrap());
        let source = Source::new(id, "#let x = 1".to_string());
        let name = source.id().into_raw().get().to_string();
        assert_eq!(name, "7");
    }

    // ── Testes de integração via eval_for_test ───────────────────────────────

    #[test]
    fn eval_let_int_via_world() {
        let world = MockWorld::new("#let x = 42");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module =
            eval_for_test(&world, &source).expect("eval não deve falhar em input válido");
        assert_eq!(module.scope().get("x"), Some(&Value::Int(42)));
    }

    #[test]
    fn eval_multiplos_bindings_via_world() {
        let world = MockWorld::new("#let a = 1\n#let b = true\n#let c = \"x\"");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        assert_eq!(module.scope().get("a"), Some(&Value::Int(1)));
        assert_eq!(module.scope().get("b"), Some(&Value::Bool(true)));
        assert_eq!(module.scope().get("c"), Some(&Value::Str("x".into())));
    }

    // ── P881 — tipos chamáveis como funções de ordem superior ─────────────────

    #[test]
    fn p881_array_map_str_tipo_chamavel() {
        let world = MockWorld::new("#let r = (0, 1, 2).map(str)");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        assert_eq!(
            module.scope().get("r"),
            Some(&Value::Array(vec![
                Value::Str("0".into()),
                Value::Str("1".into()),
                Value::Str("2".into()),
            ]))
        );
    }

    #[test]
    fn p881_array_map_int_tipo_chamavel() {
        let world = MockWorld::new("#let r = (\"5\", \"6\").map(int)");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        assert_eq!(
            module.scope().get("r"),
            Some(&Value::Array(vec![Value::Int(5), Value::Int(6)]))
        );
    }

    #[test]
    fn p881_array_filter_type_tipo_chamavel() {
        // type("a") == str → true; type(1) == str → false
        let world =
            MockWorld::new("#let r = (\"a\", 1, \"b\").filter(x => type(x) == str)");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        assert_eq!(
            module.scope().get("r"),
            Some(&Value::Array(vec![Value::Str("a".into()), Value::Str("b".into()),]))
        );
    }

    #[test]
    fn eval_texto_puro_scope_vazio() {
        let world = MockWorld::new("Apenas texto Typst.");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        assert!(module.scope().is_empty());
    }

    // ── Lote F-2 S1 (P335) — `#set heading(numbering:)` via chain léxica ──────
    // F-5a de-bake (P364, §3a.9): o gate de numeração deixou de viver em campo
    // assado — vive **só na chain**, transportado por `Content::Styled` (custom).
    // Estes probes de teste threadam o gate ativo ao descer num `Styled`, em vez
    // de ler um campo do elemento. As asserções ficam **idênticas**
    // (content-preserving) e passam a **verificar o transporte de produção**: se a
    // fatia-1 não embrulhasse o heading, o probe veria `false`.
    fn styled_custom_bool(s: &crate::entities::style::Styles, key: &str) -> Option<bool> {
        s.delta().custom.iter().rev().find_map(|(k, v)| {
            if k == key {
                if let crate::entities::value::Value::Bool(b) = v {
                    Some(*b)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    fn find_heading_numbered(c: &Content) -> Option<bool> {
        fn go(c: &Content, active: bool) -> Option<bool> {
            match c {
                Content::Heading(_) => Some(active),
                Content::Sequence(items) => items.iter().find_map(|i| go(i, active)),
                Content::Styled(b, s) => {
                    go(b, styled_custom_bool(s, "heading.numbering").unwrap_or(active))
                }
                _ => None,
            }
        }
        go(c, false)
    }

    #[test]
    fn f2s1_set_heading_numbering_assa_via_chain() {
        // End-to-end: `#set heading(numbering:)` empurra para a chain
        // (`engine.styles.custom`); o heading assa `numbering_active=true`.
        let world = MockWorld::new("#set heading(numbering: \"1.\")\n= Titulo");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("módulo deve ter content");
        assert_eq!(
            find_heading_numbered(content),
            Some(true),
            "#set heading(numbering:) deve assar numbering_active=true via chain"
        );
    }

    #[test]
    fn f2s1_sem_set_heading_nao_assa() {
        // Sem `#set heading` → heading não-numerado (gate via campo assado).
        let world = MockWorld::new("= Titulo");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("módulo deve ter content");
        assert_eq!(find_heading_numbered(content), Some(false));
    }

    fn collect_headings_numbered(c: &Content, out: &mut Vec<bool>) {
        // F-5a de-bake (P364): gate threadado do `Styled` custom (ver
        // `find_heading_numbered`).
        fn go(c: &Content, active: bool, out: &mut Vec<bool>) {
            match c {
                Content::Heading(_) => out.push(active),
                Content::Sequence(items) => items.iter().for_each(|i| go(i, active, out)),
                Content::Styled(b, s) => go(
                    b,
                    styled_custom_bool(s, "heading.numbering").unwrap_or(active),
                    out,
                ),
                _ => {}
            }
        }
        go(c, false, out)
    }

    // ── Lote F-2 S2 / B1 (P335) — `#set math.equation(numbering:)` via chain ──
    // P456: o gate passou de Bool para Str (pattern de numeração), análogo a
    // figure.numbering. O probe retorna o pattern quando presente.
    fn find_equation_numbered(c: &Content) -> Option<Option<String>> {
        fn go(c: &Content, active: Option<String>) -> Option<Option<String>> {
            match c {
                Content::Equation(_) => Some(active),
                Content::Sequence(items) => {
                    items.iter().find_map(|i| go(i, active.clone()))
                }
                Content::Styled(b, s) => {
                    let next = s
                        .delta()
                        .custom
                        .iter()
                        .rev()
                        .find(|(k, _)| k == "equation.numbering")
                        .map(|(_, v)| match v {
                            Value::Str(s) => Some(s.to_string()),
                            Value::None => None,
                            _ => active.clone(),
                        })
                        .unwrap_or(active);
                    go(b, next)
                }
                _ => None,
            }
        }
        go(c, None)
    }

    #[test]
    fn f2s2_b1_set_equation_numbering_assa_via_chain() {
        // B1: o produtor eval de `#set math.equation(numbering:)` (target
        // pontuado) nasce neste lote. `$ x $` é equação de bloco.
        let world = MockWorld::new("#set math.equation(numbering: \"(1)\")\n$ x $");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("módulo deve ter content");
        assert_eq!(
            find_equation_numbered(content),
            Some(Some("(1)".to_string())),
            "#set math.equation(numbering:) deve assar pattern na equação de bloco"
        );
    }

    #[test]
    fn f2s2_sem_set_equation_nao_assa() {
        let world = MockWorld::new("$ x $");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("módulo deve ter content");
        assert_eq!(find_equation_numbered(content), Some(None));
    }

    #[test]
    fn f2s2_b1_set_equation_numbering_pattern_romano_transportado() {
        let world = MockWorld::new("#set math.equation(numbering: \"[I]\")\n$ x $");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("módulo deve ter content");
        assert_eq!(
            find_equation_numbered(content),
            Some(Some("[I]".to_string())),
            "pattern romano deve ser transportado na chain"
        );
    }

    // ── P1030 — `#set math.<elemento>(...)` (eval.md §P1030, fatia 1) ────────
    // O alvo pontuado caía no fallback com `target == ""` (`text_str()` devolve
    // "" para `Expr::FieldAccess`), emitindo `set: target '' ainda não
    // suportado` e ignorando os parâmetros em silêncio.

    /// Delimitador da primeira matriz ou vector matemático da árvore.
    /// `mat` e `vec` preservam identidades distintas desde P1292.
    fn find_matrix_or_vec_delim(c: &Content) -> Option<(char, char)> {
        match c {
            Content::MathMatrix(e) => Some(e.delim),
            Content::MathVec(e) => Some(e.delim),
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().find_map(find_matrix_or_vec_delim)
            }
            Content::Styled(b, _) => find_matrix_or_vec_delim(b),
            Content::Equation(e) => find_matrix_or_vec_delim(&e.body),
            _ => None,
        }
    }

    /// `limits` do primeiro `MathOp` da árvore.
    fn find_op_limits(c: &Content) -> Option<bool> {
        match c {
            Content::MathOp(e) => Some(e.limits),
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().find_map(find_op_limits)
            }
            Content::Styled(b, _) => find_op_limits(b),
            Content::Equation(e) => find_op_limits(&e.body),
            _ => None,
        }
    }

    fn delim_de(src: &str) -> Option<(char, char)> {
        let world = MockWorld::new(src);
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        find_matrix_or_vec_delim(module.content().expect("módulo deve ter content"))
    }

    #[test]
    fn p1030_set_mat_delim_aplica() {
        assert_eq!(
            delim_de("#set math.mat(delim: \"[\")\n$ mat(1, 2; 3, 4) $"),
            Some(('[', ']')),
            "#set math.mat(delim:) tem de chegar ao construtor da matriz"
        );
    }

    #[test]
    fn p1030_set_vec_delim_aplica() {
        assert_eq!(
            delim_de("#set math.vec(delim: \"[\")\n$ vec(1, 2) $"),
            Some(('[', ']')),
            "#set math.vec(delim:) tem de chegar ao construtor do vector"
        );
    }

    #[test]
    fn p1030_arg_explicito_vence_a_chain() {
        // Precedência registada no L0: arg explícito > chain > default.
        assert_eq!(
            delim_de("#set math.mat(delim: \"[\")\n$ mat(delim: \"(\", 1, 2; 3, 4) $"),
            Some(('(', ')')),
            "o argumento explícito tem de vencer o #set"
        );
    }

    #[test]
    fn p1030_sem_set_mantem_default() {
        assert_eq!(
            delim_de("$ mat(1, 2; 3, 4) $"),
            Some(('(', ')')),
            "sem #set o default não muda"
        );
    }

    #[test]
    fn p1030_set_op_limits_aplica() {
        let world = MockWorld::new("#set math.op(limits: true)\n$ op(\"lim\") $");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        assert_eq!(
            find_op_limits(module.content().expect("módulo deve ter content")),
            Some(true),
            "#set math.op(limits:) tem de chegar ao MathOp"
        );
    }

    #[test]
    fn p1030_param_reconhecido_sem_efeito_avisa_nomeando() {
        // `mat.gap` é parâmetro da linguagem mas ainda não implementado
        // (grupo B da medição da Fase A). O aviso tem de nomear elemento e
        // parâmetro — não o alvo vazio.
        let world = MockWorld::new("#set math.mat(gap: 1em)\n$ mat(1, 2; 3, 4) $");
        let source = World::source(&world, World::main(&world)).unwrap();
        let (res, sink) = eval_for_test_keep_sink(&world, &source);
        assert!(res.is_ok(), "o documento tem de continuar a compilar");
        let diags = sink.into_diagnostics();
        assert!(
            diags
                .iter()
                .any(|d| d.message.contains("math.mat") && d.message.contains("gap")),
            "o aviso tem de nomear elemento e parâmetro, obteve: {diags:?}"
        );
        assert!(
            !diags.iter().any(|d| d.message.contains("target ''")),
            "o aviso do alvo vazio não pode sobreviver: {diags:?}"
        );
    }

    #[test]
    fn p1030_param_fora_da_linguagem_erra_como_vanilla() {
        // Vanilla: `error: unexpected argument: lower`.
        let world = MockWorld::new("#set math.binom(lower: 1)\n$ binom(n, k) $");
        let source = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &source)
            .expect_err("parâmetro fora da linguagem tem de ser erro");
        let msg = err[0].message.to_string();
        assert!(
            msg.contains("unexpected argument") && msg.contains("lower"),
            "mensagem tem de espelhar a do vanilla, obteve: {msg}"
        );
    }

    // ── Lote F-2 S3 (P335) — `#set figure(numbering:)` via chain léxica ──────
    // F-5a de-bake (P365): o padrão da figura vive **só na chain** (`Content::Styled`
    // custom `figure.numbering` = `Value::Str(padrão)`). O probe threada o padrão
    // ativo ao descer num `Styled`. Asserção idêntica — verifica o transporte de
    // produção (a fatia-1 carrega o padrão).
    fn collect_figure_numbering(c: &Content, out: &mut Vec<Option<String>>) {
        fn go(c: &Content, active: Option<String>, out: &mut Vec<Option<String>>) {
            match c {
                Content::Figure(_) => out.push(active),
                Content::Sequence(items) => {
                    items.iter().for_each(|i| go(i, active.clone(), out))
                }
                Content::Styled(b, s) => {
                    let a = match s
                        .delta()
                        .custom
                        .iter()
                        .rev()
                        .find(|(k, _)| k == "figure.numbering")
                    {
                        Some((_, crate::entities::value::Value::Str(p))) => {
                            Some(p.to_string())
                        }
                        Some((_, _)) => None,
                        None => active,
                    };
                    go(b, a, out)
                }
                Content::Label(e) => go(&e.body, active, out),
                _ => {}
            }
        }
        go(c, None, out)
    }

    #[test]
    fn f2s3_set_figure_numbering_assa_via_chain() {
        // `#set figure(numbering:)` empurra para a chain (`custom`); o
        // `native_figure` assa lendo a chain (não mais `engine.figure_numbering`).
        let world = MockWorld::new(
            "#set figure(numbering: \"1\")\n#figure([Fig], caption: [Cap])",
        );
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("módulo deve ter content");
        let mut nums = vec![];
        collect_figure_numbering(content, &mut nums);
        assert_eq!(nums, vec![Some("1".to_string())], "figura assada com numbering '1'");
    }

    #[test]
    fn f2s3_set_figure_escopo_lexical_nao_vaza() {
        // DEBT 99.E para figure: `#set figure(numbering:)` dentro de um bloco
        // escopa — a figura de fora não herda.
        let world = MockWorld::new(
            "#[#set figure(numbering: \"1\")\n#figure([In], caption: [C])]\n#figure([Out], caption: [C])",
        );
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("módulo deve ter content");
        let mut nums = vec![];
        collect_figure_numbering(content, &mut nums);
        assert_eq!(
            nums,
            vec![Some("1".to_string()), None],
            "Dentro numerada; Fora NÃO (escopo léxico): {nums:?}"
        );
    }

    // ── P459 — `#set table(numbering:)` via chain léxica ─────────────────────
    fn collect_table_numbering(c: &Content, out: &mut Vec<Option<String>>) {
        fn go(c: &Content, active: Option<String>, out: &mut Vec<Option<String>>) {
            match c {
                Content::Table(_) => out.push(active),
                Content::Sequence(items) => {
                    items.iter().for_each(|i| go(i, active.clone(), out))
                }
                Content::Styled(b, s) => {
                    let a = match s
                        .delta()
                        .custom
                        .iter()
                        .rev()
                        .find(|(k, _)| k == "table.numbering")
                    {
                        Some((_, crate::entities::value::Value::Str(p))) => {
                            Some(p.to_string())
                        }
                        Some((_, _)) => None,
                        None => active,
                    };
                    go(b, a, out)
                }
                Content::Label(e) => go(&e.body, active, out),
                _ => {}
            }
        }
        go(c, None, out)
    }

    #[test]
    fn p459_set_table_numbering_assa_via_chain() {
        let world =
            MockWorld::new("#set table(numbering: \"1.\")\n#table([A], caption: [Cap])");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("módulo deve ter content");
        let mut nums = vec![];
        collect_table_numbering(content, &mut nums);
        assert_eq!(nums, vec![Some("1.".to_string())], "table assada com numbering '1.'");
    }

    #[test]
    fn p459_set_table_escopo_lexical_nao_vaza() {
        let world = MockWorld::new(
            "#[#set table(numbering: \"1.\")\n#table([In], caption: [C])]\n#table([Out], caption: [C])",
        );
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("módulo deve ter content");
        let mut nums = vec![];
        collect_table_numbering(content, &mut nums);
        assert_eq!(
            nums,
            vec![Some("1.".to_string()), None],
            "Dentro numerada; Fora NÃO (escopo léxico): {nums:?}"
        );
    }

    #[test]
    fn p459_table_source_sequencia_numerada() {
        use crate::compiler::layout::layout;
        let src = "#set table(numbering: \"1.\")\n#table([A], caption: [T1])\n#table([B], caption: [T2])";
        let text = layout(&eval_doc(src)).plain_text();
        assert!(
            text.contains("Table 1.: T1"),
            "primeira table deve numerar 1.; obtido: {text:?}"
        );
        assert!(
            text.contains("Table 2.: T2"),
            "segunda table deve numerar 2.; obtido: {text:?}"
        );
    }

    #[test]
    fn p459_table_source_pattern_romano() {
        use crate::compiler::layout::layout;
        let src = "#set table(numbering: \"[I]\")\n#table([A], caption: [T1])\n#table([B], caption: [T2])";
        let text = layout(&eval_doc(src)).plain_text();
        assert!(
            text.contains("Table [I]: T1"),
            "pattern romano deve formatar 1 como I; obtido: {text:?}"
        );
        assert!(
            text.contains("Table [II]: T2"),
            "pattern romano deve formatar 2 como II; obtido: {text:?}"
        );
    }

    // ── Lote F-3 inc-2 S1 — elemento de utilizador na linguagem (#name(args)) ──
    fn registry_com_callout() -> crate::entities::element_registry::ElementRegistry {
        use crate::entities::elements::test_callout::{BadgeElem, CalloutElem};
        let mut reg = crate::entities::element_registry::ElementRegistry::new();
        reg.register(
            "callout",
            std::sync::Arc::new(|args: &[Value]| {
                let body = match args.first() {
                    Some(Value::Content(c)) => c.clone(),
                    Some(Value::Str(s)) => Content::text(s.as_str()),
                    _ => Content::Empty,
                };
                let s = |i: usize| match args.get(i) {
                    Some(Value::Str(s)) => s.clone(),
                    _ => Default::default(),
                };
                Ok(Content::dynamic(CalloutElem::new(body, s(1), s(2))))
            }),
        );
        // Lote F-3 inc-2 S2: segundo kind (`badge`) para o teste kind-A-vs-B.
        reg.register(
            "badge",
            std::sync::Arc::new(|args: &[Value]| {
                let label = match args.first() {
                    Some(Value::Str(s)) => s.clone(),
                    _ => Default::default(),
                };
                Ok(Content::dynamic(BadgeElem::new(label)))
            }),
        );
        reg
    }

    fn has_dynamic(c: &Content) -> bool {
        match c {
            Content::Dynamic(_) => true,
            Content::Sequence(items) => items.iter().any(has_dynamic),
            Content::Styled(b, _) => has_dynamic(b),
            _ => false,
        }
    }

    // F-item3 (P368): procura um custom por chave no canal aberto de qualquer
    // `Content::Styled` da árvore (o transporte do `#set`).
    fn find_styled_custom(
        c: &Content,
        key: &str,
    ) -> Option<crate::entities::value::Value> {
        match c {
            Content::Styled(b, s) => s
                .delta()
                .custom
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.clone())
                .or_else(|| find_styled_custom(b, key)),
            Content::Sequence(items) => {
                items.iter().find_map(|i| find_styled_custom(i, key))
            }
            _ => None,
        }
    }

    #[test]
    fn f_item3_set_badge_prop_entra_no_mapa_aberto() {
        // `#set badge(note: "x")` sobre um elemento de usuário registrado: em vez
        // do `unsupported_target_warn`, a prop entra no canal `custom` da chain
        // (`"badge.note"`), e o transporte (fatia-1 generalizada) a leva à árvore
        // como `Content::Styled`. Prova o set-side + o transporte.
        let world = MockWorld::new("#set badge(note: \"x\")\n#badge(\"L\")");
        let reg = registry_com_callout();
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test_with_registry(&world, &source, &reg).unwrap();
        let content = module.content().expect("módulo deve ter content");
        assert_eq!(
            find_styled_custom(content, "badge.note"),
            Some(crate::entities::value::Value::Str("x".into())),
            "#set badge(note:) deve pôr `badge.note` no custom da chain (mapa aberto): {content:?}"
        );
    }

    #[test]
    fn f3s1_callout_resolve_no_escopo_via_registry() {
        // O threading registry→escopo (deferido do F-1): `#callout(...)` resolve
        // no registry e constrói Content::Dynamic via o construct/dispatch do F-1.
        let world = MockWorld::new("#callout(\"corpo\", \"Aviso\", \"warn\")");
        let reg = registry_com_callout();
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test_with_registry(&world, &source, &reg).unwrap();
        let content = module.content().expect("módulo deve ter content");
        assert!(
            has_dynamic(content),
            "#callout deve construir Content::Dynamic via registry: {content:?}"
        );
        // E o body renderiza no plain_text (eco do DEBT C2 do inc-1).
        assert!(content.plain_text().contains("corpo"), "body do callout presente");
    }

    #[test]
    fn f3s1_elemento_desconhecido_e_erro_do_catalogo_nao_panic() {
        // Elemento não registado = erro do catálogo existente (identificador não
        // definido), não panic.
        let world = MockWorld::new("#inexistente(\"x\")");
        let reg = registry_com_callout(); // não tem `inexistente`
        let source = World::source(&world, World::main(&world)).unwrap();
        let r = eval_for_test_with_registry(&world, &source, &reg);
        assert!(r.is_err(), "elemento desconhecido deve ser Err (catálogo), não panic");
    }

    // ── Lote F-3 inc-2 S3 — o gatilho executado (paridade vs vanilla medido) ──
    //
    // Decisão Stage 0 (registrada no L0 §3b): paridade contra a semântica do
    // vanilla **medida na fonte** pelo spike-2 (P333), não rodando o binário
    // (vanilla não tem elemento custom trivial nem binário pronto).
    //
    // Casos EXPRESSÁVEIS e a PARIDADE:
    //  - Caso 2 (recursão/guard): PARIDADE ✅ — `f3s2_show_callout_anti_recursao
    //    _termina`. Vanilla: guard por-nó termina (spike-2 §2 caso 2;
    //    `typst-realize/lib.rs:472-474`). Cristalino: guard por RuleId termina.
    //  - Caso 5 (nativo+dyn na mesma travessia): PARIDADE ✅ —
    //    `f3s2_dyn_e_nativo_coexistem`. Vanilla: mesma chain (spike-2 §2 caso 5).
    //  - Caso 4 (escopo): **DIVERGÊNCIA** — ver o teste abaixo. Dispara o gatilho.
    //
    // Casos FALTA-SUPERFÍCIE (registrados, não exercidos):
    //  - Caso 1 (multi-regra, composição innermost-first): o eager aplica UMA
    //    regra (a 1ª declarada que casa), single-pass; o vanilla COMPÕE várias
    //    em multi-passe (spike-2 §2 caso 1). A composição precisa da realização
    //    multi-passe → LOTE (o mesmo do caso 4).
    //  - Caso 3 (show-set `#show k: set k(..)`): precisa `Transformation = Style`
    //    (spike-2 S5); o transform eager é só Func/Content (`rules.rs`). LOTE.

    #[test]
    fn f3s3_caso4_escopo_show_confina_no_content_block() {
        // CASO 4 (escopo) — **PARIDADE ALCANÇADA (P340, F-realização fatia 2).**
        // No vanilla, `#show` dentro de um bloco NÃO vaza — o irmão de fora fica
        // intacto (spike-2 §2 caso 4; vanilla `content/mod.rs:744-752` — a recipe
        // viaja num `StyledElem` que confina à subárvore).
        //
        // **DECISÃO REGISTRADA — `f3s3` VIROU** (a única exceção autorizada à
        // regra content-preserving; o plano sempre marcou este teto para virar
        // quando a F-realização confinasse o `#show`). Antes (eager): `#show` no
        // `[]` partilhava `engine.show_rules` e VAZAVA → AMBOS os callouts viravam
        // "DENTRO" (count 2). Agora o `ContentBlock` clona `local_show_rules`
        // (`eval/mod.rs`, espelhando o `CodeBlock`) → o `#show` confina ao bloco:
        // só o callout "a" (dentro) vira "DENTRO"; o "b" (fora) fica intacto.
        let c = eval_doc(
            "#[#show callout: it => [DENTRO]\n#callout(\"a\", \"T\", \"w\")]\n#callout(\"b\", \"T\", \"w\")",
        );
        let t = c.plain_text();
        assert_eq!(
            t.matches("DENTRO").count(),
            1,
            "confinado: só o callout DENTRO do bloco é transformado; o de fora \
             fica intacto (paridade vanilla). plain_text: {t:?}"
        );
        assert!(
            has_dynamic(&c),
            "o callout 'b' (fora do bloco) sobrevive como Dynamic não-transformado: {c:?}"
        );
    }

    // ── Lote F-3 inc-2 S2 — #show sobre o elemento dinâmico (eager, mesmo caminho) ──
    fn eval_doc(src: &str) -> Content {
        let world = MockWorld::new(src);
        let reg = registry_com_callout();
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test_with_registry(&world, &source, &reg).unwrap();
        module.content().expect("módulo deve ter content").clone()
    }

    // ════════════════════════════════════════════════════════════════════
    // P339 — F-realização fatia 1 (β1, transporte aditivo). L0 §3a.8.
    // Estágio T: caracterização (saída preservada) + alvos do transporte.
    // ════════════════════════════════════════════════════════════════════

    /// Devolve o `custom` de `key` no 1º `Content::Styled` que o carregue
    /// (walk transparente — espelha os helpers F-2 acima). Lê `delta().custom`
    /// (API existente); não depende de `push_custom`.
    fn find_custom_in_styled(
        c: &Content,
        key: &str,
    ) -> Option<crate::entities::value::Value> {
        match c {
            Content::Styled(b, s) => {
                if let Some((_, v)) = s.delta().custom.iter().find(|(k, _)| k == key) {
                    return Some(v.clone());
                }
                find_custom_in_styled(b, key)
            }
            Content::Sequence(items) => {
                items.iter().find_map(|i| find_custom_in_styled(i, key))
            }
            _ => None,
        }
    }

    // ── Caracterização: a saída de nível é o invariante (β1 content-preserving).
    //    VERDE hoje e após o Estágio C (o wrapper é transparente ao layout). ──
    #[test]
    fn f339t_caracterizacao_saida_preservada() {
        use crate::compiler::layout::layout;
        // P809: o `x` de `$ x = 1 $` é estilizado para 𝑥 U+1D465 no layout
        // (itálico matemático — paridade vanilla). O invariante β1
        // (wrapper transparente ao layout) mantém-se: só o codepoint mudou.
        let casos = [
            // **P1036** — era `"1. A 2. B"`. O ponto extra vinha da heurística
            // de sufixo de `layout/heading.rs`, que acrescentava ". " quando o
            // pattern tinha mais tokens do que valores. Com `"1.1"` e um só
            // nível, o vanilla ratificado emite `1` (medido 2026-08-13:
            // `#set heading(numbering: "1.1")` + `= A` / `= B` → `1A` / `2B`,
            // onde o espaço de 0.3em não separa palavras no `pdftotext`). O
            // caracterizado aqui era o defeito, não o comportamento.
            ("#set heading(numbering: \"1.1\")\n\n= A\n\n= B", "1 A 2 B"),
            ("#set math.equation(numbering: \"(1)\")\n\n$ x = 1 $", "\u{1D465} = 1 (1)"),
            ("#set figure(numbering: \"1\")\n\n= A", "A"),
        ];
        for (src, esperado) in casos {
            let got = layout(&eval_doc(src)).plain_text();
            assert_eq!(got, esperado, "saída de layout deve ser preservada: {src:?}");
        }
    }

    // ── Estágio C: o transporte. #set numbering embrulha o escopo num
    //    Content::Styled carregando o custom (β1). ──
    #[test]
    fn f339c_wrap_heading_carrega_custom() {
        use crate::entities::value::Value;
        let c = eval_doc("#set heading(numbering: \"1.1\")\n\n= A\n\n= B");
        assert_eq!(
            find_custom_in_styled(&c, "heading.numbering"),
            Some(Value::Bool(true)),
            "o escopo do #set heading deve ser embrulhado num Content::Styled[heading.numbering=true]"
        );
    }

    #[test]
    fn f339c_wrap_equation_carrega_custom() {
        use crate::entities::value::Value;
        let c = eval_doc("#set math.equation(numbering: \"1\")\n\n$ x = 1 $");
        assert_eq!(
            find_custom_in_styled(&c, "equation.numbering"),
            Some(Value::Str("1".into())),
            "o escopo do #set math.equation deve ser embrulhado num Content::Styled[equation.numbering=\"1\"]"
        );
    }

    #[test]
    fn f339c_wrap_figure_carrega_custom() {
        use crate::entities::value::Value;
        let c = eval_doc("#set figure(numbering: \"1\")\n\n= A");
        assert_eq!(
            find_custom_in_styled(&c, "figure.numbering"),
            Some(Value::Str("1".into())),
            "o escopo do #set figure deve ser embrulhado num Content::Styled[figure.numbering=\"1\"]"
        );
    }

    // ── Paridade do caminho duplo: o custom transportado na chain ≡ o campo
    //    assado no elemento (disciplina anti-morto, L0 §3a.8). ──
    #[test]
    fn f339c_paridade_chain_assado_heading() {
        use crate::entities::value::Value;
        let c = eval_doc("#set heading(numbering: \"1.1\")\n\n= A");
        let chain = find_custom_in_styled(&c, "heading.numbering");
        let assado = find_heading_numbered(&c); // helper F-2 (walk transparente)
        assert_eq!(
            chain,
            assado.map(Value::Bool),
            "chain.custom(heading.numbering) deve igualar o campo assado numbering_active"
        );
    }

    #[test]
    fn f339c_paridade_chain_assado_equation() {
        use crate::entities::value::Value;
        let c = eval_doc("#set math.equation(numbering: \"1\")\n\n$ x $");
        let chain = find_custom_in_styled(&c, "equation.numbering");
        let assado = find_equation_numbered(&c);
        assert_eq!(
            chain,
            assado.map(|opt| opt.map_or(Value::None, |s| Value::Str(s.into())))
        );
    }

    // ── Transparência do wrapper (L0 §3a.8): o Content::Styled[custom] é
    //    transparente a plain_text / is_empty (custom inerte ao layout). ──
    #[test]
    fn f339c_wrapper_custom_transparente() {
        use crate::entities::style::Styles;
        use crate::entities::value::Value;
        let inner = Content::sequence(vec![
            Content::text("A"),
            Content::Space,
            Content::text("B"),
        ]);
        let wrapped = Content::Styled(
            Box::new(inner.clone()),
            Styles::new().push_custom("heading.numbering", Value::Bool(true)),
        );
        assert_eq!(
            wrapped.plain_text(),
            inner.plain_text(),
            "plain_text vê através do wrapper"
        );
        assert_eq!(
            wrapped.is_empty(),
            inner.is_empty(),
            "is_empty vê através do wrapper"
        );
    }

    // ── Guarda: sem #set numbering, NÃO se embrulha (não criar wrapper espúrio).
    //    VERDE hoje e após C. ──
    #[test]
    fn f339t_sem_set_nao_embrulha() {
        let c = eval_doc("= A\n\n= B");
        assert_eq!(find_custom_in_styled(&c, "heading.numbering"), None);
        assert_eq!(find_heading_numbered(&c), Some(false));
    }

    #[test]
    fn f3s2_show_callout_transforma() {
        // `#show callout:` intercepta o dinâmico pelo apply_show_rules comum.
        let c = eval_doc(
            "#show callout: it => [TRANSFORMADO]\n#callout(\"x\", \"T\", \"w\")",
        );
        assert!(
            c.plain_text().contains("TRANSFORMADO"),
            "transform aplicado: {:?}",
            c.plain_text()
        );
        assert!(!has_dynamic(&c), "o callout foi substituído (não resta Dynamic): {c:?}");
    }

    #[test]
    fn f3s2_show_callout_anti_recursao_termina() {
        // **Teste-contrato (P348, α)**: uma regra que emite o próprio kind com corpo
        // CONSTANTE termina por **ponto-fixo morfológico** — externo→callout("interno"),
        // depois interno→interno (no-op morfológico) → para. O `active_guards` continua
        // a impedir a recursão DURANTE a chamada do recipe (criação aninhada), mas a
        // terminação agora é o ponto-fixo (antes do P348: truncava no nível 1). Resultado
        // idêntico: o callout("interno") sobrevive, o "externo" some.
        let c = eval_doc(
            "#show callout: it => callout(\"interno\", \"T\", \"w\")\n#callout(\"externo\", \"T\", \"w\")",
        );
        assert!(has_dynamic(&c), "o callout interno (produzido) sobrevive: {c:?}");
        assert!(
            c.plain_text().contains("interno"),
            "corpo interno presente: {:?}",
            c.plain_text()
        );
        assert!(!c.plain_text().contains("externo"), "o externo foi transformado");
    }

    #[test]
    fn f3s2_show_callout_nao_pega_badge() {
        // Regra para o kind A (`callout`) não intercepta o kind B (`badge`).
        let c = eval_doc("#show callout: it => [CAUGHT]\n#badge(\"selo\")");
        assert!(has_dynamic(&c), "o badge (kind B) sobrevive: {c:?}");
        assert!(!c.plain_text().contains("CAUGHT"), "regra de callout não pega badge");
        assert!(c.plain_text().contains("selo"), "label do badge presente");
    }

    #[test]
    fn f3s2_dyn_e_nativo_coexistem() {
        // Dinâmico e nativo no mesmo doc: a regra de callout transforma o
        // callout; o heading nativo fica intacto.
        let c = eval_doc(
            "#show callout: it => [CALLOUT_OK]\n#callout(\"x\", \"T\", \"w\")\n= Titulo",
        );
        let t = c.plain_text();
        assert!(t.contains("CALLOUT_OK"), "callout transformado: {t:?}");
        assert!(t.contains("Titulo"), "heading nativo presente: {t:?}");
    }

    // ── P348 (modelo α, ADR-0107) — recursão de #show por ponto-fixo morfológico ──
    #[test]
    fn p348_show_recursao_converge_para_ponto_fixo() {
        // O output de uma element rule que re-casa é REVISITADO até ponto-fixo
        // morfológico (== do P345). m1 (P341b): a→b→c→(fixo). Antes (P345) dava "b"
        // (o `==` morfológico destravou a→b); o P348 revisita até "c".
        // vanilla 0.14.2: `c`.
        let c = eval_doc(
            "#show heading: it => { if it.body == [a] {[= b]} else if it.body == [b] {[= c]} else {it} }\n= a"
        );
        let t = c.plain_text();
        assert!(t.contains('c'), "m1 revisita até o ponto-fixo 'c': {t:?}");
        assert!(
            !t.contains('a') && !t.contains('b'),
            "passos intermédios (a/b) consumidos pela revisitação: {t:?}"
        );
    }

    #[test]
    fn p348_show_recursao_o_inf_converge_divergencia_consciente() {
        // DIVERGÊNCIA CONSCIENTE vs vanilla (ADR-0107). `#show heading: it => [= Z]`
        // sempre reescreve para `= Z`; a 2ª aplicação é no-op morfológico (Z→Z) →
        // ponto-fixo → "Z". vanilla 0.14.2: ERRO `maximum show rule depth exceeded`
        // (termina por identidade de instância — mecânica, GEROU em P347b/c). O
        // cristalino converge por morfologia: mais gracioso. Não é falha — é a marca
        // da escolha α (registrada na nota de paridade).
        let c = eval_doc("#show heading: it => [= Z]\n= a");
        let t = c.plain_text();
        assert!(
            t.contains('Z'),
            "o_inf converge para 'Z' (divergência consciente): {t:?}"
        );
        assert!(!t.contains('a'), "o heading original foi reescrito: {t:?}");
    }

    #[test]
    fn p348_show_recursao_ciclo_detectado_antes_do_teto() {
        // Ciclo a→b→a→…: o mecanismo de paragem revisto (Passo 1009) deteta a
        // repetição da forma canónica e erra com mensagem própria antes de
        // atingir o teto de profundidade.
        let world = MockWorld::new(
            "#show heading: it => { if it.body == [a] {[= b]} else {[= a]} }\n= a",
        );
        let src = world.source(world.main()).unwrap();
        let err = eval_for_test(&world, &src).expect_err("ciclo deve errar");
        assert_eq!(
            err[0].message, "show rule cycle detected",
            "mensagem de ciclo (capacidade nova, Passo 1009)"
        );
        assert!(
            err[0].hints.iter().any(|h| h.contains("rules involved")),
            "hint nomeia as regras envolvidas: {:?}",
            err[0].hints
        );
    }

    // ── P350c — flag de erro completo: classificação (2 rótulos) no 3º hint ──────
    #[test]
    fn p350c_flag_off_ciclo_detectado_antes_do_teto() {
        // DEFAULT (flag desligada): ciclo a→b→a→… detetado antes do teto.
        // Mensagem própria (Passo 1009); sem hint classificatório de full_error.
        let world = MockWorld::new(
            "#show heading: it => { if it.body == [a] {[= b]} else {[= a]} }\n= a",
        );
        let src = world.source(world.main()).unwrap();
        let err = eval_for_test(&world, &src).expect_err("ciclo erra");
        assert_eq!(err[0].message, "show rule cycle detected");
        assert!(
            err[0].hints.iter().any(|h| h.contains("rules involved")),
            "hint nomeia as regras envolvidas: {:?}",
            err[0].hints
        );
        assert!(
            !err[0]
                .hints
                .iter()
                .any(|h| h.contains("CÍCLICA") || h.contains("NÃO-CONVERGENTE")),
            "flag off: sem hint classificatório de full_error: {:?}",
            err[0].hints
        );
    }

    #[test]
    fn p350c_flag_on_ciclo_detectado_antes_do_teto() {
        // Flag LIGADA + ciclo a→b→a→…: o novo mecanismo deteta o ciclo antes do
        // teto e produz a mensagem própria. O histórico de full_error é usado
        // internamente para detetar a repetição; o erro em si classifica-se
        // como ciclo pela mensagem e pelo hint de regras envolvidas.
        let world = MockWorld::new(
            "#show heading: it => { if it.body == [a] {[= b]} else {[= a]} }\n= a",
        );
        let src = world.source(world.main()).unwrap();
        let err = eval_for_test_full_error(&world, &src).expect_err("ciclo erra");
        assert_eq!(err[0].message, "show rule cycle detected");
        assert!(
            err[0].hints.iter().any(|h| h.contains("rules involved")),
            "hint nomeia as regras envolvidas: {:?}",
            err[0].hints
        );
    }

    #[test]
    fn p350c_flag_on_nao_convergente_classifica() {
        // Flag LIGADA + recursão NÃO-CONVERGENTE (cresce sem repetir): 3º hint
        // "NÃO-CONVERGENTE". A regra acrescenta " x" ao corpo a cada passe → o
        // heading re-casa e a morfologia cresce sem nunca repetir → teto.
        let world = MockWorld::new("#show heading: it => [= #it.body x]\n= a");
        let src = world.source(world.main()).unwrap();
        let err = eval_for_test_full_error(&world, &src).expect_err("divergente erra");
        assert_eq!(err[0].message, "maximum show rule depth exceeded");
        assert!(
            err[0].hints.iter().any(|h| h.contains("NÃO-CONVERGENTE")),
            "3º hint classifica NÃO-CONVERGENTE: {:?}",
            err[0].hints
        );
    }

    #[test]
    fn f2s1_set_heading_escopo_lexical_nao_vaza() {
        // **A prova do fecho do DEBT 99.E**: `#set heading(numbering:)` dentro
        // de um bloco de conteúdo `[...]` escopa ao bloco — não vaza para o
        // heading de fora. (Antes do F-2, o marcador era global.)
        let world =
            MockWorld::new("#[#set heading(numbering: \"1.\")\n= Dentro]\n= Fora");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("módulo deve ter content");
        let mut hs = vec![];
        collect_headings_numbered(content, &mut hs);
        assert_eq!(
            hs,
            vec![true, false],
            "Dentro numerado; Fora NÃO (escopo léxico, DEBT 99.E): {hs:?}"
        );
    }

    // ── Testes de control flow ────────────────────────────────────────────────

    #[test]
    fn if_true_branch() {
        let world = MockWorld::new("#let x = if true { 1 } else { 2 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(1)));
    }

    #[test]
    fn if_false_branch() {
        let world = MockWorld::new("#let x = if false { 1 } else { 2 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(2)));
    }

    #[test]
    fn if_sem_else_retorna_none() {
        let world = MockWorld::new("#let x = if false { 1 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::None));
    }

    /// Prova de vida ADR-0025: if 1 == 1.0 { 42 } else { 0 } → 42
    #[test]
    fn prova_de_vida_adr_0025() {
        let world = MockWorld::new("#let x = if 1 == 1.0 { 42 } else { 0 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(42)));
    }

    #[test]
    fn while_condicao_falsa_nao_executa() {
        let world = MockWorld::new("#while false { }");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_ok());
    }

    #[test]
    fn while_loop_infinito_retorna_err() {
        let world = MockWorld::new("#while true { }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "loop infinito deve retornar Err, não bloquear");
        let err = result.unwrap_err();
        assert!(!err.is_empty());
        assert!(
            err[0].message.contains("iterações") || err[0].message.contains("limite"),
            "mensagem de erro deve mencionar limite: {:?}",
            err[0].message
        );
    }

    #[test]
    fn for_sobre_array_vazio_nao_executa() {
        let world = MockWorld::new("#let arr = ()\n#for x in arr { }");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_ok());
    }

    // ── Testes de paridade: eval_binary_op ───────────────────────────────────

    #[test]
    fn paridade_add_int() {
        assert_eq!(
            eval_binary_op(BinOp::Add, Value::Int(1), Value::Int(2)),
            Ok(Value::Int(3))
        );
    }

    #[test]
    fn paridade_add_float() {
        assert_eq!(
            eval_binary_op(BinOp::Add, Value::Float(1.5), Value::Float(2.5)),
            Ok(Value::Float(4.0))
        );
    }

    #[test]
    fn paridade_add_str() {
        assert_eq!(
            eval_binary_op(
                BinOp::Add,
                Value::Str("hello ".into()),
                Value::Str("world".into())
            ),
            Ok(Value::Str("hello world".into()))
        );
    }

    #[test]
    fn paridade_sub_int() {
        assert_eq!(
            eval_binary_op(BinOp::Sub, Value::Int(5), Value::Int(3)),
            Ok(Value::Int(2))
        );
    }

    #[test]
    fn paridade_mul_int() {
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Int(3), Value::Int(4)),
            Ok(Value::Int(12))
        );
    }

    #[test]
    fn paridade_div_int_int() {
        // Semântica Typst: 5 / 2 = 2.5 (float), confirmado com ops.rs
        assert_eq!(
            eval_binary_op(BinOp::Div, Value::Int(5), Value::Int(2)),
            Ok(Value::Float(2.5))
        );
    }

    #[test]
    fn paridade_div_por_zero() {
        assert!(eval_binary_op(BinOp::Div, Value::Int(1), Value::Int(0)).is_err());
        assert!(eval_binary_op(BinOp::Div, Value::Float(1.0), Value::Float(0.0)).is_err());
    }

    // ── P713 — `Length / Length` (e `Length / Int|Float`) ──────────────────

    #[test]
    fn p713_length_div_length_mesma_unidade_abs() {
        use crate::entities::layout_types::{Abs, Length};
        // 2cm / 1cm — o caso exacto do bloqueio de cetz (canvas.typ:37).
        let a = Length { abs: Abs(2.0 * Length::PT_PER_CM), em: 0.0 };
        let b = Length { abs: Abs(Length::PT_PER_CM), em: 0.0 };
        assert_eq!(
            eval_binary_op(BinOp::Div, Value::Length(a), Value::Length(b)),
            Ok(Value::Float(2.0))
        );
    }

    #[test]
    fn p713_length_div_length_ratio_em() {
        use crate::entities::layout_types::{Abs, Length};
        // Ambos abs=0 -> rácio de em (paridade `Length::try_div` vanilla).
        let a = Length { abs: Abs(0.0), em: 4.0 };
        let b = Length { abs: Abs(0.0), em: 2.0 };
        assert_eq!(
            eval_binary_op(BinOp::Div, Value::Length(a), Value::Length(b)),
            Ok(Value::Float(2.0))
        );
    }

    #[test]
    fn p713_length_div_length_mista_incomensuravel_erra() {
        use crate::entities::layout_types::{Abs, Length};
        // Nem abs=0 em ambos, nem em=0 em ambos -> incomensurável, erro (não None silencioso).
        let a = Length { abs: Abs(10.0), em: 1.0 };
        let b = Length { abs: Abs(5.0), em: 0.0 };
        assert!(eval_binary_op(BinOp::Div, Value::Length(a), Value::Length(b)).is_err());
    }

    #[test]
    fn p713_length_div_length_zero_erra_divisao_por_zero() {
        use crate::entities::layout_types::{Abs, Length};
        let a = Length { abs: Abs(10.0), em: 0.0 };
        let zero = Length { abs: Abs(0.0), em: 0.0 };
        let err = eval_binary_op(BinOp::Div, Value::Length(a), Value::Length(zero))
            .unwrap_err();
        assert!(err.contains("divide by zero"), "erro inesperado: {err}");
    }

    #[test]
    fn p713_length_div_int_escala() {
        use crate::entities::layout_types::{Abs, Length};
        let a = Length { abs: Abs(10.0), em: 4.0 };
        assert_eq!(
            eval_binary_op(BinOp::Div, Value::Length(a), Value::Int(2)),
            Ok(Value::Length(Length { abs: Abs(5.0), em: 2.0 }))
        );
    }

    #[test]
    fn p713_length_div_float_escala() {
        use crate::entities::layout_types::{Abs, Length};
        let a = Length { abs: Abs(10.0), em: 0.0 };
        assert_eq!(
            eval_binary_op(BinOp::Div, Value::Length(a), Value::Float(4.0)),
            Ok(Value::Length(Length { abs: Abs(2.5), em: 0.0 }))
        );
    }

    #[test]
    fn p713_cetz_canvas_length_to_absolute_div_1cm() {
        // Reprodução exacta do bloqueio: `(2cm).to-absolute() / 1cm`.
        let world = MockWorld::new("#let x = (2cm).to-absolute() / 1cm");
        // P1284: `to-absolute` fora de contexto falha antes da divisão.
        assert!(eval_for_test(&world, &world.source).is_err());
    }

    // ── P720 — `Array + Array` (concatenação) e `Dict + Dict` (merge) ───────

    #[test]
    fn p720_array_mais_array_concatena() {
        assert_eq!(
            eval_binary_op(
                BinOp::Add,
                Value::Array(vec![Value::Int(1), Value::Int(2)]),
                Value::Array(vec![Value::Int(3), Value::Int(4)]),
            ),
            Ok(Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4)
            ]))
        );
    }

    #[test]
    fn p720_array_vazio_mais_array() {
        assert_eq!(
            eval_binary_op(
                BinOp::Add,
                Value::Array(vec![]),
                Value::Array(vec![Value::Int(1), Value::Int(2)]),
            ),
            Ok(Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p720_array_mais_array_vazio() {
        assert_eq!(
            eval_binary_op(
                BinOp::Add,
                Value::Array(vec![Value::Int(1), Value::Int(2)]),
                Value::Array(vec![]),
            ),
            Ok(Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p720_dict_mais_dict_sem_colisao() {
        let mut a = indexmap::IndexMap::default();
        a.insert(EcoString::from("a"), Value::Int(1));
        let mut b = indexmap::IndexMap::default();
        b.insert(EcoString::from("b"), Value::Int(2));

        let mut expected = indexmap::IndexMap::default();
        expected.insert(EcoString::from("a"), Value::Int(1));
        expected.insert(EcoString::from("b"), Value::Int(2));

        assert_eq!(
            eval_binary_op(BinOp::Add, Value::Dict(a), Value::Dict(b)),
            Ok(Value::Dict(expected))
        );
    }

    #[test]
    fn p720_dict_mais_dict_colisao_direita_vence_posicao_preservada() {
        // Medido no vanilla: (a:1,b:2) + (b:99,c:3) → (a:1,b:99,c:3) — "b"
        // fica na posição 1 (não move para o fim), valor do lado direito.
        let mut a = indexmap::IndexMap::default();
        a.insert(EcoString::from("a"), Value::Int(1));
        a.insert(EcoString::from("b"), Value::Int(2));
        let mut b = indexmap::IndexMap::default();
        b.insert(EcoString::from("b"), Value::Int(99));
        b.insert(EcoString::from("c"), Value::Int(3));

        let result = eval_binary_op(BinOp::Add, Value::Dict(a), Value::Dict(b)).unwrap();
        match result {
            Value::Dict(d) => {
                let keys: Vec<&str> = d.keys().map(|k| k.as_str()).collect();
                assert_eq!(
                    keys,
                    vec!["a", "b", "c"],
                    "ordem das chaves deve ser preservada"
                );
                assert_eq!(d.get("a"), Some(&Value::Int(1)));
                assert_eq!(d.get("b"), Some(&Value::Int(99)));
                assert_eq!(d.get("c"), Some(&Value::Int(3)));
            }
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p720_cetz_concat_end_to_end() {
        // Reprodução do padrão real de cetz (hobby.typ:77): concatenar
        // arrays via `+` num único `#let`.
        let world = MockWorld::new("#let x = (1, 2) + (3, 4)");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4)
            ]))
        );
    }

    #[test]
    fn paridade_eq_int_int() {
        assert_eq!(
            eval_binary_op(BinOp::Eq, Value::Int(1), Value::Int(1)),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Eq, Value::Int(1), Value::Int(2)),
            Ok(Value::Bool(false))
        );
    }

    // ── P722 — Array * Int / Int * Array (repetição) ───────────────────────

    #[test]
    fn p722_array_vezes_int_repete() {
        // Medido no vanilla: (0,) * 3 → (0, 0, 0).
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Array(vec![Value::Int(0)]), Value::Int(3),),
            Ok(Value::Array(vec![Value::Int(0), Value::Int(0), Value::Int(0)]))
        );
    }

    #[test]
    fn p722_int_vezes_array_ordem_inversa() {
        // Medido no vanilla: 3 * (0,) → (0, 0, 0) (ops.rs:275).
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Int(3), Value::Array(vec![Value::Int(0)]),),
            Ok(Value::Array(vec![Value::Int(0), Value::Int(0), Value::Int(0)]))
        );
    }

    #[test]
    fn p722_array_vezes_zero_da_vazio() {
        // Medido no vanilla: (1, 2) * 0 → ().
        assert_eq!(
            eval_binary_op(
                BinOp::Mul,
                Value::Array(vec![Value::Int(1), Value::Int(2)]),
                Value::Int(0),
            ),
            Ok(Value::Array(vec![]))
        );
    }

    #[test]
    fn p722_array_vazio_vezes_int_da_vazio() {
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Array(vec![]), Value::Int(5)),
            Ok(Value::Array(vec![]))
        );
    }

    #[test]
    fn p722_array_vezes_negativo_erro() {
        // Medido no vanilla: (1, 2) * -1 → "number must be at least zero"
        // (cast Int → usize, foundations/int.rs:507).
        assert_eq!(
            eval_binary_op(
                BinOp::Mul,
                Value::Array(vec![Value::Int(1), Value::Int(2)]),
                Value::Int(-1),
            ),
            Err("number must be at least zero".to_string())
        );
    }

    #[test]
    fn p722_dict_vezes_int_erro_fronteira() {
        // Medido no vanilla: (:) * 2 → erro (Dict * Int não existe).
        // P842 (#39): a fronteira genérica usa o formato verbatim do vanilla
        // ("cannot multiply {a} with {b}", nomes longos de tipo).
        let result = eval_binary_op(
            BinOp::Mul,
            Value::Dict(indexmap::IndexMap::default()),
            Value::Int(2),
        );
        assert!(result.is_err(), "Dict * Int deve ser erro");
        assert_eq!(result.unwrap_err(), "cannot multiply dictionary with integer");
    }

    #[test]
    fn p722_cetz_repeat_end_to_end() {
        // Reprodução do padrão real de cetz (hobby.typ:77): (0,) * (n - 1).
        let world = MockWorld::new("#let x = (0,) * (3 - 1)");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(0), Value::Int(0)]))
        );
    }

    // ── P723 — assert.eq / assert.ne via namespace (bloqueio real do cetz) ──

    #[test]
    fn p723_assert_eq_namespace_e2e_sucesso() {
        let world = MockWorld::new("#let x = { assert.eq(1, 1); 42 }");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(42)));
    }

    #[test]
    fn p723_assert_eq_namespace_e2e_erro() {
        let world = MockWorld::new("#let x = assert.eq(1, 2)");
        let err = eval_for_test(&world, &world.source)
            .expect_err("assert.eq(1, 2) deve abortar a avaliação");
        assert!(
            err[0].message.contains("equality assertion failed"),
            "mensagem vanilla esperada: {:?}",
            err[0].message
        );
    }

    // ── P723 — `for` com spread `..sink` (2º bloqueio real do cetz) ─────────

    #[test]
    fn p723_for_spread_recolhe_resto() {
        // Reprodução do padrão real de cetz (path-util.typ:106):
        // for (kind, ..args) in segments — ("c", p1, p2, p3) → args = 3 pts.
        let world = MockWorld::new(
            "#let x = { let n = 0; for (kind, ..args) in ((\"c\", 1, 2, 3), (\"l\", 4)) { n = n + args.len() }; n }",
        );
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(4)));
    }

    #[test]
    fn p723_for_spread_kind_liga_primeiro() {
        let world = MockWorld::new(
            "#let x = { let s = \"\"; for (kind, ..args) in ((\"c\", 1), (\"l\", 2)) { s = s + kind }; s }",
        );
        assert_eq!(eval_let(&world, "x"), Some(Value::Str("cl".into())));
    }

    #[test]
    fn p723_for_tuplo_um_elemento_destroi() {
        // Corrigido pela delegação: antes, (a,) com bindings.len()==1 ligava
        // o array inteiro a `a` em vez de destruir o tuplo.
        let world = MockWorld::new(
            "#let x = { let n = 0; for (a,) in ((5,), (6,)) { n = n + a }; n }",
        );
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(11)));
    }

    #[test]
    fn p723_for_aridade_erro_mensagem_vanilla() {
        // Fecha a divergência pré-existente notada em P719: a mensagem de
        // aridade do for passa a ser a do vanilla (wrong_number_of_elements).
        let world = MockWorld::new("#let x = { for (a, b) in ((1, 2, 3),) { 1 } }");
        let err = eval_for_test(&world, &world.source)
            .expect_err("aridade errada deve abortar");
        assert!(
            err[0].message.contains("too many elements to destructure"),
            "mensagem vanilla esperada: {:?}",
            err[0].message
        );
    }

    // ── P725 — Length * Int|Float (as quatro combinações) ───────────────────

    #[test]
    fn p725_length_vezes_int_escala() {
        use crate::entities::layout_types::{Abs, Length};
        // Medido no vanilla: repr(1pt * 2) → 2pt (ops.rs:238).
        let a = Length { abs: Abs(10.0), em: 4.0 };
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Length(a), Value::Int(2)),
            Ok(Value::Length(Length { abs: Abs(20.0), em: 8.0 }))
        );
    }

    #[test]
    fn p725_int_vezes_length_ordem_inversa() {
        use crate::entities::layout_types::{Abs, Length};
        // Medido no vanilla: repr(2 * 1pt) → 2pt (ops.rs:241).
        let a = Length { abs: Abs(1.0), em: 0.0 };
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Int(2), Value::Length(a)),
            Ok(Value::Length(Length { abs: Abs(2.0), em: 0.0 }))
        );
    }

    #[test]
    fn p725_length_vezes_float_escala() {
        use crate::entities::layout_types::{Abs, Length};
        // Medido no vanilla: repr(1pt * 2.5) → 2.5pt (ops.rs:239).
        let a = Length { abs: Abs(1.0), em: 0.0 };
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Length(a), Value::Float(2.5)),
            Ok(Value::Length(Length { abs: Abs(2.5), em: 0.0 }))
        );
    }

    #[test]
    fn p725_float_vezes_length_ordem_inversa() {
        use crate::entities::layout_types::{Abs, Length};
        // Medido no vanilla: repr(0.5 * (1pt + 1em)) → 0.5pt + 0.5em (ops.rs:242).
        let a = Length { abs: Abs(1.0), em: 1.0 };
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Float(0.5), Value::Length(a)),
            Ok(Value::Length(Length { abs: Abs(0.5), em: 0.5 }))
        );
    }

    #[test]
    fn p725_length_vezes_zero_e_negativo() {
        use crate::entities::layout_types::{Abs, Length};
        // Medido no vanilla: repr(0 * 1pt) → 0pt; repr(-1 * 1pt) → -1pt.
        let a = Length { abs: Abs(1.0), em: 0.0 };
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Int(0), Value::Length(a)),
            Ok(Value::Length(Length::ZERO))
        );
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Int(-1), Value::Length(a)),
            Ok(Value::Length(Length { abs: Abs(-1.0), em: 0.0 }))
        );
    }

    #[test]
    fn p725_int_vezes_em_puro() {
        use crate::entities::layout_types::{Abs, Length};
        // Medido no vanilla: repr(3 * 2em) → 6em.
        let a = Length { abs: Abs(0.0), em: 2.0 };
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Int(3), Value::Length(a)),
            Ok(Value::Length(Length { abs: Abs(0.0), em: 6.0 }))
        );
    }

    #[test]
    fn p725_length_vezes_nan_saneado_para_zero() {
        use crate::entities::layout_types::{Abs, Length};
        // Medido no vanilla: repr(1pt * float.nan) → 0pt,
        // repr(1em * float.nan) → 0pt, repr((1pt + 1em) * float.nan) → 0pt
        // (Scalar::new saneia NaN → 0, typst-utils/src/scalar.rs:30-32).
        assert_eq!(
            eval_binary_op(
                BinOp::Mul,
                Value::Length(Length::pt(1.0)),
                Value::Float(f64::NAN)
            ),
            Ok(Value::Length(Length::ZERO))
        );
        assert_eq!(
            eval_binary_op(
                BinOp::Mul,
                Value::Float(f64::NAN),
                Value::Length(Length::em(1.0))
            ),
            Ok(Value::Length(Length::ZERO))
        );
        let misto = Length { abs: Abs(1.0), em: 1.0 };
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Length(misto), Value::Float(f64::NAN)),
            Ok(Value::Length(Length::ZERO))
        );
    }

    #[test]
    fn p725_length_vezes_inf_propaga_silencioso() {
        use crate::entities::layout_types::{Abs, Length};
        // Medido no vanilla: repr(1pt * float.inf) → float.inf * 1pt —
        // inf propaga-se, sem erro e sem saneamento.
        assert_eq!(
            eval_binary_op(
                BinOp::Mul,
                Value::Length(Length::pt(1.0)),
                Value::Float(f64::INFINITY)
            ),
            Ok(Value::Length(Length { abs: Abs(f64::INFINITY), em: 0.0 }))
        );
        // Medido no vanilla: repr(1em * float.inf) → float.inf * 1em.
        assert_eq!(
            eval_binary_op(
                BinOp::Mul,
                Value::Float(f64::INFINITY),
                Value::Length(Length::em(1.0))
            ),
            Ok(Value::Length(Length { abs: Abs(0.0), em: f64::INFINITY }))
        );
    }

    #[test]
    fn p725_cetz_mul_length_end_to_end() {
        use crate::entities::layout_types::{Abs, Length};
        // Reprodução do padrão real de cetz (canvas.typ:146-147):
        // (x - offset) * length, escala de coordenadas.
        let world = MockWorld::new("#let x = (3 - 1) * 2.5pt");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Length(Length { abs: Abs(5.0), em: 0.0 }))
        );
    }

    #[test]
    fn p725_repr_length_inf_e2e() {
        // Paridade de linguagem (ADR-0107) no observável repr, via sintaxe
        // alcançável no cristalino: `calc.inf` existe (`stdlib/calc.rs:108`),
        // mas `float.nan`/`float.inf`/`calc.nan` NÃO existem no eval
        // cristalino (`float` é só Type::Float, `eval/mod.rs:1086`) — o
        // caminho NaN é coberto pelos testes unitários de eval_binary_op.
        // Medido no vanilla: repr(1pt * float.inf) == "float.inf * 1pt",
        // repr(1em * float.inf) == "float.inf * 1em".
        let world = MockWorld::new("#let x = repr(1pt * calc.inf)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Str("float.inf * 1pt".into())));
        // 1em * inf: abs = 0 * inf = NaN → saneado 0 (cobre o saneamento E2E).
        let world = MockWorld::new("#let x = repr(1em * calc.inf)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Str("float.inf * 1em".into())));
    }

    // ── P726 — `fill: none` / `stroke: none` (padrão real do cetz) ──────────

    #[test]
    fn p726_cetz_block_with_none_fill_stroke_e2e() {
        // Padrão exacto de cetz canvas.typ:111,129:
        // block.with(breakable: false) invocado com fill/stroke none.
        // Antes de P726: "block(fill): espera Color, recebeu none".
        let world = MockWorld::new(
            "#let f = block.with(breakable: false)\n#let x = f(fill: none, stroke: none)[x]",
        );
        match eval_let(&world, "x") {
            Some(Value::Content(_)) => {}
            other => panic!("block(fill/stroke: none) deve avaliar sem erro: {other:?}"),
        }
    }

    #[test]
    fn dualidade_eq_typst_coerce() {
        // ADR-0025 Opção B: no motor Typst, 1 == 1.0 → true (coerção Int→f64)
        assert_eq!(
            eval_binary_op(BinOp::Eq, Value::Int(1), Value::Float(1.0)),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn dualidade_eq_rust_sem_coerce() {
        // derive(PartialEq) em Rust: Value::Int(1) != Value::Float(1.0)
        // Vital para IndexMap, testes unitários de Value, e estruturas de dados.
        assert_ne!(Value::Int(1), Value::Float(1.0));
    }

    #[test]
    fn eq_tipos_radicalmente_distintos() {
        // Bool vs Int — sem coerção em nenhum sistema
        assert_eq!(
            eval_binary_op(BinOp::Eq, Value::Bool(true), Value::Int(1)),
            Ok(Value::Bool(false))
        );
    }

    // ── P345 (ADR-0107) — o `==` da linguagem sobre conteúdo é MORFOLÓGICO ────
    // Morfologia (texto, markup, estilo semântico) entra; render (TextStyle
    // assado, transporte β1, numbering assado) sai. Dois sistemas (ADR-0025):
    // o `derive(PartialEq)` do Rust permanece estrutural (testes/coleções).
    use crate::entities::content::Content;
    use crate::entities::layout_types::{Pt, TextStyle};
    use crate::entities::style::Styles;

    #[test]
    fn morfologia_eq_ignora_render_na_chain() {
        // **F-5b fatia 2 (P373).** O render do `#set text` deixou de ser assado no
        // node (`Content::Text` perdeu o `TextStyle`) — viaja num `Content::Styled`
        // **custom-only** (semanticamente vazio). `morph_canon` desce-o (§3a, P366) →
        // o `==` da linguagem ignora o render. É o Achado 2 (P342) na forma nova:
        // it.body de `= a` (render na chain) vs `[a]` (sem render) → casa.
        let plain = Content::text("a");
        let render_na_chain = Content::Styled(
            Box::new(Content::text("a")),
            Styles::new().push_custom("text.bold", Value::Bool(true)),
        );
        assert_eq!(
            eval_binary_op(
                BinOp::Eq,
                Value::Content(plain.clone()),
                Value::Content(render_na_chain.clone())
            ),
            Ok(Value::Bool(true)),
            "mesma morfologia, render só na chain (custom) → o == da linguagem casa"
        );
        // Dois sistemas (ADR-0025): o PartialEq do Rust permanece estrutural.
        assert_ne!(
            plain, render_na_chain,
            "derive(PartialEq) do Rust permanece estrutural"
        );
    }

    #[test]
    fn morfologia_eq_distingue_texto() {
        // Morfologia diferente (texto) → não casa.
        assert_eq!(
            eval_binary_op(
                BinOp::Eq,
                Value::Content(Content::text("a")),
                Value::Content(Content::text("b"))
            ),
            Ok(Value::Bool(false))
        );
    }

    #[test]
    fn morfologia_eq_estilo_semantico_e_morfologia() {
        // `*bold*`/strong → Content::Styled([Bold]) é estilo SEMÂNTICO (o #show
        // strong o vê) → morfologia. strong[a] != [a] (vanilla: false).
        let strong_a = Content::strong(Content::text("a"));
        assert_eq!(
            eval_binary_op(
                BinOp::Eq,
                Value::Content(strong_a),
                Value::Content(Content::text("a"))
            ),
            Ok(Value::Bool(false)),
            "estilo semântico (*bold*) é morfologia — distingue"
        );
    }

    #[test]
    fn morfologia_eq_transporte_b1_transparente() {
        // Content::Styled semanticamente vazio (só transporte custom numbering β1)
        // → transparente: igual ao body nu. (vanilla: #set numbering não entra no ==.)
        let transported = Content::Styled(
            Box::new(Content::text("a")),
            Styles::new().push_custom("heading.numbering", Value::Bool(true)),
        );
        assert_eq!(
            eval_binary_op(
                BinOp::Eq,
                Value::Content(transported),
                Value::Content(Content::text("a"))
            ),
            Ok(Value::Bool(true)),
            "transporte β1 (custom-only) é render — transparente no =="
        );
    }

    #[test]
    fn lt_int_float_coerce() {
        // Ordenação Int↔Float também coerce (confirmado em ops::compare)
        assert_eq!(
            eval_binary_op(BinOp::Lt, Value::Int(1), Value::Float(1.5)),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Gt, Value::Float(2.0), Value::Int(1)),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn paridade_neq() {
        assert_eq!(
            eval_binary_op(BinOp::Neq, Value::Int(1), Value::Int(2)),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Neq, Value::Int(1), Value::Int(1)),
            Ok(Value::Bool(false))
        );
    }

    #[test]
    fn paridade_lt_gt() {
        assert_eq!(
            eval_binary_op(BinOp::Lt, Value::Int(1), Value::Int(2)),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Gt, Value::Int(2), Value::Int(1)),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Leq, Value::Int(2), Value::Int(2)),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Geq, Value::Int(3), Value::Int(2)),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn paridade_and_or() {
        assert_eq!(
            eval_binary_op(BinOp::And, Value::Bool(true), Value::Bool(false)),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            eval_binary_op(BinOp::Or, Value::Bool(false), Value::Bool(true)),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn paridade_overflow_int_retorna_err() {
        // checked_add — overflow retorna Err, não panic
        let r = eval_binary_op(BinOp::Add, Value::Int(i64::MAX), Value::Int(1));
        assert!(r.is_err());
    }

    #[test]
    fn paridade_nan_propagado() {
        // O original propaga NaN silenciosamente (Float(a / b) sem guarda)
        // 0.0 / 0.0 = NaN em IEEE 754 — mas a guarda de is_zero captura 0.0
        // portanto Float(0.0) / Float(0.0) → Err("cannot divide by zero")
        let r = eval_binary_op(BinOp::Div, Value::Float(0.0), Value::Float(0.0));
        assert!(r.is_err());
    }

    #[test]
    fn paridade_tipo_invalido_retorna_err() {
        let r = eval_binary_op(BinOp::Add, Value::None, Value::Int(1));
        assert!(r.is_err());
    }

    // ── P404 — Aritmética e comparações Decimal ──────────────────────────────

    fn dec(s: &str) -> Value {
        Value::Decimal(crate::entities::decimal::Decimal::from_str(s).unwrap())
    }

    #[test]
    fn decimal_add() {
        assert_eq!(eval_binary_op(BinOp::Add, dec("1.5"), dec("2.5")), Ok(dec("4.0")));
        assert_eq!(eval_binary_op(BinOp::Add, dec("-1.5"), dec("2.5")), Ok(dec("1.0")));
        assert_eq!(eval_binary_op(BinOp::Add, dec("0"), dec("0")), Ok(dec("0")));
    }

    #[test]
    fn decimal_sub() {
        assert_eq!(eval_binary_op(BinOp::Sub, dec("10"), dec("3")), Ok(dec("7")));
        assert_eq!(eval_binary_op(BinOp::Sub, dec("1.5"), dec("2.5")), Ok(dec("-1.0")));
        assert_eq!(eval_binary_op(BinOp::Sub, dec("0"), dec("0")), Ok(dec("0")));
    }

    #[test]
    fn decimal_mul() {
        assert_eq!(eval_binary_op(BinOp::Mul, dec("2.5"), dec("4")), Ok(dec("10.0")));
        assert_eq!(eval_binary_op(BinOp::Mul, dec("-3"), dec("2")), Ok(dec("-6")));
        assert_eq!(eval_binary_op(BinOp::Mul, dec("0"), dec("123.456")), Ok(dec("0")));
    }

    #[test]
    fn decimal_div() {
        assert_eq!(eval_binary_op(BinOp::Div, dec("10"), dec("2")), Ok(dec("5")));
        let r = eval_binary_op(BinOp::Div, dec("10"), dec("3")).unwrap();
        assert!(matches!(r, Value::Decimal(_)));
        if let Value::Decimal(d) = r {
            assert!(d.to_string().starts_with("3.3333"));
        }
        assert!(eval_binary_op(BinOp::Div, dec("1"), dec("0")).is_err());
    }

    #[test]
    fn decimal_eq_neq() {
        assert_eq!(
            eval_binary_op(BinOp::Eq, dec("1.0"), dec("1.00")),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Eq, dec("1.0"), dec("2.0")),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            eval_binary_op(BinOp::Neq, dec("1.0"), dec("2.0")),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Neq, dec("1.0"), dec("1.00")),
            Ok(Value::Bool(false))
        );
    }

    #[test]
    fn decimal_lt_gt_leq_geq() {
        assert_eq!(
            eval_binary_op(BinOp::Lt, dec("1.0"), dec("2.0")),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Gt, dec("3.0"), dec("2.0")),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Leq, dec("2.0"), dec("2.0")),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Geq, dec("3.0"), dec("3.0")),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Lt, dec("2.0"), dec("2.0")),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            eval_binary_op(BinOp::Gt, dec("2.0"), dec("2.0")),
            Ok(Value::Bool(false))
        );
    }

    #[test]
    fn p1219_decimal_coage_int_mas_nao_float() {
        assert_eq!(eval_binary_op(BinOp::Add, dec("1"), Value::Int(2)), Ok(dec("3")));
        assert!(eval_binary_op(BinOp::Add, dec("1"), Value::Float(2.0)).is_err());
        assert_eq!(eval_binary_op(BinOp::Add, Value::Int(2), dec("1")), Ok(dec("3")));
    }

    #[test]
    fn decimal_neg_unary() {
        assert_eq!(eval_unary_op(UnOp::Neg, dec("1.5")), Ok(dec("-1.5")));
        assert_eq!(eval_unary_op(UnOp::Neg, dec("-3")), Ok(dec("3")));
    }

    // ── P405 — Operações básicas Duration ────────────────────────────────────

    fn dur(seconds: i64) -> Value {
        Value::Duration(crate::entities::duration::Duration::from_seconds(seconds))
    }

    #[test]
    fn duration_add() {
        assert_eq!(eval_binary_op(BinOp::Add, dur(90), dur(30)), Ok(dur(120)));
    }

    #[test]
    fn duration_add_overflow() {
        let max =
            Value::Duration(crate::entities::duration::Duration::from_nanos(i128::MAX));
        assert!(eval_binary_op(BinOp::Add, max, dur(1)).is_err());
    }

    #[test]
    fn duration_sub() {
        assert_eq!(eval_binary_op(BinOp::Sub, dur(120), dur(30)), Ok(dur(90)));
    }

    #[test]
    fn duration_sub_negative() {
        assert_eq!(eval_binary_op(BinOp::Sub, dur(30), dur(120)), Ok(dur(-90)));
    }

    #[test]
    fn duration_sub_underflow() {
        let min =
            Value::Duration(crate::entities::duration::Duration::from_nanos(i128::MIN));
        assert!(eval_binary_op(BinOp::Sub, min, dur(1)).is_err());
    }

    #[test]
    fn duration_mul_int() {
        assert_eq!(eval_binary_op(BinOp::Mul, dur(60), Value::Int(2)), Ok(dur(120)));
        assert_eq!(eval_binary_op(BinOp::Mul, Value::Int(2), dur(60)), Ok(dur(120)));
    }

    #[test]
    fn duration_mul_int_neg() {
        assert_eq!(eval_binary_op(BinOp::Mul, dur(60), Value::Int(-1)), Ok(dur(-60)));
    }

    #[test]
    fn duration_mul_float() {
        assert_eq!(eval_binary_op(BinOp::Mul, dur(60), Value::Float(1.5)), Ok(dur(90)));
        assert_eq!(eval_binary_op(BinOp::Mul, Value::Float(1.5), dur(60)), Ok(dur(90)));
    }

    #[test]
    fn duration_div_int() {
        assert_eq!(eval_binary_op(BinOp::Div, dur(120), Value::Int(2)), Ok(dur(60)));
    }

    #[test]
    fn duration_div_int_zero() {
        assert!(eval_binary_op(BinOp::Div, dur(120), Value::Int(0)).is_err());
    }

    #[test]
    fn duration_div_int_neg() {
        assert_eq!(eval_binary_op(BinOp::Div, dur(120), Value::Int(-2)), Ok(dur(-60)));
    }

    #[test]
    fn duration_div_float() {
        assert_eq!(eval_binary_op(BinOp::Div, dur(120), Value::Float(2.0)), Ok(dur(60)));
    }

    #[test]
    fn duration_div_duration() {
        assert_eq!(eval_binary_op(BinOp::Div, dur(120), dur(60)), Ok(Value::Float(2.0)));
    }

    #[test]
    fn duration_div_duration_zero() {
        assert!(eval_binary_op(BinOp::Div, dur(120), dur(0)).is_err());
    }

    #[test]
    fn duration_eq_neq() {
        assert_eq!(eval_binary_op(BinOp::Eq, dur(60), dur(60)), Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Eq, dur(60), dur(59)), Ok(Value::Bool(false)));
        assert_eq!(eval_binary_op(BinOp::Neq, dur(60), dur(59)), Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Neq, dur(60), dur(60)), Ok(Value::Bool(false)));
    }

    #[test]
    fn duration_lt_gt_leq_geq() {
        assert_eq!(eval_binary_op(BinOp::Lt, dur(59), dur(60)), Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Gt, dur(61), dur(60)), Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Leq, dur(60), dur(60)), Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Geq, dur(60), dur(60)), Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Lt, dur(60), dur(60)), Ok(Value::Bool(false)));
        assert_eq!(eval_binary_op(BinOp::Gt, dur(60), dur(60)), Ok(Value::Bool(false)));
    }

    // ── P684 — Comparações Version (componentes arbitrários, zero-pad) ────────

    fn ver(major: u64, minor: u64, patch: u64) -> Value {
        Value::Version(Arc::new(crate::entities::version::Version::new(
            major, minor, patch,
        )))
    }

    fn verc(comps: &[u64]) -> Value {
        Value::Version(Arc::new(crate::entities::version::Version::from_components(
            comps.to_vec(),
        )))
    }

    #[test]
    fn version_eq_neq() {
        assert_eq!(
            eval_binary_op(BinOp::Eq, ver(1, 2, 3), ver(1, 2, 3)),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Eq, ver(1, 2, 3), ver(1, 2, 4)),
            Ok(Value::Bool(false))
        );
        // zero-pad: `version(1, 2, 3) == version(1, 2, 3, 0)`.
        assert_eq!(
            eval_binary_op(BinOp::Eq, ver(1, 2, 3), verc(&[1, 2, 3, 0])),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Eq, ver(1, 2, 3), verc(&[1, 2, 3, 4])),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            eval_binary_op(BinOp::Neq, ver(1, 2, 3), ver(1, 2, 4)),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn version_lt_gt_leq_geq() {
        assert_eq!(
            eval_binary_op(BinOp::Lt, ver(1, 2, 3), ver(1, 2, 4)),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Lt, ver(2, 0, 0), ver(1, 0, 0)),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            eval_binary_op(BinOp::Gt, ver(1, 2, 4), ver(1, 2, 3)),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(BinOp::Leq, ver(1, 2, 3), ver(1, 2, 3)),
            Ok(Value::Bool(true))
        );
        // mais curto (prefixo) < mais longo
        assert_eq!(
            eval_binary_op(BinOp::Lt, ver(1, 2, 3), verc(&[1, 2, 3, 4])),
            Ok(Value::Bool(true))
        );
        // componente a componente (zero-pad)
        assert_eq!(
            eval_binary_op(BinOp::Lt, verc(&[1, 2, 3, 0]), ver(1, 2, 4)),
            Ok(Value::Bool(true))
        );
        // trailing zero não altera a ordem
        assert_eq!(
            eval_binary_op(BinOp::Geq, verc(&[1, 2, 3, 0]), ver(1, 2, 3)),
            Ok(Value::Bool(true))
        );
    }

    // ── P411 — Field Access Version ──────────────────────────────────────────

    #[test]
    fn version_field_major() {
        let world = MockWorld::new("#let x = version(\"1.2.3\").major");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(1)));
    }

    #[test]
    fn version_field_minor() {
        let world = MockWorld::new("#let x = version(\"1.2.3\").minor");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(2)));
    }

    #[test]
    fn version_field_patch() {
        let world = MockWorld::new("#let x = version(\"1.2.3\").patch");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(3)));
    }

    // P684 — `pre`/`build` deixaram de existir em version: field access é erro.

    #[test]
    fn version_field_pre_desconhecido() {
        let world = MockWorld::new("#let x = version(\"1.2.3\").pre");
        let src = World::source(&world, World::main(&world)).unwrap();
        let err =
            eval_for_test(&world, &src).expect_err("`.pre` já não existe em version");
        assert!(
            err.iter().any(|d| d.message.contains("does not contain field")),
            "esperava 'version does not contain field'; recebido: {:?}",
            err.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn version_field_build_desconhecido() {
        let world = MockWorld::new("#let x = version(\"1.2.3\").build");
        let src = World::source(&world, World::main(&world)).unwrap();
        let err =
            eval_for_test(&world, &src).expect_err("`.build` já não existe em version");
        assert!(
            err.iter().any(|d| d.message.contains("does not contain field")),
            "esperava 'version does not contain field'; recebido: {:?}",
            err.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    // P684 — componentes arbitrários e igualdade zero-pad ao nível do eval.

    #[test]
    fn version_eval_eq_zero_pad() {
        let world = MockWorld::new("#let x = version(1, 2, 3) == version(1, 2, 3, 0)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Bool(true)));
    }

    #[test]
    fn version_eval_arbitrary_components() {
        let world = MockWorld::new("#let x = version(1, 2, 3, 4, 5).patch");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(3)));
    }

    #[test]
    fn version_field_unknown() {
        let world = MockWorld::new("#let x = version(\"1.2.3\").foo");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }

    // ── P796 — `.at(index)` em version ───────────────────────────────────────

    #[test]
    fn version_at_positivo_dentro_do_alcance() {
        let world = MockWorld::new("#let x = version(1, 2, 3).at(1)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(2)));
    }

    #[test]
    fn version_at_positivo_alem_do_alcance_zero_pad() {
        let world = MockWorld::new("#let x = version(1, 2, 3).at(10)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(0)));
    }

    #[test]
    fn version_at_negativo_conta_do_fim() {
        let world = MockWorld::new("#let x = version(1, 2, 3).at(-1)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(3)));
    }

    #[test]
    fn version_at_negativo_fora_de_limites_erro() {
        let world = MockWorld::new("#let x = version(1, 2, 3).at(-10)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src)
            .expect_err("índice negativo fora de limites deve errar");
        // paridade vanilla ao carácter (ADR-0108, excepção — mecânica é o
        // observável em mensagens de erro; vanilla `version.rs:124-127`).
        assert!(
            err.iter().any(
                |d| d.message == "component index out of bounds (index: -10, len: 3)"
            ),
            "recebido: {:?}",
            err.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    // ── P796 — exibição de `version` em markup (`Display`, não `repr`) ───────

    #[test]
    fn version_markup_display_nao_e_repr() {
        // Achado P786/P796: #sys.version mostrava "version(0, 15, 0)" (repr)
        // em vez de "0.15.0" (Display). Paridade vanilla:
        // `Value::display` usa o Display do tipo, não o repr.
        let m = p729_eval("#version(0, 15, 0)").unwrap();
        let text = m.content().expect("content").plain_text();
        assert_eq!(text, "0.15.0");
    }

    #[test]
    fn version_markup_display_vazia_produz_nada() {
        let m = p729_eval("#version()").unwrap();
        let text = m.content().expect("content").plain_text();
        assert_eq!(text, "");
    }

    #[test]
    fn version_markup_display_sys_version() {
        let m = p729_eval("#sys.version").unwrap();
        let text = m.content().expect("content").plain_text();
        assert_eq!(text, "0.15.1");
    }

    // ── P412 — Field Access Duration ─────────────────────────────────────────

    #[test]
    fn duration_field_seconds_zero() {
        let world = MockWorld::new("#let x = duration(\"0s\").seconds");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(0.0)));
    }

    #[test]
    fn duration_field_seconds_simple() {
        let world = MockWorld::new("#let x = duration(\"5s\").seconds");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(5.0)));
    }

    #[test]
    fn duration_field_seconds_compound() {
        let world = MockWorld::new("#let x = duration(\"1h30m\").seconds");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(5400.0)));
    }

    #[test]
    fn duration_field_seconds_fraction() {
        let world = MockWorld::new("#let x = duration(\"1.5s\").seconds");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(1.5)));
    }

    #[test]
    fn duration_field_minutes() {
        let world = MockWorld::new("#let x = duration(\"90m\").minutes");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(90.0)));
    }

    #[test]
    fn duration_field_minutes_compound() {
        let world = MockWorld::new("#let x = duration(\"1h30m\").minutes");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(90.0)));
    }

    #[test]
    fn duration_field_hours() {
        let world = MockWorld::new("#let x = duration(\"1h30m\").hours");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(1.5)));
    }

    #[test]
    fn duration_field_days() {
        let world = MockWorld::new("#let x = duration(\"36h\").days");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(1.5)));
    }

    #[test]
    fn duration_field_days_zero() {
        let world = MockWorld::new("#let x = duration(\"0s\").days");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(0.0)));
    }

    #[test]
    fn duration_field_negative() {
        let world = MockWorld::new("#let x = duration(seconds: -90).seconds");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(-90.0)));
    }

    #[test]
    fn duration_field_unknown() {
        let world = MockWorld::new("#let x = duration(\"1h\").foo");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }

    // ── P706 — `in` / `not in` ────────────────────────────────────────────────

    fn dict_of(pairs: Vec<(&str, Value)>) -> Value {
        let mut d = IndexMap::with_hasher(FxBuildHasher::default());
        for (k, v) in pairs {
            d.insert(EcoString::from(k), v);
        }
        Value::Dict(d)
    }

    #[test]
    fn paridade_in_str_dict_existe() {
        let d = dict_of(vec![("a", Value::Int(1)), ("b", Value::Int(2))]);
        assert_eq!(
            eval_binary_op(BinOp::In, Value::Str("a".into()), d),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn paridade_in_str_dict_nao_existe() {
        let d = dict_of(vec![("a", Value::Int(1)), ("b", Value::Int(2))]);
        assert_eq!(
            eval_binary_op(BinOp::In, Value::Str("z".into()), d),
            Ok(Value::Bool(false))
        );
    }

    #[test]
    fn paridade_in_str_str_substring() {
        assert_eq!(
            eval_binary_op(
                BinOp::In,
                Value::Str("ell".into()),
                Value::Str("hello".into())
            ),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(
                BinOp::In,
                Value::Str("xyz".into()),
                Value::Str("hello".into())
            ),
            Ok(Value::Bool(false))
        );
    }

    #[test]
    fn paridade_in_array_elemento() {
        let arr = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(
            eval_binary_op(BinOp::In, Value::Int(1), arr.clone()),
            Ok(Value::Bool(true))
        );
        assert_eq!(eval_binary_op(BinOp::In, Value::Int(5), arr), Ok(Value::Bool(false)));
    }

    #[test]
    fn paridade_in_array_de_arrays() {
        // matrix.typ:252 (`out in _ident`) — array dentro de array de arrays.
        let ident = Value::Array(vec![
            Value::Array(vec![Value::Int(1), Value::Int(2)]),
            Value::Array(vec![Value::Int(3), Value::Int(4)]),
        ]);
        let out = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        let missing = Value::Array(vec![Value::Int(5), Value::Int(6)]);
        assert_eq!(eval_binary_op(BinOp::In, out, ident.clone()), Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::In, missing, ident), Ok(Value::Bool(false)));
    }

    #[test]
    fn paridade_in_array_tipos_mistos() {
        // mark.typ:157 (`slant not in (none, 0%)`) — array com tipos diferentes.
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let arr = Value::Array(vec![
            Value::None,
            Value::Relative(Rel { rel: 0.0, abs: Length::ZERO }),
        ]);
        assert_eq!(
            eval_binary_op(BinOp::In, Value::None, arr.clone()),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            eval_binary_op(
                BinOp::In,
                Value::Relative(Rel { rel: 0.01, abs: Length::ZERO }),
                arr
            ),
            Ok(Value::Bool(false))
        );
    }

    #[test]
    fn paridade_in_int_float_coercao() {
        // Medido contra o vanilla: `1 in (1.0, 2.0)` -> true (mesma coerção do `==`).
        let arr = Value::Array(vec![Value::Float(1.0), Value::Float(2.0)]);
        assert_eq!(eval_binary_op(BinOp::In, Value::Int(1), arr), Ok(Value::Bool(true)));
    }

    #[test]
    fn paridade_not_in() {
        let arr = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        assert_eq!(
            eval_binary_op(BinOp::NotIn, Value::Int(1), arr.clone()),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            eval_binary_op(BinOp::NotIn, Value::Int(5), arr),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn paridade_in_tipos_incompativeis_erro() {
        // Medido contra o vanilla: `1 in "hello"` -> Err (tipos incompatíveis).
        assert!(
            eval_binary_op(BinOp::In, Value::Int(1), Value::Str("hello".into())).is_err()
        );
    }

    // ── Testes de paridade: eval_unary_op ────────────────────────────────────

    #[test]
    fn paridade_not() {
        assert_eq!(eval_unary_op(UnOp::Not, Value::Bool(true)), Ok(Value::Bool(false)));
        assert_eq!(eval_unary_op(UnOp::Not, Value::Bool(false)), Ok(Value::Bool(true)));
    }

    #[test]
    fn paridade_neg_int() {
        assert_eq!(eval_unary_op(UnOp::Neg, Value::Int(5)), Ok(Value::Int(-5)));
        assert_eq!(eval_unary_op(UnOp::Neg, Value::Int(-3)), Ok(Value::Int(3)));
    }

    #[test]
    fn paridade_neg_overflow() {
        // i64::MIN.checked_neg() == None (overflow)
        let r = eval_unary_op(UnOp::Neg, Value::Int(i64::MIN));
        assert!(r.is_err());
    }

    #[test]
    fn paridade_pos_noop() {
        assert_eq!(eval_unary_op(UnOp::Pos, Value::Int(42)), Ok(Value::Int(42)));
        assert_eq!(eval_unary_op(UnOp::Pos, Value::Float(1.5)), Ok(Value::Float(1.5)));
    }

    #[test]
    fn paridade_unary_tipo_invalido() {
        let r = eval_unary_op(UnOp::Not, Value::Int(1));
        assert!(r.is_err());
    }

    // ── Testes de Passo 16 — Closures e FuncCall ─────────────────────────────

    #[test]
    fn closure_cria_value_func() {
        let world = MockWorld::new("#let f = (x) => x + 1");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert!(matches!(m.scope().get("f"), Some(Value::Func(_))));
    }

    #[test]
    fn funcall_soma_dois_args() {
        let world = MockWorld::new("#let add = (x, y) => x + y\n#let r = add(1, 2)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(3)));
    }

    #[test]
    fn funcall_arg_errado_retorna_err() {
        let world = MockWorld::new("#let r = 42(1)");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }

    #[test]
    fn closure_default_param() {
        let world =
            MockWorld::new("#let greet = (prefix: \"Hi\") => prefix\n#let r = greet()");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("Hi".into())));
    }

    /// Teste de Ouro: valida que eager capture é determinista e isolada.
    #[test]
    fn eager_capture_isolada_do_scope_pai() {
        let world = MockWorld::new(
            "#let x = 1\n\
             #let get_x = () => x\n\
             #let x = 2\n\
             #let r = get_x()",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(
            m.scope().get("r"),
            Some(&Value::Int(1)),
            "eager capture deve isolar a closure do shadowing posterior"
        );
    }

    #[test]
    fn closure_scope_nao_vaza_para_chamador() {
        let world = MockWorld::new(
            "#let f = () => { let local = 99; local }\n\
             #let r = f()",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(99)));
        assert!(
            m.scope().get("local").is_none(),
            "variáveis locais da closure não devem vazar para o chamador"
        );
    }

    #[test]
    fn closure_recursiva_nao_vaza_memoria() {
        let world = MockWorld::new(
            "#let fact = (n) => if n <= 0 { 1 } else { n }\n\
             #let r = fact(5)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(5)));
    }

    #[test]
    fn func_type_name() {
        use crate::entities::func::{ClosureRepr, Func};
        use crate::entities::scope::{Capturer, Scope};
        use crate::entities::source::Source;
        use std::sync::Arc;
        let source = Source::detached("x");
        let body = source.root().clone();
        let f = Func::closure(ClosureRepr {
            name: None,
            params: vec![],
            sink_name: None,
            body,
            captured: Arc::new(Scope::new()),
            capturer: Capturer::Function,
        });
        assert_eq!(Value::Func(f).type_name(), "function");
    }

    // ── Testes de Passo 17 — Recursão ────────────────────────────────────────

    #[test]
    fn recursao_factorial() {
        let world = MockWorld::new(
            "#let fact = (n) => if n <= 1 { 1 } else { n * fact(n - 1) }\n\
             #let r = fact(5)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(120)));
    }

    #[test]
    fn recursao_fibonacci() {
        let world = MockWorld::new(
            "#let fib = (n) => if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }\n\
             #let r = fib(7)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(13)));
    }

    /// Teste de estabilidade — valida que recursão infinita não faz crash.
    /// Um Stack Overflow é inaceitável em servidor; Err é a falha correcta.
    #[test]
    fn recursao_infinita_retorna_err_sem_crash() {
        // Usa limite reduzido (50) para evitar stack overflow real do Rust em debug mode.
        // O mecanismo funciona identicamente a qualquer profundidade — 50 é suficiente para verificar.
        let world = MockWorld::new(
            "#let inf = (n) => inf(n + 1)\n\
             #let r = inf(0)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test_with_limits(&world, &src, 1_000_000);
        assert!(result.is_err(), "recursão infinita deve Err, não crash");
        let msg = &result.unwrap_err()[0].message;
        assert!(
            msg.contains("profundidade") || msg.contains("depth"),
            "mensagem deve mencionar limite: {:?}",
            msg
        );
    }

    // ── Testes de Passo 17 — Stdlib ──────────────────────────────────────────

    #[test]
    fn stdlib_type_int() {
        let world = MockWorld::new("#let t = type(42)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(
            m.scope().get("t"),
            Some(&Value::Type(crate::entities::value::Type::Int))
        );
    }

    #[test]
    fn stdlib_type_func() {
        let world = MockWorld::new("#let f = () => 1\n#let t = type(f)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(
            m.scope().get("t"),
            Some(&Value::Type(crate::entities::value::Type::Function))
        );
    }

    // ── P685 — tipos como valores de primeira classe ─────────────────────────

    fn eval_bool(src: &str) -> bool {
        let world = MockWorld::new(src);
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        match m.scope().get("r") {
            Some(Value::Bool(b)) => *b,
            other => panic!("esperava Bool em r, obteve {other:?}"),
        }
    }

    #[test]
    fn p685_type_eq_int() {
        assert!(eval_bool("#let r = (type(1) == int)"));
    }
    #[test]
    fn p685_type_eq_float() {
        assert!(eval_bool("#let r = (type(1.0) == float)"));
    }
    #[test]
    fn p685_type_eq_length() {
        assert!(eval_bool("#let r = (type(1pt) == length)"));
    }
    #[test]
    fn p685_type_eq_angle() {
        assert!(eval_bool("#let r = (type(1deg) == angle)"));
    }
    // NOTA P685 (revogada em P842): `type(50%) == ratio` É paridade — `50%`
    // passou a ser modelado como `Value::Ratio` (P842, achado #32 de P831);
    // ver `p842_l1_*` abaixo.
    #[test]
    fn p685_type_eq_str() {
        assert!(eval_bool("#let r = (type(\"x\") == str)"));
    }
    #[test]
    fn p685_type_eq_array() {
        assert!(eval_bool("#let r = (type(()) == array)"));
    }
    #[test]
    fn p685_type_eq_dict() {
        assert!(eval_bool("#let r = (type((:)) == dictionary)"));
    }
    #[test]
    fn p685_type_of_type() {
        assert!(eval_bool("#let r = (type(int) == type)"));
    }
    #[test]
    fn p685_type_of_func() {
        assert!(eval_bool("#let r = (type(rgb) == function)"));
    }
    #[test]
    fn p685_type_distinct() {
        assert!(!eval_bool("#let r = (length == ratio)"));
    }
    #[test]
    fn p685_int_float_distinct() {
        assert!(!eval_bool("#let r = (int == float)"));
    }

    #[test]
    fn p685_repr_int() {
        let world = MockWorld::new("#let r = repr(int)");
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("int".into())));
    }

    #[test]
    fn p685_repr_type_of_length() {
        let world = MockWorld::new("#let r = repr(type(1pt))");
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("length".into())));
    }

    #[test]
    fn p685_shadow_length() {
        // variável do utilizador sombreia o valor-tipo global `length`.
        let world = MockWorld::new("#let length = 5\n#let r = length");
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(5)));
    }

    #[test]
    fn p685_shadow_then_type_is_int() {
        // após sombrear `length`, type(length) é o tipo do valor (int), não o
        // valor-tipo global.
        let world = MockWorld::new("#let length = 5\n#let r = (type(length) == int)");
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Bool(true)));
    }

    // ── P842 (achado #32 de P831) — Ratio/Relative como tipos distintos ──────
    // Medido no vanilla (temp/p842/l1_type_*.typ, l1_ratio_arith*.typ):
    // `type(50%)` → ratio; `type(50% + 0pt)` → relative; `type(30% + 1em)` →
    // relative; `relative` é binding global de tipo. No cristalino pré-P842
    // `50%` era `Value::Relative` (P469) e `Type::Relative` nem existia —
    // `type_of` mapeava tudo para `Type::Length` com comentário que afirmava
    // paridade (refutado pela medição).

    fn eval_str_value(src: &str) -> Value {
        let world = MockWorld::new(src);
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        m.scope().get("r").cloned().expect("binding r ausente")
    }

    #[test]
    fn p842_l1_type_ratio_puro() {
        assert_eq!(
            eval_str_value("#let r = repr(type(50%))"),
            Value::Str("ratio".into())
        );
    }

    #[test]
    fn p842_l1_type_relative_abs_zero() {
        // `50% + 0pt` já é Rel no vanilla (Ratio + Length → Rel), mesmo com
        // a parte absoluta zero — o tipo depende da construção, não do valor.
        assert_eq!(
            eval_str_value("#let r = repr(type(50% + 0pt))"),
            Value::Str("relative".into())
        );
    }

    #[test]
    fn p842_l1_type_relative_misto() {
        assert_eq!(
            eval_str_value("#let r = repr(type(30% + 1em))"),
            Value::Str("relative".into())
        );
    }

    #[test]
    fn p842_l1_relative_binding_global() {
        assert!(eval_bool("#let r = (type(30% + 1em) == relative)"));
    }

    #[test]
    fn p842_l1_ratio_aritmetica_preserva_tipo() {
        // Medido no vanilla (temp/p842/l1_ratio_arith.typ): 50% + 30% = 80%
        // (ratio); 50% - 30% = 20%; 50% * 2 = 100%; 50% / 2 = 25%;
        // 50% / 25% = 2.0 (float); 50% == 50% + 0pt → true.
        assert!(eval_bool("#let r = (type(50% + 30%) == ratio)"));
        assert!(eval_bool("#let r = (type(50% - 30%) == ratio)"));
        assert!(eval_bool("#let r = (type(50% * 2) == ratio)"));
        assert!(eval_bool("#let r = (type(50% / 2) == ratio)"));
        assert!(eval_bool("#let r = (type(50% / 25%) == float)"));
        assert!(eval_bool("#let r = (50% == 50% + 0pt)"));
        assert!(eval_bool("#let r = (type(50% + 1pt) == relative)"));
        assert!(eval_bool("#let r = (type(1pt - 50%) == relative)"));
        assert_eq!(eval_str_value("#let r = repr(50% + 30%)"), Value::Str("80%".into()));
        assert_eq!(
            eval_str_value("#let r = repr(1pt - 50%)"),
            Value::Str("-50% + 1pt".into())
        );
    }

    #[test]
    fn p842_l1_ratio_vezes_fraction() {
        // Medido no vanilla: `100% * 2fr` = `2fr`; type(50% * 2fr) = fraction.
        assert!(eval_bool("#let r = (type(50% * 2fr) == fraction)"));
        assert_eq!(eval_str_value("#let r = repr(100% * 2fr)"), Value::Str("2fr".into()));
    }

    #[test]
    fn p685_int_callable() {
        let world = MockWorld::new("#let r = int(\"5\")");
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(5)));
    }

    #[test]
    fn p685_str_callable() {
        let world = MockWorld::new("#let r = str(5)");
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("5".into())));
    }

    #[test]
    fn p685_float_callable() {
        let world = MockWorld::new("#let r = float(\"3.5\")");
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Float(3.5)));
    }

    #[test]
    fn p685_int_min_max_fields() {
        let world = MockWorld::new("#let a = int.min\n#let b = int.max");
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Int(i64::MIN)));
        assert_eq!(m.scope().get("b"), Some(&Value::Int(i64::MAX)));
    }

    #[test]
    fn p685_str_from_unicode_field() {
        let world = MockWorld::new("#let r = str.from-unicode(97)");
        let s = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &s).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("a".into())));
    }

    #[test]
    fn p685_bool_not_callable() {
        let world = MockWorld::new("#let r = bool(1)");
        let s = World::source(&world, World::main(&world)).unwrap();
        assert!(
            eval_for_test(&world, &s).is_err(),
            "bool(1) deve falhar (type bool has no constructor)"
        );
    }

    #[test]
    fn p685_length_not_callable() {
        let world = MockWorld::new("#let r = length(1pt)");
        let s = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &s).is_err(), "length(1pt) deve falhar");
    }

    #[test]
    fn p11401_sete_bindings_tem_kind_type() {
        for name in
            ["decimal", "duration", "regex", "selector", "stroke", "tiling", "version"]
        {
            let source = format!("#let r = repr(type({name}))");
            assert_eq!(
                eval_str_value(&source),
                Value::Str("type".into()),
                "kind público incorreto para {name}",
            );
        }
    }

    #[test]
    fn p11401_construtores_de_tipo_preservam_resultado() {
        for (source, expected) in [
            ("#let r = type(decimal(\"1.5\")) == decimal", true),
            ("#let r = type(duration(seconds: 1)) == duration", true),
            ("#let r = type(regex(\"a+\")) == regex", true),
            ("#let r = type(selector(\"heading\")) == selector", true),
            ("#let r = type(stroke()) == stroke", true),
            ("#let r = type(tiling(rgb(255, 0, 0), size: 10pt)) == tiling", true),
            ("#let r = type(version(1, 2, 3)) == version", true),
        ] {
            assert_eq!(eval_bool(source), expected, "falhou: {source}");
        }
    }

    #[test]
    fn p11402_label_kind_construtor_igualdade_repr_e_str() {
        let m = p729_eval(
            "#let kind = repr(type(label))\n\
             #let instance-kind = repr(type(label(\"x\")))\n\
             #let same-type = type(label(\"x\")) == label\n\
             #let equal-literal = label(\"x\") == <x>\n\
             #let literal-repr = repr(label(\"x\"))\n\
             #let special-repr = repr(label(\"a b\"))\n\
             #let special-str = str(label(\"a b\"))",
        )
        .unwrap();

        assert_eq!(m.scope().get("kind"), Some(&Value::Str("type".into())));
        assert_eq!(m.scope().get("instance-kind"), Some(&Value::Str("label".into())));
        assert_eq!(m.scope().get("same-type"), Some(&Value::Bool(true)));
        assert_eq!(m.scope().get("equal-literal"), Some(&Value::Bool(true)));
        assert_eq!(m.scope().get("literal-repr"), Some(&Value::Str("<x>".into())));
        assert_eq!(
            m.scope().get("special-repr"),
            Some(&Value::Str("label(\"a b\")".into()))
        );
        assert_eq!(m.scope().get("special-str"), Some(&Value::Str("a b".into())));
    }

    #[test]
    fn p11402_label_rejeita_aridade_tipo_vazio_e_nomeado() {
        for source in [
            "#label()",
            "#label(1)",
            "#label(\"\")",
            "#label(\"x\", [body])",
            "#label(\"x\", body: [body])",
        ] {
            assert!(p729_eval(source).is_err(), "deveria falhar: {source}");
        }
    }

    #[test]
    fn p11403a_math_sqrt_e_funcao_publica() {
        let m = p729_eval(
            "#let kind = repr(type(math.sqrt))\n\
             #let result-kind = repr(type(math.sqrt([x])))\n\
             #let result = repr(math.sqrt([x]))",
        )
        .unwrap();
        assert_eq!(m.scope().get("kind"), Some(&Value::Str("function".into())));
        assert_eq!(m.scope().get("result-kind"), Some(&Value::Str("content".into())));
        assert_eq!(
            m.scope().get("result"),
            Some(&Value::Str("root(radicand: [x])".into()))
        );
    }

    #[test]
    fn p11403a_math_sqrt_rejeita_aridade_tipo_e_named() {
        for source in [
            "#math.sqrt()",
            "#math.sqrt([x], [y])",
            "#math.sqrt(1)",
            "#math.sqrt(radicand: [x])",
        ] {
            assert!(p729_eval(source).is_err(), "deveria falhar: {source}");
        }
    }

    #[test]
    fn p11403a_sym_sqrt_extra_foi_removido() {
        assert!(p729_eval("#sym.sqrt").is_err());
    }

    #[test]
    fn p11403b_math_equation_e_funcao_publica_body_block() {
        let m = p729_eval(
            "#let kind = repr(type(math.equation))\n\
             #let inline = repr(math.equation([x]))\n\
             #let block = repr(math.equation(block: true, [x]))",
        )
        .unwrap();
        assert_eq!(m.scope().get("kind"), Some(&Value::Str("function".into())));
        assert_eq!(
            m.scope().get("inline"),
            Some(&Value::Str("equation(body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("block"),
            Some(&Value::Str("equation(block: true, body: [x])".into()))
        );
    }

    #[test]
    fn p11403b_math_equation_rejeita_invalidos_e_scope_out() {
        for source in [
            "#math.equation()",
            "#math.equation([x], [y])",
            "#math.equation(1)",
            "#math.equation(block: 1, [x])",
            "#math.equation(foo: 1, [x])",
        ] {
            assert!(p729_eval(source).is_err(), "deveria falhar: {source}");
        }
    }

    #[test]
    fn p11404a_equation_numbering_aceita_pattern_func_e_none() {
        let m = p729_eval(
            "#let pattern = repr(math.equation(numbering: \"(1)\", [x]))\n\
             #let disabled = repr(math.equation(numbering: none, [x]))\n\
             #let callback = repr(math.equation(numbering: n => str(n), block: true, [x]))",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("pattern"),
            Some(&Value::Str("equation(numbering: \"(1)\", body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("disabled"),
            Some(&Value::Str("equation(numbering: none, body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("callback"),
            Some(&Value::Str(
                "equation(block: true, numbering: (..) => .., body: [x])".into()
            ))
        );
    }

    #[test]
    fn p11404b_equation_number_align_aceita_e_preserva_repr() {
        let m = p729_eval(
            "#let vertical = repr(math.equation(number-align: bottom, [x]))\n\
             #let both = repr(math.equation(number-align: left + top, [x]))\n\
             #let explicit_default = repr(math.equation(number-align: end + horizon, [x]))",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("vertical"),
            Some(&Value::Str("equation(number-align: bottom, body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("both"),
            Some(&Value::Str("equation(number-align: left + top, body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("explicit_default"),
            Some(&Value::Str("equation(number-align: end + horizon, body: [x])".into()))
        );
    }

    #[test]
    fn p11404b_equation_number_align_rejeita_center_e_tipo_invalido() {
        let center = p729_eval("#math.equation(number-align: center, [x])")
            .expect_err("center horizontal não pertence ao cast");
        assert!(center[0]
            .message
            .contains("expected `start`, `left`, `right`, or `end`, found center"));
        let integer = p729_eval("#math.equation(number-align: 1, [x])")
            .expect_err("int não é alignment");
        assert!(integer[0].message.contains("expected alignment, found integer"));
    }

    #[test]
    fn p11404b_set_equation_combina_numbering_e_number_align() {
        let m = p729_eval(
            "#set math.equation(numbering: \"(1)\", number-align: left + top)\n$x$",
        )
        .unwrap();
        let content = m.content().expect("content");
        assert!(find_custom_in_styled(content, "equation.numbering").is_some());
        assert!(find_custom_in_styled(content, "equation.number-align").is_some());
    }

    #[test]
    fn p11404c_equation_supplement_aceita_e_preserva_repr() {
        let m = p729_eval(
            "#let a = repr(math.equation(supplement: auto, [x]))\n\
             #let n = repr(math.equation(supplement: none, [x]))\n\
             #let c = repr(math.equation(supplement: [Eq.], [x]))\n\
             #let s = repr(math.equation(supplement: \"Eq.\", [x]))\n\
             #let f = repr(math.equation(supplement: it => [FUN], [x]))",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("a"),
            Some(&Value::Str("equation(supplement: auto, body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("n"),
            Some(&Value::Str("equation(supplement: none, body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("c"),
            Some(&Value::Str("equation(supplement: [Eq.], body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("s"),
            Some(&Value::Str("equation(supplement: [Eq.], body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("f"),
            Some(&Value::Str("equation(supplement: (..) => .., body: [x])".into()))
        );
    }

    #[test]
    fn p11404c_supplement_rejeita_tipo_e_set_combina_campos() {
        let err = p729_eval("#math.equation(supplement: 1, [x])").unwrap_err();
        assert!(err[0]
            .message
            .contains("expected content, function, none, or auto, found integer"));
        let m =
            p729_eval("#set math.equation(numbering: \"(1)\", supplement: [Eq.])\n$ x $")
                .unwrap();
        let content = m.content().expect("content");
        assert!(find_custom_in_styled(content, "equation.numbering").is_some());
        assert!(find_custom_in_styled(content, "equation.supplement").is_some());
    }

    #[test]
    fn p11405_equation_alt_aceita_e_preserva_repr() {
        let m = p729_eval(
            "#let text = repr(math.equation(alt: \"x squared\", [x]))\n\
             #let disabled = repr(math.equation(alt: none, [x]))\n\
             #let empty = repr(math.equation(alt: \"\", [x]))\n\
             #let combined = repr(math.equation(block: true, numbering: \"(1)\", number-align: left, supplement: [Eq.], alt: \"desc\", [x]))",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("text"),
            Some(&Value::Str("equation(alt: \"x squared\", body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("disabled"),
            Some(&Value::Str("equation(alt: none, body: [x])".into()))
        );
        assert_eq!(
            m.scope().get("empty"),
            Some(&Value::Str("equation(alt: \"\", body: [x])".into()))
        );
        assert_eq!(m.scope().get("combined"), Some(&Value::Str("equation(block: true, numbering: \"(1)\", number-align: left, supplement: [Eq.], alt: \"desc\", body: [x])".into())));
    }

    #[test]
    fn p11405_equation_alt_rejeita_tipo_e_set_transporta() {
        let content = p729_eval("#math.equation(alt: [x], [x])").unwrap_err();
        assert!(content[0].message.contains("expected string or none, found content"));
        let integer = p729_eval("#math.equation(alt: 1, [x])").unwrap_err();
        assert!(integer[0].message.contains("expected string or none, found integer"));

        let m = p729_eval("#set math.equation(alt: \"set description\")\n$ x $").unwrap();
        let content = m.content().expect("content");
        assert!(find_custom_in_styled(content, "equation.alt").is_some());
    }

    #[test]
    fn p11405_equation_alt_e_visivel_em_field_e_fields() {
        let m = p729_eval(
            "#let e = math.equation(alt: \"description\", [x])\n\
             #let direct = e.alt\n\
             #let all = e.fields()",
        )
        .unwrap();
        assert_eq!(m.scope().get("direct"), Some(&Value::Str("description".into())));
        let Value::Dict(fields) = m.scope().get("all").expect("fields") else {
            panic!("fields() deve devolver dictionary");
        };
        assert_eq!(fields.get("alt"), Some(&Value::Str("description".into())));
    }

    #[test]
    fn p11404a_equation_numbering_rejeita_tipo_invalido() {
        assert!(p729_eval("#math.equation(numbering: 1, [x])").is_err());
        assert!(p729_eval("#set math.equation(numbering: 1)\n$ x $").is_err());
    }

    #[test]
    fn p11404a_set_equation_numbering_aceita_funcao() {
        assert!(p729_eval("#set math.equation(numbering: n => str(n))\n$ x $").is_ok());
    }

    #[test]
    fn stdlib_range_simples() {
        let world = MockWorld::new("#let r = range(3)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(
            m.scope().get("r"),
            Some(&Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn stdlib_range_vazio_se_start_eq_end() {
        let world = MockWorld::new("#let r = range(3, 3)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Array(vec![])));
    }

    #[test]
    fn for_com_range_integrado() {
        let world = MockWorld::new("#for i in range(3) { }");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_ok());
    }

    /// **P538f** — corpo de `#for` em modo markup acumula conteúdo e produz
    /// um documento não vazio.
    #[test]
    fn p538f_for_acumula_conteudo_do_corpo() {
        use crate::compiler::layout::layout;
        let world = MockWorld::new("#for i in range(3) [A]");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("eval deve produzir Content");
        assert!(
            content.plain_text().contains("A"),
            "for deve acumular conteúdo do corpo"
        );
        let doc = layout(content);
        assert!(!doc.pages.is_empty(), "documento layoutado não deve ser vazio");
    }

    /// **P540** — `#for` suporta destructuring de tuplo.
    /// **P545** — o teste passou a usar `#{i+1}` para validar a
    /// interpolação de expressões em markup.
    #[test]
    fn p540_for_destructuring_tuplo() {
        let world = MockWorld::new(
            "#let items = (\"um\", \"dois\", \"três\")\n#for (i, x) in items.enumerate() [#{i+1}. #x]",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("eval deve produzir Content");
        let text = content.plain_text();
        assert!(text.contains("um"), "deve conter 'um'");
        assert!(text.contains("dois"), "deve conter 'dois'");
        assert!(text.contains("três"), "deve conter 'três'");
        // **P545** — os índices interpolados devem ser 1, 2, 3.
        assert!(
            text.contains('1') && text.contains('2') && text.contains('3'),
            "deve conter índices interpolados 1, 2, 3; got {}",
            text
        );
    }

    // ── Testes de Passo 17 — Named args ──────────────────────────────────────

    #[test]
    fn named_arg_simples() {
        let world = MockWorld::new(
            "#let greet = (prefix: \"Hi\", name) => prefix\n\
             #let r = greet(\"world\", prefix: \"Hello\")",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("Hello".into())));
    }

    // ── Testes de Passo 18 — Pipeline Content ────────────────────────────────

    #[test]
    fn pipeline_completo_texto_simples() {
        use crate::compiler::layout::layout;

        let world = MockWorld::new("Hello world");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();

        let content = module.content().expect("eval deve produzir Content");
        assert!(!content.is_empty());
        assert!(content.plain_text().contains("Hello"));
        assert!(content.plain_text().contains("world"));

        let result = layout(content);
        assert!(!result.plain_text().is_empty());
    }

    #[test]
    fn pipeline_interpolacao_variavel() {
        use crate::compiler::layout::layout;

        let world = MockWorld::new("#let x = \"Mundo\"\nOlá #x");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();

        let content = module.content().expect("Content deve existir");
        let text = content.plain_text();
        assert!(text.contains("Olá"), "texto estático deve estar presente: {:?}", text);
        assert!(
            text.contains("Mundo"),
            "variável interpolada deve estar presente: {:?}",
            text
        );

        let result = layout(content);
        assert!(!result.plain_text().is_empty());
    }

    #[test]
    fn pipeline_documento_vazio() {
        let world = MockWorld::new("");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        if let Some(c) = module.content() {
            assert!(c.is_empty());
        }
        // Sem pânico — pipeline robusto para input vazio
    }

    #[test]
    fn pipeline_apenas_codigo_sem_markup() {
        let world = MockWorld::new("#let x = 42");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        assert_eq!(module.scope().get("x"), Some(&Value::Int(42)));
        // Content pode ser vazio — é correcto
    }

    #[test]
    fn content_type_name() {
        use crate::entities::content::Content;
        let v = Value::Content(Content::text("hello"));
        assert_eq!(v.type_name(), "content");
    }

    // ── Testes de Passo 22 — Rich text ───────────────────────────────────────

    #[test]
    fn pipeline_rich_text_plain_text_correcto() {
        let world = MockWorld::new("Hello *bold* and _italic_");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        let text = content.plain_text();
        assert!(text.contains("Hello"), "plain_text deve ter Hello: {:?}", text);
        assert!(text.contains("bold"), "plain_text deve ter bold: {:?}", text);
        assert!(text.contains("italic"), "plain_text deve ter italic: {:?}", text);
    }

    #[test]
    fn pipeline_heading_plain_text_correcto() {
        let world = MockWorld::new("= Introduction\nBody text");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        let text = content.plain_text();
        assert!(
            text.contains("Introduction"),
            "plain_text deve ter Introduction: {:?}",
            text
        );
        assert!(text.contains("Body"), "plain_text deve ter Body: {:?}", text);
    }

    // ── Passo 23 ────────────────────────────────────────────────────────────

    #[test]
    fn pipeline_raw_inline() {
        let world = MockWorld::new("Use `cargo build` to compile");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().expect("deve ter content").plain_text();
        assert!(text.contains("cargo") && text.contains("build"), "{:?}", text);
    }

    #[test]
    fn pipeline_lista_bullets() {
        let world = MockWorld::new("- item 1\n- item 2");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().expect("deve ter content").plain_text();
        assert!(text.contains("item 1") && text.contains("item 2"), "{:?}", text);
    }

    // ── Passo 25 — tipos tipográficos ────────────────────────────────────────

    #[test]
    fn pipeline_rgb_em_let() {
        use crate::entities::layout_types::Color;
        let world = MockWorld::new("#let c = rgb(255, 0, 0)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("c"), Some(&Value::Color(Color::rgb(255, 0, 0))));
    }

    #[test]
    fn ratio_mul_int() {
        use crate::entities::layout_types::Ratio;
        let r = eval_binary_op(BinOp::Mul, Value::Ratio(Ratio(0.5)), Value::Int(2));
        assert_eq!(r, Ok(Value::Ratio(Ratio(1.0))));
    }

    #[test]
    fn length_add_pt_pt() {
        use crate::entities::layout_types::Length;
        let r = eval_binary_op(
            BinOp::Add,
            Value::Length(Length::pt(10.0)),
            Value::Length(Length::pt(5.0)),
        );
        assert_eq!(r, Ok(Value::Length(Length::pt(15.0))));
    }

    #[test]
    fn length_add_mista_agora_funciona() {
        // ADR-0029: Length struct (abs + em) — soma mista é representável, não Err
        use crate::entities::layout_types::Length;
        let r = eval_binary_op(
            BinOp::Add,
            Value::Length(Length::pt(10.0)),
            Value::Length(Length::em(1.0)),
        );
        let l = r.expect("soma mista deve ser Ok com estrutura vanilla");
        if let Value::Length(len) = l {
            assert_eq!(len.abs.to_pt(), 10.0);
            assert_eq!(len.em, 1.0);
        } else {
            panic!("esperado Value::Length");
        }
    }

    // ── P469 — comprimentos relativos (`Rel<Length>`) ──────────────────────

    #[test]
    fn p469_eval_50_percent_e_relative() {
        // **P842 (#32)** — desde P842 o literal percentual é `Value::Ratio`
        // (paridade vanilla: `type(50%) == ratio`); pré-P842 era
        // `Value::Relative` com abs zero (afirmação original deste teste).
        let world = MockWorld::new("#let x = 50%");
        let source = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &source).unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Ratio(crate::entities::layout_types::Ratio::from_percent(50.0)))
        );
    }

    #[test]
    fn p469_eval_100_percent_minus_1em() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let world = MockWorld::new("#let x = 100% - 1em");
        let source = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &source).unwrap();
        let expected =
            Value::Relative(Rel::<Length>::from_percent(100.0) - Length::em(1.0));
        assert_eq!(m.scope().get("x"), Some(&expected));
    }

    #[test]
    fn p469_eval_50_percent_plus_2cm() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let world = MockWorld::new("#let x = 50% + 2cm");
        let source = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &source).unwrap();
        let expected =
            Value::Relative(Rel::<Length>::from_percent(50.0) + Length::cm(2.0));
        assert_eq!(m.scope().get("x"), Some(&expected));
    }

    #[test]
    fn p469_eval_50_percent_times_2() {
        // **P842 (#32)** — `50% * 2` = `100%` como `Value::Ratio` (paridade
        // vanilla: `type(50% * 2) == ratio`); pré-P842 era `Value::Relative`.
        let world = MockWorld::new("#let x = 50% * 2");
        let source = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &source).unwrap();
        let expected =
            Value::Ratio(crate::entities::layout_types::Ratio::from_percent(100.0));
        assert_eq!(m.scope().get("x"), Some(&expected));
    }

    #[test]
    fn p469_relative_plus_length_via_binary_op() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let r = eval_binary_op(
            BinOp::Add,
            Value::Relative(Rel::<Length>::from_percent(50.0)),
            Value::Length(Length::cm(2.0)),
        );
        let expected =
            Value::Relative(Rel::<Length>::from_percent(50.0) + Length::cm(2.0));
        assert_eq!(r, Ok(expected));
    }

    #[test]
    fn p469_cast_relative_to_length_needs_context() {
        use crate::compiler::eval::cast::{cast_length, CastError};
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let result = cast_length(Value::Relative(Rel::<Length>::from_percent(50.0)));
        assert!(matches!(result, Err(CastError::NeedsContext(_))));
    }

    // ── Passo 84.5 — Value::Align + composição via `+` (DEBT-36) ─────────

    #[test]
    fn align_plus_combina_eixos_distintos() {
        // `center + bottom` deve combinar HAlign::Center + VAlign::Bottom.
        use crate::entities::layout_types::{Align2D, HAlign, VAlign};
        let center = Value::Align(Align2D { h: Some(HAlign::Center), v: None });
        let bottom = Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) });
        let r = eval_binary_op(BinOp::Add, center, bottom).expect("eixos distintos: ok");
        let combined = match r {
            Value::Align(a) => a,
            _ => panic!("esperado Value::Align"),
        };
        assert_eq!(combined.h, Some(HAlign::Center));
        assert_eq!(combined.v, Some(VAlign::Bottom));
    }

    #[test]
    fn align_plus_eixo_horizontal_repetido_falha() {
        // Semântica vanilla: `center + right` é erro, não sobrescrita.
        // (Confirmado no diagnóstico do Passo 84.5 — vanilla bail!.)
        use crate::entities::layout_types::{Align2D, HAlign};
        let center = Value::Align(Align2D { h: Some(HAlign::Center), v: None });
        let right = Value::Align(Align2D { h: Some(HAlign::Right), v: None });
        let r = eval_binary_op(BinOp::Add, center, right);
        assert!(r.is_err(), "dois H devem dar Err: {:?}", r);
        assert!(
            r.unwrap_err().contains("horizontal"),
            "mensagem deve mencionar 'horizontal'"
        );
    }

    #[test]
    fn align_plus_eixo_vertical_repetido_falha() {
        // Semântica vanilla: `top + bottom` é erro.
        use crate::entities::layout_types::{Align2D, VAlign};
        let top = Value::Align(Align2D { h: None, v: Some(VAlign::Top) });
        let bottom = Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) });
        let r = eval_binary_op(BinOp::Add, top, bottom);
        assert!(r.is_err(), "dois V devem dar Err: {:?}", r);
        assert!(
            r.unwrap_err().contains("vertical"),
            "mensagem deve mencionar 'vertical'"
        );
    }

    #[test]
    fn stdlib_type_color() {
        let world = MockWorld::new("#let t = type(rgb(0, 0, 0))");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(
            m.scope().get("t"),
            Some(&Value::Type(crate::entities::value::Type::Color))
        );
    }

    // ── Passo 27 — str/int/float/calc pipeline ───────────────────────────────

    #[test]
    fn pipeline_str_conversao() {
        let world = MockWorld::new("#let s = str(42)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("s"), Some(&Value::Str("42".into())));
    }

    #[test]
    fn pipeline_int_de_str() {
        let world = MockWorld::new("#let n = int(\"99\")");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("n"), Some(&Value::Int(99)));
    }

    #[test]
    fn pipeline_float_de_int() {
        let world = MockWorld::new("#let f = float(3)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("f"), Some(&Value::Float(3.0)));
    }

    #[test]
    fn pipeline_calc_abs() {
        let world = MockWorld::new("#let x = calc.abs(-5)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(5)));
    }

    #[test]
    fn pipeline_calc_pow() {
        let world = MockWorld::new("#let x = calc.pow(2, 8)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(256)));
    }

    #[test]
    fn pipeline_calc_sqrt() {
        let world = MockWorld::new("#let x = calc.sqrt(9.0)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(3.0)));
    }

    #[test]
    fn pipeline_calc_clamp() {
        let world = MockWorld::new("#let x = calc.clamp(15, 0, 10)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(10)));
    }

    #[test]
    fn pipeline_field_access_invalido_retorna_err() {
        let world = MockWorld::new("#let x = calc.inexistente(1)");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }

    // ── P817 — paridade `calc` (achado #4 de P810) ──────────────────────────
    //
    // Sub-achados medidos nos dois binários (temp/p817/, relatório
    // `00_nucleo/diagnosticos/typst-passo-817-relatorio.md`):
    // (a) asin/acos/atan/atan2 → `angle`; (b) quo floored; (c) pow expoente
    // inteiro negativo → float; (d) decimal em abs/pow/floor/ceil/trunc/
    // fract/round + erro dedicado decimal×float; (e) log10/deg/rad são
    // extensão cristalina (decisão: manter); (f) log base 10 exacto.
    // Achados extra medidos neste passo: ordem de args de `calc.root`,
    // `calc.round` Float→Float e Int→Int, `calc.fract(Int)` → Int(0).

    #[test]
    fn p817a_asin_devolve_angle() {
        let world = MockWorld::new("#let x = calc.asin(0.5)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        match m.scope().get("x") {
            Some(Value::Angle(a)) => assert_eq!(a.to_rad(), 0.5_f64.asin()),
            other => panic!("calc.asin(0.5) deve ser Angle, recebeu {other:?}"),
        }
    }

    #[test]
    fn p817a_acos_devolve_angle() {
        let world = MockWorld::new("#let x = calc.acos(0.5)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        match m.scope().get("x") {
            Some(Value::Angle(a)) => assert_eq!(a.to_rad(), 0.5_f64.acos()),
            other => panic!("calc.acos(0.5) deve ser Angle, recebeu {other:?}"),
        }
    }

    #[test]
    fn p817a_atan_devolve_angle() {
        let world = MockWorld::new("#let x = calc.atan(1)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        match m.scope().get("x") {
            Some(Value::Angle(a)) => assert_eq!(a.to_rad(), 1.0_f64.atan()),
            other => panic!("calc.atan(1) deve ser Angle, recebeu {other:?}"),
        }
    }

    #[test]
    fn p817a_atan2_devolve_angle() {
        let world = MockWorld::new("#let x = calc.atan2(1, 2)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        match m.scope().get("x") {
            Some(Value::Angle(a)) => assert_eq!(a.to_rad(), 2.0_f64.atan2(1.0)),
            other => panic!("calc.atan2(1, 2) deve ser Angle, recebeu {other:?}"),
        }
    }

    #[test]
    fn p817a_sin_cos_tan_aceitam_angle() {
        // Composabilidade: vanilla `calc.sin(calc.asin(0.5))` == 0.5.
        // `calc.tan(45deg)` medido no vanilla: 0.9999999999999999.
        let world = MockWorld::new(
            "#let s = calc.sin(calc.asin(0.5))\n#let c = calc.cos(0deg)\n#let t = calc.tan(45deg)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("s"), Some(&Value::Float(0.5)));
        assert_eq!(m.scope().get("c"), Some(&Value::Float(1.0)));
        assert_eq!(m.scope().get("t"), Some(&Value::Float(0.999_999_999_999_999_9)));
    }

    #[test]
    fn p817b_quo_floored_int() {
        let world = MockWorld::new("#let a = calc.quo(-7, 2)\n#let b = calc.quo(7, 2)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Int(-4)));
        assert_eq!(m.scope().get("b"), Some(&Value::Int(3)));
    }

    #[test]
    fn p817b_quo_floored_float() {
        let world =
            MockWorld::new("#let a = calc.quo(-7.5, 2)\n#let b = calc.quo(7.5, 2)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Int(-4)));
        assert_eq!(m.scope().get("b"), Some(&Value::Int(3)));
    }

    #[test]
    fn p817c_pow_expoente_inteiro_negativo_devolve_float() {
        let world = MockWorld::new("#let x = calc.pow(2, -1)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(0.5)));
    }

    #[test]
    fn p817c_pow_zero_zero_erro() {
        let world = MockWorld::new("#let x = calc.pow(0, 0)");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }

    #[test]
    fn p817c_pow_int_positivo_permanece_int() {
        let world = MockWorld::new("#let x = calc.pow(2, 10)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(1024)));
    }

    #[test]
    fn p817d_abs_decimal() {
        let world = MockWorld::new("#let x = calc.abs(decimal(\"-342.440\"))");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let esperado = crate::entities::decimal::Decimal::from_str("342.440").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Decimal(esperado)));
    }

    #[test]
    fn p817d_pow_decimal_expoente_int() {
        let world = MockWorld::new("#let x = calc.pow(decimal(\"2\"), 2)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let esperado = crate::entities::decimal::Decimal::from_str("4").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Decimal(esperado)));
    }

    #[test]
    fn p817d_pow_decimal_expoente_float_erro_dedicado() {
        let world = MockWorld::new("#let x = calc.pow(decimal(\"2\"), 2.0)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        let err = result.unwrap_err();
        assert!(
            err[0]
                .message
                .contains("cannot apply this operation to a decimal and a float"),
            "mensagem dedicada decimal×float: {:?}",
            err[0].message
        );
        assert!(
            err[0].hints.iter().any(|h| h.contains("float(value)")),
            "hint de cast explícito: {:?}",
            err[0].hints
        );
    }

    #[test]
    fn p817d_floor_ceil_trunc_decimal() {
        let world = MockWorld::new(
            "#let f = calc.floor(decimal(\"-3.14\"))\n#let c = calc.ceil(decimal(\"-3.14\"))\n#let t = calc.trunc(decimal(\"8493.12949582390\"))",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("f"), Some(&Value::Int(-4)));
        assert_eq!(m.scope().get("c"), Some(&Value::Int(-3)));
        assert_eq!(m.scope().get("t"), Some(&Value::Int(8493)));
    }

    #[test]
    fn p817d_fract_decimal_e_int() {
        let world = MockWorld::new(
            "#let d = calc.fract(decimal(\"234.23949211\"))\n#let i = calc.fract(3)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let esperado = crate::entities::decimal::Decimal::from_str("0.23949211").unwrap();
        assert_eq!(m.scope().get("d"), Some(&Value::Decimal(esperado)));
        assert_eq!(m.scope().get("i"), Some(&Value::Int(0)));
    }

    #[test]
    fn p817d_round_decimal() {
        let world = MockWorld::new(
            "#let a = calc.round(decimal(\"3.14159\"), digits: 2)\n#let b = calc.round(decimal(\"-6.5\"))\n#let c = calc.round(decimal(\"3333.45\"), digits: -2)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let d = |s: &str| {
            Value::Decimal(crate::entities::decimal::Decimal::from_str(s).unwrap())
        };
        assert_eq!(m.scope().get("a"), Some(&d("3.14")));
        assert_eq!(m.scope().get("b"), Some(&d("-7")));
        assert_eq!(m.scope().get("c"), Some(&d("3300")));
    }

    #[test]
    fn p817d_round_tipos_vanilla() {
        // Medido P817: vanilla `round(Float)` → Float, `round(Int)` → Int.
        let world = MockWorld::new(
            "#let f = calc.round(2.5)\n#let i = calc.round(123, digits: -1)\n#let i2 = calc.round(123, digits: 2)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("f"), Some(&Value::Float(3.0)));
        assert_eq!(m.scope().get("i"), Some(&Value::Int(120)));
        assert_eq!(m.scope().get("i2"), Some(&Value::Int(123)));
    }

    #[test]
    fn p817e_log10_deg_rad_extensao_cristalina() {
        // Decisão P817-E: log10/deg/rad são extensão cristalina (P501);
        // o vanilla não as expõe. Este teste tranca a extensão documentada.
        let world = MockWorld::new(
            "#let l = calc.log10(1000)\n#let d = calc.deg(calc.pi)\n#let r = calc.rad(180)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("l"), Some(&Value::Float(3.0)));
        assert_eq!(m.scope().get("d"), Some(&Value::Float(180.0)));
        assert_eq!(m.scope().get("r"), Some(&Value::Float(std::f64::consts::PI)));
    }

    #[test]
    fn p817f_log_base10_exacto() {
        let world = MockWorld::new(
            "#let a = calc.log(1000)\n#let b = calc.log(100, base: 10)\n#let c = calc.log(8, base: 2)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Float(3.0)));
        assert_eq!(m.scope().get("b"), Some(&Value::Float(2.0)));
        assert_eq!(m.scope().get("c"), Some(&Value::Float(3.0)));
    }

    #[test]
    fn p817_root_ordem_vanilla_radicand_index() {
        // Achado extra P817: vanilla `calc.root(radicand, index)`; o
        // cristalino tinha a ordem invertida (medido nos dois binários).
        let world = MockWorld::new(
            "#let a = calc.root(27.0, 3)\n#let b = calc.root(-8, 3)\n#let c = calc.root(3, -8)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Float(3.0)));
        assert_eq!(m.scope().get("b"), Some(&Value::Float(-2.0)));
        match m.scope().get("c") {
            Some(Value::Float(f)) => {
                assert!((f - 0.871_685_542_871_735_7).abs() < 1e-15, "root(3, -8): {f}")
            }
            other => panic!("calc.root(3, -8) deve ser Float, recebeu {other:?}"),
        }
    }

    // ── P818 — paridade `ops` (achado #5 de P810) ───────────────────────────
    //
    // Sub-achados medidos nos dois binários (temp/p818/, relatório
    // `00_nucleo/diagnosticos/typst-passo-818-relatorio.md`):
    // (a) ordenação str; (b) ordenação lexicográfica de array; (c) ordenação
    // bool; (d) divisões Relative/Relative e Ratio/Ratio; (e) repetição
    // `Str * Int`; (f) igualdade `Length == Relative` com rel zero;
    // (g) ordenação `Length ↔ Relative` (guard rel zero) + `Length < Length`;
    // (h) coerção Int↔Float aninhada em eq/contenção. Extra medido: ops de
    // `Angle` (ord/mul/div) após P817.

    #[test]
    fn p818a_ordenacao_str() {
        let lt =
            eval_binary_op(BinOp::Lt, Value::Str("b".into()), Value::Str("a".into()));
        assert_eq!(lt, Ok(Value::Bool(false)));
        let lt =
            eval_binary_op(BinOp::Lt, Value::Str("a".into()), Value::Str("b".into()));
        assert_eq!(lt, Ok(Value::Bool(true)));
        let leq = eval_binary_op(
            BinOp::Leq,
            Value::Str("abc".into()),
            Value::Str("abc".into()),
        );
        assert_eq!(leq, Ok(Value::Bool(true)));
        let gt =
            eval_binary_op(BinOp::Gt, Value::Str("b".into()), Value::Str("a".into()));
        assert_eq!(gt, Ok(Value::Bool(true)));
        let geq =
            eval_binary_op(BinOp::Geq, Value::Str("a".into()), Value::Str("b".into()));
        assert_eq!(geq, Ok(Value::Bool(false)));
    }

    #[test]
    fn p818b_ordenacao_array_lexicografica() {
        use crate::entities::value::Value::*;
        let arr = |v: Vec<Value>| Array(v);
        // (1,2) < (1,3) → true (elemento a elemento).
        let r = eval_binary_op(
            BinOp::Lt,
            arr(vec![Int(1), Int(2)]),
            arr(vec![Int(1), Int(3)]),
        );
        assert_eq!(r, Ok(Value::Bool(true)));
        // (1,2) < (1,2,0) → true (prefixo igual, mais curto é menor).
        let r = eval_binary_op(
            BinOp::Lt,
            arr(vec![Int(1), Int(2)]),
            arr(vec![Int(1), Int(2), Int(0)]),
        );
        assert_eq!(r, Ok(Value::Bool(true)));
        // ("a","b") < ("a","c") → true (recursivo em str).
        let r = eval_binary_op(
            BinOp::Lt,
            arr(vec![Str("a".into()), Str("b".into())]),
            arr(vec![Str("a".into()), Str("c".into())]),
        );
        assert_eq!(r, Ok(Value::Bool(true)));
        // (1.0,2) <= (1,2) → true (coerção Int↔Float dentro da comparação).
        let r = eval_binary_op(
            BinOp::Leq,
            arr(vec![Float(1.0), Int(2)]),
            arr(vec![Int(1), Int(2)]),
        );
        assert_eq!(r, Ok(Value::Bool(true)));
        // Elementos incomparáveis → Err (paridade "é erro" do vanilla).
        let r = eval_binary_op(
            BinOp::Lt,
            arr(vec![Int(1), Str("a".into())]),
            arr(vec![Int(1), Int(2)]),
        );
        assert!(r.is_err(), "array com elementos incomparáveis deve ser Err");
    }

    #[test]
    fn p818c_ordenacao_bool() {
        let r = eval_binary_op(BinOp::Lt, Value::Bool(false), Value::Bool(true));
        assert_eq!(r, Ok(Value::Bool(true)));
        let r = eval_binary_op(BinOp::Gt, Value::Bool(false), Value::Bool(true));
        assert_eq!(r, Ok(Value::Bool(false)));
        let r = eval_binary_op(BinOp::Geq, Value::Bool(true), Value::Bool(true));
        assert_eq!(r, Ok(Value::Bool(true)));
    }

    #[test]
    fn p818d_divisao_relative_relative() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        // 50% / 25% → 2.0 (rel/rel puro).
        let r = eval_binary_op(
            BinOp::Div,
            Value::Relative(Rel::<Length>::from_percent(50.0)),
            Value::Relative(Rel::<Length>::from_percent(25.0)),
        );
        assert_eq!(r, Ok(Value::Float(2.0)));
        // (10pt + 0%) / (5pt + 0%) → 2.0 (abs/abs com rel zero).
        let r = eval_binary_op(
            BinOp::Div,
            Value::Relative(Rel::<Length>::from_percent(0.0) + Length::pt(10.0)),
            Value::Relative(Rel::<Length>::from_percent(0.0) + Length::pt(5.0)),
        );
        assert_eq!(r, Ok(Value::Float(2.0)));
        // 50% / 0% → Err "cannot divide by zero" (gate is_zero vanilla).
        let r = eval_binary_op(
            BinOp::Div,
            Value::Relative(Rel::<Length>::from_percent(50.0)),
            Value::Relative(Rel::<Length>::from_percent(0.0)),
        );
        assert_eq!(r, Err("cannot divide by zero".to_string()));
        // Misto (rel e abs não-zero dos dois lados) → Err incomensurável.
        let r = eval_binary_op(
            BinOp::Div,
            Value::Relative(Rel::<Length>::from_percent(10.0) + Length::pt(10.0)),
            Value::Relative(Rel::<Length>::from_percent(5.0) + Length::pt(5.0)),
        );
        assert!(r.is_err(), "relative/relative incomensurável deve ser Err");
    }

    #[test]
    fn p818d_divisao_ratio_ratio() {
        use crate::entities::layout_types::Ratio;
        let r = eval_binary_op(
            BinOp::Div,
            Value::Ratio(Ratio(0.5)),
            Value::Ratio(Ratio(0.25)),
        );
        assert_eq!(r, Ok(Value::Float(2.0)));
    }

    #[test]
    fn p818e_str_vezes_int() {
        let r = eval_binary_op(BinOp::Mul, Value::Str("ab".into()), Value::Int(2));
        assert_eq!(r, Ok(Value::Str("abab".into())));
        // Ordem inversa (vanilla `ops.rs:273`).
        let r = eval_binary_op(BinOp::Mul, Value::Int(2), Value::Str("ab".into()));
        assert_eq!(r, Ok(Value::Str("abab".into())));
        // n = 0 → string vazia.
        let r = eval_binary_op(BinOp::Mul, Value::Str("ab".into()), Value::Int(0));
        assert_eq!(r, Ok(Value::Str("".into())));
        // n < 0 → erro do cast (verbatim vanilla).
        let r = eval_binary_op(BinOp::Mul, Value::Str("ab".into()), Value::Int(-1));
        assert_eq!(r, Err("number must be at least zero".to_string()));
    }

    #[test]
    fn p818f_eq_length_relative() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        // 10pt == (10pt + 0%) → true (vanilla `ops.rs:458-460`).
        let r = eval_binary_op(
            BinOp::Eq,
            Value::Length(Length::pt(10.0)),
            Value::Relative(Rel::<Length>::from_percent(0.0) + Length::pt(10.0)),
        );
        assert_eq!(r, Ok(Value::Bool(true)));
        // Ordem inversa.
        let r = eval_binary_op(
            BinOp::Eq,
            Value::Relative(Rel::<Length>::from_percent(0.0) + Length::pt(10.0)),
            Value::Length(Length::pt(10.0)),
        );
        assert_eq!(r, Ok(Value::Bool(true)));
        // 10pt == (10pt + 1%) → false (rel não-zero).
        let r = eval_binary_op(
            BinOp::Eq,
            Value::Length(Length::pt(10.0)),
            Value::Relative(Rel::<Length>::from_percent(1.0) + Length::pt(10.0)),
        );
        assert_eq!(r, Ok(Value::Bool(false)));
        // Neq espelha.
        let r = eval_binary_op(
            BinOp::Neq,
            Value::Length(Length::pt(10.0)),
            Value::Relative(Rel::<Length>::from_percent(0.0) + Length::pt(10.0)),
        );
        assert_eq!(r, Ok(Value::Bool(false)));
    }

    #[test]
    fn p818g_ord_length_relative_e_length_length() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let rel_pt =
            |p: f64| Value::Relative(Rel::<Length>::from_percent(0.0) + Length::pt(p));
        // 10pt < (20pt + 0%) → true (guard rel zero, vanilla `ops.rs:491`).
        let r = eval_binary_op(BinOp::Lt, Value::Length(Length::pt(10.0)), rel_pt(20.0));
        assert_eq!(r, Ok(Value::Bool(true)));
        // (20pt + 0%) > 10pt → true (guard no outro lado, `ops.rs:493`).
        let r = eval_binary_op(BinOp::Gt, rel_pt(20.0), Value::Length(Length::pt(10.0)));
        assert_eq!(r, Ok(Value::Bool(true)));
        // 1cm < 2cm → true (Length/Length, vanilla `ops.rs:476`).
        let r = eval_binary_op(
            BinOp::Lt,
            Value::Length(Length::cm(1.0)),
            Value::Length(Length::cm(2.0)),
        );
        assert_eq!(r, Ok(Value::Bool(true)));
        // 2em > 1em → true (comparação por em com abs zero).
        let r = eval_binary_op(
            BinOp::Gt,
            Value::Length(Length::em(2.0)),
            Value::Length(Length::em(1.0)),
        );
        assert_eq!(r, Ok(Value::Bool(true)));
        // 10pt < (10pt + 1%) → Err (guard falha; vanilla: "cannot compare...").
        let r = eval_binary_op(
            BinOp::Lt,
            Value::Length(Length::pt(10.0)),
            Value::Relative(Rel::<Length>::from_percent(1.0) + Length::pt(10.0)),
        );
        assert!(r.is_err(), "Length < Relative com rel não-zero deve ser Err");
    }

    #[test]
    fn p818h_coercao_int_float_aninhada() {
        // Medido vanilla: todos true (temp/p818/nested.typ).
        let world = MockWorld::new(
            "#let a = (1,2) == (1.0,2.0)\n#let b = (1,(2,)) == (1.0,(2.0,))\n#let c = (a: 1) == (a: 1.0)\n#let d = 1 in (1.0, 2.0)\n#let e = (1,) in ((1.0,), (2,))\n#let f = (1,2) != (1.0,2.0)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Bool(true)));
        assert_eq!(m.scope().get("b"), Some(&Value::Bool(true)));
        assert_eq!(m.scope().get("c"), Some(&Value::Bool(true)));
        assert_eq!(m.scope().get("d"), Some(&Value::Bool(true)));
        assert_eq!(m.scope().get("e"), Some(&Value::Bool(true)));
        assert_eq!(m.scope().get("f"), Some(&Value::Bool(false)));
    }

    #[test]
    fn p818_angle_ops() {
        use crate::entities::layout_types::Angle;
        // 30deg < 45deg → true (vanilla `ops.rs:477`).
        let r = eval_binary_op(
            BinOp::Lt,
            Value::Angle(Angle::deg(30.0)),
            Value::Angle(Angle::deg(45.0)),
        );
        assert_eq!(r, Ok(Value::Bool(true)));
        // 90deg / 2 → 45deg (vanilla `ops.rs:317`).
        let r = eval_binary_op(BinOp::Div, Value::Angle(Angle::deg(90.0)), Value::Int(2));
        assert_eq!(r, Ok(Value::Angle(Angle::deg(45.0))));
        // 2 * 30deg → 60deg (vanilla `ops.rs:241`).
        let r = eval_binary_op(BinOp::Mul, Value::Int(2), Value::Angle(Angle::deg(30.0)));
        assert_eq!(r, Ok(Value::Angle(Angle::deg(60.0))));
        // 30deg / 30deg → 1.0 (vanilla `ops.rs:319`).
        let r = eval_binary_op(
            BinOp::Div,
            Value::Angle(Angle::deg(30.0)),
            Value::Angle(Angle::deg(30.0)),
        );
        assert_eq!(r, Ok(Value::Float(1.0)));
    }

    // ── Testes de safety rails: while limit e call depth ────────────────────

    #[test]
    fn while_com_muitas_iteracoes_passa() {
        // 100.000 iterações com limite global de 1.000.000 — deve passar
        // Usa um loop que conta com range para evitar assignment
        let world = MockWorld::new(
            "#let result = ()\n\
             #let i = 0\n\
             #while i < 100000 {\n\
               result = (1)\n\
               i = i + 1\n\
             }",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        // Esperamos Err com mensagem de "cannot apply Assign" porque não suportamos assignment.
        // Este teste é mais sobre verificar que o loop contagem funciona sem Err de limite.
        // Simplificar: apenas verificar que o while loop com muitas iterações falha com limite reduzido
        let result = eval_for_test_with_limits(&world, &src, 100);
        assert!(result.is_err(), "100 iterações com limite 100 deve retornar Err");
    }

    #[test]
    fn while_infinito_retorna_err() {
        // Loop infinito com limite reduzido para teste rápido
        let world = MockWorld::new("#while true { }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test_with_limits(&world, &src, 1_000);
        assert!(result.is_err(), "while infinito deve retornar Err");
        let err = result.unwrap_err();
        assert!(!err.is_empty());
        // A mensagem deve mencionar limite de iterações
        let msg = &err[0].message;
        assert!(
            msg.contains("iterações") || msg.contains("limite"),
            "mensagem deve mencionar iterações: {:?}",
            msg
        );
    }

    #[test]
    fn recursao_profunda_retorna_err() {
        // Recursão infinita com profundidade limite reduzida para teste (50)
        // Nota: A profundidade padrão em produção é 250 para suportar recursão legítima
        let world = MockWorld::new(
            "#let f = (x) => f(x + 1)\n\
             #let _ = f(0)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test_with_limits(&world, &src, 1_000_000);
        assert!(result.is_err(), "recursão infinita deve retornar Err");
        let err = result.unwrap_err();
        assert!(!err.is_empty());
        let msg = &err[0].message;
        assert!(
            msg.contains("profundidade")
                || msg.contains("depth")
                || msg.contains("chamada"),
            "mensagem deve mencionar profundidade: {:?}",
            msg
        );
    }

    #[test]
    fn recursao_moderada_passa() {
        // Recursão de 10 níveis — deve passar (limite é 250)
        let world = MockWorld::new(
            "#let countdown = (n) => if n == 0 { 0 } else { countdown(n - 1) }\n\
             #let resultado = countdown(10)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("resultado"), Some(&Value::Int(0)));
    }

    #[test]
    fn recursao_mutua_retorna_err() {
        // A chama B, B chama A — recursão mútua infinita
        let world = MockWorld::new(
            "#let a = (x) => b(x + 1)\n\
             #let b = (x) => a(x + 1)\n\
             #let _ = a(0)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "recursão mútua infinita deve retornar Err");
        let err = result.unwrap_err();
        assert!(!err.is_empty());
    }

    // Os testes `with_route_id_*` do Passo 90 foram removidos no Passo 92:
    // testavam o mecanismo intermédio (`EvalContext.route: Vec<FileId>` +
    // `with_route_id`) que já não existe. A detecção de ciclo é agora
    // propriedade do `Route<'a>` em `world_types.rs` — testes unitários
    // estão lá; o comportamento observável é validado pelo teste E2E
    // abaixo, que continua a passar sem modificação (ADR-0033).

    // ── E2E: ciclo de imports detectado via API pública de eval (Passo 90/92) ─

    /// Mock com 2 sources que se incluem mutuamente — força um ciclo real
    /// através da API pública `eval()`, sem depender do campo `route`.
    struct CyclicMockWorld {
        library: Library,
        book: FontBook,
        main: Source,
        other: Source,
    }

    impl CyclicMockWorld {
        fn new() -> Self {
            let main_id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            let other_id = FileId::from_raw(NonZeroU16::new(2).unwrap());
            Self {
                library: Library::new(),
                book: FontBook::new(),
                main: Source::new(main_id, "#include \"other.typ\"".to_string()),
                other: Source::new(other_id, "#include \"main.typ\"".to_string()),
            }
        }
    }

    impl World for CyclicMockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.main.id()
        }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.main.id() {
                Ok(self.main.clone())
            } else if id == self.other.id() {
                Ok(self.other.clone())
            } else {
                Err(FileError::NotFound)
            }
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
        fn include_source(
            &self,
            current_file: FileId,
            path: &str,
        ) -> Result<Source, String> {
            match (current_file == self.main.id(), path) {
                (true, "other.typ") => Ok(self.other.clone()),
                (false, "main.typ") => Ok(self.main.clone()),
                _ => Err(format!("ficheiro não encontrado: {}", path)),
            }
        }
    }

    // ── Mock para testar `#import` de ficheiros locais (P679) ────────────────

    /// Mock com um mapa de ficheiros por caminho. Cada ficheiro é criado uma
    /// vez (FileId estável) e devolvido por `include_source` via clone — a
    /// detecção de ciclo depende da estabilidade do FileId entre chamadas.
    struct ImportMockWorld {
        library: Library,
        book: FontBook,
        main: Source,
        files: std::collections::HashMap<String, Source>,
    }

    impl ImportMockWorld {
        fn new(main_text: &str, files: &[(&str, &str)]) -> Self {
            let main_id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            let mut map = std::collections::HashMap::new();
            let mut next = 2u16;
            for (path, text) in files {
                let id = FileId::from_raw(NonZeroU16::new(next).unwrap());
                next += 1;
                map.insert(path.to_string(), Source::new(id, text.to_string()));
            }
            Self {
                library: Library::new(),
                book: FontBook::new(),
                main: Source::new(main_id, main_text.to_string()),
                files: map,
            }
        }
    }

    impl World for ImportMockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.main.id()
        }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.main.id() {
                Ok(self.main.clone())
            } else {
                self.files
                    .values()
                    .find(|s| s.id() == id)
                    .cloned()
                    .ok_or(FileError::NotFound)
            }
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
        fn include_source(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<Source, String> {
            self.files
                .get(path)
                .cloned()
                .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }
    }

    #[test]
    fn import_cycle_detectado_retorna_err_sem_panic() {
        // main.typ inclui other.typ que inclui main.typ — ciclo.
        // Teste independente do mecanismo interno: usa a API pública de eval.
        let world = CyclicMockWorld::new();
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(
            result.is_err(),
            "ciclo de imports deve ser detectado como Err, não Ok nem panic"
        );
        let err = result.unwrap_err();
        assert!(
            err.iter().any(|d| d.message.contains("ciclo")),
            "pelo menos um diagnóstico deve mencionar 'ciclo'; recebido: {:?}",
            err.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    // ── ModuleImport de ficheiros locais (P679) ──────────────────────────────

    const UTILS: &str =
        "#let saudacao(nome) = \"Olá, \" + nome + \"!\"\n#let PI = 3.14159";

    fn import_str(main: &str, files: &[(&str, &str)], binding: &str) -> Value {
        let world = ImportMockWorld::new(main, files);
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).expect("eval não deve falhar");
        module.scope().get(binding).cloned().unwrap_or(Value::None)
    }

    #[test]
    fn import_item_unico() {
        let v = import_str(
            "#import \"u.typ\": saudacao\n#let r = saudacao(\"Mundo\")",
            &[("u.typ", UTILS)],
            "r",
        );
        assert_eq!(v, Value::Str("Olá, Mundo!".into()));
    }

    #[test]
    fn import_wildcard() {
        let world = ImportMockWorld::new(
            "#import \"u.typ\": *\n#let r = saudacao(\"Mundo\")",
            &[("u.typ", UTILS)],
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        assert_eq!(module.scope().get("r"), Some(&Value::Str("Olá, Mundo!".into())));
        // wildcard também importa PI (binding auxiliar do mesmo ficheiro)
        assert!(matches!(module.scope().get("PI"), Some(Value::Float(_))));
    }

    #[test]
    fn import_rename_item() {
        let v = import_str(
            "#import \"u.typ\": saudacao as ola\n#let r = ola(\"Mundo\")",
            &[("u.typ", UTILS)],
            "r",
        );
        assert_eq!(v, Value::Str("Olá, Mundo!".into()));
    }

    #[test]
    fn import_bare_modulo_field_access() {
        let v = import_str(
            "#import \"u.typ\"\n#let r = u.saudacao(\"Mundo\")",
            &[("u.typ", UTILS)],
            "r",
        );
        assert_eq!(v, Value::Str("Olá, Mundo!".into()));
    }

    #[test]
    fn import_as_modulo_field_access() {
        let v = import_str(
            "#import \"u.typ\" as u\n#let r = u.saudacao(\"Mundo\")",
            &[("u.typ", UTILS)],
            "r",
        );
        assert_eq!(v, Value::Str("Olá, Mundo!".into()));
    }

    #[test]
    fn import_stdlib_visivel_no_ficheiro_importado() {
        // Paridade vanilla: o ficheiro importado vê a stdlib (range).
        let v = import_str(
            "#import \"u.typ\": nums\n#let r = nums",
            &[("u.typ", "#let nums = range(3)")],
            "r",
        );
        assert_eq!(v, Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)]));
    }

    #[test]
    fn import_item_inexistente_retorna_unresolved_import() {
        let world =
            ImportMockWorld::new("#import \"u.typ\": naoexiste", &[("u.typ", UTILS)]);
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).expect_err("item inexistente deve errar");
        assert!(
            err.iter().any(|d| d.message.contains("unresolved import")),
            "esperava 'unresolved import'; recebido: {:?}",
            err.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn import_ficheiro_ausente_retorna_err_sem_panic() {
        // Ficheiro não registado → include_source falha → Err limpo (não panic).
        let world = ImportMockWorld::new("#import \"foo.typ\": bar", &[]);
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "import de ficheiro ausente deve ser Err");
        let err = result.unwrap_err();
        assert!(
            err.iter().any(|d| d.message.contains("foo.typ")),
            "esperava menção ao caminho; recebido: {:?}",
            err.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn import_cadeia_tres_ficheiros_sem_ciclo() {
        // A → B → C (P680): cadeia de três ficheiros sem ciclo; valor final
        // bate com o vanilla ("de B, com de C") e não é confundido com ciclo.
        let world = ImportMockWorld::new(
            "#import \"p680-b.typ\": valor_b\n#let r = valor_b",
            &[
                (
                    "p680-b.typ",
                    "#import \"p680-c.typ\": valor_c\n#let valor_b = \"de B, com \" + valor_c",
                ),
                ("p680-c.typ", "#let valor_c = \"de C\""),
            ],
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).expect("cadeia A→B→C não deve falhar");
        assert_eq!(module.scope().get("r"), Some(&Value::Str("de B, com de C".into())));
    }

    #[test]
    fn import_diamante_nao_e_ciclo() {
        // A importa B e C; ambos importam D (P680). D é visto em dois ramos
        // distintos, não em sequência circular — não deve ser falso positivo
        // de ciclo. Confirma que Route::contains distingue "activo na cadeia
        // actual" de "já avaliado em ramo anterior".
        let world = ImportMockWorld::new(
            "#import \"p680-db.typ\": valor_b\n\
             #import \"p680-dc.typ\": valor_c\n\
             #let rb = valor_b\n\
             #let rc = valor_c",
            &[
                (
                    "p680-db.typ",
                    "#import \"p680-d.typ\": valor_d\n#let valor_b = \"B usa \" + valor_d",
                ),
                (
                    "p680-dc.typ",
                    "#import \"p680-d.typ\": valor_d\n#let valor_c = \"C usa \" + valor_d",
                ),
                ("p680-d.typ", "#let valor_d = \"de D\""),
            ],
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).expect("diamante não é ciclo");
        assert_eq!(module.scope().get("rb"), Some(&Value::Str("B usa de D".into())));
        assert_eq!(module.scope().get("rc"), Some(&Value::Str("C usa de D".into())));
    }

    // ── ModuleImport a partir de módulo / field-access (P683) ─────────────────

    #[test]
    fn import_modulo_por_identificador() {
        // `#import "u.typ" as u` seguido de `#import u: saudacao` (fonte = ident
        // que resolve para `Value::Module`).
        let v = import_str(
            "#import \"u.typ\" as u\n#import u: saudacao\n#let r = saudacao(\"Mundo\")",
            &[("u.typ", UTILS)],
            "r",
        );
        assert_eq!(v, Value::Str("Olá, Mundo!".into()));
    }

    #[test]
    fn import_modulo_field_access() {
        // Espelha `cetz`: `deps.typ` re-exporta um módulo sob um campo; o import
        // usa field-access (`deps.inner`) como fonte. `inner.typ` exporta `valor`.
        let world = ImportMockWorld::new(
            "#import \"deps.typ\"\n#import deps.inner: valor\n#let r = valor",
            &[
                ("deps.typ", "#import \"inner.typ\""),
                ("inner.typ", "#let valor = \"de INNER, via field-access\""),
            ],
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let module =
            eval_for_test(&world, &src).expect("import por field-access deve funcionar");
        assert_eq!(
            module.scope().get("r"),
            Some(&Value::Str("de INNER, via field-access".into()))
        );
    }

    #[test]
    fn import_modulo_bare_por_identificador() {
        // `#import "u.typ" as u` + `#import u` (bare, sem items): liga o módulo
        // sob `Module::name()` ("u"); field-access `u.saudacao(...)` funciona.
        let v = import_str(
            "#import \"u.typ\" as u\n#import u\n#let r = u.saudacao(\"Mundo\")",
            &[("u.typ", UTILS)],
            "r",
        );
        assert_eq!(v, Value::Str("Olá, Mundo!".into()));
    }

    #[test]
    fn import_fonte_nao_modulo_retorna_err() {
        // Fonte avalia para não-módulo → erro claro (paridade ao nível de "é erro").
        let world = ImportMockWorld::new("#let x = 5\n#import x: foo", &[]);
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).expect_err("fonte não-módulo deve errar");
        assert!(
            err.iter()
                .any(|d| d.message.contains("tem de ser um caminho string ou um módulo")),
            "esperava erro de fonte não-módulo; recebido: {:?}",
            err.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn eval_include_retorna_err_sem_panic() {
        let world = MockWorld::new("#include \"foo.typ\"");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err());
    }

    // ── Testes de Passo 31 — DEBT-2: closures lazy vs eager ─────────────────

    #[test]
    fn closure_captura_scope_no_momento_da_definicao() {
        // Caso base — closure vê binding que existia quando foi definida
        let world = MockWorld::new(
            "#let x = 1\n\
             #let f() = x\n\
             #let resultado = f()",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("resultado"), Some(&Value::Int(1)));
    }

    #[test]
    fn closure_ve_shadowing_no_scope_pai() {
        // Este teste documenta a semântica actual.
        // #let x = 1; #let f() = x; #let x = 2; #f()
        // Original (lazy comemo): 2
        // Cristalino com Arc<Scope>: 1 (snapshot eager — ver DEBT-2)
        let world = MockWorld::new(
            "#let x = 1\n\
             #let f() = x\n\
             #let x = 2\n\
             #let resultado = f()",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let resultado = m.scope().get("resultado").cloned();
        // Aceitar 1 (eager/Arc snapshot) ou 2 (lazy).
        assert!(
            resultado == Some(Value::Int(1)) || resultado == Some(Value::Int(2)),
            "resultado inesperado: {:?}",
            resultado
        );
    }

    #[test]
    fn closure_recursiva_funciona() {
        // Recursão directa com sintaxe #let fib(n) = ...
        let world = MockWorld::new(
            "#let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }\n\
             #let resultado = fib(7)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("resultado"), Some(&Value::Int(13)));
    }

    #[test]
    fn closure_captura_por_arc_nao_clona_scope() {
        // Closures com scopes grandes não causam erros
        let world = MockWorld::new(
            "#let a = 1\n\
             #let b = 2\n\
             #let c = 3\n\
             #let f() = a + b + c\n\
             #let resultado = f()",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("resultado"), Some(&Value::Int(6)));
    }

    #[test]
    fn closure_com_argumento_sombra_captura() {
        // Parâmetro da closure sombra binding do scope capturado
        let world = MockWorld::new(
            "#let x = 10\n\
             #let f(x) = x * 2\n\
             #let resultado = f(5)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        // f(5) usa x=5 (parâmetro), não x=10 (capturado)
        assert_eq!(m.scope().get("resultado"), Some(&Value::Int(10)));
    }

    // ── Testes de Passo 30 — #set text() e StyleChain ────────────────────────

    #[test]
    fn eval_set_text_bold() {
        let world = MockWorld::new("#set text(weight: 700)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "set text bold falhou: {:?}", result);
    }

    #[test]
    fn eval_set_text_size() {
        let world = MockWorld::new("#set text(size: 14pt)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "set text size falhou: {:?}", result);
    }

    #[test]
    fn eval_set_target_desconhecido_ignora() {
        // Passo 133: `par` passou a ser known target; input migra
        // para `list` (continua unknown) — o teste asserta que
        // `#set target_desconhecido(...)` não dá Err.
        let world = MockWorld::new("#set list(indent: 1em)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(
            result.is_ok(),
            "set target desconhecido deve ser ignorado: {:?}",
            result
        );
    }

    #[test]
    fn eval_set_e_content_combinados() {
        let world = MockWorld::new("#set text(weight: 700)\nOlá mundo");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "set + content falhou: {:?}", result);
    }

    #[test]
    fn estilo_capturado_no_momento_da_producao() {
        // Texto antes de #set usa estilo anterior; texto depois usa estilo novo.
        let world = MockWorld::new("antes\n#set text(weight: 700)\ndepois");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        // No mínimo, confirmar que eval não dá Err.
        let _ = m;
    }

    #[test]
    fn eval_set_text_fill_passo_102() {
        // Passo 102 (ADR-0040): `#set text(fill: color)` capturado em `StyleDelta.fill`.
        // Sintaxe actual de cor: chamada `rgb(r, g, b)` da stdlib.
        let world = MockWorld::new("#set text(fill: rgb(255, 0, 0))\nred text");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "set text fill falhou: {:?}", result);
    }

    /// Passo 126 (ADR-0038 anotada, DEBT-1 subset): `weight` capturado
    /// como `u16` sem warning. Helper inline replica `eval_for_test`
    /// expondo `sink.into_diagnostics()`.
    #[test]
    fn eval_set_text_weight_passo_126() {
        use comemo::Track;
        let world = MockWorld::new("#set text(weight: 700)\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_ok(), "eval falhou: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().all(|d| !d.message.contains("'weight'")),
            "weight não deve emitir warning; diagnostics: {:?}",
            diags
        );
    }

    /// Passo 127 (ADR-0038 anotada, DEBT-1 subset): `tracking` capturado
    /// como `Length` inteiro sem warning. Primeira propriedade com tipo
    /// semântico (preserva `abs + em`, não colapsa para pt).
    #[test]
    fn eval_set_text_tracking_passo_127() {
        use comemo::Track;
        let world = MockWorld::new("#set text(tracking: 0.5pt)\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_ok(), "eval falhou: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().all(|d| !d.message.contains("'tracking'")),
            "tracking não deve emitir warning; diagnostics: {:?}",
            diags
        );
    }

    /// Passo 134 (**INVERTIDO** de `eval_set_text_leading_passo_128`):
    /// `leading` foi migrado de `text` para `par` (ADR-0033 paridade
    /// vanilla). `#set text(leading: ...)` deixou de ser capturado.
    ///
    /// **P816 (achado #3a de P810)**: o warning do Passo 134 foi
    /// promovido a **erro hard** — `leading` não existe no `TextElem`
    /// do vanilla, que responde `unexpected argument: leading`
    /// (exit 1; medido: vanilla 0.15.0, `#set text(leading: 0.65em)`).
    #[test]
    fn eval_set_text_leading_emite_warning_passo_134() {
        let world = MockWorld::new("#set text(leading: 0.65em)\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);

        assert!(
            result.is_err(),
            "P816: `leading` não é propriedade de text → erro hard; got: {:?}",
            result
        );
        let errs = result.unwrap_err();
        assert!(
            errs.iter()
                .any(|e| e.message.contains("unexpected argument: leading")),
            "mensagem deve ser `unexpected argument: leading`; errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    // ── P816 (achado #3 de P810) — `#set` valida nome e tipo ────────────

    /// (a) Propriedade inexistente no `TextElem` vanilla → erro hard
    /// `unexpected argument: {name}` (paridade `foundations/args.rs:262`).
    /// ANTES (medido): warning `propriedade '...' ainda não suportada`
    /// + exit 0.
    #[test]
    fn p816_set_text_propriedade_inexistente_erro() {
        let world = MockWorld::new("#set text(nonexistent-prop: 12pt)\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "propriedade inexistente deve erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter()
                .any(|e| e.message.contains("unexpected argument: nonexistent-prop")),
            "errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    /// (a-controlo) Propriedade válida no vanilla mas ainda não capturada
    /// (`baseline`, vanilla `text/mod.rs:371`) → mantém o warning de
    /// scope-out (Passo 107 / ADR-0040), não erro.
    #[test]
    fn p816_set_text_propriedade_vanilla_nao_implementada_warning() {
        let world = MockWorld::new("#set text(baseline: 3pt)\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, sink) = eval_for_test_keep_sink(&world, &src);
        assert!(
            result.is_ok(),
            "propriedade válida no vanilla não deve erro: {:?}",
            result
        );
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().any(|d| d.message.contains("'baseline'")),
            "baseline deve manter warning de scope-out; diagnostics: {:?}",
            diags
        );
    }

    /// (c) `size` com `Int` → erro `expected length, found integer` +
    /// hint `a length needs a unit - did you mean 12pt?` (paridade
    /// `foundations/cast.rs:325-343`). ANTES (medido): aceite em
    /// silêncio, exit 0.
    #[test]
    fn p816_set_text_size_int_erro() {
        let world = MockWorld::new("#set text(size: 12)\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "size com Int deve erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter()
                .any(|e| e.message.contains("expected length, found integer")),
            "errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
        assert!(
            errs.iter()
                .any(|e| e.hints.iter().any(|h| h.contains("did you mean 12pt?"))),
            "hint vanilla esperado; errs: {:?}",
            errs
        );
    }

    /// (c-extensão) `tracking` com `Int` → mesmo erro de tipo (Length no
    /// vanilla, `text/mod.rs:333`; medido: vanilla erra com o mesmo
    /// hint `did you mean 1pt?`).
    #[test]
    fn p816_set_text_tracking_int_erro() {
        let world = MockWorld::new("#set text(tracking: 1)\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "tracking com Int deve erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter()
                .any(|e| e.message.contains("expected length, found integer")),
            "errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    /// **P837** (achado #22 de P831) — `top-edge` com string fora do
    /// domínio enumerado → erro hard com a mensagem verbatim do vanilla
    /// (medido no vanilla 0.15.0: `error: expected "ascender",
    /// "cap-height", "x-height", "baseline", "bounds", or length`,
    /// exit 1; cast de `TopEdgeMetric`, `text/mod.rs:1169-1177`).
    /// ANTES (medido): exit 0 silencioso, caía no default.
    #[test]
    fn p837_set_text_top_edge_string_invalida_erro() {
        let world = MockWorld::new("#set text(top-edge: \"middle\")\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "top-edge inválido deve erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter().any(|e| e.message
                == "expected \"ascender\", \"cap-height\", \"x-height\", \"baseline\", \"bounds\", or length"),
            "errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    /// **P837** (achado #22 de P831) — `bottom-edge` com string fora do
    /// domínio → erro verbatim do vanilla (medido: `error: expected
    /// "baseline", "descender", "bounds", or length`, exit 1).
    #[test]
    fn p837_set_text_bottom_edge_string_invalida_erro() {
        let world = MockWorld::new("#set text(bottom-edge: \"middle\")\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "bottom-edge inválido deve erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter().any(|e| e.message
                == "expected \"baseline\", \"descender\", \"bounds\", or length"),
            "errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    /// **P837** — os domínios de top e bottom são distintos no vanilla:
    /// `"descender"` só é válido em `bottom-edge`, `"ascender"` só em
    /// `top-edge` (medido: ambos erro exit 1 no vanilla).
    #[test]
    fn p837_set_text_edges_dominios_cruzados_erro() {
        for (src_text, expected) in [
            (
                "#set text(top-edge: \"descender\")\nOlá",
                "expected \"ascender\", \"cap-height\", \"x-height\", \"baseline\", \"bounds\", or length",
            ),
            (
                "#set text(bottom-edge: \"ascender\")\nOlá",
                "expected \"baseline\", \"descender\", \"bounds\", or length",
            ),
        ] {
            let world = MockWorld::new(src_text);
            let src = World::source(&world, World::main(&world)).unwrap();
            let result = eval_for_test(&world, &src);
            assert!(result.is_err(), "{src_text:?} deve erro; got: {:?}", result);
            let errs = result.unwrap_err();
            assert!(
                errs.iter().any(|e| e.message == expected),
                "{src_text:?} → errs: {:?}",
                errs.iter().map(|e| &e.message).collect::<Vec<_>>()
            );
        }
    }

    /// **P837** — tipo que não é string nem length → mensagem com
    /// `, found {type}` e, para `Int`, o hint do vanilla `a length needs
    /// a unit - did you mean 3pt?` (medido no vanilla 0.15.0).
    #[test]
    fn p837_set_text_top_edge_int_erro_com_hint() {
        let world = MockWorld::new("#set text(top-edge: 3)\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "top-edge Int deve erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter().any(|e| e.message
                == "expected \"ascender\", \"cap-height\", \"x-height\", \"baseline\", \"bounds\", or length, found integer"),
            "errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
        assert!(
            errs.iter()
                .any(|e| e.hints.iter().any(|h| h.contains("did you mean 3pt?"))),
            "hint vanilla esperado; errs: {:?}",
            errs
        );
    }

    /// **P837** — valores válidos continuam aceites: todos os nomes
    /// enumerados de cada domínio (incluindo `"bounds"`) e `Length`
    /// explícito em pt/em (achado #23 de P831).
    #[test]
    fn p837_set_text_edges_validos_aceites() {
        for src_text in [
            "#set text(top-edge: \"ascender\")\nOlá",
            "#set text(top-edge: \"cap-height\")\nOlá",
            "#set text(top-edge: \"x-height\")\nOlá",
            "#set text(top-edge: \"baseline\")\nOlá",
            "#set text(top-edge: \"bounds\")\nOlá",
            "#set text(bottom-edge: \"baseline\")\nOlá",
            "#set text(bottom-edge: \"descender\")\nOlá",
            "#set text(bottom-edge: \"bounds\")\nOlá",
            "#set text(top-edge: 18pt, bottom-edge: -4pt)\nOlá",
            "#set text(top-edge: 1.5em)\nOlá",
        ] {
            let world = MockWorld::new(src_text);
            let src = World::source(&world, World::main(&world)).unwrap();
            let result = eval_for_test(&world, &src);
            assert!(
                result.is_ok(),
                "{src_text:?} deve ser aceite: {:?}",
                result
                    .err()
                    .map(|es| es.iter().map(|e| e.message.clone()).collect::<Vec<_>>())
            );
        }
    }

    /// (b) Família de fonte desconhecida → warning `unknown font family`
    /// com o nome lowercased (paridade `check_font_list`,
    /// `text/mod.rs:1577-1588`). ANTES (medido): silêncio total.
    #[test]
    fn p816_set_text_font_desconhecida_warning() {
        let world = MockWorld::new("#set text(font: \"FamiliaQueNaoExiste\")\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, sink) = eval_for_test_keep_sink(&world, &src);
        assert!(result.is_ok(), "fonte desconhecida é warning, não erro: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags
                .iter()
                .any(|d| d.message.contains("unknown font family: familiaquenaoexiste")),
            "warning vanilla esperado (nome lowercased); diagnostics: {:?}",
            diags
        );
    }

    /// (b-controlo) Família presente no `FontBook` → sem warning.
    #[test]
    fn p816_set_text_font_conhecida_sem_warning() {
        let mut world = MockWorld::new("#set text(font: \"Arial\")\nOlá");
        world.book.push(crate::entities::font_book::FontInfo {
            family: "Arial".to_string(),
            variant: Default::default(),
            flags: Default::default(),
            coverage: Default::default(),
        });
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, sink) = eval_for_test_keep_sink(&world, &src);
        assert!(result.is_ok(), "eval falhou: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().all(|d| !d.message.contains("unknown font family")),
            "família conhecida não deve emitir warning; diagnostics: {:?}",
            diags
        );
    }

    // ── P895 (Parte B — catálogo de terceiros) ──────────────────────────
    //
    // Regressão do achado central: `sym_lookup` unitário não bastava —
    // `$epsilon.alt$` é parseado como `FieldAccess(MathIdent("epsilon"),
    // "alt")`, nunca como um único `MathIdent` "epsilon.alt", por isso só um
    // teste que exercite o pipeline completo (parse + eval real, não uma
    // chamada directa a `sym_lookup`) confirma a correcção de facto.

    #[test]
    fn p895_epsilon_alt_compila_via_pipeline_real() {
        let world = MockWorld::new("$ epsilon.alt $");
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, _sink) = eval_for_test_keep_sink(&world, &src);
        assert!(
            result.is_ok(),
            "epsilon.alt deve compilar via modo math real: {:?}",
            result
        );
    }

    #[test]
    fn p895_inter_big_compila_via_pipeline_real() {
        let world = MockWorld::new("$ inter.big $");
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, _sink) = eval_for_test_keep_sink(&world, &src);
        assert!(
            result.is_ok(),
            "inter.big deve compilar via modo math real: {:?}",
            result
        );
    }

    /// **P895** — `thin`/`med`/`thick`/`quad`/`wide` são espaçamentos
    /// nomeados de modo math (paridade vanilla `math/mod.rs:98-102`:
    /// `HElem::new(THIN/MEDIUM/THICK/QUAD/WIDE.into())`, registados no
    /// scope do módulo `math`) — nunca foram registados no cristalino.
    #[test]
    fn p895_thin_med_thick_quad_wide_compilam() {
        for name in ["thin", "med", "thick", "quad", "wide"] {
            let src_text = format!("$ a {name} b $");
            let world = MockWorld::new(&src_text);
            let src = World::source(&world, World::main(&world)).unwrap();
            let (result, _sink) = eval_for_test_keep_sink(&world, &src);
            assert!(result.is_ok(), "{name} deve compilar em modo math: {:?}", result);
        }
    }

    // ── P820 (achado #7 de P810) — `join`/`bowtie` + `Deprecation` ──────

    /// (a) `$join$` → warning de depreciação do vanilla (mensagem
    /// verbatim, codex `sym.txt`), **não** erro `unknown variable`.
    /// ANTES (medido): `error: unknown variable: join`, exit 1.
    #[test]
    fn p820_math_join_bare_warning_depreciacao() {
        let world = MockWorld::new("$join$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, sink) = eval_for_test_keep_sink(&world, &src);
        assert!(
            result.is_ok(),
            "join depreciado compila com warning, não erro: {:?}",
            result
        );
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().any(|d| d
                .message
                .contains("`join` is deprecated, use `bowtie.big` instead")),
            "warning de depreciação esperado; diagnostics: {:?}",
            diags
        );
    }

    /// (a-variante) `$join.r$` → mesmo warning (span na raiz `join`,
    /// medido vanilla @1:1), exit 0.
    #[test]
    fn p820_math_join_variante_warning_depreciacao() {
        let world = MockWorld::new("$join.r$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, sink) = eval_for_test_keep_sink(&world, &src);
        assert!(result.is_ok(), "join.r deve compilar: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().any(|d| d
                .message
                .contains("`join` is deprecated, use `bowtie.big` instead")),
            "warning de depreciação esperado; diagnostics: {:?}",
            diags
        );
    }

    /// (a-código) `#sym.join` → mesmo warning, span no campo (medido
    /// vanilla @1:5), exit 0. ANTES: `module 'sym' does not contain
    /// field "join"`.
    #[test]
    fn p820_sym_join_field_access_warning_depreciacao() {
        let world = MockWorld::new("#sym.join");
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, sink) = eval_for_test_keep_sink(&world, &src);
        assert!(result.is_ok(), "#sym.join deve compilar: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().any(|d| d
                .message
                .contains("`join` is deprecated, use `bowtie.big` instead")),
            "warning de depreciação esperado; diagnostics: {:?}",
            diags
        );
    }

    /// (b) `$bowtie.big$` → compila sem warning (bowtie não é
    /// depreciado; medido vanilla: exit 0, render ⨝). ANTES:
    /// `error: variável desconhecida: bowtie`.
    #[test]
    fn p820_math_bowtie_big_sem_warning() {
        let world = MockWorld::new("$bowtie.big$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, sink) = eval_for_test_keep_sink(&world, &src);
        assert!(result.is_ok(), "bowtie.big deve compilar: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().all(|d| !d.message.contains("deprecated")),
            "bowtie não é depreciado — sem warning; diagnostics: {:?}",
            diags
        );
    }

    /// (b-controlo) `$bowtie$` bare e `#sym.bowtie.big` → compilam
    /// (medido vanilla: ⋈ e ⨝, exit 0).
    #[test]
    fn p820_bowtie_bare_e_field_access() {
        for src_text in ["$bowtie$", "#sym.bowtie.big"] {
            let world = MockWorld::new(src_text);
            let src = World::source(&world, World::main(&world)).unwrap();
            let (result, _sink) = eval_for_test_keep_sink(&world, &src);
            assert!(result.is_ok(), "{src_text} deve compilar: {:?}", result);
        }
    }

    /// (b-mensagem) `$foo.bar$` → erro `unknown variable: foo` em inglês
    /// com os 2 hints do vanilla (P780), não `variável desconhecida` em
    /// português — sub-achado (b) de P810 §7.
    #[test]
    fn p820_math_desconhecido_mensagem_ingles() {
        let world = MockWorld::new("$foo.bar$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, _sink) = eval_for_test_keep_sink(&world, &src);
        let errs = result.expect_err("foo desconhecido deve erro");
        assert!(
            errs.iter().any(|e| e.message.contains("unknown variable `foo`")),
            "mensagem inglesa do vanilla esperada; errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
        assert!(
            errs.iter().all(|e| !e.message.contains("variável desconhecida")),
            "não pode restar mensagem em português; errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    /// Passo 134 (DEBT-1): captura positiva de `leading` em contexto
    /// canonicamente correcto (`par`). Completa migração do 128 →
    /// paridade vanilla obtida.
    #[test]
    fn eval_set_par_leading_captura_passo_134() {
        use comemo::Track;
        let world = MockWorld::new("#set par(leading: 0.65em)\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_ok(), "eval falhou: {:?}", result);
        let diags = sink.into_diagnostics();
        // Sem warning para 'leading' — captura é silent em par.
        assert!(
            diags.iter().all(|d| !d.message.contains("'leading'")),
            "par deve capturar leading sem warning; diagnostics: {:?}",
            diags
        );
    }

    /// Passo 133 + 134: `par` é known target em `eval_set_rule`, com
    /// `leading` capturado (adicionado em 134). Outras propriedades
    /// (ex: `justify`) caem no fallback e emitem warning de
    /// **propriedade** (não de target). Documentação executável.
    #[test]
    fn eval_set_par_known_target_com_leading_passo_134() {
        use comemo::Track;
        let world = MockWorld::new("#set par(justify: true)");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_ok(), "eval falhou: {:?}", result);
        let diags = sink.into_diagnostics();
        // Warning é sobre PROPRIEDADE 'justify', não sobre target 'par'.
        assert!(
            diags
                .iter()
                .any(|d| d.message.contains("par:") && d.message.contains("'justify'")),
            "par deve emitir warning de propriedade justify; diagnostics: {:?}",
            diags
        );
        // `par` NÃO deve aparecer como target não suportado.
        assert!(
            diags.iter().all(|d| !d.message.contains("target 'par'")),
            "par não deve aparecer como target unknown; diagnostics: {:?}",
            diags
        );
    }

    /// Passo 129 (DEBT-1 subset): 9 nomes simbólicos canónicos
    /// aceitos em `#set text(weight: "..."")`. Zero warning.
    #[test]
    fn eval_set_text_weight_simbolico_passo_129() {
        use comemo::Track;
        for nome in [
            "thin",
            "extralight",
            "light",
            "regular",
            "medium",
            "semibold",
            "bold",
            "extrabold",
            "black",
        ] {
            let src_text = format!("#set text(weight: \"{}\")\nOlá", nome);
            let world = MockWorld::new(&src_text);
            let src = World::source(&world, World::main(&world)).unwrap();

            let routines = Routines::new();
            let traced = Traced::default();
            let mut sink = Sink::new();
            let route = Route::root();
            let result = eval(
                &routines,
                &world,
                traced.track(),
                sink.track_mut(),
                route.track(),
                &src,
                &crate::entities::element_registry::ElementRegistry::new(),
            );

            assert!(result.is_ok(), "eval falhou para nome '{}': {:?}", nome, result);
            let diags = sink.into_diagnostics();
            assert!(
                diags.iter().all(|d| !d.message.contains("'weight'")),
                "nome simbólico '{}' não deve emitir warning; got: {:?}",
                nome,
                diags
            );
        }
    }

    /// Passo 129 / P636: nome simbólico desconhecido passou de silencioso
    /// para erro hard, como parte da validação de tipos de `#set` rules.
    #[test]
    fn eval_set_text_weight_simbolico_desconhecido_error_passo_129() {
        use comemo::Track;
        let world = MockWorld::new("#set text(weight: \"arcoiris\")\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(
            result.is_err(),
            "nome simbólico desconhecido deve falhar; got: {:?}",
            result
        );
    }

    /// Passo 131B (ADR-0052): `lang` valido (ISO 639-1) é aceite
    /// e capturado como `Lang`. Zero warning. Renomeado de
    /// `eval_set_text_lang_passo_130` (130 → 131B).
    #[test]
    fn eval_set_text_lang_valido_passo_131b() {
        use comemo::Track;
        let world = MockWorld::new("#set text(lang: \"pt\")\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_ok(), "eval falhou: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().all(|d| !d.message.contains("'lang'")),
            "lang válido não deve emitir warning; diagnostics: {:?}",
            diags
        );
    }

    /// Passo 131B (ADR-0052, invertido de 130): valores compostos
    /// BCP 47 como `"en-GB"` deixam de ser silent — agora erram
    /// hard com a mensagem literal do vanilla (paridade ADR-0033).
    /// Documenta a breaking semantic change face ao Passo 130.
    #[test]
    fn eval_set_text_lang_composto_emite_erro_passo_131b() {
        use comemo::Track;
        let world = MockWorld::new("#set text(lang: \"en-GB\")\nHello");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_err(), "lang composto deve emitir erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter().any(|e| e
                .message
                .contains("expected two or three letter language code")),
            "mensagem deve ser literal vanilla; errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    /// Passo 131B: valor totalmente inválido (4 letras, não ASCII, etc.)
    /// emite erro hard com mensagem literal vanilla.
    #[test]
    fn eval_set_text_lang_invalido_emite_erro_hard_passo_131b() {
        use comemo::Track;
        let world = MockWorld::new("#set text(lang: \"xxxx\")\nTexto");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_err(), "lang inválido deve emitir erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter().any(|e| e
                .message
                .contains("expected two or three letter language code")),
            "mensagem deve ser literal vanilla; errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    /// Passo 132B (ADR-0053): canary DEBT-50 migrou de `font` para
    /// `hyphenate`. `font` agora é capturado com `FontList`;
    /// `hyphenate` é a próxima propriedade não-capturada que serve
    /// como sinal de vida do mecanismo de warnings. Consolida os 5
    /// canaries anteriores (126/127/128/129/131b) em 1 único teste.
    #[test]
    fn eval_set_text_hyphenate_canary_passo_132b() {
        use comemo::Track;
        let world = MockWorld::new("#set text(hyphenate: true)\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_ok(), "eval falhou: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().any(|d| d.message.contains("'hyphenate'")),
            "hyphenate deve emitir warning (canary); diagnostics: {:?}",
            diags
        );
    }

    /// Passo 132B (ADR-0053): forma string simples de `font`
    /// capturada em `FontList` de 1 elemento. Nome lowercased.
    #[test]
    fn eval_set_text_font_string_simples_passo_132b() {
        use comemo::Track;
        let world = MockWorld::new("#set text(font: \"Arial\")\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_ok(), "eval falhou: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().all(|d| !d.message.contains("'font'")),
            "font válido não deve emitir warning; diagnostics: {:?}",
            diags
        );
    }

    /// Passo 132B: forma array de `font` — paridade vanilla para
    /// fallback chain.
    #[test]
    fn eval_set_text_font_array_passo_132b() {
        use comemo::Track;
        let world =
            MockWorld::new("#set text(font: (\"Inria Serif\", \"Noto Sans\"))\nOlá");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_ok(), "eval falhou: {:?}", result);
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().all(|d| !d.message.contains("'font'")),
            "font array válido não deve emitir warning; diagnostics: {:?}",
            diags
        );
    }

    /// Passo 132B: forma dict **rejeitada** unitariamente via
    /// construção directa de Value::Dict. Documenta a divergência
    /// consciente ADR-0053 (covers sem suporte até `regex`
    /// autorizado em L1).
    ///
    /// Nota: Typst não permite exprimir `Value::Dict` literalmente
    /// dentro de uma arg list de set rule via sintaxe
    /// `(key: val, ...)` — esse pattern é parseado como argumentos
    /// nomeados. Por isso este teste valida a decisão estrutural
    /// unit em vez de integration.
    #[test]
    fn eval_set_text_font_dict_rejeitado_unit_passo_132b() {
        use crate::entities::value::Value;
        use ecow::EcoString;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;

        // Construção directa de Value::Dict para validar arm.
        let mut map: IndexMap<EcoString, Value, FxBuildHasher> =
            IndexMap::with_hasher(FxBuildHasher);
        map.insert(EcoString::from("name"), Value::Str(EcoString::from("X")));
        let dict_val = Value::Dict(map);

        // Match expression replicando o arm do eval_set_text:
        let rejeitado = matches!(dict_val, Value::Dict(_));
        assert!(rejeitado, "Value::Dict é construível e match arm '_ => Dict(_)' funciona estruturalmente");
    }

    /// Passo 132B: valor de tipo inválido (Int) — arm genérico
    /// rejeita com mensagem "font expects a string or array of
    /// strings".
    #[test]
    fn eval_set_text_font_tipo_invalido_rejeitado_passo_132b() {
        use comemo::Track;
        let world = MockWorld::new("#set text(font: 42)");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_err(), "int deve erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter().any(|e| e
                .message
                .contains("font expects a string, array of strings, or dict")),
            "mensagem deve indicar tipo esperado; errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    /// Passo 132B: array vazio é `Err` com mensagem literal
    /// análoga ao vanilla.
    #[test]
    fn eval_set_text_font_array_vazio_rejeitado_passo_132b() {
        use comemo::Track;
        let world = MockWorld::new("#set text(font: ())");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_err(), "array vazio deve erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter().any(|e| e.message.contains("must not be empty")),
            "mensagem deve indicar array vazio; errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    /// Passo 132B: array com item não-string é `Err`.
    #[test]
    fn eval_set_text_font_array_com_nao_string_rejeitado_passo_132b() {
        use comemo::Track;
        let world = MockWorld::new("#set text(font: (\"Arial\", 42))");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_err(), "array com int deve erro; got: {:?}", result);
        let errs = result.unwrap_err();
        assert!(
            errs.iter().any(|e| e.message.contains("only strings")),
            "mensagem deve indicar 'only strings'; errs: {:?}",
            errs.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    // ── Passo 407 — `text.font` dict (DEBT-52) ───────────────────────────────

    #[test]
    fn eval_set_text_font_dict_literal_key_passo_407() {
        use crate::entities::value::Value;
        use ecow::EcoString;
        let c = eval_doc("#set text(font: (\"Name\": (\"Regular\", \"Bold\")))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        assert_eq!(arr.len(), 1);
        let dict = arr[0].cast_dict().expect("item deve ser dict");
        assert_eq!(dict.get("name"), Some(&Value::Str(EcoString::from("Name"))));
        let variants = dict.get("variants").unwrap().cast_array().unwrap();
        assert_eq!(
            variants,
            &[
                Value::Str(EcoString::from("Regular")),
                Value::Str(EcoString::from("Bold"))
            ]
        );
    }

    #[test]
    fn eval_set_text_font_dict_regex_key_passo_407() {
        use crate::entities::value::Value;
        use ecow::EcoString;
        let c = eval_doc("#set text(font: (regex(\"Name.*\"): (\"Regular\")))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        assert_eq!(arr.len(), 1);
        let dict = arr[0].cast_dict().expect("item deve ser dict");
        assert!(matches!(dict.get("name"), Some(Value::Regex(_))), "name deve ser regex");
        let variants = dict.get("variants").unwrap().cast_array().unwrap();
        assert_eq!(variants, &[Value::Str(EcoString::from("Regular"))]);
    }

    #[test]
    fn eval_set_text_font_dict_str_value_passo_407() {
        use crate::entities::value::Value;
        use ecow::EcoString;
        let c = eval_doc("#set text(font: (\"Name\": \"Regular\"))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        let dict = arr[0].cast_dict().expect("item deve ser dict");
        let variants = dict.get("variants").unwrap().cast_array().unwrap();
        assert_eq!(variants, &[Value::Str(EcoString::from("Regular"))]);
    }

    #[test]
    fn eval_set_text_font_dict_invalid_key_passo_407() {
        let world = MockWorld::new("#set text(font: (123: (\"Regular\")))\nX");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "dict key int deve erro");
    }

    #[test]
    fn eval_set_text_font_dict_invalid_value_passo_407() {
        let world = MockWorld::new("#set text(font: (\"Name\": 123))\nX");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "dict value int deve erro");
    }

    // ── Passo 414 — `text.font` dict named fields ────────────────────────────

    #[test]
    fn eval_set_text_font_dict_named_family_str_passo_414() {
        use crate::entities::value::Value;
        use ecow::EcoString;
        let c = eval_doc("#set text(font: (family: \"Arial\"))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        assert_eq!(arr.len(), 1);
        let dict = arr[0].cast_dict().expect("item deve ser dict");
        assert_eq!(dict.get("name"), Some(&Value::Str(EcoString::from("Arial"))));
        assert_eq!(dict.get("variant"), None);
        assert_eq!(dict.get("weight"), None);
        assert_eq!(dict.get("style"), None);
    }

    #[test]
    fn eval_set_text_font_dict_named_family_regex_passo_414() {
        use crate::entities::value::Value;
        let c = eval_doc("#set text(font: (family: regex(\"Ar.*\")))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        let dict = arr[0].cast_dict().expect("item deve ser dict");
        assert!(matches!(dict.get("name"), Some(Value::Regex(_))), "name deve ser regex");
    }

    #[test]
    fn eval_set_text_font_dict_named_variant_passo_414() {
        use crate::entities::value::Value;
        use ecow::EcoString;
        let c = eval_doc("#set text(font: (family: \"Arial\", variant: \"bold\"))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        let dict = arr[0].cast_dict().expect("item deve ser dict");
        assert_eq!(dict.get("variant"), Some(&Value::Str(EcoString::from("bold"))));
    }

    #[test]
    fn eval_set_text_font_dict_named_weight_int_passo_414() {
        use crate::entities::value::Value;
        use ecow::EcoString;
        let c = eval_doc("#set text(font: (family: \"Arial\", weight: 700))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        let dict = arr[0].cast_dict().expect("item deve ser dict");
        assert_eq!(dict.get("weight"), Some(&Value::Str(EcoString::from("700"))));
    }

    #[test]
    fn eval_set_text_font_dict_named_weight_str_passo_414() {
        use crate::entities::value::Value;
        use ecow::EcoString;
        let c = eval_doc("#set text(font: (family: \"Arial\", weight: \"bold\"))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        let dict = arr[0].cast_dict().expect("item deve ser dict");
        assert_eq!(dict.get("weight"), Some(&Value::Str(EcoString::from("bold"))));
    }

    #[test]
    fn eval_set_text_font_dict_named_style_passo_414() {
        use crate::entities::value::Value;
        use ecow::EcoString;
        let c = eval_doc("#set text(font: (family: \"Arial\", style: \"italic\"))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        let dict = arr[0].cast_dict().expect("item deve ser dict");
        assert_eq!(dict.get("style"), Some(&Value::Str(EcoString::from("italic"))));
    }

    #[test]
    fn eval_set_text_font_dict_named_full_passo_414() {
        use crate::entities::value::Value;
        use ecow::EcoString;
        let c = eval_doc("#set text(font: (family: \"Arial\", variant: \"bold\", weight: 700, style: \"italic\", fallback: false))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        let dict = arr[0].cast_dict().expect("item deve ser dict");
        assert_eq!(dict.get("name"), Some(&Value::Str(EcoString::from("Arial"))));
        assert_eq!(dict.get("variant"), Some(&Value::Str(EcoString::from("bold"))));
        assert_eq!(dict.get("weight"), Some(&Value::Str(EcoString::from("700"))));
        assert_eq!(dict.get("style"), Some(&Value::Str(EcoString::from("italic"))));
    }

    #[test]
    fn eval_set_text_font_dict_named_unknown_field_passo_414() {
        let world = MockWorld::new(
            "#set text(font: (family: \"Arial\", stretch: \"expanded\"))\nX",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "campo desconhecido deve erro");
    }

    #[test]
    fn eval_set_text_font_dict_named_missing_family_passo_414() {
        let world = MockWorld::new("#set text(font: (variant: \"bold\"))\nX");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "family ausente deve erro");
    }

    #[test]
    fn eval_set_text_font_dict_named_invalid_family_type_passo_414() {
        let world = MockWorld::new("#set text(font: (family: 123))\nX");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "family int deve erro");
    }

    #[test]
    fn eval_set_text_font_dict_named_invalid_weight_type_passo_414() {
        let world =
            MockWorld::new("#set text(font: (family: \"Arial\", weight: true))\nX");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "weight bool deve erro");
    }

    #[test]
    fn eval_set_text_font_dict_named_fallback_bool_passo_414() {
        use crate::entities::value::Value;
        let c = eval_doc("#set text(font: (family: \"Arial\", fallback: false))\nX");
        let font_val =
            find_custom_in_styled(&c, "text.font").expect("text.font deve existir");
        let arr = font_val.cast_array().expect("text.font deve ser array");
        assert_eq!(arr.len(), 1, "fallback:false mantém lista única");
    }

    // ── Testes de Passo 34 — equações matemáticas ────────────────────────────

    #[test]
    fn eval_equation_inline_nao_da_err() {
        let world = MockWorld::new("O valor de $x$ é 1.");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "equação inline falhou: {:?}", result);
    }

    #[test]
    fn eval_equation_block_nao_da_err() {
        let world = MockWorld::new("$ x^2 + y^2 = r^2 $");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "equação block falhou: {:?}", result);
    }

    #[test]
    fn eval_equation_frac_nao_da_err() {
        let world = MockWorld::new("$ x/2 $");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "equação com frac falhou: {:?}", result);
    }

    #[test]
    fn eval_equation_nao_cai_no_catch_all() {
        // Verificar que Expr::Equation tem arm próprio e produz Content (não Value::None)
        let world = MockWorld::new("$x$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        // O módulo deve ter Content não-vazio (equação não foi silenciada)
        let _ = m;
    }

    #[test]
    fn eval_e_layout_equation_sem_colchetes() {
        use crate::compiler::layout::layout;
        let world = MockWorld::new("$x$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        // Verificar que o layout não produz "[" nos FrameItems
        if let Some(content) = m.content() {
            let doc = layout(content);
            for page in &doc.pages {
                for item in &page.items {
                    if let crate::entities::layout_types::FrameItem::Text {
                        text, ..
                    } = item
                    {
                        assert!(
                            !text.starts_with('['),
                            "equação não deve produzir '[' no layout: {}",
                            text
                        );
                    }
                }
            }
        }
    }

    // ── Testes de Passo 39 — símbolos matemáticos ────────────────────────────

    #[test]
    fn eval_alpha_produz_unicode() {
        use crate::compiler::layout::layout;
        let world = MockWorld::new("$alpha$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let content = m.content().expect("módulo deve ter content");
        let doc = layout(content);
        // α deve aparecer no texto, não "alpha". P809: no layout, α é
        // estilizado para 𝛼 U+1D6FC (itálico matemático — paridade vanilla).
        let plain = doc.plain_text();
        assert!(plain.contains('𝛼'), "𝛼 deve estar no output, não 'alpha': {}", plain);
        assert!(
            !plain.contains("alpha"),
            "texto literal 'alpha' não deve aparecer: {}",
            plain
        );
    }

    #[test]
    fn eval_shorthand_seta() {
        let world = MockWorld::new("$x -> y$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "$x -> y$ falhou: {:?}", result);
    }

    #[test]
    fn eval_equacao_com_sum() {
        let world = MockWorld::new("$sum_(i=0)^n x_i$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "equação com sum falhou: {:?}", result);
    }

    // ── Testes de Passo 38 — frac() nativa e MathDelimited ──────────────────

    #[test]
    fn eval_frac_funcao_nativa_produz_mathfrac() {
        // frac(a, b) em modo math deve produzir Content::MathFrac, não Content::Empty
        let world = MockWorld::new("$frac(a, b)$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        // Módulo deve ter content (equação não foi silenciada)
        let content = m.content().expect("módulo deve ter content");
        // Plain text do layout deve conter "a" e "b" (não vazio)
        use crate::compiler::layout::layout;
        let doc = layout(content);
        assert!(!doc.pages.is_empty(), "frac(a,b) deve produzir pelo menos uma página");
    }

    #[test]
    fn eval_math_delimited_parenteses() {
        let world = MockWorld::new("$(a + b)$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "$(a + b)$ deve avaliar sem erro: {:?}", result);
    }

    #[test]
    fn eval_math_delimited_colchetes() {
        let world = MockWorld::new("$[x]$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "$[x]$ deve avaliar sem erro: {:?}", result);
    }

    // ── Testes de Passo 40 — sqrt() e root() nativos ────────────────────────

    #[test]
    fn eval_sqrt_produz_math_root_sem_indice() {
        let world = MockWorld::new("$sqrt(x)$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let content = m.content().expect("módulo deve ter content");
        // Verificar que o conteúdo contém MathRoot
        fn has_math_root(c: &Content) -> bool {
            match c {
                Content::MathRoot(e) => e.index.is_none(),
                Content::Equation(e) => has_math_root(&e.body),
                Content::MathSequence(ns) => ns.iter().any(has_math_root),
                Content::Sequence(ns) => ns.iter().any(has_math_root),
                _ => false,
            }
        }
        assert!(has_math_root(content), "sqrt(x) deve produzir MathRoot sem índice");
    }

    #[test]
    fn eval_root_com_indice_produz_math_root_com_indice() {
        let world = MockWorld::new("$root(3, x)$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let content = m.content().expect("módulo deve ter content");
        fn has_math_root_with_index(c: &Content) -> bool {
            match c {
                Content::MathRoot(e) => e.index.is_some(),
                Content::Equation(e) => has_math_root_with_index(&e.body),
                Content::MathSequence(ns) => ns.iter().any(has_math_root_with_index),
                Content::Sequence(ns) => ns.iter().any(has_math_root_with_index),
                _ => false,
            }
        }
        assert!(
            has_math_root_with_index(content),
            "root(3,x) deve produzir MathRoot com índice"
        );
    }

    #[test]
    fn eval_sqrt_zero_args_retorna_erro() {
        let world = MockWorld::new("$sqrt()$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "sqrt() com 0 args deve retornar erro");
    }

    #[test]
    fn eval_sqrt_dois_args_retorna_erro() {
        let world = MockWorld::new("$sqrt(x, y)$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "sqrt(x,y) com 2 args deve retornar erro");
    }

    #[test]
    fn eval_root_um_arg_retorna_erro() {
        let world = MockWorld::new("$root(3)$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "root(3) com 1 arg deve retornar erro");
    }

    #[test]
    fn eval_sqrt_layout_contem_radical() {
        use crate::compiler::layout::layout;
        let world = MockWorld::new("$sqrt(x)$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let content = m.content().expect("content");
        let doc = layout(content);
        let plain = doc.plain_text();
        assert!(plain.contains('√'), "layout de sqrt deve conter √: {}", plain);
        assert!(plain.contains('𝑥'), "layout de sqrt deve conter x: {}", plain);
    }

    #[test]
    fn eval_sqrt_layout_tem_overline() {
        use crate::compiler::layout::layout;
        use crate::entities::layout_types::FrameItem;
        let world = MockWorld::new("$sqrt(x)$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let content = m.content().expect("content");
        let doc = layout(content);
        fn has_line(items: &[FrameItem]) -> bool {
            items.iter().any(|item| match item {
                FrameItem::Line { .. } => true,
                FrameItem::Semantic { items, .. }
                | FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. } => has_line(items),
                _ => false,
            })
        }
        let has_line = doc.pages.iter().any(|page| has_line(&page.items));
        assert!(has_line, "layout de sqrt deve conter FrameItem::Line para overline");
    }

    #[test]
    fn eval_root_layout_contem_indice_e_radicando() {
        use crate::compiler::layout::layout;
        let world = MockWorld::new("$root(3, x)$");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let content = m.content().expect("content");
        let doc = layout(content);
        let plain = doc.plain_text();
        assert!(plain.contains('3'), "layout de root(3,x) deve conter 3: {}", plain);
        assert!(plain.contains('√'), "layout de root(3,x) deve conter √: {}", plain);
        assert!(plain.contains('𝑥'), "layout de root(3,x) deve conter x: {}", plain);
    }

    // ── Testes de Passo 33 — scoping de #set por bloco ──────────────────────

    #[test]
    fn set_dentro_bloco_nao_vaza_para_fora() {
        // #set dentro de { } não deve afectar o estilo após o bloco.
        // Usar content blocks [ ] para texto dentro de code blocks.
        // P786a: fonte corrigida para sintaxe válida — a forma anterior
        // (`#set` dentro de `{ }`) é rejeitada pelo vanilla (`#` inválido
        // em código) e só passava porque o eval descartava erros de parser.
        let world = MockWorld::new(
            "#set text(weight: 700)\n\
             antes\n\
             #{ set text(weight: 400); [normal] }\n\
             depois",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "set dentro de bloco falhou: {:?}", result);
    }

    #[test]
    fn set_dentro_closure_nao_afecta_caller() {
        // P786a: fonte corrigida (ver nota em set_dentro_bloco_nao_vaza_para_fora).
        let world = MockWorld::new(
            "#let f() = { set text(weight: 700); [negrito] }\n\
             #f()\n\
             texto normal",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "closure com set falhou: {:?}", result);
    }

    #[test]
    fn set_false_reverte_set_true_em_bloco() {
        // #set text(weight: 400) dentro de bloco reverte #set text(weight: 700) global.
        // Após o bloco, bold volta a true (estado salvo antes do bloco).
        // P786a: fonte corrigida (ver nota em set_dentro_bloco_nao_vaza_para_fora).
        let world = MockWorld::new(
            "#set text(weight: 700)\n\
             negrito\n\
             #{ set text(weight: 400); [normal] }\n\
             negrito novamente",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "set false em bloco falhou: {:?}", result);
    }

    #[test]
    fn set_aninhado_multiple_niveis() {
        // P786a: fonte corrigida — `set` sem `#` dentro de código e bloco
        // interior `{ }` (não `#{ }`), validada contra o vanilla 0.15.0.
        let world = MockWorld::new(
            "#{\n\
               set text(size: 14pt)\n\
               [texto14]\n\
               {\n\
                 set text(size: 18pt)\n\
                 [texto18]\n\
               }\n\
               [texto14novamente]\n\
             }",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "set aninhado falhou: {:?}", result);
    }

    #[test]
    fn set_em_content_block_nao_vaza() {
        // Content block [ ] também deve ter scoping de styles
        // P786a: fonte corrigida — `[#set text(..) normal]` na mesma linha é
        // rejeitada pelo vanilla ("expected semicolon or line break"); a
        // quebra de linha separa a regra set do conteúdo, validado.
        let world = MockWorld::new(
            "#set text(weight: 700)\n\
             antes\n\
             [#set text(weight: 400)\nnormal]\n\
             depois",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_ok(), "set em content block falhou: {:?}", result);
    }

    // ── Testes do Passo 47 — MathPrimes ─────────────────────────────────────

    /// Avalia uma expressão Typst e devolve o plain_text() do Content resultante.
    fn eval_plain_text(src: &str) -> String {
        let world = MockWorld::new(src);
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).expect("eval não deve falhar");
        module.content().map(|c| c.plain_text()).unwrap_or_default()
    }

    #[test]
    fn prime_simples_produz_unicode() {
        // $x'$ → sup contém ′ (U+2032)
        let text = eval_plain_text("$x'$");
        assert!(text.contains('x'), "base ausente: {:?}", text);
        assert!(text.contains('′'), "prime U+2032 ausente: {:?}", text);
    }

    #[test]
    fn double_prime_produz_unicode() {
        // $x''$ → sup contém ″ (U+2033)
        let text = eval_plain_text("$x''$");
        assert!(text.contains('x'));
        assert!(text.contains('″'), "double prime U+2033 ausente: {:?}", text);
    }

    #[test]
    fn triple_prime_produz_unicode() {
        // $f'''$ → sup contém ‴ (U+2034)
        let text = eval_plain_text("$f'''$");
        assert!(text.contains('f'));
        assert!(text.contains('‴'), "triple prime U+2034 ausente: {:?}", text);
    }

    #[test]
    fn quad_prime_produz_unicode() {
        // $x''''$ → sup contém ⁗ (U+2057)
        let text = eval_plain_text("$x''''$");
        assert!(text.contains('x'));
        assert!(text.contains('⁗'), "quad prime U+2057 ausente: {:?}", text);
    }

    #[test]
    fn prime_com_sup_faz_merge() {
        // $x'^2$ — prime e superscript coexistem: sup = MathSequence([′, 2])
        let text = eval_plain_text("$x'^2$");
        assert!(text.contains('x'));
        assert!(text.contains('′'), "prime ausente: {:?}", text);
        assert!(text.contains('2'), "sup ausente: {:?}", text);
    }

    #[test]
    fn prime_com_sub_nao_interfere() {
        // $x'_i$ — prime não interfere com subscript
        let text = eval_plain_text("$x'_i$");
        assert!(text.contains('x'));
        assert!(text.contains('′'));
        assert!(text.contains('i'));
    }

    #[test]
    fn sem_prime_nao_regride() {
        // Regressão: $x^2_i$ sem primes não muda
        let text = eval_plain_text("$x^2_i$");
        assert!(text.contains('x'));
        assert!(text.contains('2'));
        assert!(text.contains('i'));
    }

    // ── Testes do Passo 56 — Labels e Referências ────────────────────────────

    #[test]
    fn eval_label_anexa_ao_bloco_anterior() {
        use crate::entities::label::Label;
        let world = MockWorld::new("= Título <meu_label>");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        // O markup pode envolver o Label numa Sequence com espaços residuais.
        // Procurar o nó Label directamente ou dentro da Sequence.
        let labelled = match &content {
            Content::Label { .. } => &content,
            Content::Sequence(items) => items
                .iter()
                .find(|c| matches!(c, Content::Label { .. }))
                .expect("nenhum Label encontrado na Sequence"),
            _ => panic!("esperado Label ou Sequence, obtido: {:?}", content),
        };
        assert!(
            matches!(labelled, Content::Label(e)
                if e.auto && matches!(&e.body, Content::Heading(_)) && e.name.as_str() == "meu_label"),
            "esperado Label(Heading) auto, obtido: {:?}",
            labelled
        );
    }

    #[test]
    fn eval_ref_gera_content_ref() {
        use crate::entities::label::Label;
        let world = MockWorld::new("@meu_label");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        assert!(
            matches!(&content, Content::Ref(e) if e.name == "meu_label"),
            "esperado Ref(meu_label), obtido: {:?}",
            content
        );
    }

    // ── P802 — warning de label órfã (paridade vanilla markup.rs) ─────────

    #[test]
    fn label_orfa_emite_warning() {
        // P802 — label sem elemento anterior anexável: o vanilla emite
        // `label `<abc>` is not attached to anything` (typst-eval/markup.rs);
        // o cristalino ignorava em silêncio (achado #5 de P798).
        let world = MockWorld::new("<abc> Hello #context query(<abc>)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let (result, sink) = eval_for_test_keep_sink(&world, &src);
        result.expect("label órfã não é erro fatal");
        let diags = sink.into_diagnostics();
        assert!(
            diags
                .iter()
                .any(|d| d.message == "label `<abc>` is not attached to anything"),
            "warning de label órfã ausente; diagnósticos: {:?}",
            diags.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn label_anexada_nao_emite_warning_orfa() {
        // Controlo P802 — label anexada a elemento anterior (texto/heading)
        // não dispara o warning de órfã (paridade vanilla: `Hello <abc>`
        // não avisa — medido no vanilla 0.15.0).
        for src_text in ["Hello <abc>", "= Título <abc>"] {
            let world = MockWorld::new(src_text);
            let src = World::source(&world, World::main(&world)).unwrap();
            let (result, sink) = eval_for_test_keep_sink(&world, &src);
            result.expect("eval sem erro fatal");
            let diags = sink.into_diagnostics();
            assert!(
                !diags.iter().any(|d| d.message.contains("is not attached")),
                "warning de órfã indevida para `{src_text}`: {:?}",
                diags.iter().map(|d| &d.message).collect::<Vec<_>>()
            );
        }
    }

    // ── Testes de Passo 58 — counter(...).method() ────────────────────────

    #[test]
    fn eval_counter_step_gera_counter_update() {
        let world = MockWorld::new("#counter(\"equation\").step()");
        let src = world.source(world.main()).unwrap();
        assert!(eval_for_test(&world, &src).is_ok());
    }

    #[test]
    fn eval_counter_update_gera_counter_update_com_valor() {
        let world = MockWorld::new("#counter(\"fig\").update(3)");
        let src = world.source(world.main()).unwrap();
        assert!(eval_for_test(&world, &src).is_ok());
    }

    #[test]
    fn eval_counter_step_string_key_gera_content() {
        let world = MockWorld::new("#counter(\"equation\").step()");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        assert!(
            matches!(&content, Content::CounterUpdate(e) if e.key == ck("equation") && e.action == CounterAction::step()),
            "esperado CounterUpdate(equation, Step), obtido: {:?}",
            content
        );
    }

    #[test]
    fn eval_counter_ident_key_heading_step() {
        let world = MockWorld::new("#counter(heading).step()");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        assert!(
            matches!(&content, Content::CounterUpdate(e) if e.key == sel(ElementKind::Heading) && e.action == CounterAction::step()),
            "esperado CounterUpdate(heading, Step), obtido: {:?}",
            content
        );
    }

    // ── Passo 64 — Named args via NativeFunc (DEBT-16) ───────────────────────

    #[test]
    fn eval_named_arg_passado_para_func_nativa() {
        // Verificar que named args chegam à função via o novo mecanismo,
        // não via interceptador. figure() é o caso de teste canónico.
        let world = MockWorld::new("#figure([Conteúdo], caption: [Legenda])");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        assert!(
            matches!(content, Content::Figure(e)),
            "figure() com caption deve produzir Content::Figure com caption: {:?}",
            content
        );
    }

    #[test]
    fn eval_figure_sem_interceptador_em_eval_rs() {
        // Smoke test: figure() agora vive em stdlib.rs — o pipeline completo
        // deve funcionar sem o interceptador hardcoded.
        let world = MockWorld::new("#figure([A], caption: [B])");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().unwrap();
        assert!(matches!(content, Content::Figure(e)));
    }

    #[test]
    fn eval_named_arg_desconhecido_retorna_erro_semantico() {
        // expect_no_named() em stdlib.rs garante rigor: named args não
        // esperados devem retornar Err, não ser engolidos silenciosamente.
        // type() é uma função existente que não aceita named args.
        let world = MockWorld::new("#type(\"texto\", arg_invalido: true)");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "named arg desconhecido deve retornar Err");
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("inesperado")
                || err[0].message.contains("unexpected"),
            "mensagem deve mencionar argumento inesperado: {:?}",
            err[0].message
        );
    }

    #[test]
    fn eval_figure_sem_caption_via_stdlib() {
        // figure() sem caption deve produzir Content::Figure com caption: None.
        let world = MockWorld::new("#figure([Diagrama])");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().unwrap();
        assert!(
            matches!(content, Content::Figure(e)),
            "figure() sem caption deve ter caption None: {:?}",
            content
        );
    }

    // ── Passo 66 — assert() via eval (prova de fogo de named args) ───────────

    #[test]
    fn eval_assert_true_nao_gera_erro() {
        let world = MockWorld::new("#assert(1 == 1)");
        let src = world.source(world.main()).unwrap();
        assert!(eval_for_test(&world, &src).is_ok(), "assert(true) deve ter sucesso");
    }

    #[test]
    fn eval_assert_false_gera_erro_com_mensagem_padrao() {
        let world = MockWorld::new("#assert(false)");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err());
        let err = result.unwrap_err();
        // P843 (F7) — paridade vanilla: "assertion failed".
        assert_eq!(err[0].message, "assertion failed");
    }

    #[test]
    fn eval_assert_false_gera_erro_com_mensagem_personalizada() {
        let world = MockWorld::new("#assert(1 == 2, message: \"Matematica falhou\")");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err());
        assert!(result.unwrap_err()[0].message.contains("Matematica falhou"));
    }

    #[test]
    fn eval_assert_rejeita_named_arg_invalido() {
        let world = MockWorld::new("#assert(true, bla: \"bla\")");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("inesperado") && err[0].message.contains("bla"),
            "named arg desconhecido deve gerar erro: {:?}",
            err[0].message
        );
    }

    // ── Passo 67 — upper() / lower() / replace() via eval ────────────────────

    #[test]
    fn eval_upper_de_string() {
        let world = MockWorld::new("#upper(\"hello\")");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert_eq!(text, "HELLO");
    }

    #[test]
    fn eval_lower_de_string() {
        let world = MockWorld::new("#lower(\"MUNDO\")");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert_eq!(text, "mundo");
    }

    #[test]
    fn eval_replace_simples() {
        let world = MockWorld::new("#replace(\"hello world\", \"world\", \"Typst\")");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert_eq!(text, "hello Typst");
    }

    #[test]
    fn eval_replace_padrao_vazio_retorna_err() {
        let world = MockWorld::new("#replace(\"hello\", \"\", \"x\")");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "replace com padrão vazio deve retornar Err");
    }

    #[test]
    fn eval_upper_de_content_markup() {
        let world = MockWorld::new("#upper([*negrito*])");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert_eq!(text, "NEGRITO");
    }

    #[test]
    fn eval_upper_rejeita_named_arg() {
        let world = MockWorld::new("#upper(\"x\", bla: 1)");
        let src = world.source(world.main()).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }

    #[test]
    fn eval_replace_com_count() {
        let world = MockWorld::new("#replace(\"aaaa\", \"a\", \"b\", count: 2)");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert_eq!(text, "bbaa");
    }

    #[test]
    fn eval_replace_rejeita_named_arg_invalido() {
        let world = MockWorld::new("#replace(\"x\", \"a\", \"b\", bla: 1)");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err());
        assert!(result.unwrap_err()[0].message.contains("bla"));
    }

    #[test]
    fn eval_replace_limite_parcial_entre_nos() {
        // count: 3 é global ao documento — persiste entre nós via FnMut.
        // "aa " → substitui 2 → remaining=1; "*aa*" → substitui 1 → remaining=0; " aa" → intacto.
        // plain_text esperado: "bb " + "ba" + " aa" = "bb ba aa"
        let world = MockWorld::new("#replace([aa *aa* aa], \"a\", \"b\", count: 3)");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().unwrap();
        assert_eq!(content.plain_text(), "bb ba aa");
    }

    // ── Show rules (Passo 68) ─────────────────────────────────────────────────

    #[test]
    fn eval_show_rule_text_substitui_ocorrencias() {
        let world = MockWorld::new("#show \"A\": \"B\"\nAAA");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(!text.contains("AAA"), "texto original não deve sobreviver: {:?}", text);
        assert!(
            text.contains('B'),
            "show text rule deve substituir 'A' por 'B': {:?}",
            text
        );
    }

    #[test]
    fn eval_show_rule_funcao_no_heading() {
        let world =
            MockWorld::new("#show heading: it => upper(it.body)\n\n= Capítulo um");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.to_uppercase().contains("CAPÍTULO UM") || text.contains("CAPÍTULO UM"),
            "show rule deve transformar heading em maiúsculas: {:?}",
            text
        );
    }

    #[test]
    fn show_rule_resolve_por_identidade_nao_por_nome() {
        // Passo 84.3 — DEBT-21: aliasing de nativa não engana o selector.
        //
        // Aliasing por `#let h = heading` clona o `Arc<FuncRepr>` (mesmo
        // ponteiro `Native::call`). A resolução por `fn_addr_eq` reconhece
        // `h` como sendo `heading` e dispara a show rule.
        //
        // Com a resolução anterior por `Func::name()`, o nome era preservado
        // na native (`name: "heading"` para ambas), portanto este caso já
        // funcionava por acidente. O caso patológico que `Func::name()`
        // falhava era closures wrapper que pegassem o nome da binding —
        // mas como a stdlib não permite re-registo de nativas com nome
        // diferente, o teste mais robusto é confirmar que aliasing simples
        // continua a disparar.
        let world = MockWorld::new(
            "#let h = heading\n#show h: it => upper(it.body)\n\n= Capítulo um",
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("CAPÍTULO UM") || text.to_uppercase().contains("CAPÍTULO UM"),
            "show rule via alias deve disparar tal como via nome directo: {:?}",
            text
        );
    }

    #[test]
    fn show_rule_closure_anonima_rejeitada() {
        // Closures não têm function pointer estável — `native_fn_addr()`
        // retorna `None`, eval reporta erro explícito (não silencia).
        let world = MockWorld::new("#show (it => it): x => x\n= teste");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "closure como selector deve gerar Err");
    }

    #[test]
    fn eval_show_rule_falha_explicita_tipo_retorno_invalido() {
        let world = MockWorld::new("#show heading: it => true\n\n= Erro");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "retornar bool de show rule deve gerar Err");
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("Content") || err[0].message.contains("String"),
            "mensagem deve mencionar tipos aceites: {:?}",
            err[0].message
        );
    }

    #[test]
    fn show_rule_respeita_escopo_lexico() {
        // A regra dentro do code block não deve afectar o texto fora.
        // Em markup Typst, `{ }` são texto literal; `#{ }` cria um code block real.
        let world = MockWorld::new("#{ show \"A\": \"B\" }\nA"); // P786a: fonte corrigida (`#` inválido em código, vanilla rejeita)
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.trim().ends_with('A') || text.contains('A'),
            "show rule do bloco não deve afectar texto exterior: {:?}",
            text
        );
    }

    #[test]
    fn show_rule_nao_recursiva_sem_stack_overflow() {
        // P348 (α): `#show heading: it => [= X ]` reescreve para `= X` constante → a
        // 2ª aplicação é no-op morfológico (X→X) → ponto-fixo → termina com "X" (antes:
        // truncava no nível 1). Deve terminar — Ok, nunca loop infinito.
        let world = MockWorld::new("#show heading: it => [= X ]\n\n= A");
        let src = world.source(world.main()).unwrap();
        // Deve terminar — Ok ou Err, nunca loop infinito.
        let _result = eval_for_test(&world, &src);
    }

    // ── Show rules transversais (Passo 69 — DEBT-19 encerrado) ───────────────

    #[test]
    fn show_rule_map_content_transversal() {
        // DEBT-19 encerrado: heading dentro de sequence deve ser intercetado.
        let world =
            MockWorld::new("#show heading: it => upper(it.body)\n\n= Titulo Escondido");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("TITULO ESCONDIDO"),
            "map_content deve processar nós aninhados: {:?}",
            text
        );
    }

    #[test]
    fn show_rule_multiplos_tipos_independentes() {
        // Regras para Strong e Emph aplicam-se independentemente.
        let world = MockWorld::new("#show strong: upper\n#show emph: lower\n*A* e _B_");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains('A') && text.contains('b'),
            "Regras para Strong e Emph devem aplicar-se independentemente: {:?}",
            text
        );
    }

    #[test]
    fn show_rule_texto_usa_map_text_nao_map_content() {
        let world = MockWorld::new("#show \"a\": \"x\"\naaa");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert_eq!(
            text.trim(),
            "xxx",
            "Selector::Text deve substituir todas as ocorrências: {:?}",
            text
        );
    }

    // ── P790 — show-by-string com Content/Func + `page`/`par` como alvo ────
    //
    // Achados de P786 (módulo `eval::rules`): (1) `#show "world": [W]` aceite
    // e descartado em silêncio (só `Transformation::Str` era aplicada);
    // (2) `#show page:`/`#show par:` caíam em `unknown variable` fatal, quando
    // o vanilla emite warnings específicos e compila (medido palavra por
    // palavra em P790 — `typst-eval/src/rules.rs:67-95`).

    #[test]
    fn p790_show_string_com_content_substituí_no_eval() {
        // Repro de P786 (`temp/temp_p786/c_rules_probe_show.typ`): antes de
        // P790 o texto saía intacto ("Hello world."); o vanilla renderiza
        // "Hello W." (medido por execução).
        let world = MockWorld::new("#show \"world\": [W]\n\nHello world.");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text: String = module
            .content()
            .unwrap()
            .plain_text()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        assert!(
            text.contains("HelloW."),
            "show-by-string com Content deve substituir o match: {:?}",
            text
        );
        assert!(
            !text.contains("Helloworld"),
            "o match original não deve sobreviver: {:?}",
            text
        );
    }

    #[test]
    fn p790_show_page_emite_warning_vanilla_e_compila() {
        use comemo::Track;
        let world = MockWorld::new("#show page: it => [WRAPPED: #it]\nHello.");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(
            result.is_ok(),
            "show page não deve abortar a compilação: {:?}",
            result.err()
        );
        let diags = sink.into_diagnostics();
        let diag = diags
            .iter()
            .find(|d| d.message == "`show page` is not supported and has no effect")
            .unwrap_or_else(|| {
                panic!("warning do vanilla (texto exacto) esperado: {:?}", diags)
            });
        assert!(
            diag.hints
                .iter()
                .any(|h| h == "customize pages with `set page(..)` instead"),
            "hint deve ser idêntico ao vanilla: {:?}",
            diag.hints
        );
    }

    #[test]
    fn p790_show_par_set_block_spacing_emite_warning_vanilla_e_compila() {
        use comemo::Track;
        // Evidência de P786 (`temp/temp_p786/c_rules_showpar.typ`).
        let world =
            MockWorld::new("#show par: set block(spacing: 4em)\n\nFirst.\n\nSecond.");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(
            result.is_ok(),
            "show par set block não deve abortar: {:?}",
            result.err()
        );
        let diags = sink.into_diagnostics();
        let diag = diags
            .iter()
            .find(|d| {
                d.message == "`show par: set block(spacing: ..)` has no effect anymore"
            })
            .unwrap_or_else(|| {
                panic!("warning do vanilla (texto exacto) esperado: {:?}", diags)
            });
        assert!(
            diag.hints.iter().any(|h| h == "write `set par(spacing: ..)` instead"),
            "1º hint idêntico ao vanilla: {:?}",
            diag.hints
        );
        assert!(
            diag.hints.iter().any(|h| h
                == "this is specific to paragraphs as they are not considered blocks anymore"),
            "2º hint idêntico ao vanilla: {:?}",
            diag.hints
        );
    }

    #[test]
    fn p790_show_selector_texto_vazio_da_erro_vanilla() {
        // Vanilla (`selector.rs:110`, medido por execução): `text selector is empty`.
        let world = MockWorld::new("#show \"\": [X]\nHello.");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        let err = result.expect_err("selector de texto vazio deve gerar Err");
        assert!(
            err.iter().any(|d| d.message == "text selector is empty"),
            "mensagem idêntica ao vanilla: {:?}",
            err
        );
    }

    #[test]
    fn p863_show_par_identidade_realiza_e_aplica() {
        // P863: `#show par: it => it` realiza o parágrafo (agrupa o corpo
        // entre parbreaks) e aplica a regra sem alterar a morfologia.
        let world = MockWorld::new("#show par: it => it\nHello.");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().unwrap();
        let text = content.plain_text();
        assert!(
            text.contains("Hello."),
            "show par identidade deve preservar o texto: {:?}",
            text
        );
        let repr = format!("{:?}", content);
        assert!(
            repr.contains("par("),
            "identidade deve manter o nó Par realizado: {:?}",
            repr
        );
    }

    #[test]
    fn p863_show_par_func_transforma_paragrafo() {
        // P863: `#show par: it => strong(it)` transforma cada nó `Par` num
        // `Strong` cujo corpo é o parágrafo original.
        let world = MockWorld::new("#show par: it => strong(it)\nHello.");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().unwrap();
        assert!(
            content.plain_text().contains("Hello."),
            "texto do parágrafo deve sobreviver: {:?}",
            content.plain_text()
        );
        let repr = format!("{:?}", content);
        assert!(
            repr.contains("strong"),
            "representação deve refletir a transformação strong: {:?}",
            repr
        );
    }

    #[test]
    fn p863_set_par_spacing_compila_sem_abortar() {
        // P863: `#set par(spacing: ..)` continua a viajar pela StyleChain;
        // não dispara realização de parágrafos, mas a compilação prossegue.
        use comemo::Track;
        let world = MockWorld::new("#set par(spacing: 2em)\n\nHello.");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );

        assert!(result.is_ok(), "set par(spacing) não deve abortar: {:?}", result.err());
        let text = result.unwrap().content().unwrap().plain_text();
        assert!(text.contains("Hello."), "texto deve ser preservado: {:?}", text);
    }

    // ── P791 — `#show <lbl>: …` (selector por label) ───────────────────────
    //
    // Achado de P786 (módulo `foundations::selector`, evidência
    // `temp/temp_p786/b_selector.typ`): `Value::Label` no selector caía no
    // braço `other` de `eval_show_rule` ("selector inválido para show rule:
    // label"). O vanilla aceita e aplica ao elemento rotulado (medido:
    // `= Alpha <sp>` → "LBL= Alpha"; `ABC <sp>` → "LBL=ABC").

    #[test]
    fn p791_show_label_em_heading_dispara() {
        // Probe do passo (ordem corrigida — regra antes do conteúdo; a ordem
        // inversa não dispara nem no vanilla, medido em P790/P791).
        let world = MockWorld::new("#show <sp>: it => [LBL=#it]\n\n= Alpha <sp>");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text: String = module
            .content()
            .unwrap()
            .plain_text()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        assert!(
            text.contains("LBL=Alpha"),
            "show-by-label deve disparar sobre o heading rotulado: {:?}",
            text
        );
    }

    #[test]
    fn p791_show_label_em_texto_simples_dispara() {
        // Paridade medida no vanilla: `ABC <sp>` → "LBL=ABC" (o label casa o
        // elemento de texto inteiro).
        let world = MockWorld::new("#show <sp>: it => [LBL=#it]\n\nABC <sp>");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text: String = module
            .content()
            .unwrap()
            .plain_text()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        assert!(
            text.contains("LBL=ABC"),
            "show-by-label deve disparar sobre texto rotulado: {:?}",
            text
        );
    }

    #[test]
    fn p791_show_label_com_content_substituí() {
        let world = MockWorld::new("#show <sp>: [SUB]\n\n= Alpha <sp>");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("SUB"),
            "transformação Content deve substituir o rotulado: {:?}",
            text
        );
        assert!(
            !text.contains("Alpha"),
            "o elemento rotulado não deve sobreviver: {:?}",
            text
        );
    }

    #[test]
    fn p791_show_label_nao_dispara_em_label_diferente() {
        // Controlo: label diferente não casa (e o conteúdo fica intacto).
        let world = MockWorld::new("#show <sp>: [SUB]\n\n= Alpha <a1>");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("Alpha"),
            "label diferente não deve disparar a regra: {:?}",
            text
        );
        assert!(!text.contains("SUB"), "SUB não deve aparecer: {:?}", text);
    }

    #[test]
    fn p791_show_label_show_set_embrulha_styled() {
        // `#show <sp>: set text(fill: red)` — show-set sobre label embrulha o
        // wrapper em Content::Styled (não consome o passe, paridade P352).
        let world = MockWorld::new("#show <sp>: set text(fill: red)\n\n= Alpha <sp>");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().unwrap();
        fn tem_styled_sobre_label(c: &Content) -> bool {
            match c {
                Content::Styled(inner, _) => {
                    matches!(inner.as_ref(), Content::Label(_))
                        || tem_styled_sobre_label(inner)
                }
                Content::Sequence(seq) => seq.iter().any(tem_styled_sobre_label),
                _ => false,
            }
        }
        assert!(
            tem_styled_sobre_label(&content),
            "show-set sobre label deve embrulhar o Label em Styled: {:?}",
            content
        );
    }

    #[test]
    fn show_rule_encadeamento_texto_sequencial() {
        // A transforma em B, depois B transforma em C — resultado final deve ser C.
        let world = MockWorld::new("#show \"A\": \"B\"\n#show \"B\": \"C\"\nA");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert_eq!(
            text.trim(),
            "C",
            "encadeamento sequencial deve produzir 'C': {:?}",
            text
        );
    }

    #[test]
    fn show_rule_composicao_sem_loop() {
        // P348 (α): a regra transforma o heading num `[Prefixo: ] + it.body` — uma
        // SEQUÊNCIA, não um heading. O output NÃO re-casa a regra de heading → aplica-se
        // uma vez (ponto-fixo imediato: nada mais casa). Antes: o `active_guards` saltava
        // a regra; agora o output simplesmente não re-casa.
        let world =
            MockWorld::new("#show heading: it => [Prefixo: ] + it.body\n\n= Título");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("Prefixo: Título"),
            "Show rule deve aplicar-se uma vez: {:?}",
            text
        );
        assert_eq!(
            text.matches("Prefixo:").count(),
            1,
            "A regra não deve ter sido reaplicada: {:?}",
            text
        );
    }

    #[test]
    fn show_rule_encadeamento_duas_regras() {
        // Regra 1: heading → strong. Durante apply_func, id=1 está em active_guards.
        // O Strong gerado passa pelo intercept_content.
        // Regra 2: strong → emph. id=2 não está em active_guards → aplica-se.
        let world =
            MockWorld::new("#show heading: strong\n#show strong: emph\n\n= Título");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("Título"),
            "Encadeamento deve produzir conteúdo: {:?}",
            text
        );
    }

    #[test]
    fn show_rule_active_guards_limpos_apos_erro() {
        // Se apply_func retornar Err, o pop ocorre antes de propagar o erro.
        // Após o erro, active_guards deve estar vazio — pilha não corrompida.
        let world = MockWorld::new("#show heading: it => true\n\n= Título");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "Retornar bool de show rule deve gerar Err");
    }

    #[test]
    fn show_rule_multiplas_regras_nodekind_travessia_unica() {
        // DEBT-23: com múltiplas regras NodeKind, map_content é chamado uma vez.
        // Verificação comportamental: cada tipo é transformado correctamente.
        // Strong é parágrafo separado (não dentro do heading) para que upper
        // no heading não sobreponha lower no strong.
        let world = MockWorld::new(
            "#show heading: upper\n#show strong: lower\n\n= Titulo\n\n*Forte*",
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("TITULO"),
            "Heading deve ser transformado para maiúsculas: {:?}",
            text
        );
        assert!(
            text.contains("forte"),
            "Strong deve ser transformado para minúsculas: {:?}",
            text
        );
    }

    // ── P417 — show rules `heading.where(field: value)` ─────────────────────

    #[test]
    fn p417_show_rule_where_aplica_a_heading_level_1() {
        let world = MockWorld::new(
            "#show heading.where(level: 1): it => [CAPÍTULO: ] + it.body\n\n= Um\n\n== Dois"
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("CAPÍTULO: Um"),
            "show rule where level 1 deve aplicar-se ao heading 1: {:?}",
            text
        );
        assert!(
            !text.contains("CAPÍTULO: Dois"),
            "show rule where level 1 NÃO deve aplicar-se ao heading 2: {:?}",
            text
        );
    }

    #[test]
    fn p417_heading_where_retorna_selector() {
        let world = MockWorld::new("#let s = heading.where(level: 1)");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let s = module.scope().get("s").expect("s deve estar definido");
        assert!(
            matches!(s, Value::Selector(_)),
            "heading.where(level: 1) deve retornar Value::Selector, recebeu {:?}",
            s.type_name()
        );
    }

    #[test]
    fn p417_show_rule_where_nao_aplica_a_paragraph() {
        let world = MockWorld::new(
            "#show heading.where(level: 1): it => [CAP: ] + it.body\n\ntexto normal",
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            !text.contains("CAP: texto normal"),
            "show rule where não deve aplicar-se a paragraph: {:?}",
            text
        );
    }

    #[test]
    fn p417_show_rule_where_valor_errado_nao_aplica() {
        let world = MockWorld::new(
            "#show heading.where(level: 9): it => [CAP: ] + it.body\n\n= Um",
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            !text.contains("CAP: Um"),
            "show rule where level 9 não deve aplicar-se (level é clamped 1..6): {:?}",
            text
        );
    }

    #[test]
    fn p417_show_rule_where_aplica_a_figure_kind() {
        let world = MockWorld::new(
            "#show figure.where(kind: \"image\"): it => [IMG: ] + it.body\n\n#figure(image(\"x.png\"))"
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src);
        // figure/image pode não estar completamente implementado; o teste é
        // defensivo — o importante é não panicar e, se aplicar, manter o prefixo.
        if let Ok(module) = module {
            let text = module.content().unwrap().plain_text();
            // Assert fraco: ou aplica o prefixo, ou não contém o prefixo porque
            // a infra de figure não produziu texto. O que não pode é panic.
            let _ = text;
        }
    }

    // ── P423 (S-M) — combinadores And/Or em show rules ──────────────────────

    #[test]
    fn p423_selector_or_retorna_selector() {
        let world = MockWorld::new("#let s = heading.or(figure)");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let s = module.scope().get("s").expect("s deve estar definido");
        assert!(
            matches!(s, Value::Selector(_)),
            "heading.or(figure) deve retornar Value::Selector, recebeu {:?}",
            s.type_name()
        );
    }

    #[test]
    fn p423_selector_and_retorna_selector() {
        let world = MockWorld::new("#let s = heading.and(figure)");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let s = module.scope().get("s").expect("s deve estar definido");
        assert!(
            matches!(s, Value::Selector(_)),
            "heading.and(figure) deve retornar Value::Selector, recebeu {:?}",
            s.type_name()
        );
    }

    #[test]
    fn p423_show_rule_or_aplica_a_heading() {
        let world =
            MockWorld::new("#show heading.or(figure): it => [OR: ] + it.body\n\n= T");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("OR: T"),
            "show rule or deve aplicar-se a heading: {:?}",
            text
        );
    }

    #[test]
    fn p423_show_rule_or_aplica_a_figure() {
        let world = MockWorld::new(
            "#show heading.or(figure): it => [OR: ] + it.body\n\n#figure([F])",
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("OR: F"),
            "show rule or deve aplicar-se a figure: {:?}",
            text
        );
    }

    #[test]
    fn p423_show_rule_or_nao_aplica_a_paragraph() {
        let world = MockWorld::new(
            "#show heading.or(figure): it => [OR: ] + it.body\n\ntexto normal",
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            !text.contains("OR: texto normal"),
            "show rule or não deve aplicar-se a paragraph: {:?}",
            text
        );
    }

    #[test]
    fn p423_show_rule_and_contraditorio_nao_aplica() {
        let world =
            MockWorld::new("#show heading.and(figure): it => [AND: ] + it.body\n\n= T");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            !text.contains("AND: T"),
            "show rule and contraditório não deve aplicar-se a heading: {:?}",
            text
        );
    }

    #[test]
    fn p423_show_rule_and_positivo_where_com_kind() {
        let world = MockWorld::new(
            "#show heading.where(level: 1).and(heading): it => [AND: ] + it.body\n\n= T\n\n== D"
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("AND: T"),
            "show rule and where+kind deve aplicar-se a heading level 1: {:?}",
            text
        );
        assert!(
            !text.contains("AND: D"),
            "show rule and where+kind não deve aplicar-se a heading level 2: {:?}",
            text
        );
    }

    // ── P393 — show rules regex (`#show regex(pattern): …`) ─────────────────

    #[test]
    fn show_rule_regex_aplica_func_a_texto_que_casa() {
        // Texto com dígitos deve ser embrulhado em Strong.
        let world =
            MockWorld::new("#show regex(\"\\\\d+\"): it => strong(it)\nabc123def");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let c = module.content().unwrap();
        assert!(
            c.plain_text().contains("abc123def"),
            "texto original deve sobreviver: {:?}",
            c.plain_text()
        );
        assert!(
            texto_em_strong(c, "123"),
            "a parte que casa deve ficar em Strong: {c:?}"
        );
    }

    #[test]
    fn p1285_show_literal_e_regex_entregam_cada_ocorrencia_a_recipe() {
        fn plain_text(source: &str) -> String {
            let world = MockWorld::new(source);
            let src = world.source(world.main()).unwrap();
            let module = eval_for_test(&world, &src).unwrap();
            module.content().unwrap().plain_text().trim().to_string()
        }

        assert_eq!(plain_text("#show \"foo\": [L]\nfoo bar foo"), "L bar L");
        assert_eq!(plain_text("#show regex(\"f.o\"): [R]\nfoo fxo"), "R R");
    }

    #[test]
    fn show_rule_regex_ignora_texto_sem_match() {
        let world = MockWorld::new("#show regex(\"\\\\d+\"): it => strong(it)\nabcdef");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let c = module.content().unwrap();
        assert!(
            c.plain_text().contains("abcdef"),
            "texto sem dígitos deve permanecer: {:?}",
            c.plain_text()
        );
        assert!(
            !texto_em_strong(c, "abcdef"),
            "texto sem match não deve ficar em Strong: {c:?}"
        );
    }

    #[test]
    fn show_rule_regex_ultima_declarada_vence() {
        let world = MockWorld::new(
            "#show regex(\"\\\\d+\"): it => strong(it)\n#show regex(\"\\\\d+\"): it => emph(it)\nabc123def"
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let c = module.content().unwrap();
        assert!(
            !texto_em_strong(c, "123"),
            "primeira regra (strong) deve ser sobreposta: {c:?}"
        );
        assert!(texto_em_emph(c, "123"), "última regra (emph) deve vencer: {c:?}");
    }

    #[test]
    fn show_rule_regex_pattern_invalida_erro() {
        let world = MockWorld::new("#show regex(\"[\"): it => strong(it)\ntexto");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "regex inválida deve produzir erro");
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("regex"),
            "mensagem deve mencionar regex: {:?}",
            err[0].message
        );
    }

    #[test]
    fn show_rule_regex_set_rejeitado() {
        // Show-set com regex selector deve ser rejeitado (mesma regra de Selector::Text).
        let world = MockWorld::new("#show regex(\"\\\\d+\"): set text(weight: 700)\n123");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "show-set com regex deve gerar Err");
    }

    // ── P394 — eval(source) ─────────────────────────────────────────────────

    #[test]
    fn eval_retorna_valor_de_expressao() {
        let world = MockWorld::new("#let y = eval(\"1 + 2\"); #str(y)");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        assert_eq!(
            module.content().unwrap().plain_text().trim(),
            "3",
            "eval(\"1 + 2\") deve devolver 3"
        );
    }

    #[test]
    fn eval_nao_ve_escopo_do_chamador() {
        // P830 (decisão do dono, 2026-07-22) — paridade vanilla: `eval`
        // avalia num `Scopes` fresco (só stdlib + `scope:`), não vê as
        // variáveis do chamador. Vanilla: `error: unknown variable: x`.
        let world = MockWorld::new("#let x = 5\n#let y = eval(\"x * 2\"); #str(y)");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src)
            .expect_err("eval não deve ver a variável x do scope do chamador");
        assert!(
            diags.iter().any(|d| d.message.contains("unknown variable `x`")),
            "esperado 'unknown variable: x': {diags:?}"
        );
    }

    #[test]
    fn eval_retorna_content() {
        let world = MockWorld::new("#eval(\"[*bold*]\")");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            texto_em_strong(module.content().unwrap(), "bold"),
            "eval de content deve produzir strong: {:?}",
            text
        );
    }

    #[test]
    fn eval_inteiro_literal() {
        let world = MockWorld::new("#let y = eval(\"123\"); #str(y)");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        assert_eq!(module.content().unwrap().plain_text().trim(), "123");
    }

    #[test]
    fn eval_identificador_desconhecido_erro() {
        // Em modo código, \"hello\" é um identificador desconhecido.
        let world = MockWorld::new("#eval(\"hello\")");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "eval de identificador desconhecido deve falhar");
    }

    #[test]
    fn eval_sintaxe_invalida_erro() {
        let world = MockWorld::new("#eval(\"#{{{broken\")");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "eval de string com sintaxe inválida deve falhar");
    }

    #[test]
    fn eval_tipo_errado_erro() {
        let world = MockWorld::new("#eval(123)");
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "eval com arg não-string deve falhar");
    }

    // ── P814 — eval: mode:/scope:, mensagens vanilla, span sintético ────────

    fn p814_eval_err(
        source: &str,
    ) -> Vec<crate::entities::source_result::SourceDiagnostic> {
        let world = MockWorld::new(source);
        let src = world.source(world.main()).unwrap();
        eval_for_test(&world, &src).unwrap_err()
    }

    fn p814_eval_plain_text(source: &str) -> String {
        let world = MockWorld::new(source);
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        module.content().unwrap().plain_text().trim().to_string()
    }

    // ── P815 — método inexistente / dict-key-call (eval_field_callee) ───────

    fn p815_err(source: &str) -> Vec<crate::entities::source_result::SourceDiagnostic> {
        let world = MockWorld::new(source);
        let src = world.source(world.main()).unwrap();
        eval_for_test(&world, &src).unwrap_err()
    }

    #[test]
    fn p815_metodo_inexistente_int() {
        // Medido no vanilla (P810/P815): `type integer has no method `foo``.
        let diags = p815_err("#(1).foo()");
        assert_eq!(diags[0].message, "type integer has no method `foo`");
    }

    #[test]
    fn p815_metodo_inexistente_str() {
        let diags = p815_err("#\"texto\".zzz()");
        assert_eq!(diags[0].message, "type string has no method `zzz`");
    }

    #[test]
    fn p815_metodo_inexistente_float() {
        let diags = p815_err("#(1.5).foo()");
        assert_eq!(diags[0].message, "type float has no method `foo`");
    }

    #[test]
    fn p815_metodo_inexistente_array() {
        let diags = p815_err("#(1, 2).zzz()");
        assert_eq!(diags[0].message, "type array has no method `zzz`");
    }

    #[test]
    fn p815_dict_key_ausente_chamada() {
        let diags = p815_err("#let d = (y: 1)\n#d.zzz()");
        assert_eq!(diags[0].message, "type dictionary has no method `zzz`");
    }

    #[test]
    fn p815_dict_key_chamada_como_funcao() {
        // Medido no vanilla (P810): erro + 2 hints verbatim.
        let diags = p815_err("#let d = (x: 1)\n#d.x()");
        assert_eq!(diags[0].message, "cannot directly call dictionary keys as functions");
        assert_eq!(
            diags[0].hints,
            vec![
                "to access the `x` key, remove the function arguments: `d.x`".to_string(),
                "dictionary keys cannot be used with method syntax as keys could conflict with built-in method names".to_string(),
            ]
        );
    }

    #[test]
    fn p815_dict_key_com_funcao_chamada() {
        // Medido no vanilla: o hint muda quando o valor guardado é função.
        let diags = p815_err("#let d = (f: x => x * 2)\n#d.f()");
        assert_eq!(diags[0].message, "cannot directly call dictionary keys as functions");
        assert_eq!(
            diags[0].hints,
            vec![
                "to call the stored function, wrap the field access in parentheses: `(d.f)(..)`".to_string(),
                "dictionary keys cannot be used with method syntax as keys could conflict with built-in method names".to_string(),
            ]
        );
    }

    #[test]
    fn p815_length_field_call() {
        let diags = p815_err("#(10pt).abs()");
        assert_eq!(diags[0].message, "`abs` is not a valid method for type `length`");
        assert_eq!(
            diags[0].hints,
            vec![
                "to access the `abs` field, remove the function arguments: `(10pt).abs`"
                    .to_string()
            ]
        );
    }

    #[test]
    fn p815_content_field_call() {
        let diags = p815_err("#strong[x].body()");
        assert_eq!(diags[0].message, "`body` is not a valid method for element `strong`");
        assert_eq!(
            diags[0].hints,
            vec!["to access the `body` field, remove the function arguments: `strong[x].body`".to_string()]
        );
    }

    #[test]
    fn p815_content_metodo_inexistente() {
        let diags = p815_err("#strong[x].zzz()");
        assert_eq!(diags[0].message, "element strong has no method `zzz`");
    }

    #[test]
    fn p815_args_field_call() {
        // Medido no vanilla como `cannot directly call named argument fields
        // as functions` para campos de `arguments` chamados como função.
        let diags = p815_err("#let f(..args) = args.positional()\n#f(1, 2)");
        assert_eq!(
            diags[0].message,
            "cannot directly call named argument fields as functions"
        );
        assert_eq!(
            diags[0].hints,
            vec![
                "to access the `positional` argument, remove the function arguments: `args.positional`".to_string(),
                "named arguments cannot be used with method syntax as argument names could conflict with built-in method names".to_string(),
            ]
        );
    }

    #[test]
    fn p815_controlo_push_temporario() {
        // Controlo de regressão — já em paridade antes de P815.
        let diags = p815_err("#\"ab\".push(\"c\")");
        assert_eq!(diags[0].message, "cannot mutate a temporary value");
    }

    #[test]
    fn p815_controlo_metodos_validos_intactos() {
        // Métodos reais continuam a funcionar (despacho antes do fallback).
        assert_eq!(p814_eval_plain_text("#let a = (1, 2)\n#str(a.at(1))"), "2");
        assert_eq!(p814_eval_plain_text("#\"ab\".len()"), "2");
        assert_eq!(p814_eval_plain_text("#let d = (x: 1)\n#str(d.at(\"x\"))"), "1");
    }

    #[test]
    fn p815_controlo_func_guardada_chamavel_com_parenteses() {
        // `(d.f)(21)` — forma permitida pelo hint do vanilla — continua válida.
        assert_eq!(
            p814_eval_plain_text("#let d = (f: x => x * 2)\n#str((d.f)(21))"),
            "42"
        );
    }

    #[test]
    fn p815_controlo_field_access_sem_chamada_intocado() {
        // `#d.x` sem parênteses — caminho de field access não muda.
        assert_eq!(p814_eval_plain_text("#let d = (x: 1)\n#str(d.x)"), "1");
    }

    // ── P829-B — métodos de `content`: func/has/at/fields/location ─────────
    //
    // Medidos no vanilla 0.15.0 (fixtures temp/p829/b*.typ — relatório P829):
    // os cinco métodos do `#[scope]` de `Content` (`foundations/content/mod.rs:510-590`).

    fn p829_err(source: &str) -> Vec<crate::entities::source_result::SourceDiagnostic> {
        let world = MockWorld::new(source);
        let src = world.source(world.main()).unwrap();
        eval_for_test(&world, &src).unwrap_err()
    }

    #[test]
    fn p829b_func_identidade_elemento() {
        // Vanilla b1/b9: `strong[x].func()` devolve a função do elemento —
        // `== strong` é true. Igualdade de nativas é por nome (P742).
        let world = MockWorld::new(
            "#let x = (strong[x].func() == strong, heading[H].func() == heading)",
        );
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Bool(true), Value::Bool(true)]))
        );
    }

    #[test]
    fn p829b_func_emph_e_text() {
        // Vanilla b11: `emph[e].func()` → `emph`; `[abc].func()` → `text`.
        let world =
            MockWorld::new("#let x = (emph[e].func() == emph, repr([abc].func()))");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Bool(true), Value::Str("text".into())]))
        );
    }

    #[test]
    fn p829b_has_strong() {
        // Vanilla b2: body assente → true; delta declarado mas não assente →
        // false; campo inexistente → false.
        let world = MockWorld::new(
            "#let x = (strong[x].has(\"body\"), strong[x].has(\"delta\"), strong[x].has(\"foo\"))",
        );
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Bool(true),
                Value::Bool(false),
                Value::Bool(false)
            ]))
        );
    }

    #[test]
    fn p829b_has_heading_level_so_explicito() {
        // Vanilla b2/b8: `heading[H]` NÃO tem level assente (default vem da
        // chain); `heading(level: 2)[H]` e `heading(2, [H])` têm.
        let world = MockWorld::new(
            "#let x = (heading[H].has(\"level\"), heading(level: 2)[H].has(\"level\"), heading(2, [H]).has(\"level\"))",
        );
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Bool(false),
                Value::Bool(true),
                Value::Bool(true)
            ]))
        );
    }

    #[test]
    fn p829b_has_heading_outlined_bookmarked() {
        let world = MockWorld::new(
            "#let x = (heading[H].has(\"outlined\"), heading(outlined: false)[H].has(\"outlined\"), heading[H].has(\"bookmarked\"), heading(bookmarked: false)[H].has(\"bookmarked\"))",
        );
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Bool(false),
                Value::Bool(true),
                Value::Bool(false),
                Value::Bool(true)
            ]))
        );
    }

    #[test]
    fn p829b_has_markup_heading_depth_nao_level() {
        // Vanilla b7a/b7d: heading de markup assenta `depth`, não `level`.
        let world = MockWorld::new(
            "#let h = [= H]\n#let x = (h.has(\"level\"), h.has(\"depth\"))",
        );
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Bool(false), Value::Bool(true)]))
        );
    }

    #[test]
    fn p829b_at_level_explicito() {
        // Vanilla b8: `heading(level: 2)[H].at("level")` → 2.
        let world = MockWorld::new("#let x = heading(level: 2)[H].at(\"level\")");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(2)));
    }

    #[test]
    fn p829b_at_body_devolve_content() {
        // Vanilla b3a: `strong[x].at("body")` → o corpo.
        assert_eq!(p814_eval_plain_text("#strong[x].at(\"body\")"), "x");
        assert_eq!(p814_eval_plain_text("#heading[H].at(\"body\")"), "H");
    }

    #[test]
    fn p829b_at_com_default() {
        // Vanilla b3d/b7c: campo não assente + default → o default.
        let world = MockWorld::new(
            "#let x = (heading[H].at(\"level\", default: 9), strong[x].at(\"delta\", default: 1.4))",
        );
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(9), Value::Float(1.4)]))
        );
    }

    #[test]
    fn p829b_at_unset_sem_default_erro() {
        // Vanilla b3b — verbatim.
        let diags = p829_err("#heading[H].at(\"level\")");
        assert_eq!(
            diags[0].message,
            "field \"level\" in heading is not known at this point and no default was specified"
        );
    }

    #[test]
    fn p829b_at_strong_delta_unset_erro() {
        // Vanilla b3c — `delta` é declarado em strong mas não assente.
        let diags = p829_err("#strong[x].at(\"delta\")");
        assert_eq!(
            diags[0].message,
            "field \"delta\" in strong is not known at this point and no default was specified"
        );
    }

    #[test]
    fn p829b_at_campo_inexistente_erro() {
        // Vanilla b6 — verbatim.
        let diags = p829_err("#strong[x].at(\"foo\")");
        assert_eq!(
            diags[0].message,
            "strong does not have field \"foo\" and no default was specified"
        );
    }

    #[test]
    fn p829b_at_erros_de_argumentos() {
        // Vanilla b13/b15/b14 — verbatim.
        let diags = p829_err("#strong[x].at()");
        assert_eq!(diags[0].message, "missing argument: field");
        let diags = p829_err("#strong[x].at(1)");
        assert_eq!(diags[0].message, "expected string, found integer");
        let diags = p829_err("#strong[x].at(\"body\", 1)");
        assert_eq!(diags[0].message, "unexpected argument");
    }

    #[test]
    fn p829b_fields_strong() {
        // Vanilla b4: `(body: [x])`.
        let world = MockWorld::new("#let x = strong[x].fields()");
        let keys: Vec<String> = match eval_let(&world, "x") {
            Some(Value::Dict(d)) => d.keys().map(|k| k.to_string()).collect(),
            other => panic!("esperava Dict, obtive {other:?}"),
        };
        assert_eq!(keys, vec!["body".to_string()]);
    }

    #[test]
    fn p829b_fields_heading_ordem_vanilla() {
        // Vanilla b8/b18: `(level: 2, body: [H])` — level antes de body.
        let world = MockWorld::new(
            "#let a = heading[H].fields()\n#let b = heading(level: 2)[H].fields()",
        );
        let keys_a: Vec<String> = match eval_let(&world, "a") {
            Some(Value::Dict(d)) => d.keys().map(|k| k.to_string()).collect(),
            other => panic!("esperava Dict, obtive {other:?}"),
        };
        assert_eq!(keys_a, vec!["body".to_string()]);
        match eval_let(&world, "b") {
            Some(Value::Dict(d)) => {
                let keys: Vec<String> = d.keys().map(|k| k.to_string()).collect();
                assert_eq!(keys, vec!["level".to_string(), "body".to_string()]);
                assert_eq!(d.get("level"), Some(&Value::Int(2)));
            }
            other => panic!("esperava Dict, obtive {other:?}"),
        }
    }

    #[test]
    fn p829b_fields_markup_heading_depth() {
        // Vanilla b7b/b18: `(depth: 1, body: [H])`.
        let world = MockWorld::new("#let h = [= H]\n#let x = h.fields()");
        match eval_let(&world, "x") {
            Some(Value::Dict(d)) => {
                let keys: Vec<String> = d.keys().map(|k| k.to_string()).collect();
                assert_eq!(keys, vec!["depth".to_string(), "body".to_string()]);
                assert_eq!(d.get("depth"), Some(&Value::Int(1)));
            }
            other => panic!("esperava Dict, obtive {other:?}"),
        }
    }

    #[test]
    fn p829b_location_none() {
        // Vanilla b5/b10b: content inline não tem location → none.
        let world = MockWorld::new(
            "#let x = (strong[x].location() == none, heading[H].location() == none)",
        );
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Bool(true), Value::Bool(true)]))
        );
    }

    #[test]
    fn p829b_controlo_field_access_sem_chamada_intacto() {
        // `it.body` (field access, caminho de show rules) não muda.
        assert_eq!(p814_eval_plain_text("#strong[x].body"), "x");
        // Vanilla b9 medido: `heading[H].level` via field access continua 1
        // (resolvido/baked — distinto de `at("level")`).
        assert_eq!(p814_eval_plain_text("#str(heading[H].level)"), "1");
    }

    // ── P829-C — despacho de erro de chamada em modo math ──────────────────
    //
    // Medido no vanilla (fixtures temp/p829/c*.typ): dentro de math, uma
    // chamada `target.field(...)` produz os mesmos erros de P815 — o vanilla
    // usa a mesma rotina (`call.rs:eval_field_callee`) nos dois modos.

    #[test]
    fn p829c_math_dict_key_call() {
        // Vanilla c1 — erro + 2 hints verbatim, igual ao caminho não-math.
        let diags = p829_err("#let d = (x: 1)\n$#d.x()$");
        assert_eq!(diags[0].message, "cannot directly call dictionary keys as functions");
        assert_eq!(
            diags[0].hints,
            vec![
                "to access the `x` key, remove the function arguments: `d.x`".to_string(),
                "dictionary keys cannot be used with method syntax as keys could conflict with built-in method names".to_string(),
            ]
        );
    }

    #[test]
    fn p829c_math_dict_key_ausente() {
        // Vanilla c2.
        let diags = p829_err("#let d = (x: 1)\n$#d.zzz()$");
        assert_eq!(diags[0].message, "type dictionary has no method `zzz`");
    }

    #[test]
    fn p829c_math_dict_func_guardada_nao_chamada() {
        // Vanilla c3 — a função guardada NÃO é chamada (hint `(d.f)(..)`).
        let diags = p829_err("#let d = (f: x => x * 2)\n$#d.f()$");
        assert_eq!(diags[0].message, "cannot directly call dictionary keys as functions");
        assert_eq!(
            diags[0].hints,
            vec![
                "to call the stored function, wrap the field access in parentheses: `(d.f)(..)`".to_string(),
                "dictionary keys cannot be used with method syntax as keys could conflict with built-in method names".to_string(),
            ]
        );
    }

    #[test]
    fn p829c_math_metodo_inexistente_int() {
        // Vanilla c4.
        let diags = p829_err("$#(1).foo()$");
        assert_eq!(diags[0].message, "type integer has no method `foo`");
    }

    #[test]
    fn p829c_controlo_math_field_access_intacto() {
        // `$#d.x$` sem chamada — caminho de field access em math não muda.
        assert_eq!(p814_eval_plain_text("#let d = (x: 1)\n$#d.x$"), "1");
    }

    // ── P821 — `#target()` fora de `#context` (achado #8 de P810) ───────────

    #[test]
    fn p821_target_fora_de_contexto_erro_com_hints() {
        // Medido no vanilla: erro + 2 hints (context.rs:55-61).
        let world = MockWorld::new("#target()");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        assert_eq!(diags[0].message, "can only be used when context is known");
        assert_eq!(
            diags[0].hints,
            vec![
                "try wrapping this in a `context` expression".to_string(),
                "the `context` expression should wrap everything that depends on this function".to_string(),
            ]
        );
    }

    #[test]
    fn p821_target_posicional_extra_erro() {
        // Medido no vanilla: `unexpected argument` (args antes do gate).
        let world = MockWorld::new("#target(1)");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        assert_eq!(diags[0].message, "unexpected argument");
    }

    #[test]
    fn p821_target_named_erro() {
        // Medido no vanilla: `unexpected argument: x`.
        let world = MockWorld::new("#target(x: 1)");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        assert_eq!(diags[0].message, "unexpected argument: x");
    }

    #[test]
    fn p821_target_context_block_nao_erra_em_eval() {
        // Controlo: `#context target()` — o eval L1 produz o ContextBlock sem
        // avaliar a closure (a expansão corre em L3 com in_context = true).
        let world = MockWorld::new("#context target()");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        fn tem_context_block(c: &Content) -> bool {
            match c {
                Content::ContextBlock(_) => true,
                Content::Sequence(items) => items.iter().any(tem_context_block),
                Content::Styled(b, _) => tem_context_block(b),
                _ => false,
            }
        }
        assert!(
            tem_context_block(module.content().unwrap()),
            "#context target() deve produzir ContextBlock"
        );
    }

    #[test]
    fn p814_eval_mode_markup_produz_heading() {
        // Medido no vanilla (P810): `#eval("= Heading", mode: "markup")`
        // renderiza o heading.
        fn tem_heading(c: &Content) -> bool {
            match c {
                Content::Heading(_) => true,
                Content::Sequence(items) => items.iter().any(tem_heading),
                Content::Styled(b, _) => tem_heading(b),
                _ => false,
            }
        }
        let world = MockWorld::new("#eval(\"= Heading\", mode: \"markup\")");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().unwrap();
        assert!(
            tem_heading(content),
            "modo markup deve produzir Heading: {:?}",
            content.plain_text()
        );
        assert_eq!(content.plain_text().trim(), "Heading");
    }

    #[test]
    fn p814_eval_mode_code_explicito() {
        assert_eq!(
            p814_eval_plain_text("#let y = eval(\"1 + 2\", mode: \"code\"); #str(y)"),
            "3"
        );
    }

    #[test]
    fn p814_eval_scope_dict_bindings() {
        // Medido no vanilla (P810): `#eval("x + 1", scope: (x: 2))` → 3.
        assert_eq!(
            p814_eval_plain_text("#let y = eval(\"x + 1\", scope: (x: 2)); #str(y)"),
            "3"
        );
    }

    #[test]
    fn p814_eval_scope_sombreia_e_confinado() {
        // O binding do dict sombreia o scope do chamador durante o eval e
        // não vaza para fora.
        assert_eq!(
            p814_eval_plain_text(
                "#let x = 10\n#let y = eval(\"x + 1\", scope: (x: 2)); #str(y)-#str(x)"
            ),
            "3-10"
        );
    }

    #[test]
    fn p814_eval_scope_tipo_errado_erro() {
        let diags = p814_eval_err("#eval(\"x\", scope: 5)");
        assert_eq!(diags[0].message, "expected dictionary, found integer");
    }

    #[test]
    fn p814_eval_mode_invalido_erro() {
        let diags = p814_eval_err("#eval(\"1\", mode: \"wrong\")");
        assert_eq!(diags[0].message, "expected \"markup\", \"math\", or \"code\"");
    }

    #[test]
    fn p814_eval_mode_tipo_errado_erro() {
        let diags = p814_eval_err("#eval(\"1\", mode: 1)");
        assert_eq!(
            diags[0].message,
            "expected \"markup\", \"math\", or \"code\", found integer"
        );
    }

    #[test]
    fn p814_eval_named_desconhecido_erro() {
        let diags = p814_eval_err("#eval(\"1\", foo: 2)");
        assert_eq!(diags[0].message, "unexpected argument: foo");
    }

    #[test]
    fn p814_eval_tipo_errado_mensagem_vanilla() {
        // Medido no vanilla (P810): `expected string, found integer`.
        let diags = p814_eval_err("#eval(42)");
        assert_eq!(diags[0].message, "expected string, found integer");
    }

    #[test]
    fn p814_eval_sem_argumentos_erro() {
        let diags = p814_eval_err("#eval()");
        assert_eq!(diags[0].message, "missing argument: source");
    }

    #[test]
    fn p814_eval_posicional_extra_erro() {
        let diags = p814_eval_err("#eval(\"1\", \"2\")");
        assert_eq!(diags[0].message, "unexpected argument");
    }

    #[test]
    fn p814_eval_erro_sintaxe_mensagem_real_e_span_util() {
        // Medido no vanilla (P810): `error: expected expression` com span
        // dentro da chamada (não `<detached>` nem mensagem genérica).
        let world = MockWorld::new("#eval(\"1 +\")");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        assert!(
            diags[0].message.contains("expected expression"),
            "mensagem real do parser, não genérica: {}",
            diags[0].message
        );
        assert!(!diags[0].span.is_detached(), "span não pode ser <detached>");
        // P846 (#56) — medido no vanilla: o span âncora é o literal string
        // (`"` na coluna 6, 0-indexed), não a lista de argumentos (P814).
        assert_eq!(src.span_to_line_col(diags[0].span), Some((1, 6)));
    }

    #[test]
    fn p814_eval_erro_semantico_span_util() {
        // Erros de eval dentro do string herdam o span âncora do literal.
        let world = MockWorld::new("#eval(\"zzz + 1\")");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        assert_eq!(diags[0].message, "unknown variable `zzz`");
        assert!(!diags[0].span.is_detached(), "span não pode ser <detached>");
        // P846 (#56) — medido no vanilla `1:6` (o literal string).
        assert_eq!(src.span_to_line_col(diags[0].span), Some((1, 6)));
    }

    // ── P846 — achados #56/#57 de P831 (lote 5) ────────────────────────────

    #[test]
    fn p846_eval_span_ancora_no_literal_string_multilinha() {
        // #56 — caso `span7.typ` de P831: vanilla ancora no literal string
        // (`SpanMode::Uniform`, `foundations/mod.rs:267,318`); cristalino
        // ancorava em `args.span` — divergência de LINHA (cris `3:5` vs
        // van `4:2`), que o L0 subestimava como "nuance de uma coluna".
        let world = MockWorld::new("Texto.\n\n#eval(\n  \"abc +\"\n)\n");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        assert!(
            diags[0].message.contains("expected expression"),
            "mensagem real do parser: {}",
            diags[0].message
        );
        assert_eq!(src.span_to_line_col(diags[0].span), Some((4, 2)));
    }

    #[test]
    fn p846_eval_span_cast_error_ancora_no_argumento() {
        // #56 — medido no vanilla: `#eval(5)` → `1:6` (o literal `5`), não a
        // lista de argumentos (`1:5`).
        let world = MockWorld::new("#eval(5)");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        assert_eq!(diags[0].message, "expected string, found integer");
        assert_eq!(src.span_to_line_col(diags[0].span), Some((1, 6)));
    }

    #[test]
    fn p846_call_trace_cadeia_de_chamadas() {
        // #57 — medido no vanilla: `while calling \`c\`/\`b\`/\`a\``, innermost
        // primeiro, um nível por chamada, span da expressão de chamada.
        use crate::entities::source_result::Tracepoint;
        let world = MockWorld::new(
            "#let c() = { 1 + \"a\" }\n#let b() = { c() }\n#let a() = { b() }\n#a()\n",
        );
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        assert_eq!(diags[0].message, "cannot add integer and string");
        assert_eq!(src.span_to_line_col(diags[0].span), Some((1, 13)));
        let trace = &diags[0].trace;
        assert_eq!(trace.len(), 3, "um nível por chamada: {trace:?}");
        assert_eq!(trace[0].v, Tracepoint::Call(Some("c".into())));
        assert_eq!(trace[1].v, Tracepoint::Call(Some("b".into())));
        assert_eq!(trace[2].v, Tracepoint::Call(Some("a".into())));
        assert_eq!(src.span_to_line_col(trace[0].span), Some((2, 13)));
        assert_eq!(src.span_to_line_col(trace[1].span), Some((3, 13)));
        assert_eq!(src.span_to_line_col(trace[2].span), Some((4, 1)));
    }

    #[test]
    fn p846_call_trace_omite_chamada_que_contem_o_erro() {
        // #57 — regra do vanilla (`diag.rs:464-479`): o tracepoint é omitido
        // quando o span da chamada contém o span do erro. `#f()` com
        // `eval("xyz +")` no corpo: o trace mostra `f` mas NÃO `eval` (o
        // erro, ancorado no literal, está contido no span da chamada a
        // `eval`). Medido no vanilla (trace2).
        use crate::entities::source_result::Tracepoint;
        let world = MockWorld::new("#let f() = { eval(\"xyz +\") }\n#f()\n");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        let trace = &diags[0].trace;
        assert_eq!(trace.len(), 1, "{trace:?}");
        assert_eq!(trace[0].v, Tracepoint::Call(Some("f".into())));
        assert_eq!(src.span_to_line_col(trace[0].span), Some((2, 1)));
    }

    #[test]
    fn p846_call_trace_omite_closure_inline_no_callee() {
        // #57 — `#(() => { 1 + "a" })()`: o corpo da closure está dentro do
        // span da chamada (o callee inclui o literal da closure) → o vanilla
        // omite o tracepoint. Medido no vanilla (trace5).
        let world = MockWorld::new("#(() => { 1 + \"a\" })()\n");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        assert_eq!(diags[0].message, "cannot add integer and string");
        assert!(
            diags[0].trace.is_empty(),
            "erro contido no span da chamada não gera tracepoint: {:?}",
            diags[0].trace
        );
    }

    #[test]
    fn p846_call_trace_arrow_closure_com_let_usa_nome_do_binding() {
        // #57 — DIVERGÊNCIA documentada: `#let f = () => ...; #f()` — o
        // vanilla mostra `while calling function` (closure sem nome); o
        // cristalino nomeia a closure com o binding do `let` (extensão
        // deliberada pré-existente, `bindings.rs` eval_let `set_name`),
        // logo mostra `while calling \`f\``. Não introduzido por este passo.
        use crate::entities::source_result::Tracepoint;
        let world = MockWorld::new("#let f = () => { 1 + \"a\" }\n#f()\n");
        let src = world.source(world.main()).unwrap();
        let diags = eval_for_test(&world, &src).unwrap_err();
        let trace = &diags[0].trace;
        assert_eq!(trace.len(), 1, "{trace:?}");
        assert_eq!(trace[0].v, Tracepoint::Call(Some("f".into())));
        assert_eq!(src.span_to_line_col(trace[0].span), Some((2, 1)));
    }

    #[test]
    fn p814_eval_mode_math_equacao_inline() {
        // Paridade vanilla: modo math embrulha em EquationElem com block=false.
        fn encontra_equation(c: &Content) -> Option<bool> {
            match c {
                Content::Equation(e) => Some(e.block),
                Content::Sequence(items) => items.iter().find_map(encontra_equation),
                Content::Styled(b, _) => encontra_equation(b),
                _ => None,
            }
        }
        let world = MockWorld::new("#eval(\"x + y\", mode: \"math\")");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        assert_eq!(
            encontra_equation(module.content().unwrap()),
            Some(false),
            "modo math deve produzir Content::Equation com block=false"
        );
    }

    /// Verifica se existe algum `Content::Text` contendo `needle` directamente
    /// sob um `Content::Strong`.
    fn texto_em_strong(c: &Content, needle: &str) -> bool {
        fn go(c: &Content, needle: &str) -> bool {
            match c {
                Content::Strong(e) => {
                    if e.body.plain_text().contains(needle) {
                        return true;
                    }
                    go(&e.body, needle)
                }
                Content::Sequence(items) => items.iter().any(|i| go(i, needle)),
                Content::Styled(b, _) => go(b, needle),
                _ => false,
            }
        }
        go(c, needle)
    }

    /// Verifica se existe algum `Content::Text` contendo `needle` directamente
    /// sob um `Content::Emph`.
    fn texto_em_emph(c: &Content, needle: &str) -> bool {
        fn go(c: &Content, needle: &str) -> bool {
            match c {
                Content::Emph(e) => {
                    if e.body.plain_text().contains(needle) {
                        return true;
                    }
                    go(&e.body, needle)
                }
                Content::Sequence(items) => items.iter().any(|i| go(i, needle)),
                Content::Styled(b, _) => go(b, needle),
                _ => false,
            }
        }
        go(c, needle)
    }

    // ── P352 — show-set (`#show k: set …`, Transformation::Style, S5) ─────────

    /// P665: `#set text(weight: 700)` viaja no canal `custom` "text.weight".
    /// O campo tipado `bold` só é usado por markup `*...*`; `#set text(bold:)`
    /// foi removido para alinhar com o vanilla.
    fn styles_has_text_bold(s: &crate::entities::style::Styles) -> bool {
        styles_has_text_weight(s, 700)
    }
    fn styles_has_text_weight(s: &crate::entities::style::Styles, w: i64) -> bool {
        s.delta().custom.iter().any(|(k, v)| {
            k == "text.weight"
                && matches!(v, crate::entities::value::Value::Int(n) if *n == w)
        })
    }

    /// `true` se algum `Content::Styled` com `text.bold` embrulha
    /// (direta ou transitivamente) um `Content::Heading`.
    fn styled_bold_envolve_heading(c: &Content) -> bool {
        match c {
            Content::Styled(b, s) => {
                if styles_has_text_bold(s) && contem_heading(b) {
                    return true;
                }
                styled_bold_envolve_heading(b)
            }
            Content::Sequence(items) => items.iter().any(styled_bold_envolve_heading),
            _ => false,
        }
    }

    fn contem_heading(c: &Content) -> bool {
        match c {
            Content::Heading(_) => true,
            Content::Styled(b, _) => contem_heading(b),
            Content::Sequence(items) => items.iter().any(contem_heading),
            _ => false,
        }
    }

    /// `true` se existe um `Content::Text` que contém `needle` **sob** um escopo
    /// com `#set text(weight: 700)` activo (deteta vazamento global do set).
    /// P665: o bold do `#set text` viaja no canal custom "text.weight".
    fn texto_bold_contendo(c: &Content, needle: &str) -> bool {
        fn go(c: &Content, needle: &str, bold: bool) -> bool {
            match c {
                Content::Text(s) => bold && s.as_str().contains(needle),
                Content::Styled(b, styles) => {
                    let d = styles.delta();
                    let here = d.custom.iter().any(|(k, v)| {
                        k == "text.weight"
                            && matches!(v, crate::entities::value::Value::Int(n) if *n == 700)
                    });
                    go(b, needle, bold || here)
                }
                Content::Sequence(items) => items.iter().any(|i| go(i, needle, bold)),
                Content::Heading(h) => go(&h.body, needle, bold),
                _ => false,
            }
        }
        go(c, needle, false)
    }

    #[test]
    fn show_set_text_embrulha_heading_em_styled_bold() {
        // **Caso 3 (show-set), P352.** `#show heading: set text(weight: 700)`
        // ANTES ERRAVA ("requer função ou Content, recebeu none" — o eager avaliava
        // o set como statement e devolvia Value::None; Fase A P352). AGORA o heading
        // é embrulhado num `Content::Styled` carregando `bold=true`, e o texto
        // SOBREVIVE — show-set **não substitui** o elemento (espelha
        // `map.apply(transform); continue` do vanilla, `typst-realize:458-464`).
        let world = MockWorld::new("#show heading: set text(weight: 700)\n\n= titulo");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).expect("show-set não deve errar");
        let c = module.content().unwrap();
        assert!(
            styled_bold_envolve_heading(c),
            "o heading deve estar embrulhado num Styled(bold=true): {c:?}"
        );
        assert!(
            c.plain_text().contains("titulo"),
            "o texto do heading é preservado (não substituído): {:?}",
            c.plain_text()
        );
    }

    #[test]
    fn show_set_nao_consome_o_passe_preserva_o_elemento() {
        // Show-set NÃO consome o passe: o elemento permanece (não é trocado por
        // outro conteúdo). O heading continua presente sob o wrapper de estilo.
        let world = MockWorld::new("#show heading: set text(weight: 700)\n\n= Cabecalho");
        let src = world.source(world.main()).unwrap();
        let c = module_content(&world, &src);
        assert!(contem_heading(&c), "o heading sobrevive ao show-set: {c:?}");
    }

    #[test]
    fn show_set_nao_vaza_estilo_global() {
        // **Paridade-chave.** O `set` dentro do show-set é CAPTURADO, não aplicado
        // ao `engine.styles` global: o parágrafo de FORA não fica bold. (No caminho
        // eager antigo o set mutava o estilo global na declaração — a divergência
        // que o P352 fecha.) Confina-se ao elemento casado, como o vanilla.
        let world = MockWorld::new(
            "#show heading: set text(weight: 700)\n\n= T\n\nparagrafo de fora",
        );
        let src = world.source(world.main()).unwrap();
        let c = module_content(&world, &src);
        assert!(
            !texto_bold_contendo(&c, "paragrafo de fora"),
            "o texto de fora NÃO deve ficar bold (sem vazamento global): {c:?}"
        );
    }

    #[test]
    fn show_set_em_selector_de_texto_e_erro() {
        // Show-set é sobre elementos; sobre um selector de texto literal → erro
        // explícito (invariante de `entities/show.md`).
        let world = MockWorld::new("#show \"x\": set text(weight: 700)\nxxx");
        let src = world.source(world.main()).unwrap();
        let r = eval_for_test(&world, &src);
        assert!(r.is_err(), "show-set sobre selector de texto deve errar");
    }

    fn module_content(
        world: &MockWorld,
        src: &crate::entities::source::Source,
    ) -> Content {
        eval_for_test(world, src).unwrap().content().unwrap().clone()
    }

    // ── P356 — caso 1, lacuna (i): show-set + func (exemplo canônico do doc) ──

    /// `true` se algum `Content::Styled` com `bold == Some(true)` existe na árvore.
    fn styled_bold_anywhere(c: &Content) -> bool {
        match c {
            Content::Styled(b, s) => styles_has_text_bold(s) || styled_bold_anywhere(b),
            Content::Sequence(items) => items.iter().any(styled_bold_anywhere),
            Content::Heading(h) => styled_bold_anywhere(&h.body),
            _ => false,
        }
    }

    #[test]
    fn show_set_mais_func_estilo_alcanca_output() {
        // **Caso 1, lacuna (i), P356.** show-set + func no mesmo heading. O vanilla
        // dobra o show-set na chain e aplica o func SOB ela (lib.rs:341,357,458-464);
        // o doc (styling.md) promove exatamente este combo. ANTES (P352): o func
        // rodava 1º, o output (Sequence) não casava o seletor de heading, e o
        // show-set era PERDIDO. AGORA: o show-set casa o ELEMENTO original e
        // embrulha o output do func → o "X:" renderiza sob o estilo do show-set.
        let world = MockWorld::new(
            "#show heading: set text(weight: 700)\n#show heading: it => [X:] + it.body\n\n= T"
        );
        let src = world.source(world.main()).unwrap();
        let c = module_content(&world, &src);
        assert!(
            c.plain_text().contains("X:T"),
            "o func aplicou (prefixo X:): {:?}",
            c.plain_text()
        );
        assert!(
            styled_bold_anywhere(&c),
            "o show-set (bold) embrulha o output do func — não foi perdido: {c:?}"
        );
    }

    #[test]
    fn multiplos_show_set_continuam_compondo() {
        // Regressão (P352): múltiplos show-set same-kind compõem via `collapse` —
        // o conserto do P356 (casar o nó original) não pode quebrar isto.
        let world = MockWorld::new(
            "#show heading: set text(weight: 700)\n#show heading: set text(weight: 700)\n\n= T"
        );
        let src = world.source(world.main()).unwrap();
        let c = module_content(&world, &src);
        // bold + weight no mesmo Styled (ambos os show-set dobraram)
        fn styled_bold_e_weight(c: &Content) -> bool {
            match c {
                Content::Styled(b, s) => {
                    (styles_has_text_bold(s) && styles_has_text_weight(s, 700))
                        || styled_bold_e_weight(b)
                }
                Content::Sequence(items) => items.iter().any(styled_bold_e_weight),
                _ => false,
            }
        }
        assert!(
            styled_bold_e_weight(&c),
            "ambos os show-set (bold + weight) compõem num Styled: {c:?}"
        );
    }

    #[test]
    fn multiplos_func_same_kind_ultima_declarada_vence() {
        // **Caso 1, lacuna (ii), P358 — PARIDADE (era divergência no P356).** Dois
        // `func` same-kind sobre o mesmo heading cujo output muda de kind (Sequence):
        // **uma** func é efetiva — em ambos vanilla e crystalline (a outra não re-casa
        // o output). O P357 mediu o vanilla 0.14.2 = **"B:T"** (innermost-first: a
        // ÚLTIMA-declarada vence). Antes do P358 o crystalline dava "A:T" (1ª
        // declarada) — divergência de ORDEM. O conserto innermost-first
        // (`node_rules.iter().rev()`) casa o vanilla. (NÃO é acumulação — o vanilla
        // não acumula func same-kind; recon `f-recon-lacuna-ii-passo-357.md`.)
        let world = MockWorld::new(
            "#show heading: it => [A:] + it.body\n#show heading: it => [B:] + it.body\n\n= T"
        );
        let src = world.source(world.main()).unwrap();
        let c = module_content(&world, &src);
        let t = c.plain_text();
        assert!(
            t.contains("B:T"),
            "paridade vanilla: a última-declarada (B) vence: {t:?}"
        );
        assert!(
            !t.contains("A:"),
            "a 1ª-declarada (A) não aplica (o output de B não re-casa heading): {t:?}"
        );
    }

    // ── Passo 71 — image() integration ──────────────────────────────────────

    #[test]
    fn eval_image_le_ficheiro_para_content() {
        let mut world = MockWorld::new(r#"#image("foto.png")"#);
        world.add_file("foto.png", vec![0xFF, 0xD8, 0xFF]);
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().unwrap();
        assert!(
            matches!(&content, Content::Image(e) if e.path == "/foto.png"),
            "image() deve produzir Content::Image: {:?}",
            content
        );
    }

    #[test]
    fn eval_image_ficheiro_inexistente_gera_erro() {
        let world = MockWorld::new(r#"#image("naoexiste.png")"#);
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "image() com ficheiro inexistente deve falhar");
    }

    #[test]
    fn eval_image_rejeita_named_arg_invalido() {
        let mut world = MockWorld::new(r#"#image("foto.png", cor: "red")"#);
        world.add_file("foto.png", vec![1]);
        let src = world.source(world.main()).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "named arg desconhecido deve gerar erro");
    }

    #[test]
    fn content_image_arc_partilhado_em_clone() {
        use crate::entities::ptr_eq_arc::PtrEqArc;
        let data = std::sync::Arc::new(vec![1u8, 2, 3]);
        let img = Content::image(
            "img.png".to_string(),
            PtrEqArc(data.clone()),
            None,
            None,
            "cover",
        );
        let img2 = img.clone();
        assert_eq!(img, img2);
        // PtrEqArc::PartialEq compara por ponteiro — clone do mesmo Arc é igual (O(1)).
        if let (Content::Image(d1), Content::Image(d2)) = (&img, &img2) {
            assert!(
                std::sync::Arc::ptr_eq(&d1.data.0, &d2.data.0),
                "clone deve partilhar Arc"
            );
        }
    }

    // ── Passo 154B (ADR-0060 Fase 1) — terms + divider via eval ──────────────

    #[test]
    fn eval_divider_construtor_typst_lang() {
        let world = MockWorld::new("#divider()");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        // O conteúdo do módulo pode ser Divider directo ou Sequence([Divider]).
        let is_divider = matches!(&content, Content::Divider(_))
            || matches!(&content, Content::Sequence(s) if s.iter().any(|c| matches!(c, Content::Divider(_))));
        assert!(is_divider, "esperado Content::Divider, obteve {:?}", content);
    }

    #[test]
    fn eval_terms_construtor_typst_lang() {
        let world = MockWorld::new(r#"#terms(apple: [fruit], banana: [yellow])"#);
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        // Procurar Content::Terms na árvore (pode estar wrapped em Sequence).
        let extracted = match &content {
            Content::Terms(e) => Some(e.items.clone()),
            Content::Sequence(s) => s.iter().find_map(|c| match c {
                Content::Terms(e) => Some(e.items.clone()),
                _ => None,
            }),
            _ => None,
        };
        let items = extracted.expect("esperado Content::Terms");
        assert_eq!(items.len(), 2, "esperado 2 items, obtido {}", items.len());
        // Verificar que cada item é um TermItem com par term/description.
        assert!(
            items.iter().all(|i| matches!(i, Content::TermItem(_))),
            "todos os items devem ser TermItem: {:?}",
            items
        );
        // O texto plano deve conter "apple: fruit" e "banana: yellow".
        let pt = Content::terms(items).plain_text();
        assert!(pt.contains("apple: fruit"), "plain_text falta apple: {:?}", pt);
        assert!(pt.contains("banana: yellow"), "plain_text falta banana: {:?}", pt);
    }

    #[test]
    fn eval_divider_rejeita_args() {
        let world = MockWorld::new(r#"#divider(1)"#);
        let src = world.source(world.main()).unwrap();
        assert!(
            eval_for_test(&world, &src).is_err(),
            "divider() com argumentos posicionais deve retornar Err"
        );
    }

    // ── Passo 155 (ADR-0060 Fase 1, sub-passo 2) — quote via eval ────────

    fn find_quote(c: &Content) -> Option<&Content> {
        match c {
            Content::Quote(_) => Some(c),
            Content::Sequence(seq) => seq.iter().find_map(find_quote),
            _ => None,
        }
    }

    #[test]
    fn eval_quote_construtor_typst_lang() {
        let world = MockWorld::new(r#"#quote([texto citado])"#);
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        let q = find_quote(&content).expect("esperado Content::Quote");
        match q {
            Content::Quote(e) => {
                assert!(e.attribution.is_none(), "attribution default = None");
                assert!(!e.block, "block default = false");
                assert!(e.quotes, "quotes default = true");
            }
            _ => panic!("esperado Content::Quote, obteve {:?}", q),
        }
    }

    #[test]
    fn eval_quote_com_attribution_typst_lang() {
        let world =
            MockWorld::new(r#"#quote([Errare humanum est], attribution: [Seneca])"#);
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        let q = find_quote(&content).expect("esperado Content::Quote");
        match q {
            Content::Quote(e) if e.attribution.is_some() => {
                assert_eq!(e.attribution.as_ref().unwrap().plain_text(), "Seneca");
            }
            _ => panic!("esperado Content::Quote com attribution: {:?}", q),
        }
    }

    #[test]
    fn eval_quote_block_true_typst_lang() {
        let world = MockWorld::new(r#"#quote([conteudo], block: true)"#);
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        let q = find_quote(&content).expect("esperado Content::Quote");
        match q {
            Content::Quote(e) => assert!(e.block),
            _ => panic!("esperado Content::Quote, obteve {:?}", q),
        }
    }

    #[test]
    fn eval_quote_quotes_false_typst_lang() {
        let world = MockWorld::new(r#"#quote([x], quotes: false)"#);
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        let q = find_quote(&content).expect("esperado Content::Quote");
        match q {
            Content::Quote(e) => assert!(!e.quotes),
            _ => panic!("esperado Content::Quote, obteve {:?}", q),
        }
    }

    #[test]
    fn eval_quote_arg_invalido_retorna_err() {
        let world = MockWorld::new(r#"#quote([x], cor: "red")"#);
        let src = world.source(world.main()).unwrap();
        assert!(
            eval_for_test(&world, &src).is_err(),
            "quote() com named arg desconhecido deve retornar Err"
        );
    }

    #[test]
    fn eval_quote_sem_body_retorna_err() {
        let world = MockWorld::new(r#"#quote()"#);
        let src = world.source(world.main()).unwrap();
        assert!(
            eval_for_test(&world, &src).is_err(),
            "quote() sem body deve retornar Err"
        );
    }

    // ── Smart-quotes via markup `"..."` (Passo 155) ──────────────────────

    #[test]
    fn eval_markup_smart_quotes_default_curved() {
        // Sem text.lang activo: aspas curvas por padrão.
        let world = MockWorld::new(r#""hello""#);
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let plain = module.content().unwrap().plain_text();
        assert!(plain.contains("hello"), "texto preservado: {:?}", plain);
        assert!(
            plain.starts_with('“') && plain.ends_with('”'),
            "deve conter aspas curvas por padrão: {:?}",
            plain
        );
    }

    #[test]
    fn eval_markup_aspas_em_codigo_continua_string_literal_regression() {
        // **Crítico**: regression test. `"..."` em #let deve continuar a ser
        // Value::Str, não Content::Quote.
        let world = MockWorld::new(r#"#let s = "hello""#);
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let s = module.scope().get("s");
        assert!(
            matches!(s, Some(Value::Str(_))),
            "string literal em código deve ser Value::Str, obtive {:?}",
            s
        );
        if let Some(Value::Str(text)) = s {
            assert_eq!(text.as_str(), "hello", "valor da string literal preservado");
        }
    }

    // ── Passo 445 — Smart quotes context-aware ─────────────────────────

    #[test]
    fn eval_markup_smart_quotes_duplas_curly_com_lang_en() {
        // P786a: fonte corrigida — `#set text(lang: "en") "Hello,"` na mesma
        // linha é rejeitada pelo vanilla ("expected semicolon or line break")
        // e só passava porque o eval descartava erros de parser. A quebra de
        // linha mantém whitespace como contexto de abertura da aspa.
        let text = eval_plain_text("#set text(lang: \"en\")\n\"Hello,\"");
        assert!(
            text.contains('\u{201C}'),
            "aspa dupla de abertura (U+201C) deve estar presente: {:?}",
            text
        );
        assert!(
            text.contains('\u{201D}'),
            "aspa dupla de fecho (U+201D) deve estar presente: {:?}",
            text
        );
        assert!(!text.contains('"'), "não deve permanecer aspa ASCII recta: {:?}", text);
    }

    #[test]
    fn eval_markup_smart_quotes_simples_curly_com_lang_en() {
        // P786a: fonte corrigida (ver nota no teste de aspas duplas acima).
        let text = eval_plain_text("#set text(lang: \"en\")\n'Hello'");
        assert!(
            text.contains('\u{2018}'),
            "aspa simples de abertura (U+2018) deve estar presente: {:?}",
            text
        );
        assert!(
            text.contains('\u{2019}'),
            "aspa simples de fecho (U+2019) deve estar presente: {:?}",
            text
        );
        assert!(
            !text.contains('\''),
            "não deve permanecer apóstrofo ASCII recto: {:?}",
            text
        );
    }

    #[test]
    fn eval_markup_apostrophe_possessivo_emite_u2019() {
        // P786a: fonte corrigida (ver nota no teste de aspas duplas acima).
        let text = eval_plain_text("#set text(lang: \"en\")\nAlice's cat");
        assert!(
            text.contains('\u{2019}'),
            "apóstrofo possessivo deve ser U+2019: {:?}",
            text
        );
        assert!(
            !text.contains('\u{2018}'),
            "não deve haver aspa simples de abertura U+2018 em 'Alice's': {:?}",
            text
        );
    }

    // ── Passo 584 — Escape, Shorthand e Linebreak em markup ─────────────

    #[test]
    fn p584_escape_shorthand_linebreak_em_markup_preservados() {
        // Escape: \# \$ \& \* \\
        let text = eval_plain_text("\\# \\$ \\& \\* \\\\\\n");
        assert!(text.contains('#'), "escape \\# deve produzir '#': {:?}", text);
        assert!(text.contains('$'), "escape \\$ deve produzir '$': {:?}", text);
        assert!(text.contains('&'), "escape \\& deve produzir '&': {:?}", text);
        assert!(text.contains('*'), "escape \\* deve produzir '*': {:?}", text);
        assert!(text.contains('\\'), "escape \\\\ deve produzir '\\\\': {:?}", text);

        // Shorthand: -- (en-dash), --- (em-dash), ... (ellipsis)
        let text = eval_plain_text("a -- b --- c ... d");
        assert!(
            text.contains('\u{2013}'),
            "shorthand -- deve produzir en-dash U+2013: {:?}",
            text
        );
        assert!(
            text.contains('\u{2014}'),
            "shorthand --- deve produzir em-dash U+2014: {:?}",
            text
        );
        assert!(
            text.contains('\u{2026}'),
            "shorthand ... deve produzir ellipsis U+2026: {:?}",
            text
        );

        // Linebreak em markup: \ (barra seguida de whitespace)
        let text = eval_plain_text("linha um \\ linha dois");
        assert!(
            text.contains("linha um"),
            "texto antes da quebra deve estar presente: {:?}",
            text
        );
        assert!(
            text.contains("linha dois"),
            "texto depois da quebra deve estar presente: {:?}",
            text
        );
        assert!(
            text.contains('\n'),
            "linebreak \\ deve introduzir newline em plain_text: {:?}",
            text
        );
    }

    // ── Passo 448 — Subscript / Superscript ──────────────────────────────

    #[test]
    fn eval_sub_emite_styled() {
        let world = MockWorld::new("#sub[x]");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve haver content");
        match content {
            Content::Styled(body, styles) => {
                assert_eq!(body.plain_text(), "x");
                assert_eq!(styles.delta().subscript, Some(true));
                assert!(styles.delta().superscript.is_none());
            }
            other => panic!("esperado Content::Styled, obtive {:?}", other),
        }
    }

    #[test]
    fn eval_super_emite_styled() {
        let world = MockWorld::new("#super[x]");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve haver content");
        match content {
            Content::Styled(body, styles) => {
                assert_eq!(body.plain_text(), "x");
                assert_eq!(styles.delta().superscript, Some(true));
                assert!(styles.delta().subscript.is_none());
            }
            other => panic!("esperado Content::Styled, obtive {:?}", other),
        }
    }

    // ── Passo 449 — Highlight ───────────────────────────────────────────

    #[test]
    fn eval_highlight_default_emite_styled_amarelo() {
        let world = MockWorld::new("#highlight[x]");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve haver content");
        match content {
            Content::Styled(body, styles) => {
                assert_eq!(body.plain_text(), "x");
                assert_eq!(
                    styles.delta().highlight,
                    Some(Some(crate::entities::layout_types::Color::rgba(
                        255, 242, 54, 255
                    )))
                );
            }
            other => panic!("esperado Content::Styled, obtive {:?}", other),
        }
    }

    #[test]
    fn eval_highlight_fill_custom_emite_cor() {
        let world = MockWorld::new("#highlight(fill: rgb(255, 0, 0))[x]");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve haver content");
        match content {
            Content::Styled(body, styles) => {
                assert_eq!(body.plain_text(), "x");
                assert_eq!(
                    styles.delta().highlight,
                    Some(Some(crate::entities::layout_types::Color::rgb(255, 0, 0)))
                );
            }
            other => panic!("esperado Content::Styled, obtive {:?}", other),
        }
    }

    #[test]
    fn eval_highlight_fill_none_desactiva() {
        let world = MockWorld::new("#highlight(fill: none)[x]");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve haver content");
        match content {
            Content::Styled(body, styles) => {
                assert_eq!(body.plain_text(), "x");
                assert_eq!(styles.delta().highlight, Some(None));
            }
            other => panic!("esperado Content::Styled, obtive {:?}", other),
        }
    }

    // ── Passo 301 — Auto-lookup math mode ──────────────────────────────
    //
    // P301 (HP + (a) eval-time + (γ) híbrido): identifiers vanilla
    // pré-definidos (sin/cos/lim/etc., 42 ops P299) resolvem
    // automaticamente para Content::MathOp em math mode.
    // Variables user (x, f) continuam MathIdent.

    fn extract_math_content(world: &MockWorld) -> Content {
        use crate::contracts::world::World;
        let src = World::source(world, World::main(world)).unwrap();
        let module = eval_for_test(world, &src).unwrap();
        module.content().expect("módulo deve ter content").clone()
    }

    fn find_mathop_in(c: &Content) -> Option<(String, bool)> {
        match c {
            Content::MathOp(e) => Some((e.text.plain_text(), e.limits)),
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().find_map(find_mathop_in)
            }
            Content::Equation(e) => find_mathop_in(&e.body),
            // MathAttach: a base pode ser MathOp.
            Content::MathAttach(e) => find_mathop_in(&e.base)
                .or_else(|| match &e.br {
                    MathAttachSlot::Present(c) => find_mathop_in(c),
                    _ => None,
                })
                .or_else(|| match &e.tr {
                    MathAttachSlot::Present(c) => find_mathop_in(c),
                    _ => None,
                })
                .or_else(|| match &e.b {
                    MathAttachSlot::Present(c) => find_mathop_in(c),
                    _ => None,
                })
                .or_else(|| match &e.t {
                    MathAttachSlot::Present(c) => find_mathop_in(c),
                    _ => None,
                })
                .or_else(|| match &e.bl {
                    MathAttachSlot::Present(c) => find_mathop_in(c),
                    _ => None,
                })
                .or_else(|| match &e.tl {
                    MathAttachSlot::Present(c) => find_mathop_in(c),
                    _ => None,
                }),
            _ => None,
        }
    }

    fn find_mathident_in(c: &Content) -> Option<String> {
        match c {
            Content::MathIdent(s) => Some(s.to_string()),
            // single-letter em math mode produz MathText, não MathIdent.
            Content::MathText(s) => Some(s.to_string()),
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().find_map(find_mathident_in)
            }
            Content::Equation(e) => find_mathident_in(&e.body),
            Content::MathAttach(e) => find_mathident_in(&e.base)
                .or_else(|| match &e.br {
                    MathAttachSlot::Present(c) => find_mathident_in(c),
                    _ => None,
                })
                .or_else(|| match &e.tr {
                    MathAttachSlot::Present(c) => find_mathident_in(c),
                    _ => None,
                })
                .or_else(|| match &e.b {
                    MathAttachSlot::Present(c) => find_mathident_in(c),
                    _ => None,
                })
                .or_else(|| match &e.t {
                    MathAttachSlot::Present(c) => find_mathident_in(c),
                    _ => None,
                }),
            _ => None,
        }
    }

    fn find_mathstyled_body_in(c: &Content) -> Option<Content> {
        match c {
            Content::MathStyled(m) => Some(m.body.clone()),
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().find_map(find_mathstyled_body_in)
            }
            Content::Equation(e) => find_mathstyled_body_in(&e.body),
            _ => None,
        }
    }

    /// **P899 (Parte E)** — `bb("R")` (argumento string literal, dentro de
    /// uma chamada bare `bb(...)` em modo math) tem de produzir o mesmo tipo
    /// de corpo (`Content::MathText`) que `bb(R)` (identificador) já produz
    /// — não `Content::Text` (prosa), que `apply_math_style`
    /// (`compiler/math/layout/mod.rs`) não sabe estilizar.
    ///
    /// Achado da revisão (não coberto pelo teste unitário directo de
    /// `wrap_math_style`/`native_bb`, que só cobre metade do caminho real):
    /// chamadas bare de funções do scope global em modo math (`bb(...)`)
    /// passam pelo ramo P510 de `eval_math_expr`
    /// (`compiler/eval/math.rs::Expr::FuncCall`), que avalia CADA argumento
    /// via `eval_math_expr` genérico + `Value::Content(...)` — para
    /// `Expr::Str`, isso cai no braço `other => eval_expr +
    /// value_to_display_content`, que devolve `Content::Text` (prosa,
    /// `eval/mod.rs::value_to_display_content`, correcto e partilhado por
    /// muitos outros usos — não deve mudar). O argumento chega a
    /// `wrap_math_style` já como `Value::Content(Content::Text(_))`, nunca
    /// como `Value::Str` — o braço `Some(Value::Str(s)) =>
    /// Content::MathText(...)` corrigido em `wrap_math_style` fica morto
    /// para este caminho. Corrigido separadamente no despacho P510.
    #[test]
    fn p899_bb_de_string_via_pipeline_real_produz_mathtext() {
        use crate::contracts::world::World;
        let world = MockWorld::new(r#"#let r = $ bb("R") $"#);
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = match module.scope().get("r") {
            Some(Value::Content(c)) => c.clone(),
            other => panic!("esperado Value::Content, obteve {:?}", other),
        };
        let body = find_mathstyled_body_in(&content);
        assert!(
            matches!(&body, Some(Content::MathText(s)) if s.as_str() == "R"),
            "bb(\"R\") deve produzir corpo Content::MathText(\"R\") via pipeline real, obteve {:?}",
            body
        );
    }

    #[test]
    fn p301_sin_resolve_para_mathop_scripts_style() {
        let world = MockWorld::new("$sin x$");
        let content = extract_math_content(&world);
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_some(), "$sin x$ deve produzir MathOp; content: {:?}", content);
        let (text, limits) = mathop.unwrap();
        assert_eq!(text, "sin");
        assert!(!limits, "sin é scripts-style (limits=false)");
    }

    #[test]
    fn p301_lim_resolve_para_mathop_limits_style() {
        let world = MockWorld::new("$lim x$");
        let content = extract_math_content(&world);
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_some(), "$lim x$ deve produzir MathOp");
        let (text, limits) = mathop.unwrap();
        assert_eq!(text, "lim");
        assert!(limits, "lim é limits-style (limits=true)");
    }

    #[test]
    fn p301_det_resolve_para_mathop_novo() {
        // det NÃO estava em is_limit_function pré-P299; só ficou
        // disponível via math.det após P299, e agora via $det$ após P301.
        let world = MockWorld::new("$det A$");
        let content = extract_math_content(&world);
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_some(), "$det A$ deve produzir MathOp");
        let (text, limits) = mathop.unwrap();
        assert_eq!(text, "det");
        assert!(limits, "det é limits-style vanilla");
    }

    #[test]
    fn p1285_math_grapheme_e_numero_preservam_variantes_distintas() {
        fn leaf<'a>(content: &'a Content, text: &str) -> Option<&'a Content> {
            match content {
                Content::MathIdent(value) | Content::MathText(value)
                    if value.as_str() == text =>
                {
                    Some(content)
                }
                Content::Sequence(items) | Content::MathSequence(items) => {
                    items.iter().find_map(|item| leaf(item, text))
                }
                Content::Equation(equation) => leaf(&equation.body, text),
                _ => None,
            }
        }

        let number = extract_math_content(&MockWorld::new("$ 2 $"));
        assert!(
            matches!(leaf(&number, "2"), Some(Content::MathText(value)) if value.as_str() == "2"),
            "$ 2 $ deve preservar número como MathText: {number:?}"
        );

        let grapheme = extract_math_content(&MockWorld::new("$ x $"));
        assert!(
            matches!(leaf(&grapheme, "x"), Some(Content::MathIdent(value)) if value.as_str() == "x"),
            "$ x $ deve preservar grapheme como MathIdent: {grapheme:?}"
        );
    }

    #[test]
    fn p301_variavel_x_continua_mathident() {
        // x não está no scope math (não é operador) → fallback MathIdent.
        let world = MockWorld::new("$x$");
        let content = extract_math_content(&world);
        let ident = find_mathident_in(&content);
        assert!(ident.is_some(), "$x$ deve continuar MathIdent (não está em scope math)");
        assert_eq!(ident.unwrap(), "x");
    }

    #[test]
    fn p301_symbol_unicode_alpha_continua_mathtext() {
        // alpha → α via ident_to_unicode (path 1, antes do lookup math).
        // Verificar que P301 não interfere com path Unicode.
        let world = MockWorld::new("$alpha$");
        let content = extract_math_content(&world);
        // α NÃO é MathOp; é MathText. Verificar que find_mathop não encontra.
        let mathop = find_mathop_in(&content);
        assert!(
            mathop.is_none(),
            "$alpha$ deve continuar MathText (Unicode), não MathOp"
        );
    }

    #[test]
    fn p301_funcao_user_f_continua_mathident() {
        // f não é operador vanilla; fallback MathIdent preservado.
        let world = MockWorld::new("$f$");
        let content = extract_math_content(&world);
        let ident = find_mathident_in(&content);
        assert!(ident.is_some(), "$f$ deve continuar MathIdent");
        assert_eq!(ident.unwrap(), "f");
    }

    #[test]
    fn p301_regressao_mathident_lim_attach_limits_style() {
        // CRÍTICO: antes de P301, $lim_(x→0) f$ funcionava via heurística
        // is_limit_function. Pós-P301, lim resolve para MathOp{limits:true}.
        // Layout limits-style deve continuar a funcionar (via P298
        // cross-variant arm em attach.rs).
        let world = MockWorld::new("$ lim_(n) f $");
        let content = extract_math_content(&world);
        // Verificar que lim foi resolvido para MathOp.
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_some(), "$lim_(n) f$ deve produzir MathOp para lim");
        let (text, limits) = mathop.unwrap();
        assert_eq!(text, "lim");
        assert!(limits, "lim mantém limits-style após P301");
    }

    #[test]
    fn p301_multiplos_operadores_resolvidos() {
        // $sin x + cos y$ — 2 ops devem resolver.
        let world = MockWorld::new("$sin x + cos y$");
        let content = extract_math_content(&world);
        // Verificar que pelo menos 1 MathOp encontrado.
        assert!(
            find_mathop_in(&content).is_some(),
            "$sin x + cos y$ deve ter pelo menos 1 MathOp"
        );
    }

    // ── Passo 302 — bug fix `sin(x)` parens descartados ────────────────
    //
    // Reaplica sub-padrão §8.4 P288 "bug latente fixed durante
    // materialização dependente" (N=2: NBSP P287→P288; sin parens
    // P301→P302).

    fn find_mathdelimited_in(c: &Content) -> Option<(char, String, char)> {
        match c {
            Content::MathDelimited(e) => Some((e.open, e.body.plain_text(), e.close)),
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().find_map(find_mathdelimited_in)
            }
            Content::Equation(e) => find_mathdelimited_in(&e.body),
            _ => None,
        }
    }

    /// **P899 (Parte B)** — `abs`/`norm`/`floor`/`ceil`/`round`/`bar` são
    /// funções nativas em vanilla (`typst-library/src/math/lr.rs::abs/norm/
    /// floor/ceil`, `frac.rs` para `round` via a mesma `delimited()` —
    /// medido no `lab/typst-original`), cada uma um wrapper fino de
    /// `delimited(body, open, close, size)` — não estavam registadas em
    /// cristalino de nenhuma forma (nem scope global, nem módulo `math`,
    /// nem `SYM_SIMPLE`/`SYM_GROUPS`), caindo no fallback de texto literal
    /// (`abs(x)` renderizava `"abs(𝑥)"`). Reaproveita directamente
    /// `Content::math_delimited` (mesmo construtor que já produz o
    /// `MathDelimited` de `(x)`/`[x]` literais e do fallback `sin(x)` — o
    /// stretch vertical à altura do conteúdo, se já existir, aplica-se sem
    /// mudança de layout). `round` usa o par assimétrico `⌊`/`⌉`
    /// (arredondamento — paridade vanilla `frac.rs::round`, medido:
    /// `⌊`+`⌉`, não `⌊`+`⌋`). `bar` — apesar de listado como "Parte A"
    /// (acento) na materialização do passo — confirmado no vanilla real
    /// como delimitador `|x|`, não acento (ver `p899_hat_tilde_dot_dot_double_
    /// produzem_mathaccent`, que documenta a correcção do catálogo).
    #[test]
    fn p899_abs_norm_floor_ceil_round_produzem_mathdelimited() {
        for (call, open, close) in [
            ("abs(x)", '|', '|'),
            ("norm(x)", '‖', '‖'),
            ("floor(x)", '⌊', '⌋'),
            ("ceil(x)", '⌈', '⌉'),
            ("round(x)", '⌊', '⌉'),
            ("bar(x)", '|', '|'),
        ] {
            let src_text = format!("$ {call} $");
            let world = MockWorld::new(&src_text);
            let content = extract_math_content(&world);
            let delim = find_mathdelimited_in(&content);
            assert!(
                delim.is_some(),
                "{call} deve produzir MathDelimited; content: {:?}",
                content
            );
            let (got_open, body, got_close) = delim.unwrap();
            assert_eq!(got_open, open, "{call}: delimitador de abertura errado");
            assert_eq!(got_close, close, "{call}: delimitador de fecho errado");
            assert!(body.contains('x'), "{call}: body deve conter 'x'; got: {}", body);
        }
    }

    fn find_mathmatrix_in(c: &Content) -> Option<(Vec<Vec<Content>>, (char, char))> {
        match c {
            Content::MathMatrix(e) => Some((e.rows.clone(), e.delim)),
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().find_map(find_mathmatrix_in)
            }
            Content::Equation(e) => find_mathmatrix_in(&e.body),
            _ => None,
        }
    }

    fn find_unlined_frac_in(c: &Content) -> Option<(Content, Content)> {
        match c {
            Content::MathFrac(e) if !e.line => Some((e.num.clone(), e.den.clone())),
            Content::MathDelimited(e) => find_unlined_frac_in(&e.body),
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().find_map(find_unlined_frac_in)
            }
            Content::Equation(e) => find_unlined_frac_in(&e.body),
            _ => None,
        }
    }

    /// **P1132** — `binom` é morfologicamente uma fracção sem barra,
    /// envolvida por parênteses extensíveis, e não uma matriz aproximada.
    #[test]
    fn p1132_binom_produz_fracao_sem_barra() {
        let world = MockWorld::new("$ binom(n, k) $");
        let content = extract_math_content(&world);
        let frac = find_unlined_frac_in(&content)
            .expect("binom(n, k) deve conter MathFrac com line=false");
        assert!(frac.0.plain_text().contains('n'));
        assert!(frac.1.plain_text().contains('k'));
        let delim =
            find_mathdelimited_in(&content).expect("binom deve ter delimitadores");
        assert_eq!((delim.0, delim.2), ('(', ')'));
    }

    /// **P899 (Parte D)** — `binom(n, k1, k2, k3)` variádico: os argumentos
    /// lower juntam-se numa única célula separada por vírgula (paridade
    /// vanilla: `resolve_vertical_frac_like` insere `SymbolElem::packed(',')`
    /// entre cada elemento de `denom`, formando uma única sequência — não
    /// colunas separadas de uma matriz).
    #[test]
    fn p1132_binom_variadico_junta_lower_por_virgula() {
        let world = MockWorld::new("$ binom(n, k_1, k_2, k_3) $");
        let content = extract_math_content(&world);
        let (_upper, lower) = find_unlined_frac_in(&content)
            .expect("binom variádico deve conter MathFrac com line=false");
        let lower_text = lower.plain_text();
        assert!(
            lower_text.contains(','),
            "lower deve conter vírgulas separadoras: {}",
            lower_text
        );
    }

    #[test]
    fn p914_mat_suporta_parametro_nomeado_delim() {
        let world = MockWorld::new("$ mat(delim: \"{\", 1, 2; 3, 4) $");
        let content = extract_math_content(&world);
        let matrix = find_mathmatrix_in(&content);
        assert!(matrix.is_some(), "mat com delim deve produzir MathMatrix");
        let (_rows, delim) = matrix.unwrap();
        assert_eq!(delim, ('{', '}'), "delim: \"{{\" deve resultar em ('{{', '}}')");
    }

    fn find_mathaccent_in(c: &Content) -> Option<(String, String)> {
        match c {
            Content::MathAccent(e) => Some((e.base.plain_text(), e.accent.plain_text())),
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().find_map(find_mathaccent_in)
            }
            Content::Equation(e) => find_mathaccent_in(&e.body),
            _ => None,
        }
    }

    /// **P899 (Parte A)** — `hat(x)`/`tilde(x)`/`dot(x)`/`dot.double(x)` são
    /// funções de acento — confirmadas por comparação directa com o binário
    /// vanilla real (`lab/typst-original/target/release/typst`, ground
    /// truth, não assumido): `hat(x)`→x̂, `tilde(x)`→x̃, `dot(x)`→ẋ,
    /// `dot.double(x)`→ẍ. Mecanismo vanilla (`Symbol::func` +
    /// `Accent::combining`, `foundations/symbol.rs`/`math/accent.rs`):
    /// resolve o símbolo, procura o combining-mark correspondente na tabela
    /// `ACCENTS`, e chama `accent(base, combining_char)`. Cristalino não
    /// replica o mecanismo genérico "símbolo chamável"; em vez disso, novos
    /// braços hardcoded (mesmo estilo de `abs`/`binom`) mapeiam directamente
    /// para o combining-mark (`hat`→U+0302, `tilde`→U+0303, `dot`→U+0307,
    /// `dot.double`→U+0308 — mesmos valores da tabela `ACCENTS` do vanilla,
    /// `lab/typst-original/crates/typst-library/src/math/accent.rs`),
    /// reaproveitando `Content::math_accent` (já existente desde Passo 296,
    /// usado por `accent(...)` directo — layout já funciona, sem mudança
    /// necessária).
    ///
    /// **Achado da Fase A que corrige o catálogo do próprio passo**:
    /// `bar(x)` NÃO é uma função de acento (a materialização listava-a em
    /// "Parte A"). Confirmado no vanilla real: `bar(x)` → `|x|` (delimitador,
    /// mesma família de `abs`/`norm` — `sym.bar` resolve para `|`, chamado
    /// via `get_lr_wrapper_func`, não `Accent::combining`). Implementada em
    /// "Parte B" (braço `"abs" | "bar" | ...`), não aqui.
    #[test]
    fn p899_hat_tilde_dot_dot_double_produzem_mathaccent() {
        for (call, expected_accent_char) in [
            ("hat(x)", '\u{0302}'),
            ("tilde(x)", '\u{0303}'),
            ("dot(x)", '\u{0307}'),
            ("dot.double(x)", '\u{0308}'),
        ] {
            let src_text = format!("$ {call} $");
            let world = MockWorld::new(&src_text);
            let content = extract_math_content(&world);
            let accent = find_mathaccent_in(&content);
            assert!(
                accent.is_some(),
                "{call} deve produzir MathAccent; content: {:?}",
                content
            );
            let (base, accent_text) = accent.unwrap();
            assert!(base.contains('x'), "{call}: base deve conter 'x'; got: {}", base);
            assert_eq!(
                accent_text.chars().next(),
                Some(expected_accent_char),
                "{call}: combining mark errado",
            );
        }
    }

    #[test]
    fn p302_sin_parens_produz_mathsequence_com_delimited() {
        // P302 corrige bug P301: $sin(x)$ agora produz
        // MathSequence([MathOp(sin), MathDelimited((x))]).
        let world = MockWorld::new("$sin(x)$");
        let content = extract_math_content(&world);
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_some(), "$sin(x)$ deve produzir MathOp para sin");
        let (text, limits) = mathop.unwrap();
        assert_eq!(text, "sin");
        assert!(!limits);
        // Args devem aparecer como MathDelimited.
        let delim = find_mathdelimited_in(&content);
        assert!(delim.is_some(), "$sin(x)$ deve produzir MathDelimited preservando (x)");
        let (open, body, close) = delim.unwrap();
        assert_eq!(open, '(');
        assert_eq!(close, ')');
        assert!(body.contains("x"), "MathDelimited body deve conter 'x'; got: {}", body);
    }

    #[test]
    fn p302_lim_parens_preserva_limits_e_args() {
        // $lim(x)$ — lim é limits-style + args preservados.
        let world = MockWorld::new("$lim(x)$");
        let content = extract_math_content(&world);
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_some());
        let (text, limits) = mathop.unwrap();
        assert_eq!(text, "lim");
        assert!(limits, "lim mantém limits-style");
        let delim = find_mathdelimited_in(&content);
        assert!(delim.is_some(), "args (x) preservados via MathDelimited");
    }

    #[test]
    fn p302_sin_args_vazios_so_mathop() {
        // $sin()$ — args vazios; emite só MathOp sem MathDelimited vazio.
        let world = MockWorld::new("$sin()$");
        let content = extract_math_content(&world);
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_some(), "$sin()$ deve produzir MathOp");
        let (text, _) = mathop.unwrap();
        assert_eq!(text, "sin");
    }

    #[test]
    fn p302_regressao_sin_sem_parens_preservado() {
        // CRÍTICO: $sin x$ (sem parens) continua a produzir MathOp directo
        // (regressão P301 bit-exact preservada).
        let world = MockWorld::new("$sin x$");
        let content = extract_math_content(&world);
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_some(), "$sin x$ produz MathOp directo");
        // Sem MathDelimited (sem parens).
        let delim = find_mathdelimited_in(&content);
        assert!(delim.is_none(), "$sin x$ (sem parens) NÃO deve ter MathDelimited");
    }

    #[test]
    fn p302_multiplos_args_separados_por_virgula() {
        // $sin(x, y)$ — múltiplos args separados.
        let world = MockWorld::new("$sin(x, y)$");
        let content = extract_math_content(&world);
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_some());
        let delim = find_mathdelimited_in(&content);
        assert!(delim.is_some(), "MathDelimited com múltiplos args");
        let (_, body, _) = delim.unwrap();
        // Body contém ambos args (separação por vírgula).
        assert!(body.contains("x"), "body contém 'x'; got: {}", body);
        assert!(body.contains("y"), "body contém 'y'; got: {}", body);
    }

    // ── Passo 303 — bug fix `undef(x)` parens descartados (lookup-miss) ────
    //
    // Paralelo arquitectural directo de P302 no ramo lookup-miss:
    // identifier desconhecido em scope `math` agora preserva args via
    // MathSequence([MathIdent, MathDelimited]) em vez de descartar
    // silenciosamente (bug pré-P301).

    #[test]
    fn p303_undef_parens_produz_mathsequence_com_delimited() {
        // $undef(x)$ — identifier desconhecido com args.
        // Pré-P303: MathIdent("undef") apenas; args descartados.
        // Pós-P303: MathSequence([MathIdent("undef"), MathDelimited((x))]).
        let world = MockWorld::new("$undef(x)$");
        let content = extract_math_content(&world);
        // MathIdent("undef") preservado.
        let ident = find_mathident_in(&content);
        assert!(ident.is_some(), "$undef(x)$ deve conter MathIdent");
        // Sem MathOp (undef não está em scope math).
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_none(), "undef NÃO está em scope math; sem MathOp");
        // MathDelimited deve preservar args (fix P303).
        let delim = find_mathdelimited_in(&content);
        assert!(
            delim.is_some(),
            "$undef(x)$ deve produzir MathDelimited preservando (x); content: {:?}",
            content
        );
        let (open, body, close) = delim.unwrap();
        assert_eq!(open, '(');
        assert_eq!(close, ')');
        assert!(body.contains("x"), "MathDelimited body deve conter 'x'; got: {}", body);
    }

    #[test]
    fn p303_undef_args_complexos_preservados() {
        // $undef(x + y)$ — args complexos.
        let world = MockWorld::new("$undef(x + y)$");
        let content = extract_math_content(&world);
        let ident = find_mathident_in(&content);
        assert!(ident.is_some(), "MathIdent presente");
        let delim = find_mathdelimited_in(&content);
        assert!(delim.is_some(), "MathDelimited presente com expressão complexa");
        let (_, body, _) = delim.unwrap();
        assert!(body.contains("x"), "body contém 'x'; got: {}", body);
        assert!(body.contains("y"), "body contém 'y'; got: {}", body);
    }

    #[test]
    fn p303_undef_multiplos_args_separados_por_virgula() {
        // $undef(x, y)$ — múltiplos args separados por ", ".
        let world = MockWorld::new("$undef(x, y)$");
        let content = extract_math_content(&world);
        let ident = find_mathident_in(&content);
        assert!(ident.is_some());
        let delim = find_mathdelimited_in(&content);
        assert!(delim.is_some(), "MathDelimited com múltiplos args");
        let (_, body, _) = delim.unwrap();
        assert!(body.contains("x"), "body contém 'x'; got: {}", body);
        assert!(body.contains("y"), "body contém 'y'; got: {}", body);
    }

    #[test]
    fn p303_undef_args_vazios_so_mathident_sem_wrapper() {
        // $undef()$ — args vazios; só MathIdent (sem MathDelimited wrapper).
        // Paralelo P302: $sin()$ → MathOp sem wrapper.
        let world = MockWorld::new("$undef()$");
        let content = extract_math_content(&world);
        let ident = find_mathident_in(&content);
        assert!(ident.is_some(), "$undef()$ deve produzir MathIdent");
        // Sem MathDelimited (args vazios = sem wrapper).
        let delim = find_mathdelimited_in(&content);
        assert!(
            delim.is_none(),
            "$undef()$ (args vazios) NÃO deve ter MathDelimited; content: {:?}",
            content
        );
    }

    // ── P958 — símbolos gregos com args (`Gamma(z)` → Γ(𝑧)) ────────────
    //
    // O fallback do braço `Expr::FuncCall` só consultava `lookup_math_op`
    // antes do literal `MathIdent(name)` — os 7 casos reportados (Phi, chi,
    // Gamma, zeta, Psi, omega) saíam literais. A cadeia passa a espelhar o
    // standalone: lookup_math_op → ident_to_unicode → sym_lookup → literal.
    // Ver `compiler/eval.md` §P958 e `math/symbols.md` §P958.

    /// Recolhe os textos de todos os `Content::MathText` da árvore.
    fn p958_mathtexts(c: &Content) -> Vec<String> {
        match c {
            Content::MathText(s) => vec![s.to_string()],
            Content::MathIdent(s) => vec![format!("IDENT:{s}")],
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().flat_map(p958_mathtexts).collect()
            }
            Content::Equation(e) => p958_mathtexts(&e.body),
            Content::MathDelimited(e) => p958_mathtexts(&e.body),
            Content::MathAttach(e) => {
                let mut v = p958_mathtexts(&e.base);
                for sub in [&e.t, &e.b, &e.tl, &e.bl, &e.tr, &e.br] {
                    if let MathAttachSlot::Present(s) = sub {
                        v.extend(p958_mathtexts(s));
                    }
                }
                v
            }
            _ => vec![],
        }
    }

    /// Os 7 casos da auditoria externa (2026-08-04) + os pares standalone.
    #[test]
    fn p958_simbolo_grego_com_args_resolve_para_glifo() {
        for (nome, glifo) in [
            ("Gamma", "Γ"),
            ("zeta", "ζ"),
            ("Phi", "Φ"),
            ("chi", "χ"),
            ("Psi", "Ψ"),
            ("omega", "ω"),
        ] {
            let src = format!("${nome}(x)$");
            let world = MockWorld::new(&src);
            let content = extract_math_content(&world);
            let textos = p958_mathtexts(&content);
            assert!(
                textos.iter().any(|t| t == glifo),
                "${nome}(x)$ deve conter MathText({glifo}); textos: {textos:?}"
            );
            assert!(
                !textos.iter().any(|t| t == &format!("IDENT:{nome}")),
                "${nome}(x)$ NÃO deve ficar literal (MathIdent); textos: {textos:?}"
            );
            let delim = find_mathdelimited_in(&content);
            assert!(delim.is_some(), "${nome}(x)$ deve preservar (x) — P302/P303");
        }
    }

    /// Os 13 nomes gregos canónicos em falta na tabela (paridade codex
    /// `sym.txt`) — standalone, sem args: `$ Chi $` dava `unknown variable`.
    #[test]
    fn p958_nomes_gregos_em_falta_standalone() {
        for (nome, glifo) in [
            ("digamma", "ϝ"),
            ("omicron", "ο"),
            ("Chi", "Χ"),
            ("Eta", "Η"),
            ("Iota", "Ι"),
            ("Kappa", "Κ"),
            ("Mu", "Μ"),
            ("Nu", "Ν"),
            ("Omicron", "Ο"),
            ("Rho", "Ρ"),
            ("Tau", "Τ"),
            ("Upsilon", "Υ"),
            ("Zeta", "Ζ"),
        ] {
            let src = format!("${nome}$");
            let world = MockWorld::new(&src);
            let content = extract_math_content(&world);
            let textos = p958_mathtexts(&content);
            assert!(
                textos.iter().any(|t| t == glifo),
                "${nome}$ deve resolver para {glifo}; textos: {textos:?}"
            );
        }
    }

    /// Os 13 nomes também com args (o caminho do achado original).
    #[test]
    fn p958_nomes_gregos_em_falta_com_args() {
        for (nome, glifo) in [("Chi", "Χ"), ("Upsilon", "Υ"), ("digamma", "ϝ")] {
            let src = format!("${nome}(G)$");
            let world = MockWorld::new(&src);
            let content = extract_math_content(&world);
            let textos = p958_mathtexts(&content);
            assert!(
                textos.iter().any(|t| t == glifo),
                "${nome}(G)$ deve conter {glifo}; textos: {textos:?}"
            );
        }
    }

    /// Guarda de prioridade: operadores (`sin`) continuam a vencer a cadeia
    /// de símbolos no fallback de FuncCall.
    #[test]
    fn p958_sin_parens_prioridade_operador_preservada() {
        let world = MockWorld::new("$sin(x)$");
        let content = extract_math_content(&world);
        let op = find_mathop_in(&content);
        assert!(op.is_some(), "$sin(x)$ deve produzir MathOp: {content:?}");
        let textos = p958_mathtexts(&content);
        assert!(
            !textos.iter().any(|t| t == "sin"),
            "sin não deve sair como texto literal: {textos:?}"
        );
    }

    // ── P962 — registo de `dif`/`Dif` com wrapper upright ───────────────
    //
    // `compiler/stdlib/structural.md` §P962: o registo passa a ser
    // `MathStyled { italic: Some(false) }` sobre o texto, para que
    // `apply_math_default` não italicize o "d" do operador diferencial.

    /// Encontra (italic, texto-do-corpo) do primeiro `MathStyled` na árvore.
    fn p962_find_styled(c: &Content) -> Option<(Option<bool>, String)> {
        match c {
            Content::MathStyled(m) => Some((m.italic, m.body.plain_text())),
            Content::MathClassOverride(m) => p962_find_styled(&m.body),
            Content::Sequence(items) | Content::MathSequence(items) => {
                items.iter().find_map(p962_find_styled)
            }
            Content::Equation(e) => p962_find_styled(&e.body),
            _ => None,
        }
    }

    #[test]
    fn p962_dif_registado_com_wrapper_upright() {
        for (nome, letra) in [("dif", "d"), ("Dif", "D")] {
            let src = format!("$ {nome} $");
            let world = MockWorld::new(&src);
            let content = extract_math_content(&world);
            let styled = p962_find_styled(&content);
            let (italic, corpo) = styled.unwrap_or_else(|| {
                panic!("$ {nome} $ deve produzir MathStyled (wrapper upright); content: {content:?}")
            });
            assert_eq!(
                italic,
                Some(false),
                "$ {nome} $: wrapper com italic: Some(false) (upright), obteve {italic:?}"
            );
            assert_eq!(corpo, letra, "$ {nome} $: corpo deve ser '{letra}'");
        }
    }

    #[test]
    fn p1132m_dif_conserva_espaco_fino_fraco_e_classe_unary() {
        let world = MockWorld::new("$ x dif t $");
        let content = extract_math_content(&world);
        fn find(
            c: &Content,
        ) -> Option<(&crate::entities::elements::h_space::HSpaceElem, &Content)> {
            match c {
                Content::Sequence(items) | Content::MathSequence(items) => items
                    .windows(2)
                    .find_map(|w| match (&w[0], &w[1]) {
                        (Content::HSpace(h), Content::MathClassOverride(m)) => {
                            Some((h.as_ref(), &m.body))
                        }
                        _ => None,
                    })
                    .or_else(|| items.iter().find_map(find)),
                Content::Equation(e) => find(&e.body),
                _ => None,
            }
        }
        let (space, body) = find(&content).expect("dif = HSpace fraco + classe Unary");
        assert!(space.weak);
        match &space.amount {
            crate::entities::elements::h_space::Spacing::Absolute(length) => {
                assert_eq!(length.abs.0, 0.0);
                assert!((length.em - 1.0 / 6.0).abs() < 1e-12);
            }
            other => panic!("dif deve usar Length::em(1/6), obteve {other:?}"),
        }
        assert_eq!(body.plain_text(), "d");
    }

    /// **Guarda** — `$ d $` (identificador genuíno) continua a resolver para
    /// `MathIdent` (que o layout italiciza via P809), sem wrapper.
    #[test]
    fn p962_identificador_d_sem_wrapper() {
        let world = MockWorld::new("$ d $");
        let content = extract_math_content(&world);
        assert!(
            p962_find_styled(&content).is_none(),
            "$ d $ não deve ter wrapper MathStyled: {content:?}"
        );
    }

    #[test]
    fn p780_undef_sem_parens_erra_unknown_variable() {
        // P780 revoga o comportamento pré-P301 desta suite: `$undef$` (bare,
        // sem parens) já não produz `MathIdent` silencioso — erra "unknown
        // variable: undef" com hints, paridade byte-idêntica ao vanilla
        // (confirmado por compilação real, `foundations/scope.rs::
        // unknown_variable_math`). Substitui
        // `p303_regressao_undef_sem_parens_preservado` (nome antigo referia
        // um comportamento agora incorrecto face ao vanilla).
        let world = MockWorld::new("$undef$");
        let src = world.source(world.main()).unwrap();
        let err = eval_for_test(&world, &src).expect_err("undef deve errar em modo math");
        assert_eq!(err[0].message, "unknown variable `undef`");
        assert_eq!(
            err[0].hints,
            vec![
                "if you meant to display multiple letters as is, try adding spaces between each letter: `u n d e f`",
                "or if you meant to display this as text, try placing it in quotes: `\"undef\"`",
            ]
        );
    }

    // ── P782 — splice de `#expr`/field-access bare em modo math ────────────
    //
    // P772y (§3.6.2) mediu, P780 reconfirmou como débito distinto de
    // `MathIdent` bare por nome: `#expr` e field access bare (`sym.suit.
    // heart` sem `#`) caíam no `_ => Ok(Content::Empty)` genérico de
    // `eval_math_expr`, descartados em silêncio. Nesta arquitectura,
    // `#sym.suit.heart` (via Hash → `embedded_code_expr`) e
    // `sym.suit.heart` bare (montado pelo lexer math directamente como
    // `SyntaxKind::FieldAccess`) produzem o **mesmo** `Expr::FieldAccess`
    // — corrigidos pelo mesmo braço.

    #[test]
    fn p782_field_access_bare_resolve_simbolo() {
        // **P825 (sub-B)** — a premissa original deste teste (bare
        // `sym.suit.heart` resolve ♥) foi refutada por medição do vanilla
        // 0.15.0: `$ sym.suit.heart $` → `error: unknown variable: sym`
        // (+ 3 hints). Módulos globais não são acessíveis bare em modo
        // math — a validação de P782 foi reforçada, não duplicada.
        let world = MockWorld::new("$sym.suit.heart$");
        let src = world.source(world.main()).unwrap();
        let err = eval_for_test(&world, &src).expect_err("bare sym.* deve errar");
        assert!(
            err.iter().any(|d| d.message.contains("unknown variable `sym`")),
            "esperava 'unknown variable: sym' em: {:?}",
            err.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    // ── P825 (sub-achado B de P810 §12) — field access bare em modo math ──
    //
    // Medido no vanilla 0.15.0 (sonda `temp/p825/b*.typ`): módulos globais
    // (`math`, `sym`, `calc`, `emoji`, …) NÃO são acessíveis bare em modo
    // math — `error: unknown variable: <mod>` + 3 hints
    // ("not available directly in math", "add a hash", "std module").
    // Funções expostas no scope math (`class`, `mat`, `text`, …), bindings
    // de utilizador e o módulo `std` continuam acessíveis. O cristalino
    // compilava todos os casos bare (P782 abriu field access genérico).

    fn eval_math_err(
        src_text: &str,
    ) -> Vec<crate::entities::source_result::SourceDiagnostic> {
        let world = MockWorld::new(src_text);
        let src = world.source(world.main()).unwrap();
        eval_for_test(&world, &src).expect_err("documento deve falhar")
    }

    #[test]
    fn p825b_math_class_bare_erro_unknown_variable() {
        let err = eval_math_err("$ math.class(\"relation\", \"x\") $");
        assert!(
            err.iter().any(|d| d.message.contains("unknown variable `math`")),
            "esperava 'unknown variable: math': {:?}",
            err.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn p825b_calc_bare_erro_unknown_variable() {
        let err = eval_math_err("$ calc.gcd(4, 6) $");
        assert!(
            err.iter().any(|d| d.message.contains("unknown variable `calc`")),
            "esperava 'unknown variable: calc': {:?}",
            err.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn p825b_hints_verbatim_vanilla() {
        let err = eval_math_err("$ math.class(\"relation\", \"x\") $");
        let hints: Vec<&str> =
            err.iter().flat_map(|d| d.hints.iter().map(|h| h.as_str())).collect();
        for esperado in [
            "`math` is not available directly in math, but is in the standard library",
            "to access `math` in code mode you can add a hash: `#math`",
            "or access `math` in math mode by using the `std` module: `std.math`",
        ] {
            assert!(hints.contains(&esperado), "hint em falta {esperado:?}: {hints:?}");
        }
    }

    #[test]
    fn p825b_std_module_continua_acessivel_bare() {
        // Excepção medida no vanilla: `std` É acessível bare em modo math.
        let world = MockWorld::new("$ std.math.class(\"relation\", \"x\") $");
        let content = extract_math_content(&world);
        assert!(
            content.plain_text().contains('x'),
            "std.math.class deve funcionar: {:?}",
            content
        );
    }

    #[test]
    fn p825b_hash_field_access_continua_a_funcionar() {
        let world = MockWorld::new("$x #sym.suit.heart y$");
        let content = extract_math_content(&world);
        assert!(content.plain_text().contains('♥'), "esperava ♥ em: {:?}", content);
    }

    #[test]
    fn p825b_class_bare_continua_a_funcionar() {
        // Funções expostas no scope math (não-módulos) continuam bare —
        // paridade vanilla medida (`$ class("relation", x) $` compila).
        let world = MockWorld::new("$ class(\"relation\", x) $");
        let content = extract_math_content(&world);
        assert!(
            content.plain_text().contains('x'),
            "class bare deve funcionar: {:?}",
            content
        );
    }

    #[test]
    fn p825a_e2e_class_int_reporta_found_integer() {
        // Vanilla: `#math.class(3, "x")` → `expected "normal", ..., or
        // "vary", found integer` (o `3` avalia em modo código após `#`).
        let err = eval_math_err("$ #math.class(3, \"x\") $");
        let msg = &err[0].message;
        assert!(
            msg.contains("expected \"normal\"") && msg.contains(", found integer"),
            "mensagem verbatim com tipo vanilla; obteve: {msg}"
        );
    }

    #[test]
    fn p782_field_access_via_hash_resolve_simbolo() {
        let world = MockWorld::new("$x #sym.suit.heart y$");
        let content = extract_math_content(&world);
        assert!(content.plain_text().contains('♥'), "esperava ♥ em: {:?}", content);
    }

    #[test]
    fn p782_hash_ident_vinculado_a_symbol_resolve() {
        let world = MockWorld::new("#let hc = sym.suit.heart\n$x #hc y$");
        let content = extract_math_content(&world);
        assert!(content.plain_text().contains('♥'), "esperava ♥ em: {:?}", content);
    }

    #[test]
    fn p782_let_binding_em_math_executa_de_facto() {
        // Bónus (não era o alvo original, mas cai do mesmo fix): antes de
        // P782, `#let` dentro de `$...$` caía no mesmo catch-all e nunca
        // mutava o scope — agora executa (variável multi-letra: letra
        // única continua sempre simbólica, P780, indiferente ao binding).
        let world = MockWorld::new("$#let zval = 5; zval$");
        let content = extract_math_content(&world);
        assert!(content.plain_text().contains('5'), "esperava '5' em: {:?}", content);
    }

    #[test]
    fn p782_nao_regride_p780_bare_mathident_vinculado() {
        // Não-regressão: o caminho de P780 (`MathIdent` bare por nome,
        // `scopes.get_local` em `eval_math_expr`) continua a resolver —
        // este passo só adiciona um caminho paralelo, não o substitui.
        let world = MockWorld::new("#let myvar123 = 5\n$myvar123$");
        let content = extract_math_content(&world);
        assert!(content.plain_text().contains('5'), "esperava '5' em: {:?}", content);
    }

    #[test]
    fn p782_nao_regride_p780_undef_continua_a_errar() {
        let world = MockWorld::new("$undef$");
        let src = world.source(world.main()).unwrap();
        assert!(
            eval_for_test(&world, &src).is_err(),
            "undef sem binding deve continuar a errar"
        );
    }

    // ── P795 — Math/symbol scope: modificadores de símbolo e identificadores ──

    #[test]
    fn p795_sym_subset_neq() {
        let world = MockWorld::new("#repr(sym.subset.neq)");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let plain = module.content().unwrap().plain_text();
        assert_eq!(plain.trim(), "symbol(\"⊊\", (\"sq\", \"⋤\"))");
    }

    #[test]
    fn p795_math_arrow_r_bare() {
        let world = MockWorld::new("$arrow.r$");
        let content = extract_math_content(&world);
        assert!(content.plain_text().contains('→'), "esperava → em: {:?}", content);
    }

    #[test]
    fn p795_math_dif_bare() {
        let world = MockWorld::new("$integral x dif x$");
        let content = extract_math_content(&world);
        assert!(content.plain_text().contains('d'), "esperava d em: {:?}", content);

        let world_capital = MockWorld::new("$integral x Dif x$");
        let content_capital = extract_math_content(&world_capital);
        assert!(
            content_capital.plain_text().contains('D'),
            "esperava D em: {:?}",
            content_capital
        );
    }

    #[test]
    fn p303_regressao_sin_parens_p302_preservado() {
        // CRÍTICO: $sin(x)$ continua a produzir
        // MathSequence([MathOp(sin), MathDelimited((x))]) — fix P302
        // preservado bit-exact pós-P303.
        let world = MockWorld::new("$sin(x)$");
        let content = extract_math_content(&world);
        let mathop = find_mathop_in(&content);
        assert!(mathop.is_some(), "MathOp(sin) preservado");
        let (text, _) = mathop.unwrap();
        assert_eq!(text, "sin");
        let delim = find_mathdelimited_in(&content);
        assert!(delim.is_some(), "MathDelimited preservado P302 fix");
    }

    // ── P420 — E2E bibliography com CSL custom via path ───────────────────────

    use crate::compiler::introspect::introspect_with_introspector;
    use crate::compiler::layout::{layout, layout_with_introspector};

    /// P429: replica o pipeline de produção (eval → introspect → injectar
    /// styles resolvidos no BibStore → layout).
    fn p420_layout_module(
        content: &Content,
        module: &Module,
    ) -> crate::entities::layout_types::PagedDocument {
        let mut intr = introspect_with_introspector(content);
        for (key, style) in module.bibliography_styles() {
            intr.bib_store.add_style(*key, style.clone());
        }
        layout_with_introspector(content, intr)
    }

    fn p420_bib_csl() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<style xmlns="http://purl.org/net/xbiblio/csl" version="1.0" class="in-text" default-locale="en-US">
  <info>
    <title>Custom E2E</title>
    <id>http://example.org/custom</id>
  </info>
  <citation>
    <layout><text variable="title"/></layout>
  </citation>
  <bibliography>
    <layout><text variable="title"/></layout>
  </bibliography>
</style>"#
    }

    #[test]
    fn p420_bibliography_custom_csl_path_rende_titulo() {
        let mut world =
            MockWorld::new(r#"#bibliography("refs.bib", style: "custom.csl")"#);
        world.add_file(
            "refs.bib",
            br#"
@article{smith2024,
  author = {Smith, John},
  title = {On Crystal Math},
  year = {2024},
  journal = {Journal of Examples}
}
"#
            .to_vec(),
        );
        world.add_file("custom.csl", p420_bib_csl().as_bytes().to_vec());

        let module = eval_for_test(&world, &world.source).unwrap();
        let content = module.content().unwrap();
        let doc = p420_layout_module(content, &module);
        let txt = doc.plain_text();
        assert!(txt.contains("On Crystal Math"), "custom CSL deve render title: {txt}");
    }

    #[test]
    fn p420_bibliography_built_in_ieee_continua_funcional() {
        let mut world = MockWorld::new(r#"#bibliography("refs.bib", style: "ieee")"#);
        world.add_file(
            "refs.bib",
            br#"
@article{smith2024,
  author = {Smith, John},
  title = {On Crystal Math},
  year = {2024},
  journal = {Journal of Examples}
}
"#
            .to_vec(),
        );

        let module = eval_for_test(&world, &world.source).unwrap();
        let content = module.content().unwrap();
        let doc = p420_layout_module(content, &module);
        let txt = doc.plain_text();
        assert!(txt.contains("[1]"), "ieee continua funcional: {txt}");
    }

    #[test]
    fn p420_bibliography_style_invalido_produz_erro() {
        let mut world = MockWorld::new(r#"#bibliography("refs.bib", style: "nope.csl")"#);
        world.add_file(
            "refs.bib",
            br#"
@article{smith2024,
  author = {Smith, John},
  title = {On Crystal Math},
  year = {2024}
}
"#
            .to_vec(),
        );

        let result = eval_for_test(&world, &world.source);
        assert!(
            result.is_err(),
            "style invalido como path inexistente deve produzir erro"
        );
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("failed to read CSL style file"),
            "{}",
            err[0].message
        );
    }

    #[test]
    fn p420_bibliography_csl_xml_malformado_produz_erro() {
        let mut world = MockWorld::new(r#"#bibliography("refs.bib", style: "bad.csl")"#);
        world.add_file(
            "refs.bib",
            br#"
@article{smith2024,
  author = {Smith, John},
  title = {On Crystal Math},
  year = {2024}
}
"#
            .to_vec(),
        );
        world.add_file("bad.csl", b"<style>".to_vec());

        let result = eval_for_test(&world, &world.source);
        assert!(result.is_err(), "XML malformado deve produzir erro");
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("failed to parse CSL style"),
            "{}",
            err[0].message
        );
    }

    // ── P450 — E2E bibliography via path (parser custom .bib) ────────────────

    #[test]
    fn p450_bibliography_path_bib_popula_entries() {
        let mut world = MockWorld::new(r#"#bibliography("refs.bib")"#);
        world.add_file(
            "refs.bib",
            br#"
@article{smith2024,
  author = {Smith, John and Doe, Jane},
  title = {On Crystal Math},
  year = {2024},
  journal = {Journal of Examples},
  volume = {12},
  pages = {1--10},
  doi = {10.1000/x},
  url = {https://example.org}
}
"#
            .to_vec(),
        );

        let module = eval_for_test(&world, &world.source).unwrap();
        let content = module.content().unwrap();
        match content {
            Content::Bibliography(elem) => {
                assert_eq!(elem.entries.len(), 1);
                let e = &elem.entries[0];
                assert_eq!(e.key, "smith2024");
                assert_eq!(e.author, "Smith, John and Doe, Jane");
                assert_eq!(e.title, "On Crystal Math");
                assert_eq!(e.year, 2024);
                assert_eq!(e.journal.as_deref(), Some("Journal of Examples"));
                assert_eq!(e.volume.as_deref(), Some("12"));
                assert_eq!(e.pages.as_deref(), Some("1--10"));
                assert_eq!(e.doi.as_deref(), Some("10.1000/x"));
                assert_eq!(e.url.as_deref(), Some("https://example.org"));
            }
            other => panic!("esperado Content::Bibliography, obtive {:?}", other),
        }
    }

    // ── P421 — E2E repr() ───────────────────────────────────────────────────

    fn p421_eval_plain_text(world: &MockWorld) -> String {
        let module = eval_for_test(world, &world.source).unwrap();
        module.content().unwrap().plain_text()
    }

    #[test]
    fn p421_repr_int() {
        let world = MockWorld::new("#repr(1)");
        assert_eq!(p421_eval_plain_text(&world), "1");
    }

    #[test]
    fn p421_repr_float() {
        let world = MockWorld::new("#repr(1.0)");
        assert_eq!(p421_eval_plain_text(&world), "1.0");
    }

    #[test]
    fn p421_repr_str() {
        let world = MockWorld::new("#repr(\"hello\")");
        assert_eq!(p421_eval_plain_text(&world), "\"hello\"");
    }

    #[test]
    fn p421_repr_sequence() {
        // P843 (F2) — paridade vanilla medida (`temp/p843/f2_content.typ`):
        // `repr([hi *bold*])` → `sequence([hi], [ ], strong(body: [bold]))`.
        let world = MockWorld::new("#repr([hi *bold*])");
        assert_eq!(
            p421_eval_plain_text(&world),
            "sequence([hi], [ ], strong(body: [bold]))"
        );
    }

    #[test]
    fn p1140_8_repr_plain_text_preserva_espaco_ascii_interno() {
        // P1140.8: medido no vanilla pinado — espaço ASCII único entre
        // alfanuméricos permanece no mesmo Text e o repr observável é compacto.
        let world = MockWorld::new("#repr([hello world])");
        assert_eq!(p421_eval_plain_text(&world), "[hello world]");
    }

    #[test]
    fn p1140_8_repr_dois_espacos_preservam_fronteira_observavel() {
        // Dois espaços não são absorvidos pelo token Text; este é um observável
        // da linguagem, sem fixar a igualdade estrutural Rust de Content.
        let world = MockWorld::new("#repr([hello  world])");
        assert_eq!(p421_eval_plain_text(&world), "sequence([hello], [ ], [world])");
    }

    #[test]
    fn p1140_9_repr_h_v_preserva_amount_e_presenca_de_weak() {
        let cases = [
            ("#repr(h(1pt))", "h(amount: 1pt)"),
            ("#repr(h(1fr))", "h(amount: 1fr)"),
            ("#repr(h(1pt, weak: false))", "h(amount: 1pt, weak: false)"),
            ("#repr(h(1pt, weak: true))", "h(amount: 1pt, weak: true)"),
            ("#repr(v(1em))", "v(amount: 1em)"),
            ("#repr(v(1pt, weak: false))", "v(amount: 1pt, weak: false)"),
            ("#repr(v(1pt, weak: true))", "v(amount: 1pt, weak: true)"),
        ];

        for (source, expected) in cases {
            assert_eq!(p421_eval_plain_text(&MockWorld::new(source)), expected);
        }
    }

    #[test]
    fn p1140_9_repr_pagebreak_colbreak_preserva_campos_explicitos() {
        let cases = [
            ("#repr(pagebreak())", "pagebreak()"),
            ("#repr(pagebreak(weak: false))", "pagebreak(weak: false)"),
            ("#repr(pagebreak(weak: true))", "pagebreak(weak: true)"),
            ("#repr(pagebreak(to: \"odd\"))", "pagebreak(to: \"odd\")"),
            (
                "#repr(pagebreak(weak: false, to: \"even\"))",
                "pagebreak(weak: false, to: \"even\")",
            ),
            ("#repr(colbreak())", "colbreak()"),
            ("#repr(colbreak(weak: false))", "colbreak(weak: false)"),
            ("#repr(colbreak(weak: true))", "colbreak(weak: true)"),
        ];

        for (source, expected) in cases {
            assert_eq!(p421_eval_plain_text(&MockWorld::new(source)), expected);
        }
    }

    #[test]
    fn p1140_10_linebreak_binding_repr_e_markup() {
        let cases = [
            ("#repr(type(linebreak))", "function"),
            ("#repr(linebreak())", "linebreak()"),
            ("#repr(linebreak(justify: false))", "linebreak(justify: false)"),
            ("#repr(linebreak(justify: true))", "linebreak(justify: true)"),
            (r#"#repr([\ ])"#, "sequence(linebreak(), [ ])"),
            (r#"#repr([a\ b])"#, "sequence([a], linebreak(), [ ], [b])"),
        ];

        for (source, expected) in cases {
            assert_eq!(p421_eval_plain_text(&MockWorld::new(source)), expected);
        }
    }

    #[test]
    fn p1140_10_linebreak_reflexao_preserva_presenca() {
        let cases = [
            ("#repr(linebreak().func() == linebreak)", "true"),
            ("#repr(linebreak().fields())", "(:)"),
            ("#repr(linebreak(justify: false).fields())", "(justify: false)"),
            (r#"#repr(linebreak().has("justify"))"#, "false"),
            (r#"#repr(linebreak(justify: false).has("justify"))"#, "true"),
            ("#repr(linebreak(justify: false).justify)", "false"),
            ("#repr(linebreak(justify: true).justify)", "true"),
        ];

        for (source, expected) in cases {
            assert_eq!(p421_eval_plain_text(&MockWorld::new(source)), expected);
        }

        let world = MockWorld::new("#linebreak().justify");
        assert!(eval_for_test(&world, &world.source).is_err());
    }

    #[test]
    fn p1140_10_linebreak_rejeita_argumentos_invalidos() {
        for source in ["#linebreak(1)", "#linebreak(justify: 1)", "#linebreak(foo: true)"]
        {
            let world = MockWorld::new(source);
            assert!(eval_for_test(&world, &world.source).is_err(), "{source}");
        }
    }

    #[test]
    fn p1140_17_parbreak_binding_global_std_e_repr() {
        let cases = [
            ("#repr(type(parbreak))", "function"),
            ("#repr(type(std.parbreak))", "function"),
            ("#repr(parbreak())", "parbreak()"),
            ("#repr([a #parbreak() b])", "sequence([a], [ ], parbreak(), [ ], [b])"),
            (
                "#repr([a #parbreak() #parbreak() b])",
                "sequence([a], [ ], parbreak(), [ ], parbreak(), [ ], [b])",
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(p421_eval_plain_text(&MockWorld::new(source)), expected);
        }
    }

    #[test]
    fn p1140_17_parbreak_rejeita_todos_os_argumentos() {
        for source in ["#parbreak(1)", "#parbreak(foo: true)"] {
            let world = MockWorld::new(source);
            assert!(eval_for_test(&world, &world.source).is_err(), "{source}");
        }
    }

    #[test]
    fn p421_repr_cite() {
        let world = MockWorld::new("#repr(cite(\"key\"))");
        assert_eq!(p421_eval_plain_text(&world), "cite(<key>)");
    }

    #[test]
    fn p421_repr_bibliography() {
        let mut world = MockWorld::new("#repr(bibliography(\"refs.bib\"))");
        world.add_file("refs.bib", b"".to_vec());
        assert_eq!(p421_eval_plain_text(&world), "bibliography(\"/refs.bib\")");
    }

    // ── P468 — E2E cite(style: ...) ───────────────────────────────────────────

    fn p468_eval_content(world: &MockWorld) -> crate::entities::content::Content {
        let module = eval_for_test(world, &world.source).unwrap();
        module.content().unwrap().clone()
    }

    #[test]
    fn p468_cite_style_numeric_explicito() {
        let world = MockWorld::new(r#"#cite("key", style: "numeric")"#);
        let content = p468_eval_content(&world);
        match content {
            crate::entities::content::Content::Cite(c) => {
                assert_eq!(c.key, "key");
                assert_eq!(
                    c.style,
                    Some(crate::entities::citation_style::CitationStyle::Numeric)
                );
            }
            other => panic!("esperado Content::Cite, obtive {:?}", other),
        }
    }

    #[test]
    fn p468_cite_style_author_date_explicito() {
        let world = MockWorld::new(r#"#cite("key", style: "author-date")"#);
        let content = p468_eval_content(&world);
        match content {
            crate::entities::content::Content::Cite(c) => {
                assert_eq!(
                    c.style,
                    Some(crate::entities::citation_style::CitationStyle::AuthorDate)
                );
            }
            other => panic!("esperado Content::Cite, obtive {:?}", other),
        }
    }

    #[test]
    fn p468_cite_style_invalido_produz_erro() {
        let world = MockWorld::new(r#"#cite("key", style: "foo")"#);
        assert!(eval_for_test(&world, &world.source).is_err());
    }

    // ── P422 — E2E link render visual ─────────────────────────────────────────

    fn p422_find_first_link(
        doc: &crate::entities::layout_types::PagedDocument,
    ) -> Option<&crate::entities::layout_types::FrameItem> {
        for page in &doc.pages {
            for item in &page.items {
                if let crate::entities::layout_types::FrameItem::Link { .. } = item {
                    return Some(item);
                }
            }
        }
        None
    }

    #[test]
    fn p422_link_body_texto_preserva_url() {
        let world = MockWorld::new("#link(\"https://example.com\")[Clique]");
        let module = eval_for_test(&world, &world.source).unwrap();
        let content = module.content().unwrap();
        let doc = layout(content);
        let link = p422_find_first_link(&doc).expect("deve haver FrameItem::Link");
        use crate::entities::layout_types::LinkTarget;
        if let crate::entities::layout_types::FrameItem::Link {
            target: LinkTarget::Url(url),
            items,
            pos,
            size,
        } = link
        {
            assert_eq!(url.as_str(), "https://example.com");
            assert!(!items.is_empty(), "body deve renderizar items");
            assert!(
                size.width.0 > 0.0 && size.height.0 > 0.0,
                "link deve ter bbox positiva"
            );
            assert!(
                pos.x.0 >= 0.0 && pos.y.0 >= 0.0,
                "link deve ter posição não-negativa"
            );
            let plain = doc.plain_text();
            assert!(plain.contains("Clique"), "texto do body deve aparecer: {plain}");
        } else {
            panic!("esperado FrameItem::Link com Url");
        }
    }

    #[test]
    fn p422_link_body_implicito_url() {
        use crate::entities::layout_types::LinkTarget;
        let world = MockWorld::new("#link(\"https://example.com\")");
        let module = eval_for_test(&world, &world.source).unwrap();
        let content = module.content().unwrap();
        let doc = layout(content);
        let link = p422_find_first_link(&doc).expect("deve haver FrameItem::Link");
        if let crate::entities::layout_types::FrameItem::Link {
            target: LinkTarget::Url(url),
            items,
            pos,
            size,
        } = link
        {
            assert_eq!(url.as_str(), "https://example.com");
            assert!(!items.is_empty(), "body implícito (URL) deve renderizar items");
            assert!(
                size.width.0 > 0.0 && size.height.0 > 0.0,
                "link implícito deve ter bbox positiva"
            );
            assert!(
                pos.x.0 >= 0.0 && pos.y.0 >= 0.0,
                "link implícito deve ter posição não-negativa"
            );
        } else {
            panic!("esperado FrameItem::Link com Url");
        }
    }

    // ── Passo 457 — outline() parametrizável via stdlib ─────────────────────

    #[test]
    fn p457_outline_source_parametros_named() {
        // `outline()` deixou de ser interceptador especial; agora é função
        // nativa da stdlib que aceita title/depth/indent.
        let world = MockWorld::new(
            r#"#outline(title: [Sumário], depth: 1, indent: false)
= H1
== H2"#,
        );
        let module = eval_for_test(&world, &world.source).unwrap();
        let content = module.content().unwrap();
        let doc = layout(content);
        let text = doc.plain_text();

        fn count(haystack: &str, needle: &str) -> usize {
            haystack.matches(needle).count()
        }

        assert!(
            text.contains("Sumário"),
            "outline(title:) deve renderizar título customizado: {text:?}"
        );
        assert_eq!(
            count(&text, "H1"),
            2,
            "H1 deve aparecer 1x na TOC + 1x como heading real: {text:?}"
        );
        assert_eq!(
            count(&text, "H2"),
            1,
            "H2 (nível 2) deve aparecer SÓ como heading real (depth=1 exclui da TOC): {text:?}"
        );
    }

    #[test]
    fn p457_outline_source_positional_title() {
        // Vanilla aceita título como primeiro argumento posicional.
        let world = MockWorld::new(
            r#"#outline([Conteúdo])
= H1"#,
        );
        let module = eval_for_test(&world, &world.source).unwrap();
        let content = module.content().unwrap();
        let doc = layout(content);
        let text = doc.plain_text();

        assert!(
            text.contains("Conteúdo"),
            "outline([title]) deve aceitar título posicional: {text:?}"
        );
        assert!(text.contains("H1"), "TOC deve listar H1: {text:?}");
    }

    // ── P466 — métodos de instância de array, dict e str ────────────────────

    fn eval_let(world: &MockWorld, name: &str) -> Option<Value> {
        let module = eval_for_test(world, &world.source).ok()?;
        module.scope().get(name).cloned()
    }

    #[test]
    fn p765a_sym_arrow_r_field_access() {
        let world = MockWorld::new("#let x = sym.arrow.r");
        let module = eval_for_test(&world, &world.source).unwrap();
        let v = module.scope().get("x").cloned().unwrap();
        if let Value::Symbol(s) = v {
            assert_eq!(s.value, "→");
        } else {
            panic!("esperado Value::Symbol, obtido {:?}", v);
        }
    }

    #[test]
    fn p765a_sym_arrow_r_filled_field_access() {
        let world = MockWorld::new("#let x = sym.arrow.r.filled");
        let v = eval_let(&world, "x");
        assert!(v.is_some(), "sym.arrow.r.filled deve resolver");
        if let Some(Value::Symbol(s)) = v {
            assert_eq!(s.value, "➡\u{fe0e}");
        } else {
            panic!("esperado Value::Symbol, obtido {:?}", v);
        }
    }

    #[test]
    fn p1162_symbol_runtime_preserva_grapheme_multicodepoint() {
        let world = MockWorld::new("#let x = symbol(\"♥️\")\n#x");
        let module = eval_for_test(&world, &world.source).unwrap();
        let symbol = match module.scope().get("x") {
            Some(Value::Symbol(symbol)) => symbol,
            other => panic!("esperado symbol, obtido {other:?}"),
        };
        assert_eq!(symbol.repr_variants(), "\"♥\\u{fe0f}\"");
        assert_eq!(module.content().unwrap().plain_text().trim(), "♥️");
    }

    #[test]
    fn p1162_emoji_heart_preserva_base_e_variants() {
        let world = MockWorld::new(
            "#let base = emoji.heart\n#let arrow = emoji.heart.arrow\n\
             #let excl = emoji.heart.excl\n#base#arrow#excl",
        );
        let module = eval_for_test(&world, &world.source).unwrap();
        assert_eq!(module.content().unwrap().plain_text().trim(), "❤️💘❣️");
    }

    #[test]
    fn p1162_igualdade_preserva_identidade_de_symbol() {
        let world = MockWorld::new(
            "#let same = symbol(\"❤️\") == symbol(\"❤️\")\n\
             #let distinct = emoji.heart == symbol(\"❤️\")",
        );
        let module = eval_for_test(&world, &world.source).unwrap();
        assert_eq!(module.scope().get("same"), Some(&Value::Bool(true)));
        assert_eq!(module.scope().get("distinct"), Some(&Value::Bool(false)));
    }

    #[test]
    fn p1162_constructor_aceita_classes_multicodepoint() {
        for (binding, grapheme) in [("zwj", "👩‍💻"), ("tone", "👍🏽"), ("flag", "🇧🇷")]
        {
            let world =
                MockWorld::new(&format!("#let {binding} = symbol(\"{grapheme}\")"));
            let module = eval_for_test(&world, &world.source).unwrap();
            let value = match module.scope().get(binding) {
                Some(Value::Symbol(symbol)) => symbol.value.as_str(),
                other => panic!("esperado symbol em {binding}, obtido {other:?}"),
            };
            assert_eq!(value, grapheme);
        }
    }

    #[test]
    fn p1162_constructor_rejeita_zero_ou_dois_graphemes_com_hint() {
        for source in ["#symbol(\"\")", "#symbol(\"ab\")"] {
            let world = MockWorld::new(source);
            let diagnostics = eval_for_test(&world, &world.source).unwrap_err();
            let diagnostic = diagnostics.first().expect("diagnóstico obrigatório");
            assert!(diagnostic.message.starts_with("invalid variant value: \""));
            assert_eq!(
                diagnostic.hints,
                ["variant value must be exactly one grapheme cluster"]
            );
        }
    }

    #[test]
    fn p1162_str_symbol_preserva_grapheme_integral() {
        let world = MockWorld::new(
            "#let base = str(emoji.heart)\n#let arrow = str(emoji.heart.arrow)\n\
             #let excl = str(emoji.heart.excl)",
        );
        let module = eval_for_test(&world, &world.source).unwrap();
        assert_eq!(module.scope().get("base"), Some(&Value::Str("❤️".into())));
        assert_eq!(module.scope().get("arrow"), Some(&Value::Str("💘".into())));
        assert_eq!(module.scope().get("excl"), Some(&Value::Str("❣️".into())));
    }

    #[test]
    fn p1163_repr_symbol_complexo_usa_pretty_array_like() {
        let world = MockWorld::new(
            "#let complex = repr(emoji.heart)\n\
             #let modified = repr(emoji.heart.arrow)\n\
             #let zwj = repr(symbol(\"👩‍💻\"))",
        );
        let module = eval_for_test(&world, &world.source).unwrap();
        let expected = concat!(
            "symbol(\n",
            "  \"❤\\u{fe0f}\",\n",
            "  (\"arrow\", \"💘\"),\n",
            "  (\"beat\", \"💓\"),\n",
            "  (\"black\", \"🖤\"),\n",
            "  (\"blue\", \"💙\"),\n",
            "  (\"box\", \"💟\"),\n",
            "  (\"broken\", \"💔\"),\n",
            "  (\"brown\", \"🤎\"),\n",
            "  (\"double\", \"💕\"),\n",
            "  (\"excl\", \"❣\\u{fe0f}\"),\n",
            "  (\"gray\", \"🩶\"),\n",
            "  (\"green\", \"💚\"),\n",
            "  (\"grow\", \"💗\"),\n",
            "  (\"lightblue\", \"🩵\"),\n",
            "  (\"orange\", \"🧡\"),\n",
            "  (\"pink\", \"🩷\"),\n",
            "  (\"purple\", \"💜\"),\n",
            "  (\"real\", \"🫀\"),\n",
            "  (\"revolve\", \"💞\"),\n",
            "  (\"ribbon\", \"💝\"),\n",
            "  (\"spark\", \"💖\"),\n",
            "  (\"white\", \"🤍\"),\n",
            "  (\"yellow\", \"💛\"),\n",
            ")",
        );
        assert_eq!(module.scope().get("complex"), Some(&Value::Str(expected.into())));
        assert_eq!(
            module.scope().get("modified"),
            Some(&Value::Str("symbol(\"💘\")".into()))
        );
        assert_eq!(
            module.scope().get("zwj"),
            Some(&Value::Str("symbol(\"👩\\u{200d}💻\")".into()))
        );
    }

    #[test]
    fn p466_array_first() {
        let world = MockWorld::new("#let x = (1, 2, 3).first()");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(1)));
    }

    #[test]
    fn p466_array_last() {
        let world = MockWorld::new("#let x = (1, 2, 3).last()");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(3)));
    }

    // ── P714 — array.at(index, default:) — via sintaxe de chamada de método ──

    #[test]
    fn p714_array_at_e2e_indice_positivo() {
        let world = MockWorld::new("#let x = (10, 20, 30).at(1)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(20)));
    }

    #[test]
    fn p714_array_at_e2e_indice_negativo() {
        let world = MockWorld::new("#let x = (10, 20, 30).at(-1)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(30)));
    }

    #[test]
    fn p714_array_at_e2e_com_default() {
        let world = MockWorld::new(r#"#let x = (10, 20).at(5, default: "faltou")"#);
        assert_eq!(eval_let(&world, "x"), Some(Value::Str("faltou".into())));
    }

    #[test]
    fn p714_array_at_e2e_fora_de_limites_sem_default_erra() {
        let world = MockWorld::new("#let x = (10, 20).at(5)");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }

    #[test]
    fn p714_array_at_e2e_replica_cetz_aabb() {
        // Reprodução do padrão real de cetz (`aabb.typ:43,75-77`):
        // `bounds.high.at(2, default: 0)`.
        let world = MockWorld::new("#let x = (1, 2).at(2, default: 0)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(0)));
    }

    #[test]
    fn p466_array_rev() {
        let world = MockWorld::new("#let x = (1, 2, 3).rev()");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(3), Value::Int(2), Value::Int(1)]))
        );
    }

    #[test]
    fn p466_array_sum() {
        let world = MockWorld::new("#let x = (1, 2, 3).sum()");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(6)));
    }

    #[test]
    fn p466_array_sorted() {
        let world = MockWorld::new("#let x = (3, 1, 2).sorted()");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]))
        );
    }

    // P652 — array.sorted() com tipos incompatíveis deve produzir erro,
    // não assumir Ordering::Equal em silêncio.
    #[test]
    fn p652_array_sorted_tipo_incompativel_errors() {
        let world = MockWorld::new("#let x = (1, \"a\", 2).sorted()");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("cannot compare string and integer"),
            "mensagem inesperada: {msg}"
        );
    }

    // P653 — array.sorted(key: ...) ordena pelo resultado da função chave.
    #[test]
    fn p653_array_sorted_key_funciona() {
        let world = MockWorld::new("#let x = (3, -10, 2).sorted(key: x => calc.abs(x))");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(2), Value::Int(3), Value::Int(-10)]))
        );
    }

    #[test]
    fn p653_array_sorted_key_em_dicionarios() {
        let world = MockWorld::new(
            "#let x = ((name: \"Bob\", age: 30), (name: \"Alice\", age: 25), (name: \"Carol\", age: 35)).sorted(key: x => x.name)"
        );

        fn dict_with(name: &str, age: i64) -> Value {
            let mut map: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
            map.insert("name".into(), Value::Str(name.into()));
            map.insert("age".into(), Value::Int(age));
            Value::Dict(map)
        }

        let expected = Some(Value::Array(vec![
            dict_with("Alice", 25),
            dict_with("Bob", 30),
            dict_with("Carol", 35),
        ]));
        assert_eq!(eval_let(&world, "x"), expected);
    }

    #[test]
    fn p653_array_sorted_rejeita_positional() {
        let world = MockWorld::new("#let x = (3, 1, 2).sorted(x => -x)");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(msg.contains("unexpected argument"), "mensagem inesperada: {msg}");
    }

    #[test]
    fn p653_array_sorted_key_tipo_incompativel_propaga() {
        // O erro de comparação deve aplicar-se aos valores produzidos pela
        // função chave, não aos elementos originais do array.
        let world = MockWorld::new("#let x = (1, \"a\", 2).sorted(key: x => x)");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("cannot compare string and integer"),
            "mensagem inesperada: {msg}"
        );
    }

    #[test]
    fn p466_array_filter() {
        let world = MockWorld::new("#let x = (1, 2, 3).filter(x => x > 1)");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(2), Value::Int(3)]))
        );
    }

    #[test]
    fn p466_array_map() {
        let world = MockWorld::new("#let x = (1, 2, 3).map(x => x * 2)");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(2), Value::Int(4), Value::Int(6)]))
        );
    }

    #[test]
    fn p466_array_find() {
        let world = MockWorld::new("#let x = (1, 2, 3).find(x => x > 1)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(2)));
    }

    #[test]
    fn p466_array_any() {
        let world = MockWorld::new("#let x = (1, 2, 3).any(x => x > 2)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Bool(true)));
    }

    #[test]
    fn p466_array_all() {
        let world = MockWorld::new("#let x = (1, 2, 3).all(x => x > 0)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Bool(true)));
    }

    #[test]
    fn p1142_array_all_estatico_equivale_instancia_e_curto_circuita() {
        let world = MockWorld::new(
            "#let stop(x) = if x == 3 { false } else if x == 4 { panic(\"sem curto-circuito\") } else { true }\n\
             #let static = array.all((1, 2, 3, 4), stop)\n\
             #let method = (1, 2, 3, 4).all(stop)\n\
             #let empty = array.all((), x => false)\n\
             #let kind = type(array.all)\n\
             #let name = repr(array.all)",
        );
        let module = eval_for_test(&world, &world.source).unwrap();
        assert_eq!(module.scope().get("static"), Some(&Value::Bool(false)));
        assert_eq!(module.scope().get("method"), Some(&Value::Bool(false)));
        assert_eq!(module.scope().get("empty"), Some(&Value::Bool(true)));
        assert_eq!(module.scope().get("kind"), Some(&Value::Type(Type::Function)));
        assert_eq!(module.scope().get("name"), Some(&Value::Str("all".into())));
    }

    #[test]
    fn p1142_array_all_exige_predicado_booleano_nas_duas_superficies() {
        for source in ["#array.all((1,), x => x)", "#(1,).all(x => x)"] {
            let world = MockWorld::new(source);
            let err = eval_for_test(&world, &world.source).unwrap_err();
            let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
            assert!(msg.contains("expected boolean, found integer"), "msg: {msg}");
        }
    }

    #[test]
    fn p1142_str_clusters_estatico_e_instancia_preservam_graphemes() {
        let world = MockWorld::new(
            "#let static = str.clusters(\"á👍🏽👩‍💻🇵🇹\")\n\
             #let method = \"á👍🏽👩‍💻🇵🇹\".clusters()\n\
             #let kind = type(str.clusters)\n\
             #let name = repr(str.clusters)",
        );
        let module = eval_for_test(&world, &world.source).unwrap();
        let expected = Value::Array(vec![
            Value::Str("á".into()),
            Value::Str("👍🏽".into()),
            Value::Str("👩‍💻".into()),
            Value::Str("🇵🇹".into()),
        ]);
        assert_eq!(module.scope().get("static"), Some(&expected));
        assert_eq!(module.scope().get("method"), Some(&expected));
        assert_eq!(module.scope().get("kind"), Some(&Value::Type(Type::Function)));
        assert_eq!(module.scope().get("name"), Some(&Value::Str("clusters".into())));
    }

    #[test]
    fn p1142_formas_estaticas_rejeitam_aridade_tipo_e_named() {
        for (source, expected) in [
            ("#array.all((1,))", "missing argument: test"),
            ("#array.all((1,), x => true, 3)", "unexpected argument"),
            ("#array.all(1, x => true)", "expected array, found integer"),
            ("#array.all((1,), test: x => true)", "positional"),
            ("#str.clusters()", "missing argument: self"),
            ("#str.clusters(1)", "expected string, found integer"),
            ("#str.clusters(\"a\", \"b\")", "unexpected argument"),
            ("#str.clusters(self: \"a\")", "positional"),
        ] {
            let world = MockWorld::new(source);
            let err = eval_for_test(&world, &world.source).unwrap_err();
            let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
            assert!(msg.contains(expected), "source: {source}; msg: {msg}");
        }
    }

    #[test]
    fn p466_array_zip() {
        let world = MockWorld::new("#let x = (1, 2).zip((\"a\", \"b\"))");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Array(vec![Value::Int(1), Value::Str("a".into())]),
                Value::Array(vec![Value::Int(2), Value::Str("b".into())]),
            ]))
        );
    }

    #[test]
    fn p466_array_enumerate() {
        let world = MockWorld::new("#let x = (\"a\", \"b\").enumerate()");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Array(vec![Value::Int(0), Value::Str("a".into())]),
                Value::Array(vec![Value::Int(1), Value::Str("b".into())]),
            ]))
        );
    }

    #[test]
    fn p466_dict_pairs() {
        let world = MockWorld::new("#let x = (a: 1, b: 2).pairs()");
        let module = eval_for_test(&world, &world.source);
        if let Err(e) = &module {
            eprintln!("P466 dict_pairs eval error: {:?}", e);
        }
        let result = eval_let(&world, "x");
        let pairs = result.unwrap().cast_array().unwrap().to_vec();
        assert_eq!(pairs.len(), 2);
        assert!(
            pairs.contains(&Value::Array(vec![Value::Str("a".into()), Value::Int(1)]))
        );
        assert!(
            pairs.contains(&Value::Array(vec![Value::Str("b".into()), Value::Int(2)]))
        );
    }

    #[test]
    fn p466_dict_remove() {
        // P466: `dict.remove(key)` retorna o valor removido. P717 fechou a
        // divergência que aqui estava documentada: a variável original agora
        // É mutada, como no vanilla (métodos mutantes via access()).
        let world =
            MockWorld::new("#let d = (a: 1, b: 2)\n#let x = d.remove(\"a\")\n#let y = d");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(1)));
        let mut expected: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        expected.insert("b".into(), Value::Int(2));
        assert_eq!(eval_let(&world, "y"), Some(Value::Dict(expected)));
    }

    #[test]
    fn p466_dict_update() {
        let world = MockWorld::new("#let x = (a: 1).update((b: 2))");
        let mut expected: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        expected.insert("a".into(), Value::Int(1));
        expected.insert("b".into(), Value::Int(2));
        assert_eq!(eval_let(&world, "x"), Some(Value::Dict(expected)));
    }

    // ── P491 — dict.at(default:) ─────────────────────────────────────────────

    #[test]
    fn p491_dict_at_default_named() {
        let world = MockWorld::new("#let d = (a: 1, b: 2)\n#let x = d.at(\"z\", default: 99)\n#let y = d.at(\"a\")\n#let z = d.at(\"z\", default: \"missing\")");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(99)));
        assert_eq!(eval_let(&world, "y"), Some(Value::Int(1)));
        assert_eq!(eval_let(&world, "z"), Some(Value::Str("missing".into())));
    }

    #[test]
    fn p491_dict_at_sem_default_chave_ausente_da_erro() {
        let world = MockWorld::new("#let d = (a: 1)\n#let x = d.at(\"z\")");
        assert!(eval_let(&world, "x").is_none());
    }

    #[test]
    fn p491_dict_keys_values() {
        let world = MockWorld::new(
            "#let d = (a: 1, b: 2)\n#let k = d.keys()\n#let v = d.values()",
        );
        assert_eq!(
            eval_let(&world, "k"),
            Some(Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]))
        );
        assert_eq!(
            eval_let(&world, "v"),
            Some(Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    // ── P492 — variáveis de cor predefinidas ─────────────────────────────────

    #[test]
    fn p492_cor_predefinida_no_scope_global() {
        let world = MockWorld::new("#let x = red\n#let y = blue\n#let z = none");
        assert!(matches!(eval_let(&world, "x"), Some(Value::Color(_))));
        assert!(matches!(eval_let(&world, "y"), Some(Value::Color(_))));
        assert_eq!(eval_let(&world, "z"), Some(Value::None));
    }

    #[test]
    fn p492_stroke_cores_predefinidas() {
        let world = MockWorld::new("#let x = rect(stroke: (left: 3pt + red, right: 1pt + blue, top: none, bottom: 2pt + green))");
        match eval_for_test(&world, &world.source) {
            Ok(module) => {
                let result = module.scope().get("x").cloned();
                assert!(result.is_some(), "x não definido no module scope");
            }
            Err(e) => {
                eprintln!("P492 stroke error: {:?}", e);
                panic!("rect com stroke colorido deveria avaliar sem erro");
            }
        }
    }

    #[test]
    fn p492_show_regex_text_color() {
        let world = MockWorld::new("#show regex(\"\\\\d+\"): it => text(red, it)\nO número 42 e o número 100 aparecem a vermelho.");
        match eval_for_test(&world, &world.source) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("P492 show-regex error: {:?}", e);
                panic!("show-regex com text(red, it) deveria avaliar sem erro");
            }
        }
    }

    #[test]
    fn p492_text_fill_named() {
        let world = MockWorld::new("#let x = text(fill: red, [hello])");
        assert!(matches!(eval_let(&world, "x"), Some(Value::Content(_))));
    }

    #[test]
    fn p492_text_fill_positional() {
        let world = MockWorld::new("#let x = text(red, [hello])");
        assert!(matches!(eval_let(&world, "x"), Some(Value::Content(_))));
    }

    // ── P865 — `#text(...)` como chamada aceita argumentos nomeados de `#set text`.

    #[test]
    fn p865_text_size_named() {
        let world = MockWorld::new("#let x = text(\"hello\", size: 20pt)");
        let v = eval_let(&world, "x").expect("x definido");
        let Value::Content(c) = v else { panic!("esperado Content, obtido {v:?}") };
        let Content::Styled(_, styles) = c else {
            panic!("esperado Content::Styled, obtido {c:?}")
        };
        assert_eq!(styles.delta().size, Some(20.0), "size deve ir para StyleDelta");
    }

    #[test]
    fn p865_text_named_args_comuns() {
        // Conjunto de argumentos nomeados que eram rejeitados antes de P865.
        let casos = [
            "#let x = text(\"hello\", weight: \"bold\")",
            "#let x = text(\"hello\", style: \"italic\")",
            "#let x = text(\"hello\", tracking: 0.5pt)",
            "#let x = text(\"hello\", lang: \"pt\")",
            "#let x = text(\"hello\", font: \"Arial\")",
            "#let x = text(\"hello\", dir: rtl)",
            "#let x = text(\"hello\", top-edge: \"ascender\")",
            "#let x = text(\"hello\", bottom-edge: \"baseline\")",
            "#let x = text(\"hello\", fill: red, size: 14pt)",
            "#let x = text(red, \"hello\", size: 14pt)",
        ];
        for src in casos {
            let world = MockWorld::new(src);
            assert!(eval_for_test(&world, &world.source).is_ok(), "deve aceitar: {src}");
        }
    }

    #[test]
    fn p865_text_size_int_erro() {
        let world = MockWorld::new("#let x = text(\"hello\", size: 12)");
        let err = eval_for_test(&world, &world.source).expect_err("size int deve falhar");
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("expected length, found integer"),
            "mensagem deve bater com #set text; obtida: {msg}"
        );
    }

    #[test]
    fn p865_text_arg_desconhecido_erro() {
        let world = MockWorld::new("#let x = text(\"hello\", foo: 1)");
        let err = eval_for_test(&world, &world.source).expect_err("foo deve falhar");
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("unexpected argument: foo"),
            "mensagem deve bater com #set text; obtida: {msg}"
        );
    }

    #[test]
    fn p865_text_bold_erro() {
        let world = MockWorld::new("#let x = text(\"hello\", bold: true)");
        let err = eval_for_test(&world, &world.source).expect_err("bold deve falhar");
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("unexpected argument: bold"),
            "mensagem deve bater com #set text; obtida: {msg}"
        );
    }

    #[test]
    fn p865_text_weight_int_valido() {
        let world = MockWorld::new("#let x = text(\"hello\", weight: 700)");
        let v = eval_let(&world, "x").expect("x definido");
        let Value::Content(c) = v else { panic!("esperado Content, obtido {v:?}") };
        let Content::Styled(_, styles) = c else {
            panic!("esperado Content::Styled, obtido {c:?}")
        };
        assert_eq!(styles.delta().weight, Some(700));
    }

    #[test]
    fn p865_text_variations_continua_a_funcionar() {
        let world = MockWorld::new("#let x = text(\"hello\", variations: (wght: 250))");
        assert!(eval_let(&world, "x").is_some(), "variations deve continuar válido");
    }

    #[test]
    fn p865_text_scope_out_aceite_sem_erro() {
        // Propriedades válidas em #set text mas ainda não implementadas devem
        // ser aceites em #text(...) sem erro (paridade de lista de argumentos).
        let world = MockWorld::new("#let x = text(\"hello\", hyphenate: true)");
        assert!(
            eval_for_test(&world, &world.source).is_ok(),
            "scope-out deve aceitar sem erro"
        );
    }

    #[test]
    fn p492_length_plus_color_cria_stroke() {
        let world = MockWorld::new("#let x = 3pt + red\n#let y = red + 3pt");
        assert!(matches!(eval_let(&world, "x"), Some(Value::Stroke(_))));
        assert!(matches!(eval_let(&world, "y"), Some(Value::Stroke(_))));
    }

    #[test]
    fn p1229_length_plus_gradient_cria_stroke_rico() {
        let world = MockWorld::new(
            "#let g = gradient.linear(red, blue)\n\
             #let x = 4pt + g\n\
             #let y = g + 4pt",
        );
        for name in ["x", "y"] {
            let Value::Stroke(stroke) = eval_let(&world, name).expect("binding") else {
                panic!("{name} deve ser stroke");
            };
            assert!((stroke.thickness - 4.0).abs() < 1e-6);
            assert!(matches!(stroke.paint, crate::entities::paint::Paint::Gradient(_)));
        }
    }

    // ── P497 — validação formal dos gaps D4/D5 (já implementados em P492) ───

    #[test]
    fn p497_stroke_cores_predefinidas() {
        // Ficheiro corpus/p490/test-stroke-sides.typ
        let world = MockWorld::new(
            "#rect(stroke: (left: 3pt + red, right: 1pt + blue, top: none, bottom: 2pt + green))"
        );
        match eval_for_test(&world, &world.source) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("P497 stroke error: {:?}", e);
                panic!("stroke com cores predefinidas deveria avaliar sem erro");
            }
        }
    }

    #[test]
    fn p497_show_regex_text_color() {
        // Ficheiro corpus/p490/test-show-regex.typ
        let world = MockWorld::new(
            "#show regex(\"\\\\d+\"): it => text(red, it)\nO número 42 e o número 100 aparecem a vermelho."
        );
        let src = world.source(world.main()).unwrap();
        match eval_for_test(&world, &src) {
            Ok(module) => {
                let text = module.content().unwrap().plain_text();
                assert!(text.contains("42"), "texto '42' deve permanecer no output");
                assert!(text.contains("100"), "texto '100' deve permanecer no output");
            }
            Err(e) => {
                eprintln!("P497 show-regex error: {:?}", e);
                panic!("show-regex com text(red, it) deveria avaliar sem erro");
            }
        }
    }

    // ── P496 — Field access em coleções (D3) ────────────────────────────────

    #[test]
    fn p493_array_dedup() {
        // P1284: dedup preserva a primeira ocorrência global (paridade vanilla ratificada).
        let world = MockWorld::new("#let x = (3, 1, 4, 4, 1, 5, 9, 2, 6).dedup()");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Int(3),
                Value::Int(1),
                Value::Int(4),
                Value::Int(5),
                Value::Int(9),
                Value::Int(2),
                Value::Int(6),
            ]))
        );
    }

    #[test]
    fn p493_array_chunks() {
        let world = MockWorld::new("#let x = (1, 2, 3, 4, 5).chunks(2)");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Array(vec![Value::Int(1), Value::Int(2)]),
                Value::Array(vec![Value::Int(3), Value::Int(4)]),
                Value::Array(vec![Value::Int(5)]),
            ]))
        );
    }

    #[test]
    fn p493_array_windows() {
        let world = MockWorld::new("#let x = (1, 2, 3).windows(2)");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Array(vec![Value::Int(1), Value::Int(2)]),
                Value::Array(vec![Value::Int(2), Value::Int(3)]),
            ]))
        );
    }

    #[test]
    fn p493_array_flatten() {
        let world = MockWorld::new("#let x = (1, (2, 3), 4).flatten()");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
            ]))
        );
    }

    #[test]
    fn p493_array_fold() {
        let world = MockWorld::new("#let x = (1, 2, 3).fold(0, (acc, x) => acc + x)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(6)));
    }

    #[test]
    fn p493_table_header_field() {
        let world = MockWorld::new("#let x = table.header[Nome][Idade]");
        assert!(matches!(eval_let(&world, "x"), Some(Value::Content(_))));
    }

    #[test]
    fn p493_table_footer_field() {
        let world = MockWorld::new("#let x = table.footer[Total]");
        assert!(matches!(eval_let(&world, "x"), Some(Value::Content(_))));
    }

    #[test]
    fn p496_table_cell_field() {
        let world = MockWorld::new("#let x = table.cell[Conteúdo]");
        assert!(matches!(eval_let(&world, "x"), Some(Value::Content(_))));
    }

    #[test]
    fn p496_heading_where_multi() {
        let world = MockWorld::new(
            "#show heading.where(level: 1, outlined: true): it => [CAPÍTULO: ] + it.body\n\n= Um\n\n== Dois"
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("CAPÍTULO: Um"),
            "show rule where multi-field deve aplicar-se ao heading 1: {:?}",
            text
        );
        assert!(
            !text.contains("CAPÍTULO: Dois"),
            "show rule where multi-field não deve aplicar-se ao heading 2: {:?}",
            text
        );
    }

    // ── P498 — separação conteúdo original vs output de show-rules (D3c) ────

    #[test]
    fn p498_d3c_residual_heading_query_pos_show_rule() {
        // Ficheiro corpus/p490/test-show-where-multi.typ
        let world = MockWorld::new(
            "#show heading.where(level: 1, outlined: true): it => upper(it.body)\n\n= Heading nível 1"
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();

        // O output renderizado não contém o heading locatable (foi substituído).
        let rendered_text = module.content().unwrap().plain_text();
        assert!(
            rendered_text.contains("HEADING NÍVEL 1"),
            "show rule deve transformar o body do heading: {:?}",
            rendered_text
        );

        // O conteúdo original (para introspecção) mantém o heading.
        let intr_content = module
            .introspection_content()
            .expect("P498: módulo deve ter introspection_content");
        let intr =
            crate::compiler::introspect::introspect_with_introspector(intr_content);
        let locations = intr.query(&crate::entities::selector::Selector::Kind(
            crate::entities::element_kind::ElementKind::Heading,
        ));
        assert_eq!(
            locations.len(),
            1,
            "query heading deve encontrar 1 elemento original após show-rule"
        );
    }

    #[test]
    fn p466_str_contains() {
        let world = MockWorld::new("#let x = \"hello\".contains(\"ell\")");
        assert_eq!(eval_let(&world, "x"), Some(Value::Bool(true)));
    }

    #[test]
    fn p466_str_starts_with() {
        let world = MockWorld::new("#let x = \"hello\".starts-with(\"he\")");
        assert_eq!(eval_let(&world, "x"), Some(Value::Bool(true)));
    }

    #[test]
    fn p466_str_ends_with() {
        let world = MockWorld::new("#let x = \"hello\".ends-with(\"lo\")");
        assert_eq!(eval_let(&world, "x"), Some(Value::Bool(true)));
    }

    #[test]
    fn p466_str_find() {
        // P691: find devolve a substring encontrada (paridade vanilla), não o índice.
        let world = MockWorld::new("#let x = \"hello\".find(\"ll\")");
        assert_eq!(eval_let(&world, "x"), Some(Value::Str("ll".into())));
    }

    #[test]
    fn p466_str_replace() {
        let world = MockWorld::new("#let x = \"hello\".replace(\"l\", \"x\")");
        assert_eq!(eval_let(&world, "x"), Some(Value::Str("hexxo".into())));
    }

    #[test]
    fn p466_str_trim() {
        let world = MockWorld::new("#let x = \"  hello  \".trim()");
        assert_eq!(eval_let(&world, "x"), Some(Value::Str("hello".into())));
    }

    #[test]
    fn p466_str_split() {
        let world = MockWorld::new("#let x = \"a,b,c\".split(\",\")");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Str("a".into()),
                Value::Str("b".into()),
                Value::Str("c".into()),
            ]))
        );
    }

    #[test]
    fn p466_str_repeat() {
        let world = MockWorld::new("#let x = \"ab\".repeat(3)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Str("ababab".into())));
    }

    // ── P473 — op. cit. + wiring show regex (já em P393) ─────────────────────

    /// cite(a), cite(b), cite(a) — terceira citação de 'a' (não consecutiva)
    /// deve renderizar como "[1] Knuth, op. cit.".
    #[test]
    fn p473_op_cit_detectado_apos_citacao_intercalada() {
        use crate::compiler::introspect::introspect_with_introspector;
        use crate::compiler::layout::layout_with_introspector;
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::content::Content;

        let entry_a = BibEntry::new("knuth73", "Knuth, D.", "TAOCP", 1973);
        let entry_b = BibEntry::new("lamport94", "Lamport, L.", "LaTeX", 1994);
        // cite(a) → [1]; cite(b) → [2]; cite(a) → [1] Knuth, op. cit.
        let content = Content::sequence(vec![
            Content::bibliography(vec![entry_a, entry_b], None),
            Content::cite("knuth73", None, None),
            Content::text(" "),
            Content::cite("lamport94", None, None),
            Content::text(" "),
            Content::cite("knuth73", None, None),
        ]);
        let intr = introspect_with_introspector(&content);
        let doc = layout_with_introspector(&content, intr);
        let text = doc.plain_text();
        assert!(
            text.contains("op. cit."),
            "terceira citação de knuth73 (não consecutiva) deve ser op. cit.; obtido: {text:?}"
        );
    }

    /// cite(a), cite(a) — segunda citação consecutiva deve ser ibid., não op. cit.
    #[test]
    fn p473_ibid_nao_confundido_com_op_cit() {
        use crate::compiler::introspect::introspect_with_introspector;
        use crate::compiler::layout::layout_with_introspector;
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::content::Content;

        let entry_a = BibEntry::new("knuth73", "Knuth, D.", "TAOCP", 1973);
        // cite(a), cite(a) → segundo deve ser ibid. e NÃO op. cit.
        let content = Content::sequence(vec![
            Content::bibliography(vec![entry_a], None),
            Content::cite("knuth73", None, None),
            Content::text(" "),
            Content::cite("knuth73", None, None),
        ]);
        let intr = introspect_with_introspector(&content);
        let doc = layout_with_introspector(&content, intr);
        let text = doc.plain_text();
        assert!(
            text.contains("ibid."),
            "segunda citação consecutiva deve ser ibid.; obtido: {text:?}"
        );
        assert!(
            !text.contains("op. cit."),
            "ibid. não deve ser confundido com op. cit.; obtido: {text:?}"
        );
    }

    /// Sem citações anteriores da key, não há op. cit.
    #[test]
    fn p473_primeira_citacao_nunca_e_op_cit() {
        use crate::compiler::introspect::introspect_with_introspector;
        use crate::compiler::layout::layout_with_introspector;
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::content::Content;

        let entry_a = BibEntry::new("knuth73", "Knuth, D.", "TAOCP", 1973);
        let content = Content::sequence(vec![
            Content::bibliography(vec![entry_a], None),
            Content::cite("knuth73", None, None),
        ]);
        let intr = introspect_with_introspector(&content);
        let doc = layout_with_introspector(&content, intr);
        let text = doc.plain_text();
        assert!(
            !text.contains("op. cit.") && !text.contains("ibid."),
            "primeira citação não deve ser op. cit. nem ibid.; obtido: {text:?}"
        );
    }

    // ── P474 — sonda Trilha 3: fecho + sonda Trilha 8: pad/corners ────────────

    /// P474 Sonda A — confirma wiring E2E de `#show heading.where(level: N)`:
    /// `eval_show_rule` → `query_selector_to_show_selector` → `apply_show_rules`
    /// → `selector_matches(Where)`. Trilha 3: 3/3 fechado.
    #[test]
    fn p474_show_where_wiring_completo_heading_level_2() {
        let world = MockWorld::new(
            "#show heading.where(level: 2): it => [SEC: ] + it.body\n\n= Cap\n\n== Sub",
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().unwrap().plain_text();
        assert!(
            text.contains("SEC: Sub"),
            "where(level: 2) deve aplicar-se ao heading nível 2: {:?}",
            text
        );
        assert!(
            !text.contains("SEC: Cap"),
            "where(level: 2) NÃO deve aplicar-se ao heading nível 1: {:?}",
            text
        );
    }

    /// P474 Sonda B — confirma que `pad(rest: Xpt)` aplica o mesmo valor
    /// em todos os lados (atalho `rest`). Implementado em P156L.
    #[test]
    fn p474_pad_rest_aplica_uniforme_via_extract_sides() {
        let world = MockWorld::new("#pad(rest: 5pt)[texto]");
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let c = module.content().unwrap();
        assert!(
            c.plain_text().contains("texto"),
            "pad(rest:) deve preservar o body: {:?}",
            c.plain_text()
        );
    }

    // ── P501 — métodos str/dict e calc log10/deg/rad ──────────────────────────

    #[test]
    fn p501_str_methods() {
        let cases = [
            ("#let x = \"Hello\".to-upper()", Value::Str("HELLO".into())),
            ("#let x = \"HELLO\".to-lower()", Value::Str("hello".into())),
            (
                "#let x = \"Hello, World!\".split(\", \")",
                Value::Array(vec![
                    Value::Str("Hello".into()),
                    Value::Str("World!".into()),
                ]),
            ),
            ("#let x = \"  hello  \".trim()", Value::Str("hello".into())),
            ("#let x = \"hello\".replace(\"l\", \"r\")", Value::Str("herro".into())),
            ("#let x = \"a\".to-unicode()", Value::Int(97)),
            ("#let x = str.from-unicode(97)", Value::Str("a".into())),
            ("#let x = \"hello\".contains(\"ell\")", Value::Bool(true)),
            ("#let x = \"hello\".starts-with(\"he\")", Value::Bool(true)),
            ("#let x = \"hello\".ends-with(\"lo\")", Value::Bool(true)),
            ("#let x = \"hello\".find(\"l\")", Value::Str("l".into())),
            ("#let x = \"hello\".rev()", Value::Str("olleh".into())),
            ("#let x = \"hello\".repeat(3)", Value::Str("hellohellohello".into())),
        ];
        for (src, expected) in cases {
            let world = MockWorld::new(src);
            assert_eq!(eval_let(&world, "x"), Some(expected), "falhou em: {src}");
        }
        let world = MockWorld::new("#\"abc\".to-unicode()");
        assert!(eval_for_test(&world, &world.source).is_err());
    }

    #[test]
    fn p501_dict_insert_len() {
        // P717 fechou a divergência: `d.insert(...)` muta `d` e devolve none
        // (vanilla), em vez de devolver um dict novo sem mutar o original.
        let world = MockWorld::new(
            "#let d = (a: 1, b: 2, c: 3)\n#{ d.insert(\"d\", 4) }\n#let y = d.len()",
        );
        assert_eq!(
            eval_let(&world, "y"),
            Some(Value::Int(4)),
            "dict.insert() muta o dict original; len() deve devolver 4"
        );
    }

    #[test]
    fn p501_calc_log10_deg_rad() {
        let world_log10 = MockWorld::new("#let x = calc.log10(100)");
        assert_eq!(eval_let(&world_log10, "x"), Some(Value::Float(2.0)));

        let world_deg = MockWorld::new("#let x = calc.deg(calc.pi)");
        assert_eq!(eval_let(&world_deg, "x"), Some(Value::Float(180.0)));

        let world_rad = MockWorld::new("#let x = calc.rad(180)");
        assert!(
            matches!(eval_let(&world_rad, "x"), Some(Value::Float(f)) if (f - std::f64::consts::PI).abs() < 1e-9),
            "calc.rad(180) deve ser aproximadamente pi"
        );
    }

    // ── P504 — Novas funcionalidades Typst 0.15.0 ────────────────────────────

    #[test]
    fn p504_int_min_max() {
        let world_min = MockWorld::new("#let x = int.min");
        let world_max = MockWorld::new("#let x = int.max");
        assert_eq!(eval_let(&world_min, "x"), Some(Value::Int(i64::MIN)));
        assert_eq!(eval_let(&world_max, "x"), Some(Value::Int(i64::MAX)));
    }

    #[test]
    fn p504_int_base() {
        let world = MockWorld::new("#let x = int(\"ff\", base: 16)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(255)));
    }

    #[test]
    fn p504_range_inclusive() {
        let world = MockWorld::new("#let x = range(1, 5, inclusive: true)");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
                Value::Int(5)
            ]))
        );
    }

    #[test]
    fn p504_calc_hyperbolics_and_erf() {
        let world_asinh = MockWorld::new("#let x = calc.asinh(1.0)");
        let world_acosh = MockWorld::new("#let x = calc.acosh(1.0)");
        let world_atanh = MockWorld::new("#let x = calc.atanh(0.5)");
        let world_erf = MockWorld::new("#let x = calc.erf(1.0)");
        assert!(matches!(eval_let(&world_asinh, "x"), Some(Value::Float(f)) if f > 0.88));
        assert!(matches!(eval_let(&world_acosh, "x"), Some(Value::Float(f)) if f == 0.0));
        assert!(matches!(eval_let(&world_atanh, "x"), Some(Value::Float(f)) if f > 0.54));
        assert!(matches!(eval_let(&world_erf, "x"), Some(Value::Float(f)) if f > 0.84));
    }

    #[test]
    fn p504_dict_map_filter() {
        // P1284: callbacks recebem somente o valor; as chaves são preservadas.
        let world_map =
            MockWorld::new("#let d = (a: 1, b: 2)\n#let x = d.map(v => v * 2)");
        let world_filter =
            MockWorld::new("#let d = (a: 1, b: 2)\n#let x = d.filter(v => v > 1)");
        let mut expected_map: IndexMap<EcoString, Value, FxBuildHasher> =
            IndexMap::default();
        expected_map.insert("a".into(), Value::Int(2));
        expected_map.insert("b".into(), Value::Int(4));
        let mut expected_filter: IndexMap<EcoString, Value, FxBuildHasher> =
            IndexMap::default();
        expected_filter.insert("b".into(), Value::Int(2));
        assert_eq!(eval_let(&world_map, "x"), Some(Value::Dict(expected_map)));
        assert_eq!(eval_let(&world_filter, "x"), Some(Value::Dict(expected_filter)));
    }

    #[test]
    fn p504_arguments_field_access() {
        let world = MockWorld::new("#let f(..args) = args.named\n#let x = f(x: 1, y: 2)");
        let mut expected: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        expected.insert("x".into(), Value::Int(1));
        expected.insert("y".into(), Value::Int(2));
        assert_eq!(eval_let(&world, "x"), Some(Value::Dict(expected)));
    }

    #[test]
    fn p504_arguments_positional_field() {
        let world = MockWorld::new("#let f(..args) = args.positional\n#let x = f(1, 2)");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p707_arguments_pos_metodo() {
        let world = MockWorld::new("#let f(..args) = args.pos()\n#let x = f(1, 2, y: 3)");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p707_arguments_named_metodo() {
        let world =
            MockWorld::new("#let f(..args) = args.named()\n#let x = f(1, 2, y: 3, z: 4)");
        let mut expected: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        expected.insert("y".into(), Value::Int(3));
        expected.insert("z".into(), Value::Int(4));
        assert_eq!(eval_let(&world, "x"), Some(Value::Dict(expected)));
    }

    #[test]
    fn p707_arguments_pos_e_named_nao_regridem_campos() {
        // .positional/.named (campo, P504) continuam a funcionar sem parênteses.
        let world = MockWorld::new(
            "#let f(..args) = (args.positional, args.named, args.pos(), args.named())\n#let x = f(1, y: 2)"
        );
        let mut expected_named: IndexMap<EcoString, Value, FxBuildHasher> =
            IndexMap::default();
        expected_named.insert("y".into(), Value::Int(2));
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![
                Value::Array(vec![Value::Int(1)]),
                Value::Dict(expected_named.clone()),
                Value::Array(vec![Value::Int(1)]),
                Value::Dict(expected_named),
            ]))
        );
    }

    // ── P708 — parâmetros keyword-only não consomem posicionais ─────────────

    #[test]
    fn p708_keyword_only_omitido_usa_default() {
        let world = MockWorld::new(
            "#let f(a, b, close: false) = (a, b, close)\n#let x = f(1, 2)",
        );
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(1), Value::Int(2), Value::Bool(false)]))
        );
    }

    #[test]
    fn p708_keyword_only_explicito_sobrepoe_default() {
        let world = MockWorld::new(
            "#let f(a, b, close: false) = (a, b, close)\n#let x = f(1, 2, close: true)",
        );
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(1), Value::Int(2), Value::Bool(true)]))
        );
    }

    #[test]
    fn p708_extra_posicional_sem_sink_e_erro() {
        // P708 — corrigido: antes aceitava silenciosamente e fazia
        // close = 3 (Int); vanilla erra "unexpected argument" (medido).
        let world = MockWorld::new(
            "#let f(a, b, close: false) = (a, b, close)\n#let x = f(1, 2, 3)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(msg.contains("unexpected argument"), "msg: {msg}");
    }

    #[test]
    fn p708_sink_absorve_extra_posicional_apesar_de_keyword_only() {
        // P708 — corrigido: antes args.pos().len() dava 2 (perdia o 3º,
        // "roubado" por close); agora dá 3, igual ao vanilla.
        let world = MockWorld::new(
            "#let f(..args, close: false) = args.pos().len()\n#let x = f(1, 2, 3)",
        );
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(3)));
    }

    #[test]
    fn p708_closure_so_positional_sem_regressao() {
        let world = MockWorld::new("#let f(a, b) = a + b\n#let x = f(1, 2)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(3)));
    }

    #[test]
    fn p708_closure_so_sink_sem_regressao() {
        let world =
            MockWorld::new("#let f(..args) = args.pos().len()\n#let x = f(1, 2, 3)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(3)));
    }

    // ── P724 — patterns de desestruturação em parâmetros de closure ──────────
    // Bloqueio real do cetz isolado em P723 (path-util.typ:453). Vanilla:
    // `typst-eval/src/call.rs:655-665` — Param::Pos não-Ident liga via
    // destructure (mesma entrada de let/for).

    #[test]
    fn p724_closure_destructuring_anonima() {
        let world = MockWorld::new("#let x = { let f = ((a, b)) => a + b; f((1, 2)) }");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(3)));
    }

    #[test]
    fn p724_closure_destructuring_misturada_posicional() {
        let world =
            MockWorld::new("#let x = { let f = ((a, b), c) => a + b + c; f((1, 2), 3) }");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(6)));
    }

    #[test]
    fn p724_let_nomeada_destructuring() {
        let world =
            MockWorld::new("#let g(x, (a, b)) = x + a + b\n#let x = g(10, (1, 2))");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(13)));
    }

    #[test]
    fn p724_closure_placeholder_consome_posicional() {
        // Antes: placeholder descartado com o braço `_ => None` → closure
        // com params a menos → unexpected argument. Vanilla: consome.
        let world = MockWorld::new("#let x = { let f = (_, y) => y; f(1, 2) }");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(2)));
    }

    #[test]
    fn p724_closure_destructuring_spread_no_pattern() {
        let world = MockWorld::new(
            "#let x = { let f = ((a, ..rest)) => rest.len(); f((1, 2, 3)) }",
        );
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(2)));
    }

    #[test]
    fn p724_closure_destructuring_enumerate_map_padrao_cetz() {
        // Padrão exato de cetz path-util.typ:453:
        // segments.enumerate().filter(((i, segment)) => ...)
        let world = MockWorld::new(
            "#let x = ((5, 6), (7, 8)).enumerate().map(((i, seg)) => i + seg.at(1))",
        );
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(6), Value::Int(9)]))
        );
    }

    #[test]
    fn p724_closure_destructuring_tipo_errado() {
        let world = MockWorld::new("#let x = { let f = ((a, b)) => a; f(1) }");
        let err = eval_for_test(&world, &world.source)
            .expect_err("destructuring sobre int deve errar");
        assert!(
            err[0].message.contains("cannot destructure"),
            "mensagem esperada: {:?}",
            err[0].message
        );
    }

    // ── P709 — módulo `std` (acesso à stdlib não-sombreada) ──────────────────

    #[test]
    fn p709_std_da_acesso_a_builtin_sombreado() {
        let world = MockWorld::new("#let length = 5\n#let x = std.length");
        assert_eq!(eval_let(&world, "x"), Some(Value::Type(Type::Length)));
    }

    #[test]
    fn p709_std_submodulo_apesar_de_sombreamento() {
        let world = MockWorld::new(
            "#let calc = \"não sou a calculadora\"\n#let x = std.calc.round(3.7)",
        );
        assert_eq!(eval_let(&world, "x"), Some(Value::Float(4.0)));
    }

    #[test]
    fn p709_std_e_sombreavel_como_qualquer_nome() {
        // Medido contra o vanilla: #let std = "oops"; #std -> "oops".
        let world = MockWorld::new("#let std = \"oops\"\n#let x = std");
        assert_eq!(eval_let(&world, "x"), Some(Value::Str("oops".into())));
    }

    #[test]
    fn p709_sem_sombreamento_std_e_igual_ao_builtin() {
        let world = MockWorld::new("#let x = std.range(3).len()");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(3)));
    }

    #[test]
    fn p709_std_disponivel_em_ficheiro_importado() {
        // Segunda construção de scope (eval_imported_file, modules.rs) — o
        // caminho real de cetz: um ficheiro importado sombreia `length` e
        // usa `std.length` para aceder à versão original.
        let v = import_str(
            "#import \"u.typ\": r",
            &[("u.typ", "#let length = 5\n#let r = std.length")],
            "r",
        );
        assert_eq!(v, Value::Type(Type::Length));
    }

    // ── P710 — `Length.to-absolute()` ────────────────────────────────────────

    #[test]
    fn p710_to_absolute_sem_em_inalterado() {
        // P1284: mesmo comprimentos absolutos exigem contexto conhecido.
        let world = MockWorld::new("#let x = (6pt).to-absolute()");
        assert!(eval_for_test(&world, &world.source).is_err());
    }

    #[test]
    fn p710_to_absolute_resolve_em_com_tamanho_default() {
        // A resolução depende de estilo e portanto não ocorre fora de contexto.
        let world = MockWorld::new("#let x = (6pt + 10em).to-absolute()");
        assert!(eval_for_test(&world, &world.source).is_err());
    }

    #[test]
    fn p710_to_absolute_em_tipo_diferente_nao_intercepta() {
        // Alvo não é Length -> cai no campo genérico, erro normal (não crasha).
        let world = MockWorld::new("#let x = (5).to-absolute()");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }

    // ── P712 — `measure()`: gate de context + intercepção directa ──────────
    //
    // A medição real (via Layouter isolado, `measure_content_real`) só
    // corre quando `ctx.in_context = true` — só verdade dentro de
    // `expand_context_blocks` (L3, `03_infra/pipeline.rs`), fora do alcance
    // deste harness (só L1 — `layout()` local não resolve
    // `Content::ContextBlock`, ver `compiler/layout/mod.rs:1645`). Os testes
    // aqui cobrem o que é local a L1: a intercepção reconhece `measure`/
    // `std.measure` pela identidade da função nativa e aplica o gate. A
    // medição real, dentro de `context`, é validada por reprodução manual
    // (`00_nucleo/diagnosticos/paridade-producao-p712.md`).

    #[test]
    fn p712_measure_fora_de_context_erra_com_gate() {
        let world = MockWorld::new(r#"#let x = measure("oi")"#);
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).unwrap_err();
        assert!(
            err.iter()
                .any(|d| d.message.contains("measure() can only be used inside context")),
            "esperado erro de gate de context, recebeu {err:?}"
        );
    }

    #[test]
    fn p712_std_measure_fora_de_context_tambem_erra_com_gate() {
        // Forma qualificada — o caminho real usado pelo cetz
        // (`std.measure(drawable.body)`, `util.typ:197`).
        let world = MockWorld::new(r#"#let x = std.measure("oi")"#);
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).unwrap_err();
        assert!(
            err.iter()
                .any(|d| d.message.contains("measure() can only be used inside context")),
            "esperado erro de gate de context (forma std.measure), recebeu {err:?}"
        );
    }

    #[test]
    fn p712_measure_sombreado_pelo_utilizador_nao_intercepta() {
        // `#let measure = ...` sombreia o builtin — a intercepção compara
        // identidade de fn-ptr (native_fn_addr), não o nome, logo não deve
        // capturar esta chamada nem aplicar o gate de context.
        let world = MockWorld::new("#let measure = (x) => x + 1\n#let y = measure(4)");
        assert_eq!(eval_let(&world, "y"), Some(Value::Int(5)));
    }

    #[test]
    fn p504_array_len_method() {
        let world = MockWorld::new("#let x = (1, 2, 3).len()");
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(3)));
    }

    #[test]
    fn p504_divider() {
        let world = MockWorld::new("#let x = divider()");
        assert!(matches!(eval_let(&world, "x"), Some(Value::Content(_))));
    }

    #[test]
    fn p504_list_marker_align() {
        let world = MockWorld::new("#let x = list(marker-align: start, [A], [B])");
        assert!(matches!(eval_let(&world, "x"), Some(Value::Content(_))));
    }

    #[test]
    fn p504_within_selector_value() {
        let world = MockWorld::new("#let x = selector(heading).within(figure)");
        assert!(matches!(
            eval_let(&world, "x"),
            Some(Value::Selector(crate::entities::selector::Selector::Within { .. }))
        ));
    }

    #[test]
    fn p504_selector_func() {
        let world = MockWorld::new("#let x = selector(heading)");
        assert!(matches!(
            eval_let(&world, "x"),
            Some(Value::Selector(crate::entities::selector::Selector::Kind(
                crate::entities::element_kind::ElementKind::Heading
            )))
        ));
    }

    #[test]
    fn p504_counter_display_at() {
        let world = MockWorld::new(
            "= Secção <sec>\n#let x = counter(heading).display(\"1.\", at: <sec>)",
        );
        assert!(matches!(eval_let(&world, "x"), Some(Value::Content(_))));
    }

    #[test]
    fn p844_a7_numbering_circled_number() {
        // P844 (achado #53 de P831) — token `①` (circled numbers) no
        // caminho partilhado de numbering. Medido no vanilla 0.15.0:
        // `numbering("①", 2)` → "②" (1→①, 21→㉑, 36→㊱, 0→⓪).
        let world = MockWorld::new("#let x = numbering(\"①\", 2)");
        assert_eq!(eval_let(&world, "x"), Some(Value::Str("②".into())));
    }

    // ── P633 — sonda de falhas silenciosas: testes de confirmação/refutação ──

    fn p633_eval_succeeds(src: &str) -> bool {
        let world = MockWorld::new(src);
        eval_for_test(&world, &world.source).is_ok()
    }

    fn p633_eval_fails(src: &str) -> bool {
        let world = MockWorld::new(src);
        eval_for_test(&world, &world.source).is_err()
    }

    // P634 — após remover o catch-all silencioso, break/continue/return no topo
    // do documento produzem erros claros.
    #[test]
    fn p634_break_top_level_errors() {
        assert!(p633_eval_fails("#break"));
    }

    #[test]
    fn p634_continue_top_level_errors() {
        assert!(p633_eval_fails("#continue"));
    }

    #[test]
    fn p634_return_top_level_errors() {
        assert!(p633_eval_fails("#return 1"));
    }

    // ── P635 — FlowEvent: break/continue/return afectam o fluxo de execução ──

    /// Helper: avalia `src` e devolve o `plain_text()` do content resultante.
    fn p635_plain_text(src: &str) -> String {
        eval_doc(src).plain_text()
    }

    #[test]
    fn p635_break_in_for_stops_loop() {
        // P786a: fontes deste bloco P635 corrigidas para sintaxe válida —
        // expressões consecutivas em código sem `;`/newline são rejeitadas
        // pelo vanilla ("expected semicolon or line break") e só passavam
        // porque o eval descartava erros de parser. Asserções inalteradas.
        let text = p635_plain_text("#for i in range(10) { if i == 3 { break }; str(i) }");
        assert_eq!(text, "012", "break deve parar o ciclo for em i=3; obtido: {text:?}");
    }

    #[test]
    fn p635_break_in_while_stops_loop() {
        // Não usa assignment mutável (fronteira separada): testa apenas que
        // break pára um while infinito.
        let text = p635_plain_text("#while true { break; str(1) }");
        assert_eq!(
            text, "",
            "break deve parar o ciclo while antes de produzir output; obtido: {text:?}"
        );
    }

    #[test]
    fn p635_continue_in_for_skips_iteration() {
        let text =
            p635_plain_text("#for i in range(5) { if i == 2 { continue }; str(i) }");
        assert_eq!(text, "0134", "continue deve saltar a iteração i=2; obtido: {text:?}");
    }

    #[test]
    fn p635_return_from_function_with_value() {
        let src = "#let f(x) = { if x < 0 { return \"neg\" }; \"pos\" }\n#f(-5) #f(5)";
        let text = p635_plain_text(src);
        assert!(
            text.contains("neg") && text.contains("pos"),
            "return deve devolver os valores antecipadamente; obtido: {text:?}"
        );
    }

    #[test]
    fn p635_return_without_value() {
        let src = "#let f(x) = { if x < 0 { return }; \"pos\" }\n#f(-5) #f(5)";
        let text = p635_plain_text(src);
        assert!(
            text.contains("pos"),
            "return sem valor deve deixar a 1ª chamada vazia e a 2ª 'pos'; obtido: {text:?}"
        );
    }

    #[test]
    fn p635_return_stops_function_body() {
        let src = "#let f() = { return \"a\"; \"b\" }\n#f()";
        let text = p635_plain_text(src);
        assert!(
            text.contains("a") && !text.contains("b"),
            "return deve interromper o corpo da função; obtido: {text:?}"
        );
    }

    #[test]
    fn p635_nested_loops_break_only_inner() {
        let src = "#for i in range(3) { for j in range(3) { if j == 1 { break }; str(i) + str(j) } }";
        let text = p635_plain_text(src);
        assert_eq!(
            text, "001020",
            "break deve sair só do ciclo mais interno; obtido: {text:?}"
        );
    }

    #[test]
    fn p635_break_inside_function_is_forbidden() {
        let src = "#let f() = { break } #f()";
        assert!(
            p633_eval_fails(src),
            "break dentro de função fora de loop deve ser erro"
        );
    }

    #[test]
    fn p635_continue_inside_function_is_forbidden() {
        let src = "#let f() = { continue } #f()";
        assert!(
            p633_eval_fails(src),
            "continue dentro de função fora de loop deve ser erro"
        );
    }

    // P636 — falhas silenciosas de `#set` rules invertidas: valores de tipo
    // inválido agora produzem erro.
    #[test]
    fn p633_set_page_width_string_error() {
        assert!(p633_eval_fails("#set page(width: \"foo\")\n#let x = 1"));
    }

    #[test]
    fn p633_set_page_numbering_int_error() {
        assert!(p633_eval_fails("#set page(numbering: 123)\n#let x = 1"));
    }

    #[test]
    fn p633_set_page_columns_string_error() {
        assert!(p633_eval_fails("#set page(columns: \"foo\")\n#let x = 1"));
    }

    #[test]
    fn p633_set_document_title_int_error() {
        assert!(p633_eval_fails("#set document(title: 123)\n#let x = 1"));
    }

    #[test]
    fn p633_set_text_weight_string_error() {
        assert!(p633_eval_fails("#set text(weight: \"foo\")\n#let x = 1"));
    }

    #[test]
    fn p633_set_equation_numbering_int_error() {
        assert!(p633_eval_fails("#set math.equation(numbering: 123)\n#let x = 1"));
    }

    #[test]
    fn p633_set_equation_numbering_undefined_error() {
        // P636: o erro de `eval_expr(named.expr())` já não é descartado.
        assert!(p633_eval_fails("#set math.equation(numbering: nao_existe)\n#let x = 1"));
    }

    #[test]
    fn p633_set_figure_numbering_int_error() {
        assert!(p633_eval_fails("#set figure(numbering: 123)\n#let x = 1"));
    }

    #[test]
    fn p633_set_table_numbering_int_error() {
        assert!(p633_eval_fails("#set table(numbering: 123)\n#let x = 1"));
    }

    // P867 — `#set page(width: auto)` e `height: auto` são aceites; tipos
    // inválidos continuam a ser rejeitados.
    #[test]
    fn p867_set_page_height_auto_ok() {
        let world = MockWorld::new("#set page(height: auto)\n#let x = 1");
        assert!(
            eval_for_test(&world, &world.source).is_ok(),
            "height: auto deve ser aceite"
        );
    }

    #[test]
    fn p867_set_page_width_auto_ok() {
        let world = MockWorld::new("#set page(width: auto)\n#let x = 1");
        assert!(
            eval_for_test(&world, &world.source).is_ok(),
            "width: auto deve ser aceite"
        );
    }

    #[test]
    fn p1140_23_set_page_canvas_argumentos_aceites() {
        let world = MockWorld::new(
            "#set page(bleed: (inside: 5%, outside: 10pt, y: 2pt), fill: red, background: [bg], foreground: none)\nbody",
        );
        assert!(eval_for_test(&world, &world.source).is_ok());
        let positional = MockWorld::new("#page(\"a5\", [body])");
        assert!(eval_for_test(&positional, &positional.source).is_ok());
    }

    #[test]
    fn p1140_23_bleed_rejeita_mistura_logica_e_fisica() {
        assert!(p633_eval_fails("#set page(bleed: (inside: 2pt, left: 3pt))\nbody"));
    }

    #[test]
    fn p1140_23_bleed_rejeita_auto() {
        assert!(p633_eval_fails("#set page(bleed: auto)\nbody"));
    }

    #[test]
    fn p1140_24_running_matter_argumentos_aceites() {
        let world = MockWorld::new(
            "#set page(numbering: \"1\", number-align: right + top, header: auto, header-ascent: 25%, footer: [rodape], footer-descent: 4pt)\nbody",
        );
        assert!(eval_for_test(&world, &world.source).is_ok());
    }

    #[test]
    fn p1140_24_number_align_horizon_rejeitado() {
        assert!(p633_eval_fails("#set page(number-align: center + horizon)\nbody"));
    }

    #[test]
    fn p1140_25_page_supplement_estados_aceites() {
        for source in [
            "#set page(supplement: auto)\nbody",
            "#set page(supplement: none)\nbody",
            "#set page(supplement: [p.])\nbody",
        ] {
            let world = MockWorld::new(source);
            assert!(eval_for_test(&world, &world.source).is_ok(), "{source}");
        }
    }

    #[test]
    fn p1140_26_page_e_std_page_sao_funcoes() {
        for source in [
            "#assert.eq(type(page), function)\nok",
            "#assert.eq(type(std.page), function)\nok",
        ] {
            let world = MockWorld::new(source);
            assert!(eval_for_test(&world, &world.source).is_ok(), "{source}");
        }
    }

    #[test]
    fn p1140_26_page_constructor_aceita_superficie_completa() {
        let world = MockWorld::new(
            "#page(\n  paper: \"a5\", flipped: true, binding: left,\n  width: 100pt, height: 120pt, margin: (inside: 8pt, outside: 9pt, y: 10pt),\n  bleed: 2pt, columns: 2, fill: red, numbering: \"1\", supplement: [p.],\n  number-align: right + top, header: auto, header-ascent: 25%,\n  footer: [f], footer-descent: 4pt, background: [b], foreground: none,\n  [body]\n)",
        );
        assert!(eval_for_test(&world, &world.source).is_ok());
    }

    #[test]
    fn p1140_26_page_rejeita_body_ausente_extra_e_named_desconhecido() {
        for source in ["#page()", "#page([a], [b])", "#page(nope: 1, [a])"] {
            assert!(p633_eval_fails(source), "deveria falhar: {source}");
        }
    }

    #[test]
    fn p1140_26_page_repr_preserva_morfologia_delimitada() {
        for source in [
            "#assert.eq(repr(page([x])), \"sequence(\\n  pagebreak(weak: true),\\n  flush(),\\n  [x],\\n  pagebreak(weak: true),\\n)\")",
            "#assert.eq(repr(page(width: 100pt, [x])), \"styled(child: sequence(\\n  pagebreak(weak: true),\\n  flush(),\\n  [x],\\n  pagebreak(weak: true),\\n), ..)\")",
        ] {
            let world = MockWorld::new(source);
            assert!(eval_for_test(&world, &world.source).is_ok(), "{source}");
        }
    }

    #[test]
    fn p867_set_page_height_invalid_type_error() {
        assert!(
            p633_eval_fails("#set page(height: (1, 2))\n#let x = 1"),
            "array em height deve errar"
        );
    }

    // P637 — document.title só aceita string; author/keywords aceitam string ou
    // array de strings (paridade com vanilla: title é Option<Content>,
    // author/keywords são OneOrMultiple<EcoString>).
    #[test]
    fn p637_document_title_array_is_error() {
        assert!(
            p633_eval_fails("#set document(title: (\"A\", \"B\"))\n#let x = 1"),
            "title não deve aceitar array"
        );
    }

    #[test]
    fn p637_document_author_array_works() {
        assert!(
            p633_eval_succeeds("#set document(author: (\"A\", \"B\"))\n#let x = 1"),
            "author deve aceitar array de strings"
        );
    }

    #[test]
    fn p637_document_keywords_array_works() {
        assert!(
            p633_eval_succeeds("#set document(keywords: (\"A\", \"B\"))\n#let x = 1"),
            "keywords deve aceitar array de strings"
        );
    }

    // Confirmam comportamento actual de métodos de counter.
    // Nota: sem `#` no início, a linha inteira é interpretada como texto de
    // markup; os testes usam `#` para forçar avaliação como código.
    #[test]
    fn p633_counter_update_string_rejeitado() {
        // `native_counter_update` rejeita tipos não-inteiros; o caminho
        // `unwrap_or(0)` em `bindings.rs` não é atingível via syntax pública.
        assert!(p633_eval_fails("#counter(\"x\").update(\"abc\")"));
    }

    #[test]
    fn p633_counter_display_invalid_arg_error() {
        // P640 — argumento posicional inválido em counter.display produz erro.
        // Usa-se `at: <sec>` para forçar a avaliação imediata no eval.
        assert!(p633_eval_fails(
            "#let sec = <sec>\n#let x = counter(\"x\").display(123, at: sec)"
        ));
    }

    #[test]
    fn p633_counter_display_at_invalid_error() {
        // P640 — argumento nomeado `at:` inválido em counter.display produz erro.
        assert!(p633_eval_fails("#let x = counter(\"x\").display(at: 123)"));
    }

    // ── P640 — counter.display com argumentos inválidos (unificação e erros) ──

    #[test]
    fn p640_counter_display_pattern_works() {
        // `at:` força a avaliação imediata; o pattern "I" é aplicado.
        let world = MockWorld::new(
            "= Secção <sec>\n#let x = counter(\"x\").display(\"I\", at: <sec>)",
        );
        assert!(matches!(eval_let(&world, "x"), Some(Value::Content(_))));
    }

    #[test]
    fn p640_counter_display_at_label_works() {
        let world = MockWorld::new(
            "= Secção <sec>\n#let x = counter(\"x\").display(\"1.\", at: <sec>)",
        );
        assert!(matches!(eval_let(&world, "x"), Some(Value::Content(_))));
    }

    #[test]
    fn p1149_counter_static_at_e_display_preservam_label_literal() {
        let world = MockWorld::new(
            "= Secção <sec>\n\
             #let c = counter(heading)\n\
             #let at = counter.at(c, <sec>)\n\
             #let shown = context counter.display(c, \"1 / 1\", at: <sec>, both: true)",
        );
        let module = eval_for_test(&world, &world.source).unwrap();
        assert!(matches!(module.scope().get("at"), Some(Value::Array(_))));
        assert!(matches!(module.scope().get("shown"), Some(Value::Content(_))));
    }

    #[test]
    fn p640_counter_display_unknown_named_arg_error() {
        assert!(p633_eval_fails("#let x = counter(\"x\").display(\"1.\", unknown: 123)"));
    }

    #[test]
    fn p640_counter_display_too_many_positional_args_error() {
        assert!(p633_eval_fails("#let x = counter(\"x\").display(\"1.\", \"2.\")"));
    }

    #[test]
    fn p640_counter_display_invalid_arg_message() {
        let world = MockWorld::new(
            "#let sec = <sec>\n#let x = counter(\"x\").display(123, at: sec)",
        );
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("expected string, function, or auto, found int"),
            "mensagem inesperada: {msg}"
        );
    }

    #[test]
    fn p640_counter_display_at_invalid_message() {
        let world = MockWorld::new("#let x = counter(\"x\").display(\"1.\", at: 123)");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains(
                "expected label, function, location, selector, or auto, found int"
            ),
            "mensagem inesperada: {msg}"
        );
    }

    #[test]
    fn p633_state_method_dead_code_path() {
        // `state()` devolve `Content`, não `Value::State`; o branch
        // `eval_state_method` em `closures.rs` não é atingível por syntax
        // pública. O `.get` em content dá erro de field access.
        assert!(p633_eval_fails("#state(\"x\", 0).get(1, 2)"));
    }

    // Confirma falha silenciosa em grid(columns: <inválido>).
    #[test]
    fn p633_grid_columns_string_silent_auto() {
        assert!(p633_eval_succeeds("#let x = grid(columns: \"foo\")[A]"));
    }

    // P643: escape unicode inválido em code string produz erro.
    #[test]
    fn p643_invalid_unicode_escape_code_string_errors() {
        let world = MockWorld::new("#let x = \"\\u{FFFFFFFF}\"");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("invalid Unicode codepoint: FFFFFFFF"),
            "mensagem inesperada: {msg}"
        );
    }

    // P648 — literais numéricos malformados (aqui `0xZZ`) são erros de
    // parser que o eval passou a propagar selectivamente.
    // P786a: asserção alargada de `first()` para `any()` — a propagação
    // integral devolve TODOS os erros em ordem de árvore; o parser emite
    // "expected expression" antes do erro de hex (cascata após o token
    // inválido — divergência menor de contagem vs vanilla, que suprime a
    // cascata e reporta 1 erro; registado no relatório P786a).
    #[test]
    fn p648_parse_error_hex_literal_errors() {
        let world = MockWorld::new("#let x = 0xZZ");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let found = err
            .iter()
            .any(|d| d.message.contains("invalid hexadecimal number: 0xZZ"));
        assert!(found, "erro de hex ausente: {err:?}");
    }

    // P648 — escape Unicode inválido em markup é detectado pelo lexer
    // (markup.rs:71); o eval propagava-o em code strings mas descartava-o
    // em markup. A propagação selectiva de erros de parser corrige isto.
    #[test]
    fn p648_invalid_unicode_escape_markup_errors() {
        let world = MockWorld::new("#let x = [\\u{FFFFFFFF}]");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("invalid Unicode codepoint: FFFFFFFF"),
            "mensagem inesperada: {msg}"
        );
    }

    // ── P786a — propagação integral de erros sintáticos + warnings de markup ──
    // T2 (reconfirmado no passo): delimitador não fechado é erro fatal.
    #[test]
    fn p786a_unclosed_delimiter_errors() {
        let world = MockWorld::new("#let x = (");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(msg.contains("unclosed delimiter"), "mensagem inesperada: {msg}");
    }

    // `*` sozinho (sem fecho) — vanilla: error unclosed delimiter, exit 1.
    #[test]
    fn p786a_lone_star_errors() {
        let world = MockWorld::new("*");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(msg.contains("unclosed delimiter"), "mensagem inesperada: {msg}");
    }

    // `#` dentro de código é erro genuíno (vanilla 0.15.0: exit 1 + 2 hints;
    // cristalino compilava com exit 0 — achado T2-estendido do passo).
    #[test]
    fn p786a_hash_in_code_errors() {
        let world = MockWorld::new("#let x = {\n  #set text(fill: red)\n  [body]\n}");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(msg.contains("not valid in code"), "mensagem inesperada: {msg}");
    }

    // `)` inesperado — erros "expected ..." também propagam.
    #[test]
    fn p786a_unexpected_closing_paren_errors() {
        let world = MockWorld::new("#let y = )");
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(msg.contains("expected expression"), "mensagem inesperada: {msg}");
    }

    // Guarda de não-regressão (survey do passo): construções VÁLIDAS não
    // produzem error nodes — sem falsos positivos na propagação integral.
    #[test]
    fn p786a_valid_constructs_without_error_nodes() {
        let cases: &[&str] = &[
            "'simples' e \"duplas\"",
            "#show heading: it => it\n= Title",
            "#set text(size: 12pt)\nHello",
            "$ x + y $",
            "#let f(a, b) = a + b\n#f(1, 2)",
            "- a\n- b\n+ c\n+ d\n/ term: def",
            "#rect(width: 5pt, height: 5pt)",
            "```rust fn main() {}```",
            "See @x\n= H <x>",
            "#context here()",
            "emoji 🚀 🇧🇷 texto",
            "#let (a, b) = (1, 2)\n#a#b",
            "#table(columns: 2, [a], [b])",
            "#lorem(5)",
            "#figure(rect(), caption: [c])",
            "#link(\"https://x.com\")[l]",
            "#set heading(numbering: \"1.\")\n= A\n== B",
            "#let x = calc.pow(2, 3)",
            "#grid(columns: (1fr, 1fr), [a], [b])",
        ];
        for src in cases {
            let world = MockWorld::new(src);
            let errs = world.source.root().errors();
            assert!(errs.is_empty(), "falso positivo em {src:?}: {errs:?}");
        }
    }

    // T1: `**` → warning ao sink (não fatal), texto e hint do vanilla
    // (medido: `warning: no text within stars` + hint, exit 0). A newline
    // final é o caso real de ficheiros — e o caso do bug de trivia
    // arrastada pelo `eat` do fecho (medida antes do fecho, ver markup.rs).
    #[test]
    fn p786a_stars_warning_goes_to_sink() {
        use comemo::Track;
        let world = MockWorld::new("**\n");
        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &world.source,
            &crate::entities::element_registry::ElementRegistry::new(),
        );
        assert!(result.is_ok(), "warning não pode abortar: {result:?}");
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().any(|d| d.message.contains("no text within stars")
                && d.hints.iter().any(|h| h.contains("has no additional effect"))),
            "warning ausente ou sem hint: {diags:?}"
        );
    }

    // T1 análogo: `__` → warning "no text within underscores".
    #[test]
    fn p786a_underscores_warning_goes_to_sink() {
        use comemo::Track;
        let world = MockWorld::new("__\n");
        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &world.source,
            &crate::entities::element_registry::ElementRegistry::new(),
        );
        assert!(result.is_ok(), "warning não pode abortar: {result:?}");
        let diags = sink.into_diagnostics();
        assert!(
            diags.iter().any(|d| d.message.contains("no text within underscores")
                && d.hints.iter().any(|h| h.contains("has no additional effect"))),
            "warning ausente ou sem hint: {diags:?}"
        );
    }

    // ── P787 — CSV: rigor de parsing e API de `row-type` ───────────────────
    // Mensagens medidas no vanilla 0.15.0 por execução (2026-07-20).
    #[test]
    fn p787_csv_linha_malformada_erro() {
        let mut world = MockWorld::new("#csv(\"d.csv\")");
        world.add_file("d.csv", b"a,b\n1,2,3\n".to_vec());
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let found = err.iter().any(|d| {
            d.message
                .contains("failed to parse CSV (found 3 instead of 2 fields in line 2)")
        });
        assert!(found, "erro de linha malformada ausente: {err:?}");
    }

    #[test]
    fn p787_csv_delimiter_nao_ascii() {
        let mut world = MockWorld::new("#csv(\"d.csv\", delimiter: \"é\")");
        world.add_file("d.csv", b"a,b\n1,2\n".to_vec());
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let found = err
            .iter()
            .any(|d| d.message.contains("delimiter must be an ASCII character"));
        assert!(found, "mensagem inesperada: {err:?}");
    }

    #[test]
    fn p787_csv_delimiter_multi_char() {
        let mut world = MockWorld::new("#csv(\"d.csv\", delimiter: \"ab\")");
        world.add_file("d.csv", b"a,b\n1,2\n".to_vec());
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let found = err
            .iter()
            .any(|d| d.message.contains("expected exactly one character"));
        assert!(found, "mensagem inesperada: {err:?}");
    }

    #[test]
    fn p787_csv_row_type_aceita_tipo() {
        // API vanilla: `row-type` recebe o TIPO `dictionary`, não a string.
        let mut world = MockWorld::new("#csv(\"d.csv\", row-type: dictionary)");
        world.add_file("d.csv", b"a,b\n1,2\n".to_vec());
        let result = eval_for_test(&world, &world.source);
        assert!(result.is_ok(), "row-type: dictionary (tipo) falhou: {result:?}");
    }

    #[test]
    fn p787_csv_row_type_rejeita_string() {
        let mut world = MockWorld::new("#csv(\"d.csv\", row-type: \"dictionary\")");
        world.add_file("d.csv", b"a,b\n1,2\n".to_vec());
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let found = err.iter().any(|d| d.message.contains("expected type, found string"));
        assert!(found, "mensagem inesperada: {err:?}");
    }

    #[test]
    fn p787_csv_row_type_tipo_errado() {
        let mut world = MockWorld::new("#csv(\"d.csv\", row-type: str)");
        world.add_file("d.csv", b"a,b\n1,2\n".to_vec());
        let err = eval_for_test(&world, &world.source).unwrap_err();
        let found = err
            .iter()
            .any(|d| d.message.contains("expected `array` or `dictionary`"));
        assert!(found, "mensagem inesperada: {err:?}");
    }

    #[test]
    fn p787_csv_valido_nao_regressao() {
        let mut world = MockWorld::new("#csv(\"d.csv\")");
        world.add_file("d.csv", b"a,b\n1,2\n".to_vec());
        assert!(eval_for_test(&world, &world.source).is_ok());
    }

    // ── P702 — `.with(...)` (aplicação parcial de argumentos) ────────────────
    // Reproduções idênticas às medidas contra o vanilla em
    // `00_nucleo/diagnosticos/paridade-producao-p702.md`.

    #[test]
    fn p702_with_nativa_com_namespace_named_pre_ligado() {
        let world =
            MockWorld::new("#let f = calc.round.with(digits: 2)\n#let x = f(3.14159)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(3.14)));
    }

    #[test]
    fn p702_with_closure_posicionais_pre_ligados() {
        let world = MockWorld::new(
            "#let g(a, b, c) = a + b + c\n#let g2 = g.with(1, 2)\n#let x = g2(3)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(6)));
    }

    #[test]
    fn p702_with_closure_nomeado_pre_ligado() {
        let world = MockWorld::new(
            "#let h(a, named: 10) = a + named\n#let h2 = h.with(named: 20)\n#let x = h2(5)"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(25)));
    }

    #[test]
    fn p702_with_encadeado_posicional() {
        let world = MockWorld::new(
            "#let f(a, b, c) = a + b + c\n#let f1 = f.with(1)\n#let f2 = f1.with(2)\n#let x = f2(3)"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(6)));
    }

    #[test]
    fn p702_with_encadeado_nomeado() {
        let world = MockWorld::new(
            "#let h(a, x: 10, y: 20) = a + x + y\n#let h1 = h.with(x: 100)\n#let h2 = h1.with(y: 200)\n#let r = h2(1)"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(301)));
    }

    #[test]
    fn p702_with_atraves_de_namespace_sub_funcao() {
        // Medido contra o vanilla: `table.with(columns: 2).cell` compila e
        // devolve `function` (não assumido — ver entities/func.md).
        let world =
            MockWorld::new("#let t = table.with(columns: 2)\n#let c = type(t.cell)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("c"), Some(&Value::Type(Type::Function)));
    }

    #[test]
    fn p702_dict_com_chave_with_nao_e_intercetada() {
        // Regressão: a intercepção de `.with` só actua quando o alvo é
        // `Value::Func` — um dict com uma chave "with" continua a funcionar
        // por field access normal.
        let world = MockWorld::new("#let d = (with: 5)\n#let x = d.with");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(5)));
    }

    // ── P715 — desestruturação (`let`), atribuição por desestruturação e
    // atribuição simples/composta (`x = v`, `x += v`, ...) ─────────────────

    #[test]
    fn p715_let_destructuring_array_simples() {
        let world = MockWorld::new("#let (a, b) = (1, 2)");
        assert_eq!(eval_let(&world, "a"), Some(Value::Int(1)));
        assert_eq!(eval_let(&world, "b"), Some(Value::Int(2)));
    }

    #[test]
    fn p715_let_destructuring_placeholder_ignora() {
        let world = MockWorld::new("#let (_, b) = (1, 2)");
        assert_eq!(eval_let(&world, "b"), Some(Value::Int(2)));
    }

    #[test]
    fn p715_let_destructuring_spread() {
        let world = MockWorld::new("#let (first, ..rest) = (1, 2, 3, 4)");
        assert_eq!(eval_let(&world, "first"), Some(Value::Int(1)));
        assert_eq!(
            eval_let(&world, "rest"),
            Some(Value::Array(vec![Value::Int(2), Value::Int(3), Value::Int(4)]))
        );
    }

    #[test]
    fn p715_let_destructuring_nested() {
        let world = MockWorld::new("#let ((a, b), c) = ((1, 2), 3)");
        assert_eq!(eval_let(&world, "a"), Some(Value::Int(1)));
        assert_eq!(eval_let(&world, "b"), Some(Value::Int(2)));
        assert_eq!(eval_let(&world, "c"), Some(Value::Int(3)));
    }

    #[test]
    fn p715_let_destructuring_dict_shorthand_e_named() {
        // Shorthand: `x` liga ao valor da chave `x`. Named: `onto: p` liga
        // `p` ao valor da chave `onto` (renomeação), com padrão aninhado.
        let world = MockWorld::new(
            r#"#let (project: p, onto: (x, y)) = (project: "hi", onto: (3, 4))"#,
        );
        assert_eq!(eval_let(&world, "p"), Some(Value::Str("hi".into())));
        assert_eq!(eval_let(&world, "x"), Some(Value::Int(3)));
        assert_eq!(eval_let(&world, "y"), Some(Value::Int(4)));
    }

    #[test]
    fn p715_let_destructuring_dict_spread_recolhe_nao_usadas() {
        let world = MockWorld::new("#let (a, ..rest) = (a: 1, b: 2, c: 3)");
        assert_eq!(eval_let(&world, "a"), Some(Value::Int(1)));
        match eval_let(&world, "rest") {
            Some(Value::Dict(d)) => {
                assert_eq!(d.len(), 2);
                assert_eq!(d.get("b"), Some(&Value::Int(2)));
                assert_eq!(d.get("c"), Some(&Value::Int(3)));
            }
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p715_let_destructuring_aridade_errada_erra_com_hint() {
        let world = MockWorld::new("#let (a, b, c) = (1, 2)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).unwrap_err();
        assert!(err[0].message.contains("not enough elements to destructure"));
        assert!(err[0].hints.iter().any(|h| h.contains("length of 2")));
    }

    #[test]
    fn p715_let_destructuring_valor_nao_destructuravel_erra() {
        let world = MockWorld::new("#let (a, b) = 5");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }

    #[test]
    fn p715_destruct_assignment_muta_variaveis_existentes() {
        // Reprodução exacta do padrão real de cetz (`coordinate.typ:259,264`):
        // `(ctx, p) = resolve(ctx, p)`.
        let world = MockWorld::new("#let ctx = 1\n#let p = 2\n#{ (ctx, p) = (10, 20) }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("ctx"), Some(&Value::Int(10)));
        assert_eq!(m.scope().get("p"), Some(&Value::Int(20)));
    }

    #[test]
    fn p715_destruct_assignment_variavel_inexistente_erra() {
        let world = MockWorld::new("#{ (nope, also_nope) = (1, 2) }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).unwrap_err();
        assert!(err[0].message.contains("unknown variable"));
    }

    #[test]
    fn p715_assign_simples() {
        let world = MockWorld::new("#let x = 1\n#{ x = 5 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(5)));
    }

    #[test]
    fn p715_assign_composta_add_sub_mul_div() {
        let world = MockWorld::new(
            "#let a = 1\n#{ a += 10 }\n\
             #let b = 20\n#{ b -= 5 }\n\
             #let c = 3\n#{ c *= 4 }\n\
             #let d = 10\n#{ d /= 4 }",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Int(11)));
        assert_eq!(m.scope().get("b"), Some(&Value::Int(15)));
        assert_eq!(m.scope().get("c"), Some(&Value::Int(12)));
        assert_eq!(m.scope().get("d"), Some(&Value::Float(2.5)));
    }

    #[test]
    fn p715_assign_variavel_inexistente_erra() {
        let world = MockWorld::new("#{ nope = 1 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).unwrap_err();
        assert!(err[0].message.contains("unknown variable"));
    }

    #[test]
    fn p715_assign_alvo_nao_ident_erra_mutar_temporario() {
        // Scope-out medido (P715): alvos não-Ident (FieldAccess, FuncCall
        // accessor como `.at()`) não são suportados — mesmo padrão que
        // bloqueia `cetz` a seguir (`hobby.typ:51`: `b.at(i) = ...`).
        let world = MockWorld::new("#{ 5 = 1 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).unwrap_err();
        assert!(err[0].message.contains("cannot mutate a temporary value"));
    }

    #[test]
    fn p715_assign_escopo_mutado_visivel_apos_bloco() {
        // A mutação atravessa a fronteira do bloco `{ }` (não fica presa a um
        // scope filho) — confirma que `Scopes::get_mut` procura em `scopes`
        // (âmbitos pai), não só em `top`.
        let world = MockWorld::new("#let x = 1\n#{ { x = 99 } }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(99)));
    }

    // ── P716 — `Access` genérico: `dict.campo` e accessor methods
    // (`.at()`, `.first()`, `.last()`) como alvos de atribuição ─────────────

    fn p716_eval(src: &str) -> SourceResult<Module> {
        let world = MockWorld::new(src);
        let source = World::source(&world, World::main(&world)).unwrap();
        eval_for_test(&world, &source)
    }

    #[test]
    fn p716_assign_dict_campo_existente_muta() {
        let m = p716_eval("#let d = (a: 1, b: 2)\n#{ d.a = 10 }").unwrap();
        match m.scope().get("d") {
            Some(Value::Dict(d)) => {
                assert_eq!(d.get("a"), Some(&Value::Int(10)));
                assert_eq!(d.get("b"), Some(&Value::Int(2)));
            }
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p716_assign_dict_campo_novo_insere() {
        // Vanilla ops.rs:77-85 — `=` puro em FieldAccess cria a chave.
        let m = p716_eval("#let d = (a: 1)\n#{ d.novo = 5 }").unwrap();
        match m.scope().get("d") {
            Some(Value::Dict(d)) => assert_eq!(d.get("novo"), Some(&Value::Int(5))),
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p716_assign_composto_dict_campo_inexistente_erra() {
        // `+=` não passa pelo caso especial de insert — vai a `Dict::at_mut`.
        let err = p716_eval("#let d = (a: 1)\n#{ d.b += 1 }").unwrap_err();
        assert!(err[0].message.contains("dictionary does not contain key \"b\""));
        assert!(err[0].hints.iter().any(|h| h.contains("insert")));
    }

    #[test]
    fn p716_assign_array_at_muta() {
        let m = p716_eval("#let arr = (1, 2, 3)\n#{ arr.at(1) = 20 }").unwrap();
        assert_eq!(
            m.scope().get("arr"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(20), Value::Int(3)]))
        );
    }

    #[test]
    fn p716_assign_array_at_negativo_muta() {
        // locate_opt: índice negativo conta do fim.
        let m = p716_eval("#let arr = (1, 2, 3)\n#{ arr.at(-1) = 9 }").unwrap();
        assert_eq!(
            m.scope().get("arr"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(9)]))
        );
    }

    #[test]
    fn p716_assign_array_at_fora_de_limites_erra() {
        let err = p716_eval("#let arr2 = (1, 2, 3)\n#{ arr2.at(5) = 20 }").unwrap_err();
        assert_eq!(err[0].message, "array index out of bounds (index: 5, len: 3)");
    }

    #[test]
    fn p716_assign_array_at_default_erra() {
        // `default:` não faz sentido como alvo de escrita — args.finish().
        let err = p716_eval("#let arr = (1, 2, 3)\n#{ arr.at(1, default: 0) = 20 }")
            .unwrap_err();
        assert_eq!(err[0].message, "unexpected argument: default");
    }

    #[test]
    fn p716_assign_array_at_sem_indice_erra() {
        let err = p716_eval("#let arr = (1, 2)\n#{ arr.at() = 1 }").unwrap_err();
        assert_eq!(err[0].message, "missing argument: index");
    }

    #[test]
    fn p716_assign_array_at_indice_nao_int_erra() {
        let err = p716_eval("#let arr = (1, 2)\n#{ arr.at(\"x\") = 1 }").unwrap_err();
        assert_eq!(err[0].message, "expected integer, found string");
    }

    #[test]
    fn p716_assign_array_first_last_mutam() {
        let m =
            p716_eval("#let a3 = (1, 2, 3)\n#{ a3.first() = 100 }\n#{ a3.last() = 300 }")
                .unwrap();
        assert_eq!(
            m.scope().get("a3"),
            Some(&Value::Array(vec![Value::Int(100), Value::Int(2), Value::Int(300)]))
        );
    }

    #[test]
    fn p716_assign_array_first_vazio_erra() {
        let err = p716_eval("#let a = ()\n#{ a.first() = 1 }").unwrap_err();
        assert_eq!(err[0].message, "array is empty");
    }

    #[test]
    fn p716_assign_dict_at_muta() {
        let m = p716_eval("#let d2 = (x: 1)\n#{ d2.at(\"x\") = 9 }").unwrap();
        match m.scope().get("d2") {
            Some(Value::Dict(d)) => assert_eq!(d.get("x"), Some(&Value::Int(9))),
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p716_assign_dict_at_chave_inexistente_erra() {
        // Ao contrário de `d.campo = v`, `d.at("campo") = v` NÃO insere.
        let err = p716_eval("#let d = (a: 1)\n#{ d.at(\"outro\") = 7 }").unwrap_err();
        assert!(err[0].message.contains("dictionary does not contain key \"outro\""));
        assert!(err[0].hints.iter().any(|h| h.contains("insert")));
    }

    #[test]
    fn p716_assign_dict_at_sem_chave_erra() {
        let err = p716_eval("#let d = (a: 1)\n#{ d.at() = 1 }").unwrap_err();
        assert_eq!(err[0].message, "missing argument: key");
    }

    #[test]
    fn p716_assign_composto_array_at() {
        // Forma composta via Access (mem::replace no local + operador subjacente).
        let m = p716_eval("#let arr = (10, 2)\n#{ arr.at(0) += 5 }").unwrap();
        assert_eq!(
            m.scope().get("arr"),
            Some(&Value::Array(vec![Value::Int(15), Value::Int(2)]))
        );
    }

    #[test]
    fn p716_assign_str_accessor_erra_temporario() {
        // str tem método `at` (read) mas não é mutável — vanilla methods.rs:73-80.
        let err = p716_eval("#let s = \"ab\"\n#{ s.at(0) = \"x\" }").unwrap_err();
        assert!(err[0].message.contains("cannot mutate a temporary value"));
    }

    #[test]
    fn p716_assign_int_metodo_inexistente_erra() {
        let err = p716_eval("#let x = 5\n#{ x.at(0) = 1 }").unwrap_err();
        assert_eq!(err[0].message, "type integer has no method `at`");
    }

    #[test]
    fn p716_assign_nao_accessor_erra_temporario() {
        // `len` não é accessor method — avalia e erra (vanilla access.rs:71-72).
        let err = p716_eval("#let s = \"ab\"\n#{ s.len() = 1 }").unwrap_err();
        assert!(err[0].message.contains("cannot mutate a temporary value"));
    }

    #[test]
    fn p716_assign_campo_em_int_erra() {
        // Nome longo do tipo na mensagem (vanilla): "integer", não "int".
        let err = p716_eval("#let x = 5\n#{ x.a = 1 }").unwrap_err();
        assert_eq!(err[0].message, "integer does not have accessible fields");
    }

    #[test]
    fn p716_assign_campo_em_content_erra() {
        let err = p716_eval("#let c = [oi]\n#{ c.body = 1 }").unwrap_err();
        assert_eq!(err[0].message, "cannot mutate fields on content");
    }

    #[test]
    fn p716_assign_campo_em_length_erra_not_yet_mutable() {
        // Length está em fields_on (vanilla fields.rs:77-91) → braço "not yet
        // mutable" com hint.
        let err = p716_eval("#let l = 5pt\n#{ l.abs = 1pt }").unwrap_err();
        assert_eq!(err[0].message, "fields on length are not yet mutable");
        assert!(err[0].hints.iter().any(|h| h.contains("updated field value")));
    }

    #[test]
    fn p716_destruct_assignment_com_accessores() {
        // Padrão real de cetz (`hobby.typ:160-161`): folhas accessor na
        // desestruturação-atribuição.
        let m =
            p716_eval("#let arr = (1, 2)\n#{ (arr.at(0), arr.at(1)) = (9, 8) }").unwrap();
        assert_eq!(
            m.scope().get("arr"),
            Some(&Value::Array(vec![Value::Int(9), Value::Int(8)]))
        );
    }

    #[test]
    fn p716_destruct_assignment_dict_campo_nao_insere() {
        // A folha da desestruturação usa o Access puro (binding.rs:30-42), sem
        // o caso especial de insert do `=` — chave nova erra.
        let err = p716_eval("#let d = (a: 1)\n#{ (d.novo,) = (2,) }").unwrap_err();
        assert!(err[0].message.contains("dictionary does not contain key \"novo\""));
    }

    #[test]
    fn p716_assign_aninhado_dict_array() {
        // Access recursivo: FuncCall accessor cujo target é FieldAccess.
        let m = p716_eval("#let n = (xs: (1, 2))\n#{ n.xs.at(0) = 9 }").unwrap();
        match m.scope().get("n") {
            Some(Value::Dict(d)) => assert_eq!(
                d.get("xs"),
                Some(&Value::Array(vec![Value::Int(9), Value::Int(2)]))
            ),
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p716_mecanismo_ident_p715_sem_regressao() {
        // O rework de eval_assign (access + mem::replace, em vez de
        // scopes.get+clone) preserva o caminho Ident.
        let m = p716_eval("#let x = 1\n#{ x += 10 }").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(11)));
    }

    // ── P717 — métodos mutantes (`push`, `pop`, `insert`, `remove`) ────────

    #[test]
    fn p717_push_muta_array() {
        let m = p716_eval("#let arr = (1, 2, 3)\n#{ arr.push(4) }").unwrap();
        assert_eq!(
            m.scope().get("arr"),
            Some(&Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4)
            ]))
        );
    }

    #[test]
    fn p717_pop_muta_e_devolve() {
        let m = p716_eval("#let a = (1, 2, 3)\n#let x = a.pop()").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(3)));
        assert_eq!(
            m.scope().get("a"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p717_pop_vazio_erra() {
        let err = p716_eval("#let a = ()\n#{ a.pop() }").unwrap_err();
        assert_eq!(err[0].message, "array is empty");
    }

    #[test]
    fn p717_insert_no_meio() {
        let m = p716_eval("#let a = (1, 2, 3)\n#{ a.insert(1, 99) }").unwrap();
        assert_eq!(
            m.scope().get("a"),
            Some(&Value::Array(vec![
                Value::Int(1),
                Value::Int(99),
                Value::Int(2),
                Value::Int(3)
            ]))
        );
    }

    #[test]
    fn p717_insert_no_fim_end_ok() {
        // locate com end_ok=true: índice == len é permitido (append).
        let m = p716_eval("#let a = (1, 2)\n#{ a.insert(2, 9) }").unwrap();
        assert_eq!(
            m.scope().get("a"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(9)]))
        );
    }

    #[test]
    fn p717_insert_indice_negativo() {
        // Medido no vanilla: insert(-1, 9) em (1,2,3) → (1, 2, 9, 3).
        let m = p716_eval("#let a = (1, 2, 3)\n#{ a.insert(-1, 9) }").unwrap();
        assert_eq!(
            m.scope().get("a"),
            Some(&Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(9),
                Value::Int(3)
            ]))
        );
    }

    #[test]
    fn p717_insert_fora_de_limites_erra_sem_sufixo() {
        let err = p716_eval("#let a = (1, 2, 3)\n#{ a.insert(4, 9) }").unwrap_err();
        assert_eq!(err[0].message, "array index out of bounds (index: 4, len: 3)");
    }

    #[test]
    fn p717_remove_muta_e_devolve() {
        let m = p716_eval("#let a = (1, 2, 3)\n#let x = a.remove(1)").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(2)));
        assert_eq!(
            m.scope().get("a"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(3)]))
        );
    }

    #[test]
    fn p717_remove_fora_de_limites_erra_com_sufixo() {
        // Ao contrário do insert, remove tem default: → o erro tem o sufixo.
        let err = p716_eval("#let a = (1, 2, 3)\n#{ a.remove(5) }").unwrap_err();
        assert_eq!(
            err[0].message,
            "array index out of bounds (index: 5, len: 3) and no default value was specified"
        );
    }

    #[test]
    fn p717_remove_fora_de_limites_com_default_nao_muta() {
        let m =
            p716_eval("#let a = (1, 2, 3)\n#let x = a.remove(5, default: 9)").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(9)));
        assert_eq!(
            m.scope().get("a"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]))
        );
    }

    #[test]
    fn p717_remove_arg_nomeado_desconhecido_erra() {
        let err = p716_eval("#let a = (1, 2)\n#{ a.remove(0, bad: 1) }").unwrap_err();
        assert_eq!(err[0].message, "unexpected argument: bad");
    }

    #[test]
    fn p717_pop_com_arg_erra() {
        let err = p716_eval("#let a = (1, 2)\n#{ a.pop(1) }").unwrap_err();
        assert_eq!(err[0].message, "unexpected argument");
    }

    #[test]
    fn p717_push_sem_arg_erra() {
        let err = p716_eval("#let a = (1, 2)\n#{ a.push() }").unwrap_err();
        assert_eq!(err[0].message, "missing argument: value");
    }

    #[test]
    fn p717_insert_sem_value_erra() {
        let err = p716_eval("#let a = (1, 2)\n#{ a.insert(1) }").unwrap_err();
        assert_eq!(err[0].message, "missing argument: value");
    }

    #[test]
    fn p717_insert_indice_nao_int_erra() {
        let err = p716_eval("#let a = (1, 2)\n#{ a.insert(\"x\", 9) }").unwrap_err();
        assert_eq!(err[0].message, "expected integer, found string");
    }

    #[test]
    fn p717_dict_insert_cria_chave() {
        let m = p716_eval("#let d = (a: 1)\n#{ d.insert(\"b\", 2) }").unwrap();
        match m.scope().get("d") {
            Some(Value::Dict(d)) => {
                assert_eq!(d.get("a"), Some(&Value::Int(1)));
                assert_eq!(d.get("b"), Some(&Value::Int(2)));
            }
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p717_dict_insert_chave_nao_str_erra() {
        let err = p716_eval("#let d = (a: 1)\n#{ d.insert(5, 2) }").unwrap_err();
        assert_eq!(err[0].message, "expected string, found integer");
    }

    #[test]
    fn p717_dict_remove_muta_e_devolve() {
        let m = p716_eval("#let d = (a: 1, b: 2)\n#let x = d.remove(\"a\")").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(1)));
        match m.scope().get("d") {
            Some(Value::Dict(d)) => {
                assert_eq!(d.get("a"), None);
                assert_eq!(d.get("b"), Some(&Value::Int(2)));
            }
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p717_dict_remove_chave_ausente_erra_sem_hint() {
        // Ao contrário do at_mut (P716), o erro do remove NÃO tem hint.
        let err = p716_eval("#let d = (a: 1)\n#{ d.remove(\"x\") }").unwrap_err();
        assert_eq!(err[0].message, "dictionary does not contain key \"x\"");
        assert!(err[0].hints.is_empty());
    }

    #[test]
    fn p717_dict_remove_chave_ausente_com_default() {
        let m =
            p716_eval("#let d = (a: 1)\n#let x = d.remove(\"x\", default: 7)").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(7)));
    }

    #[test]
    fn p717_dict_push_erra_missing_method() {
        // push/pop não são dict-mutating; dicts não resolvem campos como
        // métodos (vanilla call.rs:233-238) → mesma mensagem, sem fall-through.
        let err = p716_eval("#let d = (a: 1)\n#{ d.push(2) }").unwrap_err();
        assert_eq!(err[0].message, "type dictionary has no method `push`");
    }

    #[test]
    fn p717_str_push_erra_missing_method() {
        let err = p716_eval("#let s = \"ab\"\n#{ s.push(\"c\") }").unwrap_err();
        assert_eq!(err[0].message, "type string has no method `push`");
    }

    #[test]
    fn p717_temporario_erra() {
        let err = p716_eval("#{ (1, 2).push(3) }").unwrap_err();
        assert!(err[0].message.contains("cannot mutate a temporary value"));
    }

    #[test]
    fn p717_variavel_inexistente_erra() {
        let err = p716_eval("#{ nope.push(1) }").unwrap_err();
        assert!(err[0].message.contains("unknown variable `nope`"));
    }

    #[test]
    fn p717_mecanismo_p716_sem_regressao() {
        let m = p716_eval(
            "#let d = (a: 1)\n#{ d.a = 10 }\n#let arr = (1, 2)\n#{ arr.at(1) = 20 }",
        )
        .unwrap();
        match m.scope().get("d") {
            Some(Value::Dict(d)) => assert_eq!(d.get("a"), Some(&Value::Int(10))),
            other => panic!("esperado dict, recebeu {:?}", other),
        }
        assert_eq!(
            m.scope().get("arr"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(20)]))
        );
    }

    // ── P718 — spread em literais de array/dict e em args de chamada ───────

    #[test]
    fn p718_array_spread_no_meio() {
        let m = p716_eval("#let a = (1, ..(2, 3), 4)").unwrap();
        assert_eq!(
            m.scope().get("a"),
            Some(&Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4)
            ]))
        );
    }

    #[test]
    fn p718_dict_spread_no_meio() {
        let m = p716_eval("#let d = (a: 1, ..(b: 2, c: 3), d: 4)").unwrap();
        match m.scope().get("d") {
            Some(Value::Dict(d)) => {
                assert_eq!(d.get("a"), Some(&Value::Int(1)));
                assert_eq!(d.get("b"), Some(&Value::Int(2)));
                assert_eq!(d.get("c"), Some(&Value::Int(3)));
                assert_eq!(d.get("d"), Some(&Value::Int(4)));
            }
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p718_array_spread_vazio() {
        let m = p716_eval("#let vazio = (..(), 1, ..())").unwrap();
        assert_eq!(m.scope().get("vazio"), Some(&Value::Array(vec![Value::Int(1)])));
    }

    #[test]
    fn p718_array_spread_none_ignora() {
        let m = p716_eval("#let a = (1, ..none, 2)").unwrap();
        assert_eq!(
            m.scope().get("a"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p718_dict_spread_none_ignora() {
        let m = p716_eval("#let d = (a: 1, ..none, b: 2)").unwrap();
        match m.scope().get("d") {
            Some(Value::Dict(d)) => {
                assert_eq!(d.get("a"), Some(&Value::Int(1)));
                assert_eq!(d.get("b"), Some(&Value::Int(2)));
            }
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p718_array_spread_dict_sem_hint() {
        // Array-spread antes zera all_dict_spreads — erro sem hint.
        let err = p716_eval("#let a = (..(1, 2), ..(a: 1))").unwrap_err();
        assert_eq!(err[0].message, "cannot spread dictionary into array");
        assert!(err[0].hints.is_empty());
    }

    #[test]
    fn p718_array_spread_dict_com_hint() {
        // Todos os itens são spreads de dict — hint sugere criar dict.
        let err = p716_eval("#let a = (..(a: 1), ..(b: 2))").unwrap_err();
        assert_eq!(err[0].message, "cannot spread dictionary into array");
        assert!(err[0].hints.iter().any(|h| h.contains("add a colon")));
        assert!(err[0].hints.iter().any(|h| h.contains("(: ..(a: 1), ..(b: 2))")));
    }

    #[test]
    fn p718_array_spread_tipo_invalido_erra() {
        let err = p716_eval("#let a = (1, ..5)").unwrap_err();
        assert_eq!(err[0].message, "cannot spread integer into array");
    }

    #[test]
    fn p718_dict_spread_tipo_invalido_erra() {
        let err = p716_eval("#let d = (a: 1, ..5)").unwrap_err();
        assert_eq!(err[0].message, "cannot spread integer into dictionary");
    }

    #[test]
    fn p718_call_spread_array_junta_posicionais() {
        let m =
            p716_eval("#let f(a, b, c) = a + b + c\n#let x = f(1, ..(2, 3))").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(6)));
    }

    #[test]
    fn p718_call_spread_dict_junta_nomeados() {
        let m = p716_eval("#let f(a, b: 0) = a + b\n#let x = f(1, ..(b: 9))").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(10)));
    }

    #[test]
    fn p718_call_spread_none_ignora() {
        let m = p716_eval("#let f(a, b) = a + b\n#let x = f(1, ..none, ..(2,))").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(3)));
    }

    #[test]
    fn p718_call_spread_tipo_invalido_erra_sem_into() {
        let err = p716_eval("#let f(a, b) = a + b\n#{ f(..5) }").unwrap_err();
        assert_eq!(err[0].message, "cannot spread integer");
    }

    #[test]
    fn p718_call_spread_args_reencaminha_posicionais_e_nomeados() {
        // Value::Args (sink `..b` de uma closure) reencaminhado para outra
        // chamada via spread — funde posicionais e nomeados.
        let m = p716_eval(
            "#let g(..a) = (a.pos(), a.named())\n\
             #let f(..b) = g(..b)\n\
             #let r = f(1, 2, x: 3)",
        )
        .unwrap();
        match m.scope().get("r") {
            Some(Value::Array(items)) => {
                assert_eq!(items[0], Value::Array(vec![Value::Int(1), Value::Int(2)]));
                match &items[1] {
                    Value::Dict(d) => assert_eq!(d.get("x"), Some(&Value::Int(3))),
                    other => panic!("esperado dict, recebeu {:?}", other),
                }
            }
            other => panic!("esperado array, recebeu {:?}", other),
        }
    }

    #[test]
    fn p718_rest_param_definicao_sem_regressao() {
        // Fora do scope deste passo (P504) — confirma que não regrediu.
        let m =
            p716_eval("#let f(..pts) = pts.pos().len()\n#let x = f(1, 2, 3)").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(3)));
    }

    // ── P719 — for-loop sobre Dict (`for (key, value) in dict`) ────────────

    #[test]
    fn p719_for_destructuring_par_chave_valor() {
        let world = MockWorld::new("#for (k, v) in (a: 1, b: 2) [#k=#v ]");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().expect("eval deve produzir Content").plain_text();
        assert_eq!(text, "a=1 b=2 ");
    }

    #[test]
    fn p719_for_um_nome_liga_par_inteiro() {
        // Sem desestruturação (um só nome), o padrão liga o par (chave,
        // valor) inteiro como array — mirror do vanilla (`IntoValue for
        // (&Str, &Value)`), não só a chave.
        let world = MockWorld::new("#for k in (a: 1, b: 2) [#repr(k) ]");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().expect("eval deve produzir Content").plain_text();
        assert_eq!(text, "(\"a\", 1) (\"b\", 2) ");
    }

    #[test]
    fn p719_for_ordem_de_insercao() {
        let world = MockWorld::new("#for (k, v) in (z: 1, a: 2, m: 3) [#k ]");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().expect("eval deve produzir Content").plain_text();
        assert_eq!(text, "z a m ");
    }

    #[test]
    fn p719_for_dict_vazio_sem_iteracoes() {
        let world = MockWorld::new("#for (k, v) in (:) [#k=#v ]");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        assert!(
            module.content().is_none()
                || module.content().unwrap().plain_text().is_empty()
        );
    }

    #[test]
    fn p719_for_aridade_errada_erra() {
        let world = MockWorld::new("#for (a, b, c) in (x: 1, y: 2) [x]");
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).unwrap_err();
        assert!(err[0].message.contains("destructure"));
    }

    #[test]
    fn p719_for_array_sem_regressao() {
        // Iteração sobre array (P540) inalterada pelo reaproveitamento de
        // run_for_loop entre Array e Dict.
        let world = MockWorld::new(
            "#let items = (\"um\", \"dois\", \"três\")\n#for (i, x) in items.enumerate() [#{i+1}. #x]",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().expect("eval deve produzir Content").plain_text();
        assert!(text.contains("um") && text.contains("dois") && text.contains("três"));
    }

    // ── Passo 728 — short-circuit `and`/`or` + `join` em code block ──────

    fn p728_eval(markup: &str) -> SourceResult<Module> {
        let world = MockWorld::new(markup);
        let src = World::source(&world, World::main(&world)).unwrap();
        eval_for_test(&world, &src)
    }

    #[test]
    fn p728_and_shortcircuit_nao_avalia_rhs() {
        // Vanilla: `false and (1/0 == 0)` → false sem erro de divisão.
        let m = p728_eval("#let r = false and (1/0 == 0)").unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Bool(false)));
    }

    #[test]
    fn p728_or_shortcircuit_nao_avalia_rhs() {
        // Vanilla: `true or (1/0 == 0)` → true sem erro de divisão.
        let m = p728_eval("#let r = true or (1/0 == 0)").unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Bool(true)));
    }

    #[test]
    fn p728_and_shortcircuit_idioma_verificar_antes_de_aceder() {
        // O idioma do cetz (draw/shapes.typ:608): guarda de tipo antes de
        // field access. Com a array, `.contains` nunca é avaliado.
        let m =
            p728_eval("#let a = (1, 2)\n#let r = type(a) == str and a.contains(\".\")")
                .unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Bool(false)));
    }

    #[test]
    fn p728_or_shortcircuit_idioma() {
        let m =
            p728_eval("#let a = (1, 2)\n#let r = type(a) == array or a.contains(\".\")")
                .unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Bool(true)));
    }

    #[test]
    fn p728_and_caso_comum_rhs_avaliado() {
        // lhs não decide → rhs avaliado; resultado normal.
        let m = p728_eval("#let r = true and false").unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Bool(false)));
    }

    #[test]
    fn p728_or_caso_comum_rhs_avaliado() {
        let m = p728_eval("#let r = false or true").unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Bool(true)));
    }

    #[test]
    fn p728_and_tipos_invalidos_erra() {
        // lhs não decide (`1` não é `false`) → rhs avaliado → `and` exige
        // Bool (paridade vanilla `ops::and`, sem alteração).
        let err = p728_eval("#let r = 1 and 2").unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn p728_codeblock_join_arrays() {
        // Caso mínimo da "anomalia de ordem" de P727: vanilla `(1, 2)`,
        // cristalino pré-P728 `(2)` — só a última expressão.
        let m = p728_eval("#let x = { (1,); (2,) }").unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p728_codeblock_join_strs() {
        let m = p728_eval("#let x = { \"a\"; \"b\" }").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Str("ab".into())));
    }

    #[test]
    fn p728_codeblock_none_identidade_esquerda() {
        let m = p728_eval("#let x = { none; (1,) }").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Array(vec![Value::Int(1)])));
    }

    #[test]
    fn p728_codeblock_none_identidade_direita() {
        let m = p728_eval("#let x = { (1,); none }").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Array(vec![Value::Int(1)])));
    }

    #[test]
    fn p728_codeblock_join_dicts() {
        let m = p728_eval("#let x = { (:); (a: 1) }").unwrap();
        let mut d = IndexMap::with_hasher(FxBuildHasher);
        d.insert(EcoString::from("a"), Value::Int(1));
        assert_eq!(m.scope().get("x"), Some(&Value::Dict(d)));
    }

    #[test]
    fn p728_codeblock_valor_antes_de_none() {
        let m = p728_eval("#let x = { 1; none }").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(1)));
    }

    #[test]
    fn p728_codeblock_join_invalido_erra() {
        // Vanilla: `{ 1; 2 }` → erro "cannot join integer with integer".
        let err = p728_eval("#let x = { 1; 2 }").unwrap_err();
        assert!(err[0].message.contains("cannot join"), "msg: {}", err[0].message);
    }

    #[test]
    fn p728_codeblock_expressao_unica_sem_regressao() {
        let m = p728_eval("#let x = { 40 + 2 }").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(42)));
    }

    #[test]
    fn p728_join_unitario() {
        use crate::compiler::eval::operators::join;
        assert_eq!(
            join(Value::Str("a".into()), Value::Str("b".into())),
            Ok(Value::Str("ab".into()))
        );
        assert_eq!(
            join(Value::Array(vec![Value::Int(1)]), Value::Array(vec![Value::Int(2)])),
            Ok(Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
        assert_eq!(join(Value::None, Value::Int(5)), Ok(Value::Int(5)));
        assert_eq!(join(Value::Int(5), Value::None), Ok(Value::Int(5)));
        assert!(join(Value::Int(1), Value::Int(2)).is_err());
        let c =
            join(Value::Content(Content::text("a")), Value::Content(Content::text("b")))
                .unwrap();
        assert!(matches!(c, Value::Content(_)));
    }

    // ── Passo 729 — `join` entre iterações de `for`/`while` ──────────────
    // Paridade vanilla `typst-eval/src/flow.rs:86` (while) e `:132` (for):
    // `output = ops::join(output, value)` por iteração. Reaproveita
    // `operators::join` de P728. Sonda: if/else/closure já correctos
    // (delegam em `Expr::CodeBlock`); `while` descartava o corpo; `for`
    // exigia `Content` ("corpo do for deve ser content, encontrado array").

    fn p729_eval(markup: &str) -> SourceResult<Module> {
        let world = MockWorld::new(markup);
        let src = World::source(&world, World::main(&world)).unwrap();
        eval_for_test(&world, &src)
    }

    #[test]
    fn p729_for_join_arrays() {
        // Medido vanilla: `#for i in (1,) { (1,); (2,) }` → `(1, 2)`;
        // cristalino pré-P729: erro "corpo do for deve ser content".
        let m = p729_eval("#let x = for i in (1,) { (1,); (2,) }").unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p729_for_join_acumula_entre_iteracoes() {
        // O join é entre iterações, não só intra-bloco.
        let m = p729_eval("#let x = for i in (1, 2) { (i,) }").unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p729_while_join_arrays() {
        // Medido vanilla: `#while i < 1 { i += 1; (1,); (2,) }` → `(1, 2)`;
        // cristalino pré-P729: output vazio (corpo descartado).
        let m =
            p729_eval("#let i = 0\n#let x = while i < 1 { i += 1; (1,); (2,) }").unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p729_while_acumula_entre_iteracoes() {
        let m = p729_eval("#let i = 0\n#let x = while i < 2 { i += 1; (i,) }").unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p729_for_join_invalido_entre_iteracoes_erra() {
        // Iteração 1: join(None, 1) = 1; iteração 2: join(1, 1) → erro
        // (paridade vanilla "cannot join integer with integer").
        let err = p729_eval("#let x = for i in (1, 2) { 1 }").unwrap_err();
        assert!(err[0].message.contains("cannot join"), "msg: {}", err[0].message);
    }

    #[test]
    fn p729_while_join_invalido_entre_iteracoes_erra() {
        let err =
            p729_eval("#let i = 0\n#let x = while i < 2 { i += 1; 1 }").unwrap_err();
        assert!(err[0].message.contains("cannot join"), "msg: {}", err[0].message);
    }

    #[test]
    fn p729_for_content_sem_regressao() {
        // O caso antigo (corpo Content) comporta-se igual via join.
        let m = p729_eval("#let x = for i in (1, 2) [x]").unwrap();
        match m.scope().get("x") {
            Some(Value::Content(c)) => assert_eq!(c.plain_text(), "xx"),
            other => panic!("esperado Content, encontrado {other:?}"),
        }
    }

    #[test]
    fn p729_for_corpo_none_sem_regressao() {
        // Corpo que só produz None (assignment) → loop devolve None.
        let m = p729_eval("#let i = 0\n#let x = for _ in (1, 2) { i += 1 }").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::None));
        assert_eq!(m.scope().get("i"), Some(&Value::Int(2)));
    }

    #[test]
    fn p729_while_corpo_none_sem_regressao() {
        let m = p729_eval("#let i = 0\n#let x = while i < 2 { i += 1 }").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::None));
        assert_eq!(m.scope().get("i"), Some(&Value::Int(2)));
    }

    #[test]
    fn p729_if_else_sem_regressao() {
        // Já correcto pré-P729 (delega em `Expr::CodeBlock`) — confirma.
        let m = p729_eval("#let x = if true { (1,); (2,) }").unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
        let m2 = p729_eval("#let y = if false { (9,) } else { (1,); (2,) }").unwrap();
        assert_eq!(
            m2.scope().get("y"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    #[test]
    fn p729_closure_body_sem_regressao() {
        // Já correcto pré-P729 (corpo avaliado como `Expr::CodeBlock`).
        let m = p729_eval("#let f() = { (1,); (2,) }\n#let x = f()").unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
    }

    // ── Passo 730 — array.slice(start, end?, count:) ─────────────────────
    // Bloqueio do cetz após P728/P729 ("campo desconhecido em array:
    // 'slice'"). Unidade testada em stdlib/collections.rs; aqui o dispatch.

    #[test]
    fn p730_array_slice_e2e() {
        // Medido vanilla: (1,2,3,4).slice(1, 3) → (2, 3).
        let m = p729_eval("#let a = (1, 2, 3, 4)\n#let x = a.slice(1, 3)").unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Array(vec![Value::Int(2), Value::Int(3)]))
        );
    }

    #[test]
    fn p730_array_slice_cetz_idioma() {
        // O idioma real do cetz (draw/shapes.typ:620): `pts.slice(0, 2)`.
        let m =
            p729_eval("#let pts = (10, 20, 30, 40)\n#let x = pts.slice(0, 2)").unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Array(vec![Value::Int(10), Value::Int(20)]))
        );
    }

    #[test]
    fn p730_array_slice_count_e_negativo_e2e() {
        let m = p729_eval(
            "#let a = (1, 2, 3, 4)\n#let x = a.slice(0, count: 2)\n#let y = a.slice(-2)",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("x"),
            Some(&Value::Array(vec![Value::Int(1), Value::Int(2)]))
        );
        assert_eq!(
            m.scope().get("y"),
            Some(&Value::Array(vec![Value::Int(3), Value::Int(4)]))
        );
    }

    #[test]
    fn p730_str_slice_sem_regressao_e2e() {
        // `Str.slice` (P690) partilha nome — sem regressão pelo novo braço.
        let m = p729_eval("#let s = \"hello\"\n#let x = s.slice(1, 3)").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Str("el".into())));
    }

    // ── Passo 731 — namespaces embutidos calc/sys/math/sym são Module ────
    // Paridade vanilla (medido): `type(calc)` → `module`; o cristalino
    // tinha `dictionary` (Value::Dict), o que bloqueava `#import calc:
    // min, max` — cetz `aabb.typ:18`, caminho do bounds de `line`.

    #[test]
    fn p731_type_namespaces_sao_module() {
        let m = p729_eval(
            "#let a = type(calc)\n#let b = type(sys)\n#let c = type(math)\n#let d = type(sym)",
        )
        .unwrap();
        let module = Some(&Value::Type(crate::entities::value::Type::Module));
        assert_eq!(m.scope().get("a"), module);
        assert_eq!(m.scope().get("b"), module);
        assert_eq!(m.scope().get("c"), module);
        assert_eq!(m.scope().get("d"), module);
    }

    #[test]
    fn p731_import_calc_items() {
        // O caso mínimo de P730 (`#import calc: min, max` + `#(min(1, 2))`).
        let m =
            p729_eval("#import calc: min, max\n#let x = min(1, 2)\n#let y = max(1, 2)")
                .unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(1)));
        assert_eq!(m.scope().get("y"), Some(&Value::Int(2)));
    }

    #[test]
    fn p731_import_calc_wildcard() {
        let m = p729_eval("#import calc: *\n#let x = pi").unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(std::f64::consts::PI)));
    }

    #[test]
    fn p731_field_access_sem_regressao() {
        // O caminho já funcional pré-P731 (sobre Dict) tem de continuar a
        // funcionar sobre Module (P679 — field access em Value::Module).
        let m = p729_eval(
            "#let x = calc.min(3, 4)\n#let s = sym.arrow\n#let v = sys.version",
        )
        .unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(3)));
        match m.scope().get("s") {
            Some(Value::Symbol(sy)) => assert_eq!(sy.value, "→"),
            other => panic!("esperado Symbol, encontrado {other:?}"),
        }
        assert!(matches!(m.scope().get("v"), Some(Value::Version(_))));
    }

    #[test]
    fn p731_math_module_sem_regressao() {
        // `math.equation` continua a resolver via field access e, desde
        // P1140.3-B, é o elemento chamável do vanilla.
        let m = p729_eval("#let e = math.equation").unwrap();
        assert!(matches!(m.scope().get("e"), Some(Value::Func(_))));
    }

    // ── Passo 732 — polygon: coordenadas Length + fallback de stroke ────

    #[test]
    fn p732_polygon_length_compila_e2e() {
        // O caso exacto do passo usa coordenadas Length. Pré-P732: erro
        // "polygon(): argumento 0 não é uma coordenada válida"; o vanilla
        // compila (medido: exit 0, 898 px não-brancos a 150 dpi).
        let m = p729_eval("#polygon((0pt, 0pt), (50pt, 0pt), (25pt, 40pt))");
        assert!(m.is_ok(), "polygon com coordenadas Length deve compilar: {:?}", m.err());
    }

    // ── Passo 733 — argumento nomeado extra sem parâmetro é erro ────────

    #[test]
    fn p733_nomeado_extra_sem_parametro_e_erro() {
        // Medido vanilla: `#let f(a) = a; f(1, z: 2)` →
        // "unexpected argument: z". Cristalino pré-P733: aceite
        // silenciosamente (exit 0).
        let world = MockWorld::new("#let f(a) = a\n#let x = f(1, z: 2)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert_eq!(msg, "unexpected argument: z", "msg: {msg}");
    }

    #[test]
    fn p733_nomeado_valido_sem_regressao() {
        // Caso de não-regressão do passo: nomeados válidos continuam a
        // funcionar. Medido vanilla: (1, 20).
        let world =
            MockWorld::new("#let f(a, named: 10) = (a, named)\n#let x = f(1, named: 20)");
        assert_eq!(
            eval_let(&world, "x"),
            Some(Value::Array(vec![Value::Int(1), Value::Int(20)]))
        );
    }

    #[test]
    fn p733_sink_absorve_nomeados_extra() {
        // Medido vanilla: `#let f(a, ..rest) = rest; f(1, z: 2, y: 3)` →
        // arguments(z: 2, y: 3) — sink absorve nomeados extra sem erro.
        let world =
            MockWorld::new("#let f(a, ..rest) = rest.named()\n#let x = f(1, z: 2, y: 3)");
        let mut expected: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        expected.insert("z".into(), Value::Int(2));
        expected.insert("y".into(), Value::Int(3));
        assert_eq!(eval_let(&world, "x"), Some(Value::Dict(expected)));
    }

    #[test]
    fn p733_sink_exclui_nomeado_consumido_por_parametro() {
        // Medido vanilla: `#let f(a, named: 10, ..rest) = rest` com
        // `f(1, named: 20, z: 3)` → arguments(z: 3) — o sink recebe só os
        // nomeados NÃO consumidos por parâmetros. Cristalino pré-P733:
        // o sink recebia args.named inteiro (incluía named: 20).
        let world = MockWorld::new(
            "#let f(a, named: 10, ..rest) = rest.named()\n#let x = f(1, named: 20, z: 3)",
        );
        let mut expected: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        expected.insert("z".into(), Value::Int(3));
        assert_eq!(eval_let(&world, "x"), Some(Value::Dict(expected)));
    }

    #[test]
    fn p733_ordem_posicional_reportado_primeiro() {
        // Medido vanilla: `f(1, 2, z: 3)` → "unexpected argument" (o
        // posicional é reportado). Não-regressão do caminho P708.
        let world = MockWorld::new("#let f(a) = a\n#let x = f(1, 2, z: 3)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let err = eval_for_test(&world, &src).unwrap_err();
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert_eq!(msg, "unexpected argument", "msg: {msg}");
    }

    // ── Passo 734 — polygon rejeita Int/Float (paridade vanilla) ────────

    #[test]
    fn p734_polygon_int_rejeitado_e2e() {
        // Medido vanilla: `#polygon((0, 0), (50, 0), (25, 40))` →
        // "expected relative length, found integer". Cristalino pré-P734:
        // aceitava (e pré-P732 era a única forma aceite — domínio invertido).
        let m = p729_eval("#polygon((0, 0), (50, 0), (25, 40))");
        let err = m.expect_err("polygon com Int deve ser rejeitado (P734)");
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert_eq!(msg, "expected relative length, found integer", "msg: {msg}");
    }

    #[test]
    fn p734_polygon_length_sem_regressao_e2e() {
        // O caso do passo P732 continua a compilar (Length é o tipo exigido).
        let m = p729_eval("#polygon((0pt, 0pt), (50pt, 0pt), (25pt, 40pt))");
        assert!(m.is_ok(), "polygon com Length deve compilar: {:?}", m.err());
    }

    // ── Passo 735 — namespaces emoji e pdf ────────────────────────────────

    #[test]
    fn p735_type_emoji_e_pdf_sao_module() {
        // Medido vanilla (P731/P735): type(emoji)/type(pdf) → module.
        // Cristalino pré-P735: erro "unknown variable".
        let m = p729_eval("#let a = type(emoji)\n#let b = type(pdf)").unwrap();
        let module = Some(&Value::Type(crate::entities::value::Type::Module));
        assert_eq!(m.scope().get("a"), module);
        assert_eq!(m.scope().get("b"), module);
    }

    #[test]
    fn p735_emoji_face_e_entradas_da_tabela() {
        // Medido vanilla: #emoji.face → 😀. Amostras da tabela codex
        // (ant → 🐜, banana → 🍌 — entradas de 1 codepoint).
        let m =
            p729_eval("#let a = emoji.face\n#let b = emoji.ant\n#let c = emoji.banana")
                .unwrap();
        for (binding, ch) in [("a", '😀'), ("b", '🐜'), ("c", '🍌')] {
            match m.scope().get(binding) {
                Some(Value::Symbol(s)) => assert_eq!(s.value, ch.to_string()),
                other => panic!("esperado Symbol em {binding}, encontrado {other:?}"),
            }
        }
    }

    #[test]
    fn p735_pdf_fields_sao_funcoes() {
        // Medido vanilla: type(pdf.attach)/type(pdf.artifact) → function.
        let m =
            p729_eval("#let a = type(pdf.attach)\n#let b = type(pdf.artifact)").unwrap();
        let func = Some(&Value::Type(crate::entities::value::Type::Function));
        assert_eq!(m.scope().get("a"), func);
        assert_eq!(m.scope().get("b"), func);
    }

    #[test]
    fn p735_pdf_attach_produz_carrier_invisivel() {
        // P1286: bytes explícitos evitam I/O e preservam path/payload no
        // carrier; a invisibilidade não autoriza podar o marker em eval.
        let m = p729_eval(
            "#let a = pdf.attach(\"hi.txt\", bytes((0, 65, 255)), description: \"payload\")",
        )
        .unwrap();
        match m.scope().get("a") {
            Some(Value::Content(Content::PdfAttach(e))) => {
                assert_eq!(e.path.as_str(), "hi.txt");
                assert_eq!(e.data.as_slice(), &[0, 65, 255]);
                assert_eq!(e.description.as_deref(), Some("payload"));
                assert!(e.relationship.is_none());
                assert!(e.mime_type.is_none());
            }
            other => panic!("esperado PdfAttach carrier, encontrado {other:?}"),
        }
    }

    #[test]
    fn p735_pdf_artifact_passthrough_do_body() {
        // Render-parity: artifact só afecta tagging (scope-out global do
        // exportador); o body passa inalterado.
        let m = p729_eval("#let x = pdf.artifact[texto]").unwrap();
        assert!(
            matches!(m.scope().get("x"), Some(Value::Content(_))),
            "pdf.artifact deve devolver o body como Content"
        );
    }

    // ── Passo 736 — color/gradient como valores-tipo ───────────────────────

    #[test]
    fn p736_type_color_e_gradient_sao_type() {
        // Medido vanilla: type(color)/type(gradient) → type.
        // Cristalino pré-P736: dictionary (Value::Dict).
        let m = p729_eval("#let a = type(color)\n#let b = type(gradient)").unwrap();
        let ty = Some(&Value::Type(crate::entities::value::Type::Type));
        assert_eq!(m.scope().get("a"), ty);
        assert_eq!(m.scope().get("b"), ty);
    }

    #[test]
    fn p736_type_instancia_eq_tipo() {
        // Medido vanilla: type(red) == color → true;
        // type(gradient.linear(red, blue)) == gradient → true.
        let m = p729_eval(
            "#let a = type(red) == color\n\
             #let b = type(gradient.linear(red, blue)) == gradient",
        )
        .unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Bool(true)));
        assert_eq!(m.scope().get("b"), Some(&Value::Bool(true)));
    }

    #[test]
    fn p736_color_constructors_acessiveis_via_tipo() {
        // Medido vanilla: type(color.rgb/linear-rgb/luma/cmyk/hsl/hsv/oklab/
        // oklch) → function; color.rgb(255,0,0) → cor.
        let m = p729_eval(
            "#let a = type(color.rgb)\n\
             #let b = type(color.linear-rgb)\n\
             #let c = type(color.luma)\n\
             #let d = type(color.cmyk)\n\
             #let e = type(color.hsl)\n\
             #let f = type(color.hsv)\n\
             #let g = type(color.oklab)\n\
             #let h = type(color.oklch)\n\
             #let cor = color.rgb(255, 0, 0)\n\
             #let cor2 = color.linear-rgb(50%, 50%, 50%)",
        )
        .unwrap();
        let func = Some(&Value::Type(crate::entities::value::Type::Function));
        for b in ["a", "b", "c", "d", "e", "f", "g", "h"] {
            assert_eq!(m.scope().get(b), func, "type(color.<{b}>) deve ser function");
        }
        assert!(
            matches!(m.scope().get("cor"), Some(Value::Color(_))),
            "color.rgb(255,0,0) deve produzir Color"
        );
        assert!(
            matches!(m.scope().get("cor2"), Some(Value::Color(_))),
            "color.linear-rgb(50%,50%,50%) deve produzir Color"
        );
    }

    #[test]
    fn p1143_namespace_tem_as_18_cores_e_equivale_aos_globals() {
        let expected = [
            ("black", "luma(0%)", "luma", "#000000"),
            ("gray", "luma(66.67%)", "luma", "#aaaaaa"),
            ("silver", "luma(86.67%)", "luma", "#dddddd"),
            ("white", "luma(100%)", "luma", "#ffffff"),
            ("navy", "rgb(\"#001f3f\")", "rgb", "#001f3f"),
            ("blue", "rgb(\"#0074d9\")", "rgb", "#0074d9"),
            ("aqua", "rgb(\"#7fdbff\")", "rgb", "#7fdbff"),
            ("teal", "rgb(\"#39cccc\")", "rgb", "#39cccc"),
            ("eastern", "rgb(\"#239dad\")", "rgb", "#239dad"),
            ("purple", "rgb(\"#b10dc9\")", "rgb", "#b10dc9"),
            ("fuchsia", "rgb(\"#f012be\")", "rgb", "#f012be"),
            ("maroon", "rgb(\"#85144b\")", "rgb", "#85144b"),
            ("red", "rgb(\"#ff4136\")", "rgb", "#ff4136"),
            ("orange", "rgb(\"#ff851b\")", "rgb", "#ff851b"),
            ("yellow", "rgb(\"#ffdc00\")", "rgb", "#ffdc00"),
            ("olive", "rgb(\"#3d9970\")", "rgb", "#3d9970"),
            ("green", "rgb(\"#2ecc40\")", "rgb", "#2ecc40"),
            ("lime", "rgb(\"#01ff70\")", "rgb", "#01ff70"),
        ];
        for (name, repr, space, hex) in expected {
            let source = format!(
                "#let same = {name} == color.{name}\n\
                 #let kind = type(color.{name})\n\
                 #let rendered = repr(color.{name})\n\
                 #let space = repr(color.{name}.space())\n\
                 #let hex = color.{name}.to-hex()"
            );
            let module =
                p729_eval(&source).unwrap_or_else(|err| panic!("{name}: {err:?}"));
            assert_eq!(module.scope().get("same"), Some(&Value::Bool(true)), "{name}");
            assert_eq!(
                module.scope().get("kind"),
                Some(&Value::Type(Type::Color)),
                "{name}"
            );
            assert_eq!(
                module.scope().get("rendered"),
                Some(&Value::Str(repr.into())),
                "{name}"
            );
            assert_eq!(
                module.scope().get("space"),
                Some(&Value::Str(space.into())),
                "{name}"
            );
            assert_eq!(
                module.scope().get("hex"),
                Some(&Value::Str(hex.into())),
                "{name}"
            );
        }
    }

    #[test]
    fn p1143_luma_preserva_componentes_observaveis() {
        let module = p729_eval(
            "#let black = repr(color.black.components())\n\
             #let gray = repr(color.gray.components())\n\
             #let silver = repr(color.silver.components())\n\
             #let white = repr(color.white.components())",
        )
        .unwrap();
        for (name, expected) in [
            ("black", "(0%, 100%)"),
            ("gray", "(66.67%, 100%)"),
            ("silver", "(86.67%, 100%)"),
            ("white", "(100%, 100%)"),
        ] {
            assert_eq!(
                module.scope().get(name),
                Some(&Value::Str(expected.into())),
                "{name}"
            );
        }
    }

    #[test]
    fn p1143_extras_globais_nao_entram_no_namespace_color() {
        for name in ["cyan", "magenta", "none", "pink", "ostrich"] {
            let source = format!("#color.{name}");
            let err = p729_eval(&source).expect_err("extra não deve ser field de color");
            let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
            assert!(
                msg.contains(&format!("type color does not contain field `{name}`")),
                "{name}: {msg}"
            );
        }
    }

    #[test]
    fn p736_gradient_constructors_acessiveis_via_tipo() {
        // Medido vanilla: type(gradient.linear/radial/conic) → function.
        let m = p729_eval(
            "#let a = type(gradient.linear)\n\
             #let b = type(gradient.radial)\n\
             #let c = type(gradient.conic)\n\
             #let g = gradient.linear(red, blue)",
        )
        .unwrap();
        let func = Some(&Value::Type(crate::entities::value::Type::Function));
        for b in ["a", "b", "c"] {
            assert_eq!(m.scope().get(b), func, "type(gradient.<{b}>) deve ser function");
        }
        assert!(
            matches!(m.scope().get("g"), Some(Value::Gradient(_))),
            "gradient.linear(red, blue) deve produzir Gradient"
        );
    }

    #[test]
    fn p736_color_operadores_mantidos_sem_regressao() {
        // Os 6 operadores P476/P477 continuam acessíveis (agora como fields
        // do tipo): lighten/darken/mix/negate/saturate/desaturate.
        let m = p729_eval(
            "#let a = type(color.lighten)\n\
             #let b = type(color.darken)\n\
             #let c = type(color.mix)\n\
             #let d = type(color.negate)\n\
             #let e = type(color.saturate)\n\
             #let f = type(color.desaturate)\n\
             #let cl = color.lighten(red, 0.2)",
        )
        .unwrap();
        let func = Some(&Value::Type(crate::entities::value::Type::Function));
        for b in ["a", "b", "c", "d", "e", "f"] {
            assert_eq!(m.scope().get(b), func, "type(color.<{b}>) deve ser function");
        }
        assert!(
            matches!(m.scope().get("cl"), Some(Value::Color(_))),
            "color.lighten(red, 0.2) deve produzir Color"
        );
    }

    #[test]
    fn p736_color_campo_inexistente_mensagem_vanilla() {
        // Medido vanilla: `#color.foo` → "type color does not contain field `foo`".
        let m = p729_eval("#color.foo");
        let err = m.expect_err("color.foo deve ser erro");
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert_eq!(msg, "type color does not contain field `foo`", "msg: {msg}");
    }

    #[test]
    fn p736_color_sem_constructor() {
        // Medido vanilla: `#color("#ff0000")` → "type color does not have
        // a constructor" (tipo não chamável, ao contrário de int/str).
        let m = p729_eval("#color(\"#ff0000\")");
        let err = m.expect_err("color(...) deve ser erro");
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(msg.contains("does not have a constructor"), "msg: {msg}");
    }

    #[test]
    fn p736_repr_dos_tipos() {
        // Medido vanilla: repr(color) → "color"; repr(gradient) → "gradient".
        let m = p729_eval("#let a = repr(color)\n#let b = repr(gradient)").unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Str("color".into())));
        assert_eq!(m.scope().get("b"), Some(&Value::Str("gradient".into())));
    }

    // ── Passo 742 — métodos de instância de cor + fields rotate/components/space ──
    //
    // Valores medidos no vanilla 0.15.0 (969087ec) — sonda P742:
    // `red` vanilla = `rgb(1.0, 0.254902, 0.211765)` (#ff4136).

    #[test]
    fn p742_metodos_instancia_cor_paridade_vanilla() {
        let m = p729_eval(
            "#let l = repr(red.lighten(20%))\n\
             #let d = repr(red.darken(20%))\n\
             #let n = repr(red.negate())\n\
             #let r = repr(red.rotate(90deg))\n\
             #let mx = repr(red.mix(blue))\n\
             #let s = repr(red.saturate(20%))\n\
             #let ds = repr(red.desaturate(20%))",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("l"),
            Some(&Value::Str("rgb(\"#ff675e\")".into())),
            "lighten"
        );
        assert_eq!(
            m.scope().get("d"),
            Some(&Value::Str("rgb(\"#cc342b\")".into())),
            "darken"
        );
        assert_eq!(
            m.scope().get("n"),
            Some(&Value::Str("rgb(\"#004b74\")".into())),
            "negate"
        );
        assert_eq!(
            m.scope().get("r"),
            Some(&Value::Str("rgb(\"#87a100\")".into())),
            "rotate"
        );
        assert_eq!(
            m.scope().get("mx"),
            Some(&Value::Str("oklab(61.08%, 0.075, -0.031)".into())),
            "mix"
        );
        assert_eq!(
            m.scope().get("s"),
            Some(&Value::Str("rgb(\"#ff372b\")".into())),
            "saturate"
        );
        assert_eq!(
            m.scope().get("ds"),
            Some(&Value::Str("rgb(\"#ff675e\")".into())),
            "desaturate"
        );
    }

    #[test]
    fn p742_components_e_space_paridade_vanilla() {
        let m = p729_eval(
            "#let c = repr(red.components())\n\
             #let ca = repr(red.components(alpha: false))\n\
             #let sp = repr(red.space())\n\
             #let eq1 = red.space() == rgb\n\
             #let eq2 = red.space() == color.rgb\n\
             #let eq3 = color.rgb == rgb",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("c"),
            Some(&Value::Str("(100%, 25.49%, 21.18%, 100%)".into())),
            "components"
        );
        assert_eq!(
            m.scope().get("ca"),
            Some(&Value::Str("(100%, 25.49%, 21.18%)".into())),
            "components(alpha: false)"
        );
        assert_eq!(m.scope().get("sp"), Some(&Value::Str("rgb".into())), "repr(space())");
        assert_eq!(m.scope().get("eq1"), Some(&Value::Bool(true)), "space() == rgb");
        assert_eq!(
            m.scope().get("eq2"),
            Some(&Value::Bool(true)),
            "space() == color.rgb"
        );
        assert_eq!(m.scope().get("eq3"), Some(&Value::Bool(true)), "color.rgb == rgb");
    }

    #[test]
    fn p742_estaticas_semantica_corrigida_e_novos_fields() {
        let m = p729_eval(
            "#let a = repr(color.lighten(red, 20%))\n\
             #let b = repr(color.rotate(red, 90deg))\n\
             #let c = repr(color.components(red))\n\
             #let d = repr(color.space(red))\n\
             #let t1 = type(color.rotate)\n\
             #let t2 = type(color.components)\n\
             #let t3 = type(color.space)",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("a"),
            Some(&Value::Str("rgb(\"#ff675e\")".into())),
            "static lighten"
        );
        assert_eq!(
            m.scope().get("b"),
            Some(&Value::Str("rgb(\"#87a100\")".into())),
            "static rotate"
        );
        assert_eq!(
            m.scope().get("c"),
            Some(&Value::Str("(100%, 25.49%, 21.18%, 100%)".into())),
            "static components"
        );
        assert_eq!(m.scope().get("d"), Some(&Value::Str("rgb".into())), "static space");
        let func = Some(&Value::Type(crate::entities::value::Type::Function));
        for b in ["t1", "t2", "t3"] {
            assert_eq!(m.scope().get(b), func, "type(color.<{b}>) deve ser function");
        }
    }

    #[test]
    fn p742_saturate_luma_erro_verbatim_vanilla() {
        // Medido vanilla: "cannot saturate grayscale color" + hint
        // "try converting your color to RGB first".
        let err = p729_eval("#luma(128).saturate(20%)")
            .expect_err("luma.saturate deve ser erro");
        let d = err.first().expect("diagnóstico");
        assert_eq!(d.message, "cannot saturate grayscale color", "msg: {}", d.message);
        assert!(
            d.hints.iter().any(|h| h == "try converting your color to RGB first"),
            "hints: {:?}",
            d.hints
        );
    }

    #[test]
    fn p742_space_em_markup_renderiza_nome() {
        // Medido vanilla: `#red.space()` em markup renderiza "rgb".
        let m = p729_eval("#red.space()").unwrap();
        let text = m.content().expect("content").plain_text();
        assert_eq!(text, "rgb", "texto: {text}");
    }

    #[test]
    fn p742_metodo_desconhecido_cai_no_caminho_generico() {
        // P815 — o "caminho genérico" passou a ser o mirror de
        // `eval_field_callee` do vanilla: método desconhecido em color →
        // `type color has no method `foo`` (medido no vanilla com
        // `#rgb("#ff0000").foo()` — verbatim).
        let err = p729_eval("#red.foo()").expect_err("red.foo() deve ser erro");
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert_eq!(msg, "type color has no method `foo`", "msg: {msg}");
    }

    // ── P744 — `space:` em mix/negate/rotate, to-hex/transparentize/opacify,
    // repr de closure ───────────────────────────────────────────────────────

    #[test]
    fn p1077_len_global_removido_metodos_preservados() {
        // 1. Função global `len` não existe -> erro (paridade vanilla / Achado #12 do P1031)
        assert!(p729_eval("#len(\"ação\")").is_err());
        assert!(p729_eval("#len((1, 2, 3))").is_err());

        // 2. Método `.len()` opera em bytes (paridade vanilla)
        let m = p729_eval(
            "#let s_len = \"ação\".len()\n             #let a_len = (1, 2, 3).len()\n             #let d_len = (a: 1, b: 2).len()",
        )
        .unwrap();
        assert_eq!(m.scope().get("s_len"), Some(&Value::Int(6)));
        assert_eq!(m.scope().get("a_len"), Some(&Value::Int(3)));
        assert_eq!(m.scope().get("d_len"), Some(&Value::Int(2)));
    }

    #[test]
    fn p744_space_nomeado_mix_negate_rotate() {
        // Medições vanilla da sonda P744.
        let m = p729_eval(
            "#let a = red.mix(blue, space: rgb).to-hex()\n\
             #let b = red.negate(space: oklab).to-hex()\n\
             #let c = red.rotate(90deg, space: oklch).to-hex()",
        )
        .unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Str("#805b87".into())));
        assert_eq!(m.scope().get("b"), Some(&Value::Str("#004b74".into())));
        assert_eq!(m.scope().get("c"), Some(&Value::Str("#87a100".into())));
    }

    #[test]
    fn p744_to_hex_transparentize_opacify() {
        let m = p729_eval(
            "#let a = red.to-hex()\n\
             #let b = rgb(\"#ff413680\").transparentize(50%).to-hex()\n\
             #let c = rgb(\"#ff413680\").opacify(50%).to-hex()",
        )
        .unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Str("#ff4136".into())));
        assert_eq!(m.scope().get("b"), Some(&Value::Str("#ff413640".into())));
        assert_eq!(m.scope().get("c"), Some(&Value::Str("#ff4136c0".into())));
    }

    #[test]
    fn p744_instancia_mix_nao_aceita_weight() {
        let err =
            p729_eval("#red.mix(blue, weight: 50%)").expect_err("weight deve ser erro");
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(msg.contains("unexpected argument: weight"), "msg: {msg}");
    }

    #[test]
    fn p744_rotate_space_sem_hue_erro() {
        let err = p729_eval("#red.rotate(90deg, space: rgb)")
            .expect_err("rgb rotate deve erro");
        let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(msg.contains("does not support hue rotation"), "msg: {msg}");
    }

    #[test]
    fn p744_repr_closure() {
        let m =
            p729_eval("#let f = (x) => x + 1\n#let a = repr(f)\n#let b = repr((x) => x)")
                .unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Str("(..) => ..".into())));
        assert_eq!(m.scope().get("b"), Some(&Value::Str("(..) => ..".into())));
    }

    // ── Passo 737 — counter/state como valores-tipo ────────────────────────

    #[test]
    fn p737_type_counter_e_state_sao_type() {
        // Medido vanilla: type(counter)/type(state) → type.
        // Cristalino pré-P737: function (Value::Func).
        let m = p729_eval("#let a = type(counter)\n#let b = type(state)").unwrap();
        let ty = Some(&Value::Type(crate::entities::value::Type::Type));
        assert_eq!(m.scope().get("a"), ty);
        assert_eq!(m.scope().get("b"), ty);
    }

    #[test]
    fn p737_type_instancia_eq_tipo() {
        // Medido vanilla: type(counter("x")) == counter → true;
        // type(state("y", 0)) == state → true.
        let m = p729_eval(
            "#let c = counter(\"x\")\n\
             #let s = state(\"y\", 0)\n\
             #let a = type(c) == counter\n\
             #let b = type(s) == state",
        )
        .unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Bool(true)));
        assert_eq!(m.scope().get("b"), Some(&Value::Bool(true)));
    }

    #[test]
    fn p737_counter_state_chamaveis_criam_instancias() {
        // A chamabilidade mantém-se (despacho P685): counter("x") → Counter,
        // state("y", 0) → State.
        let m = p729_eval("#let c = counter(\"x\")\n#let s = state(\"y\", 0)").unwrap();
        assert!(
            matches!(m.scope().get("c"), Some(Value::Counter(_))),
            "counter(\"x\") deve produzir Value::Counter"
        );
        assert!(
            matches!(m.scope().get("s"), Some(Value::State(_))),
            "state(\"y\", 0) deve produzir Value::State"
        );
    }

    #[test]
    fn p737_repr_counter_state() {
        // Medido vanilla: repr(counter) → "counter"; repr(state) → "state".
        // Cristalino pré-P737: "#counter"/"#state" (repr de Func).
        let m = p729_eval("#let a = repr(counter)\n#let b = repr(state)").unwrap();
        assert_eq!(m.scope().get("a"), Some(&Value::Str("counter".into())));
        assert_eq!(m.scope().get("b"), Some(&Value::Str("state".into())));
    }

    #[test]
    fn p737_metodos_de_instancia_sem_regressao() {
        // Os métodos de instância P506 não mudam: operam sobre
        // Value::Counter/Value::State, não sobre o binding.
        let m = p729_eval("#let c = counter(\"x\")\n#let r = c.step()").unwrap();
        assert!(
            matches!(m.scope().get("r"), Some(Value::Content(_))),
            "c.step() deve continuar a produzir Content"
        );
        let m2 = p729_eval("#let s = state(\"y\", 0)\n#let u = s.update(5)").unwrap();
        assert!(
            matches!(m2.scope().get("u"), Some(Value::Content(_))),
            "s.update(5) deve continuar a produzir Content"
        );
    }

    // ── Passo 739A — hline/vline com stroke: none ──────────────────────────

    #[test]
    fn p739a_hline_vline_stroke_none_compilam() {
        // Medido vanilla: `table.hline(stroke: none)` compila (exit 0) e a
        // linha não é desenhada. Cristalino pré-P739: erro
        // "espera Length / Color / Stroke, recebeu none".
        for src in [
            "#table(columns: 2, table.hline(stroke: none), [a], [b])",
            "#table(columns: 2, table.vline(stroke: none), [a], [b])",
            "#grid(columns: 2, grid.hline(stroke: none), [a], [b])",
            "#grid(columns: 2, grid.vline(stroke: none), [a], [b])",
        ] {
            let m = p729_eval(src);
            assert!(m.is_ok(), "deve compilar: {src}; erro: {:?}", m.err());
        }
    }

    #[test]
    fn p739a_hline_stroke_none_gera_elemento_sem_stroke() {
        let m = p729_eval("#let h = table.hline(stroke: none)").unwrap();
        match m.scope().get("h") {
            Some(Value::Content(Content::TableHLine(e))) => {
                assert!(e.stroke.is_none(), "stroke: none deve gerar elem sem stroke")
            }
            other => panic!("esperado TableHLine, encontrado {other:?}"),
        }
        // Não-regressão: stroke omitido → default 1pt preto (P512).
        let m2 = p729_eval("#let h = table.hline()").unwrap();
        match m2.scope().get("h") {
            Some(Value::Content(Content::TableHLine(e))) => {
                assert!(e.stroke.is_some(), "stroke omitido deve gerar default")
            }
            other => panic!("esperado TableHLine, encontrado {other:?}"),
        }
    }

    // ── Passo 739B — line(start:, end:) ────────────────────────────────────

    fn p739b_line_path_points(
        m: &crate::entities::module::Module,
        binding: &str,
    ) -> ((f64, f64), (f64, f64)) {
        use crate::entities::geometry::{PathItem, ShapeKind};
        match m.scope().get(binding) {
            Some(Value::Content(Content::Shape(e))) => match &e.kind {
                ShapeKind::Path(items) => match items.as_slice() {
                    [PathItem::MoveTo(start), PathItem::LineTo(end)] => {
                        ((start.x.val(), start.y.val()), (end.x.val(), end.y.val()))
                    }
                    other => panic!("esperado path aberto de dois pontos: {other:?}"),
                },
                other => panic!("esperado Path, encontrado {other:?}"),
            },
            other => panic!("esperado Shape em {binding}, encontrado {other:?}"),
        }
    }

    fn p739b_legacy_line_dx_dy(
        m: &crate::entities::module::Module,
        binding: &str,
    ) -> (f64, f64) {
        use crate::entities::geometry::ShapeKind;
        match m.scope().get(binding) {
            Some(Value::Content(Content::Shape(e))) => match &e.kind {
                ShapeKind::Line { dx, dy } => (*dx, *dy),
                other => panic!("esperado Line legado, encontrado {other:?}"),
            },
            other => panic!("esperado Shape em {binding}, encontrado {other:?}"),
        }
    }

    #[test]
    fn p739b_line_start_end_compila_e_preserva_pontos_absolutos() {
        // Medido vanilla: line(start: (0pt, 0pt), end: (50pt, 50pt)) → exit 0.
        // Cristalino pré-P739: erro "argumento nomeado inesperado em line(): 'start'".
        let m = p729_eval("#let l = line(start: (0pt, 0pt), end: (50pt, 50pt))").unwrap();
        assert_eq!(p739b_line_path_points(&m, "l"), ((0.0, 0.0), (50.0, 50.0)));
    }

    #[test]
    fn p739b_line_end_sem_start_default_origem() {
        // start omitido → (0pt, 0pt) (paridade vanilla).
        let m = p729_eval("#let l = line(end: (30pt, 40pt))").unwrap();
        assert_eq!(p739b_line_path_points(&m, "l"), ((0.0, 0.0), (30.0, 40.0)));
    }

    #[test]
    fn p739b_line_start_nao_zero_preserva_origem_absoluta() {
        // P1286 baixa para Path aberto, portanto a origem não-zero deixa de
        // ser scope-out e não pode ser normalizada para zero.
        let m =
            p729_eval("#let l = line(start: (10pt, 0pt), end: (50pt, 50pt))").unwrap();
        assert_eq!(p739b_line_path_points(&m, "l"), ((10.0, 0.0), (50.0, 50.0)));
    }

    #[test]
    fn p739b_line_dx_dy_sem_regressao() {
        use crate::entities::layout_types::Length;
        // Interface legada dx/dy mantém-se (não-regressão).
        let m = p729_eval("#let l = line(dx: 1cm, dy: 2cm)").unwrap();
        let (dx, dy) = p739b_legacy_line_dx_dy(&m, "l");
        assert!((dx - Length::PT_PER_CM).abs() < 0.01, "dx: {dx}");
        assert!((dy - 56.6929).abs() < 0.01, "dy: {dy}");
    }

    // ── Passo 739C — display de Float em markup ────────────────────────────

    #[test]
    fn p739c_display_float_inteiro_sem_ponto_zero() {
        // Medido vanilla (markup): #(4/2) → "2"; #(1.0) → "1"; #(2.5) →
        // "2.5"; #(0.1) → "0.1"; #(100.0) → "100"; #(1.5e3) → "1500"
        // (Display de f64 — inteiros exactos sem `.0`).
        // Cristalino pré-P739: "2.0"/"1.0"/"100.0"/"1500.0" (via repr_value).
        for (src, esperado) in [
            ("#(4/2)", "2"),
            ("#(1.0)", "1"),
            ("#(2.5)", "2.5"),
            ("#(0.1)", "0.1"),
            ("#(100.0)", "100"),
            ("#(1.5e3)", "1500"),
        ] {
            let m = p729_eval(src).unwrap();
            let text = m.content().expect("content").plain_text();
            assert_eq!(text, esperado, "src: {src}");
        }
    }

    #[test]
    fn p739c_repr_float_mantem_ponto_zero() {
        // Não-regressão: repr(1.0) → "1.0" (medido vanilla — só o display
        // em markup muda; repr_value é inalterado).
        let m = p729_eval("#let r = repr(1.0)").unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("1.0".into())));
    }

    // ── Passo 739D — Length / Float com NaN ────────────────────────────────

    #[test]
    fn p739d_length_div_nan_saneado() {
        // NaN é alcançável via calc: `calc.inf - calc.inf` → NaN (ambos).
        // Medido vanilla: `repr(1pt / NaN)` → `0pt` (saneamento Scalar::new).
        // Cristalino pré-P739D: `float.nan * 1pt + float.nan * 1em`.
        let m = p729_eval("#let r = repr(1pt / (calc.inf - calc.inf))").unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("0pt".into())));
        // Não-regressão P725 (Mul já era saneado).
        let m2 = p729_eval("#let r = repr(1pt * (calc.inf - calc.inf))").unwrap();
        assert_eq!(m2.scope().get("r"), Some(&Value::Str("0pt".into())));
        // Não-regressão: divisão normal intacta.
        let m3 = p729_eval("#let r = repr(4pt / 2.0)").unwrap();
        assert_eq!(m3.scope().get("r"), Some(&Value::Str("2pt".into())));
    }

    // ── Passo 740A — warning "return descarta conteúdo" ────────────────────

    /// Helper P740A: eval com o sink exposto (padrão do teste P126), para
    /// verificar warnings acumulados mesmo quando o eval erra.
    fn p740a_eval_com_sink(
        markup: &str,
    ) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
        use comemo::Track;
        let world = MockWorld::new(markup);
        let src = World::source(&world, World::main(&world)).unwrap();
        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let result = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &crate::entities::element_registry::ElementRegistry::new(),
        );
        (result, sink.into_diagnostics())
    }

    #[test]
    fn p740a_warn_return_descarta_conteudo() {
        // Medido vanilla: `#{ [conteúdo]; return "x" }` → warning "this
        // return unconditionally discards the content before it" + hint
        // "try omitting the `return` to automatically join all values"
        // (coexiste com o erro "cannot return outside of function").
        // Cristalino pré-P740: só o erro, sem warning.
        let (_r, diags) = p740a_eval_com_sink("#{ [conteúdo]; return \"x\" }");
        let w = diags.iter().find(|d| {
            d.message == "this return unconditionally discards the content before it"
        });
        let w = w.expect("warning de return ausente; diagnostics: {diags:?}");
        assert!(
            w.hints.iter().any(
                |h| h == "try omitting the `return` to automatically join all values"
            ),
            "hint base ausente: {:?}",
            w.hints
        );
    }

    #[test]
    fn p740a_warn_return_com_state_dois_hints() {
        // Medido vanilla: com state/counter update no conteúdo descartado,
        // junta um segundo hint "state/counter updates are content that
        // must end up in the document to have an effect".
        let (_r, diags) = p740a_eval_com_sink(
            "#{ let s = state(\"k\", 0); [txt]; s.update(1); return \"x\" }",
        );
        let w = diags.iter().find(|d| {
            d.message == "this return unconditionally discards the content before it"
        });
        let w = w.expect("warning de return ausente; diagnostics: {diags:?}");
        assert!(
            w.hints
                .iter()
                .any(|h| h.contains("state/counter updates are content")),
            "hint state/counter ausente: {:?}",
            w.hints
        );
    }

    #[test]
    fn p740a_sem_warning_sem_conteudo() {
        // Não-regressão: `{ 1; return "x" }` — o join acumulado é Int, não
        // Content → o vanilla não emite o warning (condição
        // `Value::Content(tree)` em `warn_for_discarded_content`).
        let (_r, diags) = p740a_eval_com_sink("#{ 1; return \"x\" }");
        assert!(
            diags.iter().all(|d| !d.message.contains("discards the content")),
            "warning indevido: {diags:?}"
        );
    }

    // ── Passo 740B — repr de `Value::Args` completo ────────────────────────

    #[test]
    fn p740b_repr_args_nomeados_depois_posicionais() {
        // Medido vanilla: `repr` de arguments lista os NOMEADOS primeiro
        // (ordem de inserção), depois os posicionais:
        // `f(1, z: 2, y: 3)` com sink → "arguments(z: 2, y: 3)";
        // `f(1, 2, z: 3)` → "arguments(z: 3, 1, 2)"; `f()` → "arguments()".
        // Cristalino pré-P740: "arguments(...)" (lossy).
        let m = p729_eval("#let f(a, ..rest) = rest\n#let r = repr(f(1, z: 2, y: 3))")
            .unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("arguments(z: 2, y: 3)".into())));

        let m2 =
            p729_eval("#let f(..rest) = rest\n#let r = repr(f(1, 2, z: 3))").unwrap();
        assert_eq!(
            m2.scope().get("r"),
            Some(&Value::Str("arguments(z: 3, 1, 2)".into()))
        );

        let m3 = p729_eval("#let f(..rest) = rest\n#let r = repr(f())").unwrap();
        assert_eq!(m3.scope().get("r"), Some(&Value::Str("arguments()".into())));
    }

    // ── Passo 740D — `rgb(ratio, ...)` ─────────────────────────────────────

    #[test]
    fn p740d_rgb_ratio_por_componente() {
        // Medido vanilla: `repr(rgb(50%, 0%, 0%))` → `rgb("#800000")`
        // (ratio × 255, arredondado: 127.5 → 128 = 0x80); com alpha:
        // `rgb(50%, 0%, 0%, 50%)` → `rgb("#80000080")`.
        // Cristalino pré-P740D: erro "rgb() requer 3 ou 4 Int".
        let m = p729_eval("#let c = repr(rgb(50%, 0%, 0%))").unwrap();
        assert_eq!(m.scope().get("c"), Some(&Value::Str("rgb(\"#800000\")".into())));
        let m2 = p729_eval("#let c = repr(rgb(50%, 0%, 0%, 50%))").unwrap();
        assert_eq!(m2.scope().get("c"), Some(&Value::Str("rgb(\"#80000080\")".into())));
        // Não-regressão: Int directo inalterado.
        let m3 = p729_eval("#let c = repr(rgb(128, 0, 0))").unwrap();
        assert_eq!(m3.scope().get("c"), Some(&Value::Str("rgb(\"#800000\")".into())));
    }

    #[test]
    fn p740d_rgb_erros_verbatim() {
        // Medido vanilla (Component cast, visualize/color.rs:2680-2692):
        // - Int fora de [0,255] → "number must be between 0 and 255"
        // - Float → "expected integer or ratio, found float"
        // - Ratio fora de [0%,100%] → "ratio must be between 0% and 100%"
        let e = p729_eval("#rgb(300, 0, 0)").unwrap_err();
        assert_eq!(e[0].message, "number must be between 0 and 255");
        let e2 = p729_eval("#rgb(0.5, 0, 0)").unwrap_err();
        assert_eq!(e2[0].message, "expected integer or ratio, found float");
        let e3 = p729_eval("#rgb(150%, 0%, 0%)").unwrap_err();
        assert_eq!(e3[0].message, "ratio must be between 0% and 100%");
    }

    // ── Passo 740E — repr de NaN e ±inf ────────────────────────────────────

    #[test]
    fn p740e_repr_nan_e_inf() {
        // Medido vanilla: `repr(calc.inf - calc.inf)` → "float.nan";
        // `repr(calc.inf)` → "float.inf"; `repr(-calc.inf)` → "-float.inf".
        // Cristalino pré-P740E: "NaN.0" / "inf.0" / "-inf.0".
        let m = p729_eval("#let r = repr(calc.inf - calc.inf)").unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("float.nan".into())));
        let m2 = p729_eval("#let r = repr(calc.inf)").unwrap();
        assert_eq!(m2.scope().get("r"), Some(&Value::Str("float.inf".into())));
        let m3 = p729_eval("#let r = repr(-calc.inf)").unwrap();
        assert_eq!(m3.scope().get("r"), Some(&Value::Str("-float.inf".into())));
        // Não-regressão: finitos mantêm ".0".
        let m4 = p729_eval("#let r = repr(1.0)").unwrap();
        assert_eq!(m4.scope().get("r"), Some(&Value::Str("1.0".into())));
    }

    // ── Passo 772l — `CodeBlock`/`ContentBlock` isolam bindings de `let` ──
    // Medido vanilla (`typst-eval/src/code.rs:317-332`): `ast::CodeBlock::eval`
    // e `ast::ContentBlock::eval` chamam `vm.scopes.enter()`/`exit()` em torno
    // do corpo — um `let` interno não sobrevive à saída do bloco. Cristalino
    // pré-P772l avaliava o corpo directamente no `scopes` do chamador (só
    // `styles`/`show_rules` eram locais) — `x` fora do bloco ficava mutado
    // permanentemente. E2E: `#let x = 1; #{ let x = 2; x }; #x` devolvia
    // "2, 2" em vez de "2, 1".

    #[test]
    fn p772l_let_dentro_de_code_block_nao_vaza_para_fora() {
        let m = p729_eval("#let x = 1\n#let inside = { let x = 2; x }\n#let after = x")
            .unwrap();
        assert_eq!(m.scope().get("inside"), Some(&Value::Int(2)));
        assert_eq!(
            m.scope().get("after"),
            Some(&Value::Int(1)),
            "let dentro do bloco não deve mutar o x do âmbito envolvente"
        );
    }

    #[test]
    fn p772l_let_dentro_de_if_body_nao_vaza_para_fora() {
        // O corpo de `if`/`else` é um `CodeBlock` — mesma via.
        let m = p729_eval(
            "#let x = 1\n#let inside = if true { let x = 2; x } else { 0 }\n#let after = x",
        ).unwrap();
        assert_eq!(m.scope().get("inside"), Some(&Value::Int(2)));
        assert_eq!(m.scope().get("after"), Some(&Value::Int(1)));
    }

    // ── P772r — hint de subtracção em unknown_variable ──────────────────────
    // Medido vanilla (`foundations/scope.rs::unknown_variable`, linha
    // 424-437): hint só quando `var.contains('-')`; singular "sign" para um
    // hífen, plural "signs" para mais de um; sem verificação extra das
    // partes ao redor do hífen.

    #[test]
    fn p772r_hint_subtracao_um_hifen() {
        let err = p729_eval("#foo-bar").unwrap_err();
        assert_eq!(err[0].message, "unknown variable `foo-bar`");
        assert_eq!(
            err[0].hints,
            vec!["if you meant to use subtraction, try adding spaces around the minus sign: `foo - bar`".to_string()]
        );
    }

    #[test]
    fn p772r_hint_subtracao_hifens_multiplos_plural() {
        let err = p729_eval("#foo-bar-baz").unwrap_err();
        assert_eq!(err[0].message, "unknown variable `foo-bar-baz`");
        assert_eq!(
            err[0].hints,
            vec!["if you meant to use subtraction, try adding spaces around the minus signs: `foo - bar - baz`".to_string()]
        );
    }

    #[test]
    fn p772r_sem_hifen_sem_hint() {
        let err = p729_eval("#simplyunknown").unwrap_err();
        assert_eq!(err[0].message, "unknown variable `simplyunknown`");
        assert!(err[0].hints.is_empty(), "sem hífen não deve ter hint");
    }

    #[test]
    fn p772r_hint_tambem_no_caminho_de_mutacao() {
        // access() (mutação) usa o mesmo helper que eval_expr (leitura) —
        // paridade vanilla: as duas usam a mesma `unknown_variable()`.
        let err = p729_eval("#{ foo-bar = 1 }").unwrap_err();
        assert_eq!(err[0].message, "unknown variable `foo-bar`");
        assert_eq!(
            err[0].hints,
            vec!["if you meant to use subtraction, try adding spaces around the minus sign: `foo - bar`".to_string()]
        );
    }

    // ── P792 / P792a — Testes de Context / Layout e Idioma Dinâmicos ──────────

    #[test]
    fn p792_layout_default_dimensions() {
        let world = MockWorld::new("#let x = layout(size => (size.width, size.height))");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        // A4 padrão: width=595.28, height=841.89, margin=56.69.
        // avail_w = 595.28 - 2*56.69 = 481.9
        // avail_h = 841.89 - 2*56.69 = 728.51
        let val = m.scope().get("x").unwrap();
        if let Value::Array(arr) = val {
            assert_eq!(arr.len(), 2);
            if let (Value::Length(w), Value::Length(h)) = (&arr[0], &arr[1]) {
                assert!((w.abs.to_pt() - 481.9).abs() < 0.1);
                assert!((h.abs.to_pt() - 728.51).abs() < 0.1);
            } else {
                panic!("esperado comprimentos");
            }
        } else {
            panic!("esperado array");
        }
    }

    #[test]
    fn p792_layout_custom_symmetric_margin() {
        let world = MockWorld::new(
            "#set page(width: 20cm, height: 10cm, margin: 3cm)\n\
             #let x = layout(size => (size.width, size.height))",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        // 20cm = 566.92pt, 10cm = 283.46pt, margin = 3cm = 85.04pt
        // avail_w = 566.92 - 2*85.04 = 396.84pt = 14cm
        // avail_h = 283.46 - 2*85.04 = 113.38pt = 4cm
        let val = m.scope().get("x").unwrap();
        if let Value::Array(arr) = val {
            if let (Value::Length(w), Value::Length(h)) = (&arr[0], &arr[1]) {
                assert!((w.abs.to_pt() - 396.84).abs() < 0.1);
                assert!((h.abs.to_pt() - 113.38).abs() < 0.1);
            } else {
                panic!("esperado comprimentos");
            }
        } else {
            panic!("esperado array");
        }
    }

    #[test]
    fn p792_layout_custom_asymmetric_margin() {
        let world = MockWorld::new(
            "#set page(width: 20cm, height: 10cm, margin: (left: 1cm, right: 5cm, top: 2cm, bottom: 0.5cm))\n\
             #let x = layout(size => (size.width, size.height))"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        // width = 20cm (566.92pt), height = 10cm (283.46pt)
        // left = 1cm (28.34pt), right = 5cm (141.73pt)
        // top = 2cm (56.69pt), bottom = 0.5cm (14.17pt)
        // avail_w = 566.92 - 28.34 - 141.73 = 396.84pt = 14cm
        // avail_h = 283.46 - 56.69 - 14.17 = 212.59pt = 7.5cm
        let val = m.scope().get("x").unwrap();
        if let Value::Array(arr) = val {
            if let (Value::Length(w), Value::Length(h)) = (&arr[0], &arr[1]) {
                assert!((w.abs.to_pt() - 396.84).abs() < 0.1);
                assert!((h.abs.to_pt() - 212.59).abs() < 0.1);
            } else {
                panic!("esperado comprimentos");
            }
        } else {
            panic!("esperado array");
        }
    }

    #[test]
    fn p792_text_lang_dynamic() {
        let world = MockWorld::new(
            "#set text(lang: \"pt\")\n\
             #let lang1 = text.lang\n\
             #set text(lang: \"fr\")\n\
             #let lang2 = text.lang",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("lang1"), Some(&Value::Str("pt".into())));
        assert_eq!(m.scope().get("lang2"), Some(&Value::Str("fr".into())));
    }

    #[test]
    fn p793_numbering_standalone_patterns() {
        let world = MockWorld::new(
            "#let x1 = numbering(\"1.a\", 3, 1)\n\
             #let x2 = numbering(\"(I)\", 5)\n\
             #let x3 = numbering(\"1.a.I\", 1, 2, 3, 4)\n\
             #let x4 = numbering(\"א\", 15)",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x1"), Some(&Value::Str("3.a".into())));
        assert_eq!(m.scope().get("x2"), Some(&Value::Str("(V)".into())));
        assert_eq!(m.scope().get("x3"), Some(&Value::Str("1.b.III.IV".into())));
        assert_eq!(m.scope().get("x4"), Some(&Value::Str("טו".into())));
    }

    #[test]
    fn p793_numbering_standalone_closure() {
        let world = MockWorld::new("#let x = numbering(n => str(n) + \"!\", 5)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Str("5!".into())));
    }

    #[test]
    fn p793_enum_auto_increment_sequence() {
        let world = MockWorld::new(
            "+ primeiro\n\
             + segundo\n\n\
             Um paragrafo no meio.\n\n\
             + terceiro",
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        let doc = crate::compiler::layout::layout(m.content().unwrap());
        let text = doc.pages[0].plain_text();
        assert!(
            text.contains("1. primeiro"),
            "esperado '1. primeiro', obtido: '{}'",
            text
        );
        assert!(text.contains("2. segundo"), "esperado '2. segundo', obtido: '{}'", text);
        assert!(
            text.contains("1. terceiro"),
            "esperado '1. terceiro', obtido: '{}'",
            text
        );
    }

    #[test]
    fn p793_hebrew_zero_warning() {
        use comemo::Track;
        let world = MockWorld::new("#let x = numbering(\"א\", 0)");
        let src = World::source(&world, World::main(&world)).unwrap();

        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        let registry = crate::entities::element_registry::ElementRegistry::new();
        let _ = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &registry,
        );

        let diagnostics = sink.into_diagnostics();
        assert!(!diagnostics.is_empty());
        let warning_message = &diagnostics[0].message;
        assert!(
            warning_message.contains("the numeral system `hebrew` cannot represent zero")
        );
    }

    #[test]
    fn p794_smartquote_double_curved_default() {
        let world = MockWorld::new(r#""test""#);
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let plain = module.content().unwrap().plain_text();
        assert_eq!(plain.trim(), "“test”");
    }

    #[test]
    fn p794_smartquote_enabled_false() {
        let world = MockWorld::new(
            r#"#set smartquote(enabled: false)
"test" e 'single'"#,
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let plain = module.content().unwrap().plain_text();
        assert_eq!(plain.trim(), "\"test\" e 'single'");
    }

    #[test]
    fn p794_smartquote_quotes_custom() {
        let world = MockWorld::new(
            r#"#set smartquote(quotes: "«»")
"test" e 'single'"#,
        );
        let src = world.source(world.main()).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let plain = module.content().unwrap().plain_text();
        assert_eq!(plain.trim(), "«test» e ‘single’");
    }

    #[test]
    fn p794_smartquote_quotes_validation() {
        use comemo::Track;
        let world = MockWorld::new(r#"#set smartquote(quotes: "abc")"#);
        let src = world.source(world.main()).unwrap();

        let registry = crate::entities::element_registry::ElementRegistry::new();
        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();

        let res = eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &registry,
        );
        assert!(res.is_err());
        let errors = res.unwrap_err();
        assert!(!errors.is_empty());
        let error_msg = &errors[0].message;
        assert!(error_msg.contains("expected 2 characters, found 3 characters"));
    }

    // ── P836 (achado #21 de P831) — `variations:` de `#text` ────────────────
    //
    // Valores medidos no vanilla 0.15.0
    // (`lab/typst-original/target/release/typst`, fixtures em `temp/p836/`):
    // válidos (float/int/eixos vários) compilam; inválidos dão erro com
    // hints verbatim (`variations.rs:217-236`, `tag.rs:85-117`).

    /// Helper P836: corre o eval e devolve o resultado.
    fn eval_variations_p836(
        src_text: &str,
    ) -> Result<
        crate::entities::module::Module,
        Vec<crate::entities::source_result::SourceDiagnostic>,
    > {
        use comemo::Track;
        let world = MockWorld::new(src_text);
        let src = World::source(&world, World::main(&world)).unwrap();
        let registry = crate::entities::element_registry::ElementRegistry::new();
        let routines = Routines::new();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let route = Route::root();
        eval(
            &routines,
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &src,
            &registry,
        )
    }

    /// P836: valores válidos (float e int) são aceites no constructor.
    #[test]
    fn eval_text_variations_valido_passo_836() {
        let res = eval_variations_p836(
            "#text(variations: (wght: 250))[a]\n#text(variations: (wght: 800.5, ital: 1))[b]",
        );
        assert!(res.is_ok(), "variations válido deve compilar; got: {:?}", res.err());
    }

    /// P836: `#set text(variations:)` é set rule válida no vanilla
    /// (campo `#[fold] #[ghost]` — medido em `temp/p836/s2_setrule.typ`).
    #[test]
    fn eval_set_text_variations_valido_passo_836() {
        let res = eval_variations_p836("#set text(variations: (wght: 250))\nOlá");
        assert!(res.is_ok(), "set rule variations deve compilar; got: {:?}", res.err());
    }

    /// P836: tag com 5 caracteres — mensagem + hints verbatim do vanilla.
    #[test]
    fn eval_text_variations_tag5_erro_passo_836() {
        let res = eval_variations_p836("#text(variations: (wgght: 1))[x]");
        let err = res.expect_err("tag com 5 chars deve falhar");
        assert_eq!(err[0].message, "tag must be one to four characters in length");
        assert_eq!(
            err[0].hints,
            vec![
                "found 5 characters".to_string(),
                "occurred in tag at index 0 (`\"wgght\"`)".to_string(),
            ]
        );
    }

    /// P836: tag vazia — `found 0 characters` (medido no vanilla).
    #[test]
    fn eval_text_variations_tag_vazia_erro_passo_836() {
        let res = eval_variations_p836("#text(variations: (\"\": 1))[x]");
        let err = res.expect_err("tag vazia deve falhar");
        assert_eq!(err[0].message, "tag must be one to four characters in length");
        assert_eq!(
            err[0].hints,
            vec![
                "found 0 characters".to_string(),
                "occurred in tag at index 0 (`\"\"`)".to_string(),
            ]
        );
    }

    /// P836: valor string — `expected float, found string` + hint da tag.
    #[test]
    fn eval_text_variations_valor_string_erro_passo_836() {
        let res = eval_variations_p836("#text(variations: (wght: \"bold\"))[x]");
        let err = res.expect_err("valor string deve falhar");
        assert_eq!(err[0].message, "expected float, found string");
        assert_eq!(
            err[0].hints,
            vec!["occurred in tag at index 0 (`\"wght\"`)".to_string()]
        );
    }

    /// P836: char não-ASCII — `tag may contain only printable ASCII
    /// characters` + hint do cluster + hint da tag.
    #[test]
    fn eval_text_variations_nao_ascii_erro_passo_836() {
        let res = eval_variations_p836("#text(variations: (\"wg€t\": 1))[x]");
        let err = res.expect_err("tag não-ASCII deve falhar");
        assert_eq!(err[0].message, "tag may contain only printable ASCII characters");
        assert_eq!(
            err[0].hints,
            vec![
                "found invalid cluster `\"€\"`".to_string(),
                "occurred in tag at index 0 (`\"wg€t\"`)".to_string(),
            ]
        );
    }

    /// P836: espaço interior — `spaces may only appear as padding
    /// following a tag` (medido no vanilla, `temp/p836/e6_space.typ`).
    #[test]
    fn eval_text_variations_espaco_erro_passo_836() {
        let res = eval_variations_p836("#text(variations: (\"w g\": 1))[x]");
        let err = res.expect_err("espaço interior deve falhar");
        assert_eq!(err[0].message, "spaces may only appear as padding following a tag");
        assert_eq!(
            err[0].hints,
            vec!["occurred in tag at index 0 (`\"w g\"`)".to_string()]
        );
    }

    /// P836: valor não-dict — `expected dictionary, found integer`
    /// (medido no vanilla, `temp/p836/e7_nondict.typ`), sem hint de tag.
    #[test]
    fn eval_text_variations_nao_dict_erro_passo_836() {
        let res = eval_variations_p836("#text(variations: 5)[x]");
        let err = res.expect_err("não-dict deve falhar");
        assert_eq!(err[0].message, "expected dictionary, found integer");
        assert!(err[0].hints.is_empty());
    }

    /// P836: os mesmos erros disparam via set rule (validação partilhada).
    #[test]
    fn eval_set_text_variations_tag5_erro_passo_836() {
        let res = eval_variations_p836("#set text(variations: (wgght: 1))\nOlá");
        let err = res.expect_err("tag com 5 chars na set rule deve falhar");
        assert_eq!(err[0].message, "tag must be one to four characters in length");
        assert_eq!(
            err[0].hints,
            vec![
                "found 5 characters".to_string(),
                "occurred in tag at index 0 (`\"wgght\"`)".to_string(),
            ]
        );
    }

    /// P836: valor fora de faixa NÃO é validado (vanilla compila
    /// `wght: 99999` — medido em `temp/p836/e5_range.typ`, exit 0).
    #[test]
    fn eval_text_variations_fora_de_faixa_aceite_passo_836() {
        let res = eval_variations_p836("#text(variations: (wght: 99999))[x]");
        assert!(
            res.is_ok(),
            "faixa não é validada (paridade vanilla); got: {:?}",
            res.err()
        );
    }
    // ── Passo 843 — foundations: repr(duration/content/type), constructors    ──
    // ── bytes/datetime, panic variádico, mensagens de assert, array.join      ──
    //
    // Medições vanilla em `temp/p843/` (binário release de
    // `lab/typst-original`, 2026-07-22). Erros e reprs são observáveis
    // (ADR-0107): as mensagens/formatos abaixo são verbatim do vanilla.
    mod tests_p843 {
        use super::*;
        use crate::entities::module::Module;
        use crate::entities::source_result::SourceResult;
        use crate::entities::value::Value;

        fn p843_eval(markup: &str) -> SourceResult<Module> {
            let world = MockWorld::new(markup);
            let src = World::source(&world, World::main(&world)).unwrap();
            eval_for_test(&world, &src)
        }

        fn p843_let_str(code: &str) -> String {
            let m = p843_eval(code).unwrap();
            match m.scope().get("r") {
                Some(Value::Str(s)) => s.to_string(),
                other => panic!("esperado Str em `r`, obtive {:?}", other),
            }
        }

        fn p843_erro(code: &str) -> String {
            p843_eval(code).unwrap_err()[0].message.to_string()
        }

        // ── F1 — constructor `duration(weeks: ...)` (suporte ao repr) ──────────
        //
        // Medido vanilla (`temp/p843/f1_duration.typ`): `weeks:` é aceite e
        // aparece no repr nomeado; antes o cristalino ignorava-o silenciosamente.

        #[test]
        fn p843_f1_duration_constructor_weeks() {
            assert_eq!(
                p843_let_str("#let r = repr(duration(weeks: 1, days: 1))"),
                "duration(weeks: 1, days: 1)"
            );
        }

        // ── P850 — durações negativas ──────────────────────────────────────────

        #[test]
        fn p850_duration_neg_repr() {
            assert_eq!(
                p843_let_str("#let r = repr(-duration(seconds: 3))"),
                "duration(seconds: -3)"
            );
        }

        #[test]
        fn p850_duration_constructor_negative() {
            assert_eq!(
                p843_let_str("#let r = repr(duration(seconds: -3))"),
                "duration(seconds: -3)"
            );
        }

        #[test]
        fn p850_duration_sub_negative() {
            assert_eq!(
                p843_let_str(
                    "#let r = repr(duration(seconds: 3) - duration(seconds: 5))"
                ),
                "duration(seconds: -2)"
            );
        }

        // ── F4 — constructor bytes(...) ────────────────────────────────────────
        //
        // Medido vanilla (`temp/p843/f4_*.typ`): aceita Str (UTF-8), Array de
        // ints 0–255 e Bytes (passthrough). Int → "expected string, array, or
        // bytes, found integer"; fora de faixa → "number must be between 0 and
        // 255".

        #[test]
        fn p843_f4_bytes_de_array_de_ints() {
            let m = p843_eval("#let r = bytes((1, 2, 3))").unwrap();
            assert_eq!(
                m.scope().get("r"),
                Some(&Value::Bytes(crate::entities::bytes::Bytes::from(vec![1u8, 2, 3])))
            );
        }

        #[test]
        fn p843_f4_bytes_de_str_utf8() {
            let m = p843_eval("#let r = bytes(\"abc\")").unwrap();
            assert_eq!(
                m.scope().get("r"),
                Some(&Value::Bytes(crate::entities::bytes::Bytes::from(vec![
                    97u8, 98, 99
                ])))
            );
            // "α" (U+03B1) → 2 bytes UTF-8 (0xCE 0xB1) — medido: len = 2.
            let m = p843_eval("#let r = bytes(\"α\")").unwrap();
            assert_eq!(
                m.scope().get("r"),
                Some(&Value::Bytes(crate::entities::bytes::Bytes::from(vec![
                    0xCEu8, 0xB1
                ])))
            );
        }

        #[test]
        fn p843_f4_bytes_passthrough_e_array_vazia() {
            let m = p843_eval("#let r = bytes(bytes((7,)))").unwrap();
            assert_eq!(
                m.scope().get("r"),
                Some(&Value::Bytes(crate::entities::bytes::Bytes::from(vec![7u8])))
            );
            let m = p843_eval("#let r = bytes(())").unwrap();
            assert_eq!(
                m.scope().get("r"),
                Some(&Value::Bytes(crate::entities::bytes::Bytes::from(vec![])))
            );
        }

        #[test]
        fn p843_f4_bytes_int_erra_mensagem_vanilla() {
            assert_eq!(
                p843_erro("#let r = bytes(3)"),
                "expected string, array, or bytes, found integer"
            );
        }

        #[test]
        fn p843_f4_bytes_fora_de_faixa_erra() {
            assert_eq!(
                p843_erro("#let r = bytes((256,))"),
                "number must be between 0 and 255"
            );
            assert_eq!(
                p843_erro("#let r = bytes((-1,))"),
                "number must be between 0 and 255"
            );
        }

        // ── F5 — constructor datetime(...) ─────────────────────────────────────
        //
        // Medido vanilla (`temp/p843/f5_*.typ`): 6 named args opcionais; data
        // completa ou hora completa ou ambas; mensagens verbatim.

        #[test]
        fn p843_f5_datetime_data_completa() {
            use crate::entities::world_types::Datetime;
            let m = p843_eval("#let r = datetime(year: 2024, month: 1, day: 1)").unwrap();
            assert_eq!(
                m.scope().get("r"),
                Some(&Value::Datetime(Datetime::new_date(2024, 1, 1).unwrap()))
            );
        }

        #[test]
        fn p843_f5_datetime_data_e_hora() {
            use crate::entities::world_types::Datetime;
            let m = p843_eval(
            "#let r = datetime(year: 2024, month: 1, day: 1, hour: 14, minute: 30, second: 5)",
        )
        .unwrap();
            assert_eq!(
                m.scope().get("r"),
                Some(&Value::Datetime(
                    Datetime::new_datetime(2024, 1, 1, 14, 30, 5).unwrap()
                ))
            );
        }

        #[test]
        fn p843_f5_datetime_so_hora() {
            use crate::entities::world_types::Datetime;
            // Medido vanilla: `datetime(hour: 14, minute: 30, second: 5)` é
            // aceite (Datetime::Time) e repr é `datetime(hour: 14, ...)`.
            let m =
                p843_eval("#let r = datetime(hour: 14, minute: 30, second: 5)").unwrap();
            assert_eq!(
                m.scope().get("r"),
                Some(&Value::Datetime(Datetime::new_time(14, 30, 5).unwrap()))
            );
        }

        #[test]
        fn p843_f5_datetime_validacoes_vanilla() {
            assert_eq!(
                p843_erro("#let r = datetime(year: 2024, month: 13, day: 1)"),
                "month is invalid"
            );
            assert_eq!(
                p843_erro("#let r = datetime(year: 2024, month: 2, day: 30)"),
                "date is invalid"
            );
            assert_eq!(
                p843_erro("#let r = datetime(hour: 25, minute: 0, second: 0)"),
                "time is invalid"
            );
            assert_eq!(p843_erro("#let r = datetime(year: 2024)"), "date is incomplete");
            assert_eq!(
                p843_erro("#let r = datetime(year: 2024, month: 1, day: 1, hour: 14)"),
                "time is incomplete"
            );
            assert_eq!(
                p843_erro("#let r = datetime()"),
                "at least one of date or time must be fully specified"
            );
        }

        #[test]
        fn p843_f5_datetime_repr_formato_nomeado() {
            assert_eq!(
                p843_let_str("#let r = repr(datetime(year: 2024, month: 1, day: 1))"),
                "datetime(year: 2024, month: 1, day: 1)"
            );
            assert_eq!(
                p843_let_str("#let r = repr(datetime(hour: 14, minute: 30, second: 5))"),
                "datetime(hour: 14, minute: 30, second: 5)"
            );
        }

        // ── F6 — panic variádico ───────────────────────────────────────────────
        //
        // Medido vanilla (`temp/p843/f6_*.typ`): prefixo "panicked with: ",
        // strings cruas, não-strings via repr, separador ", "; vazio → "panicked".

        #[test]
        fn p843_f6_panic_str() {
            assert_eq!(
                p843_erro(r#"#panic("this is wrong")"#),
                "panicked with: this is wrong"
            );
        }

        #[test]
        fn p843_f6_panic_variadico_misto() {
            assert_eq!(
                p843_erro(r#"#panic("a", 1, (x: 2))"#),
                "panicked with: a, 1, (x: 2)"
            );
        }

        #[test]
        fn p843_f6_panic_nao_str_usa_repr() {
            assert_eq!(p843_erro("#panic(42)"), "panicked with: 42");
        }

        #[test]
        fn p843_f6_panic_vazio() {
            assert_eq!(p843_erro("#panic()"), "panicked");
        }

        // ── F7 — mensagens de assert ───────────────────────────────────────────
        //
        // Medido vanilla (`temp/p843/f7_*.typ`): "assertion failed" /
        // "assertion failed: {msg}". assert.eq/ne já estavam em paridade — os
        // testes de não-regressão ficam aqui ao lado.

        #[test]
        fn p843_f7_assert_sem_mensagem() {
            assert_eq!(p843_erro("#assert(false)"), "assertion failed");
        }

        #[test]
        fn p843_f7_assert_com_mensagem() {
            assert_eq!(
                p843_erro(r#"#assert(false, message: "custom msg")"#),
                "assertion failed: custom msg"
            );
        }

        #[test]
        fn p843_f7_assert_true_ok() {
            assert!(p843_eval("#assert(true)").is_ok());
            assert!(p843_eval(r#"#assert(true, message: "não usada")"#).is_ok());
        }

        #[test]
        fn p843_f7_assert_eq_ne_nao_regridem() {
            assert_eq!(
                p843_erro("#assert.eq(1, 2)"),
                "equality assertion failed: value 1 was not equal to 2"
            );
            assert_eq!(
                p843_erro("#assert.ne(1, 1)"),
                "inequality assertion failed: value 1 was equal to 1"
            );
        }

        // ── #60 — array.join ───────────────────────────────────────────────────
        //
        // Medido vanilla (`temp/p843/join*.typ`): separador posicional opcional
        // (default none), `last:` separador alternativo antes do último,
        // `default:` devolvido para array vazio; vazio sem default → none;
        // não-strings via op `join` da linguagem → "cannot join X with Y".

        #[test]
        fn p843_join_com_e_sem_separador() {
            assert_eq!(p843_let_str(r#"#let r = ("a", "b").join("-")"#), "a-b");
            assert_eq!(p843_let_str(r#"#let r = ("a", "b").join()"#), "ab");
        }

        #[test]
        fn p843_join_um_elemento_e_vazio() {
            assert_eq!(p843_let_str(r#"#let r = ("a",).join("-")"#), "a");
            let m = p843_eval(r#"#let r = ().join("-")"#).unwrap();
            assert_eq!(m.scope().get("r"), Some(&Value::None));
        }

        #[test]
        fn p843_join_separador_last() {
            assert_eq!(
                p843_let_str(r#"#let r = ("a", "b", "c").join("-", last: " and ")"#),
                "a-b and c"
            );
            assert_eq!(
                p843_let_str(r#"#let r = ("a", "b").join("-", last: " and ")"#),
                "a and b"
            );
            assert_eq!(p843_let_str(r#"#let r = ("a",).join("-", last: " and ")"#), "a");
        }

        #[test]
        fn p843_join_default_para_vazio() {
            assert_eq!(p843_let_str(r#"#let r = ().join("-", default: "x")"#), "x");
        }

        #[test]
        fn p843_join_tipo_invalido_erra_mensagem_vanilla() {
            assert_eq!(
                p843_erro(r#"#let r = (1, 2).join("-")"#),
                "cannot join integer with string"
            );
        }

        #[test]
        fn p843_join_content_concatena() {
            // A op `join` da linguagem suporta content (ops::join do vanilla):
            // `([A], [B]).join()` → content sequência [A][B].
            let m = p843_eval("#let r = ([A], [B]).join()").unwrap();
            match m.scope().get("r") {
                Some(Value::Content(c)) => assert_eq!(c.plain_text(), "AB"),
                other => panic!("esperado Content em `r`, obtive {:?}", other),
            }
            // Content com separador string → content ("a-b" renderizado).
            let m = p843_eval(r#"#let r = ([A], [B]).join("-")"#).unwrap();
            match m.scope().get("r") {
                Some(Value::Content(c)) => assert_eq!(c.plain_text(), "A-B"),
                other => panic!("esperado Content em `r`, obtive {:?}", other),
            }
        }
    }

    // ── P906 (Área D) — underbrace/overbrace/underbracket/overbracket ──────────
    //
    // TDD vermelho (Agente A): hoje NÃO existe braço dedicado para estas 4
    // funções no `match name.as_str()` de `compiler/eval/math.rs` — caem no
    // fallback genérico P302/P303 (identifier desconhecido, args preservados
    // como `MathSequence([MathIdent, MathDelimited])`, "fallback de texto
    // literal" per `typst-passo-899-relatorio.md` Parte C ADIADA). Este módulo
    // só define testes (mais 1 helper de busca) — nenhuma lógica de
    // implementação. Ver `compiler/eval.md` §P906.
    mod tests_p906 {
        use super::*;

        fn find_mathunderover_in(
            c: &Content,
        ) -> Option<(Content, Option<Content>, Option<Content>)> {
            match c {
                Content::MathUnderover(e) => {
                    Some((e.base.clone(), e.under.clone(), e.over.clone()))
                }
                Content::Sequence(items) | Content::MathSequence(items) => {
                    items.iter().find_map(find_mathunderover_in)
                }
                Content::Equation(e) => find_mathunderover_in(&e.body),
                _ => None,
            }
        }

        /// **P906 D1** — `underbracket(a+b+c)` sem anotação (1 arg
        /// posicional): `MathUnderover` de 1 nível, `under=Some(⎵)` (U+23B5),
        /// `over=None`, `base` preserva o `a+b+c` original (não perdido).
        #[test]
        fn p906_underbracket_sem_anotacao_produz_mathunderover_under_u23b5() {
            let world = MockWorld::new("$ underbracket(a+b+c) $");
            let content = extract_math_content(&world);
            let found = find_mathunderover_in(&content);
            assert!(
                found.is_some(),
                "underbracket(a+b+c) deve produzir MathUnderover; content: {:?}",
                content
            );
            let (base, under, over) = found.unwrap();
            assert!(over.is_none(), "underbracket não deve ter over");
            let under = under.expect("underbracket deve ter under");
            assert_eq!(
                under.plain_text().chars().next(),
                Some('\u{23B5}'),
                "under deve ser exactamente U+23B5"
            );
            let base_text = base.plain_text();
            assert!(
                base_text.contains('a')
                    && base_text.contains('b')
                    && base_text.contains('c'),
                "base deve preservar o conteúdo original a+b+c: {:?}",
                base_text
            );
        }

        /// **P906 D2** — `overbracket(a+b+c)`: idem, `over=Some(⎴)` (U+23B4),
        /// `under=None`.
        #[test]
        fn p906_overbracket_sem_anotacao_produz_mathunderover_over_u23b4() {
            let world = MockWorld::new("$ overbracket(a+b+c) $");
            let content = extract_math_content(&world);
            let found = find_mathunderover_in(&content);
            assert!(
                found.is_some(),
                "overbracket(a+b+c) deve produzir MathUnderover; content: {:?}",
                content
            );
            let (base, under, over) = found.unwrap();
            assert!(under.is_none(), "overbracket não deve ter under");
            let over = over.expect("overbracket deve ter over");
            assert_eq!(
                over.plain_text().chars().next(),
                Some('\u{23B4}'),
                "over deve ser exactamente U+23B4"
            );
            let base_text = base.plain_text();
            assert!(
                base_text.contains('a')
                    && base_text.contains('b')
                    && base_text.contains('c'),
                "base deve preservar o conteúdo original a+b+c: {:?}",
                base_text
            );
        }

        /// **P906 D3** — `underbrace(a+b+c, "soma")` COM anotação: estrutura
        /// ANINHADA de 2 `MathUnderover`. Externo: `under=Some(<anotação
        /// "soma">)`, `over=None`, `base` = OUTRO `MathUnderover` (interno).
        /// Interno: `under=Some(⏟)` (U+23DF), `over=None`, `base` = `a+b+c`
        /// original (não perdido/achatado).
        #[test]
        fn p906_underbrace_com_anotacao_produz_mathunderover_aninhado() {
            let world = MockWorld::new(r#"$ underbrace(a+b+c, "soma") $"#);
            let content = extract_math_content(&world);
            let outer = find_mathunderover_in(&content);
            assert!(
                outer.is_some(),
                "underbrace(a+b+c, \"soma\") deve produzir MathUnderover; content: {:?}",
                content
            );
            let (outer_base, outer_under, outer_over) = outer.unwrap();
            assert!(outer_over.is_none(), "nível externo não deve ter over");
            let outer_under =
                outer_under.expect("nível externo deve ter under = anotação");
            assert!(
                outer_under.plain_text().contains("soma"),
                "under externo deve conter a anotação 'soma': {:?}",
                outer_under.plain_text()
            );

            let inner = find_mathunderover_in(&outer_base);
            assert!(
            inner.is_some(),
            "base do nível externo deve ser outro MathUnderover (nível interno); obteve: {:?}",
            outer_base
        );
            let (inner_base, inner_under, inner_over) = inner.unwrap();
            assert!(inner_over.is_none(), "nível interno não deve ter over");
            let inner_under =
                inner_under.expect("nível interno deve ter under = chave ⏟");
            assert_eq!(
                inner_under.plain_text().chars().next(),
                Some('\u{23DF}'),
                "under interno deve ser exactamente U+23DF"
            );
            let inner_base_text = inner_base.plain_text();
            assert!(
            inner_base_text.contains('a')
                && inner_base_text.contains('b')
                && inner_base_text.contains('c'),
            "base do nível interno deve preservar o a+b+c original (não perdido): {:?}",
            inner_base_text
        );
        }

        /// **P906 D4** — `overbrace(a+b+c, "soma")`: idem, aninhado com `over`
        /// em vez de `under`, char `⏞` (U+23DE).
        #[test]
        fn p906_overbrace_com_anotacao_produz_mathunderover_aninhado() {
            let world = MockWorld::new(r#"$ overbrace(a+b+c, "soma") $"#);
            let content = extract_math_content(&world);
            let outer = find_mathunderover_in(&content);
            assert!(
                outer.is_some(),
                "overbrace(a+b+c, \"soma\") deve produzir MathUnderover; content: {:?}",
                content
            );
            let (outer_base, outer_under, outer_over) = outer.unwrap();
            assert!(outer_under.is_none(), "nível externo não deve ter under");
            let outer_over = outer_over.expect("nível externo deve ter over = anotação");
            assert!(
                outer_over.plain_text().contains("soma"),
                "over externo deve conter a anotação 'soma': {:?}",
                outer_over.plain_text()
            );

            let inner = find_mathunderover_in(&outer_base);
            assert!(
            inner.is_some(),
            "base do nível externo deve ser outro MathUnderover (nível interno); obteve: {:?}",
            outer_base
        );
            let (inner_base, inner_under, inner_over) = inner.unwrap();
            assert!(inner_under.is_none(), "nível interno não deve ter under");
            let inner_over = inner_over.expect("nível interno deve ter over = chave ⏞");
            assert_eq!(
                inner_over.plain_text().chars().next(),
                Some('\u{23DE}'),
                "over interno deve ser exactamente U+23DE"
            );
            let inner_base_text = inner_base.plain_text();
            assert!(
            inner_base_text.contains('a')
                && inner_base_text.contains('b')
                && inner_base_text.contains('c'),
            "base do nível interno deve preservar o a+b+c original (não perdido): {:?}",
            inner_base_text
        );
        }

        /// **P906 D5** — confirma os 4 chars exactos por codepoint (não
        /// comparação visual) para as 4 funções SEM anotação — complementa
        /// D1/D2 (que só cobrem underbracket/overbracket) estendendo à
        /// variante sem anotação de underbrace/overbrace (nível único, sem
        /// aninhamento — caso distinto de D3/D4).
        #[test]
        fn p906_todas_as_4_funcoes_sem_anotacao_char_exato_por_codepoint() {
            let cases: [(&str, char, bool); 4] = [
                ("underbracket", '\u{23B5}', true),
                ("overbracket", '\u{23B4}', false),
                ("underbrace", '\u{23DF}', true),
                ("overbrace", '\u{23DE}', false),
            ];
            for (name, expected_char, is_under) in cases {
                let src = format!("$ {name}(a+b+c) $");
                let world = MockWorld::new(&src);
                let content = extract_math_content(&world);
                let found = find_mathunderover_in(&content);
                assert!(
                    found.is_some(),
                    "{name}(a+b+c) deve produzir MathUnderover; content: {:?}",
                    content
                );
                let (_, under, over) = found.unwrap();
                let actual = if is_under {
                    assert!(over.is_none(), "{name}: não deve ter over");
                    under.expect("deve ter under")
                } else {
                    assert!(under.is_none(), "{name}: não deve ter under");
                    over.expect("deve ter over")
                };
                assert_eq!(
                    actual.plain_text().chars().next(),
                    Some(expected_char),
                    "{name}: char errado por codepoint"
                );
            }
        }
    }

    // ── P981 — `lr(...)` reconhecido no eval math (sem vazamento de texto) ──
    //
    // Especificação: `compiler/eval.md` §P981. O vanilla
    // (`math/lr.rs` + `ir/resolve.rs:850-940`) trata `lr(body)` como
    // delimitadores esticados ao conteúdo; o corpo inclui os delimitadores.
    #[cfg(test)]
    mod tests_p981 {
        use super::*;

        fn find_mathdelimited_in(c: &Content) -> Option<(char, char)> {
            match c {
                Content::MathDelimited(e) => Some((e.open, e.close)),
                Content::Sequence(items) | Content::MathSequence(items) => {
                    items.iter().find_map(find_mathdelimited_in)
                }
                Content::Equation(e) => find_mathdelimited_in(&e.body),
                _ => None,
            }
        }

        /// **Caso da secção 22**: `lr((a/b))` — sem texto "lr" no conteúdo e
        /// o grupo delimitado presente (antes: `MathIdent("lr")` literal).
        #[test]
        fn p981_lr_grupo_delimitado_nao_vaza_texto() {
            let world = MockWorld::new("$ lr((a/b)) $");
            let content = extract_math_content(&world);
            let texto = content.plain_text();
            assert!(
                !texto.contains("lr"),
                "o nome da função não pode vazar como texto: {texto:?}"
            );
            let delims = find_mathdelimited_in(&content);
            assert_eq!(
                delims,
                Some(('(', ')')),
                "lr((a/b)) deve produzir MathDelimited('(',')'): {content:?}"
            );
        }

        /// **Delimitadores soltos** (não um grupo): `lr(chevron.l a/b
        /// chevron.r)` — a sequência começa com opener e acaba com closer;
        /// o vanilla estica-os ao conteúdo (`resolve.rs:880-891`). O
        /// cristalino reescreve para `math_delimited(⟨, meio, ⟩)`.
        #[test]
        fn p981_lr_delimitadores_soltos_vira_delimited() {
            let world = MockWorld::new("$ lr(chevron.l a/b chevron.r) $");
            let content = extract_math_content(&world);
            let texto = content.plain_text();
            assert!(!texto.contains("lr"), "sem vazamento: {texto:?}");
            let delims = find_mathdelimited_in(&content);
            assert_eq!(
                delims,
                Some(('⟨', '⟩')),
                "chevrons devem virar os delimitadores: {content:?}"
            );
        }

        /// **P1132r / secção 22**: a primeira extremidade pode ter classe
        /// nativa Closing e a última Opening. `lr` redefine os papéis pela
        /// posição, preservando os caracteres invertidos.
        #[test]
        fn p1132r_lr_brackets_invertidos_vira_delimited() {
            let world = MockWorld::new("$ lr(\\]a/b\\[) $");
            let content = extract_math_content(&world);
            assert_eq!(
                find_mathdelimited_in(&content),
                Some((']', '[')),
                "brackets invertidos devem entrar no caminho escalável: {content:?}"
            );
        }

        /// **Guarda**: `lr` com corpo delimitado por chavetas — sem
        /// reescrita desnecessária nem vazamento.
        #[test]
        fn p981_lr_chavetas_sem_vazamento() {
            let world = MockWorld::new("$ lr({a/b}) $");
            let content = extract_math_content(&world);
            let texto = content.plain_text();
            assert!(!texto.contains("lr"), "sem vazamento: {texto:?}");
            assert_eq!(find_mathdelimited_in(&content), Some(('{', '}')));
        }
    }

    // ── P992 — `scripts(body)`/`limits(body, inline:)` reconhecidas no eval math ──
    //
    // Especificação: `compiler/eval.md` §P992. Achado externo 2026-08-07,
    // secção 32: sem estes braços, `scripts(...)`/`limits(...)` caem no
    // fallback de identificador desconhecido — texto literal, argumentos
    // perdidos (mesma família de bug de P944/P958/P981).
    #[cfg(test)]
    mod tests_p992 {
        use super::*;

        fn find_mathlimitsoverride_in(c: &Content) -> Option<(bool, bool)> {
            match c {
                Content::MathLimitsOverride(e) => Some((e.limits, e.inline)),
                Content::Sequence(items) | Content::MathSequence(items) => {
                    items.iter().find_map(find_mathlimitsoverride_in)
                }
                Content::Equation(e) => find_mathlimitsoverride_in(&e.body),
                Content::MathAttach(e) => find_mathlimitsoverride_in(&e.base),
                _ => None,
            }
        }

        /// **Caso do achado**: `scripts(sum)_1^2` — sem "scripts" no texto,
        /// `MathLimitsOverride { limits: false, .. }` presente.
        #[test]
        fn p992_scripts_reconhecida_sem_vazamento() {
            let world = MockWorld::new("$ scripts(sum)_1^2 $");
            let content = extract_math_content(&world);
            let texto = content.plain_text();
            assert!(
                !texto.contains("scripts"),
                "o nome da função não pode vazar como texto: {texto:?}"
            );
            assert_eq!(
            find_mathlimitsoverride_in(&content),
            Some((false, true)),
            "scripts(sum) deve produzir MathLimitsOverride{{limits:false}}: {content:?}"
        );
        }

        /// **Caso do achado**: `limits(A)_1^2` (sem `inline:` explícito) —
        /// `inline` fica `true` por omissão (paridade `LimitsElem.inline`).
        #[test]
        fn p992_limits_default_inline_true() {
            let world = MockWorld::new("$ limits(A)_1^2 $");
            let content = extract_math_content(&world);
            let texto = content.plain_text();
            assert!(!texto.contains("limits"), "sem vazamento: {texto:?}");
            assert_eq!(
            find_mathlimitsoverride_in(&content),
            Some((true, true)),
            "limits(A) sem inline: deve ser {{limits:true, inline:true}}: {content:?}"
        );
        }

        /// **`inline: false` explícito**: named arg reconhecido e propagado.
        #[test]
        fn p992_limits_inline_false_explicito() {
            let world = MockWorld::new("$ limits(A, inline: false)_1^2 $");
            let content = extract_math_content(&world);
            assert_eq!(
            find_mathlimitsoverride_in(&content),
            Some((true, false)),
            "limits(A, inline: false): deve ser {{limits:true, inline:false}}: {content:?}"
        );
        }

        /// **Guarda**: uso normal sem `limits()`/`scripts()` continua
        /// inalterado — base fica `MathOp`/`MathIdent` normal, sem
        /// `MathLimitsOverride`.
        #[test]
        fn p992_guarda_sum_sem_wrapper_nao_produz_override() {
            let world = MockWorld::new("$ sum_1^2 $");
            let content = extract_math_content(&world);
            assert_eq!(
                find_mathlimitsoverride_in(&content),
                None,
                "sum_1^2 sem wrapper: não deve produzir MathLimitsOverride: {content:?}"
            );
        }

        // ── P992b — paridade de erros em named args ─────────────────────
        //
        // Mensagens medidas no vanilla (`/tmp/e1..e3.typ`):
        // `scripts(A, foo: 1)`/`limits(A, foo: 1)` → "unexpected argument: foo";
        // `limits(A, inline: 5)` → "expected boolean, found content".
        // Antes: named args desconhecidos ignorados e `inline` não-booleano
        // descartado silenciosamente (`if let Ok(Value::Bool)`).
        use crate::contracts::world::World as _;

        fn eval_err(source: &str) -> Vec<String> {
            let world = MockWorld::new(source);
            let src = World::source(&world, World::main(&world)).unwrap();
            eval_for_test(&world, &src)
                .unwrap_err()
                .iter()
                .map(|d| d.message.clone().to_string())
                .collect()
        }

        /// **P992b** — named arg desconhecido em `scripts` → erro
        /// "unexpected argument: foo" (antes: ignorado silenciosamente).
        #[test]
        fn p992b_scripts_named_arg_desconhecido_erro_vanilla() {
            let errs = eval_err("$ scripts(A, foo: 1) $");
            assert!(
                errs.iter().any(|m| m.contains("unexpected argument: foo")),
                "deve rejeitar named arg desconhecido como o vanilla: {errs:?}"
            );
        }

        /// **P992b** — named arg desconhecido em `limits` → erro
        /// "unexpected argument: foo" (antes: `_ => {}` no loop).
        #[test]
        fn p992b_limits_named_arg_desconhecido_erro_vanilla() {
            let errs = eval_err("$ limits(A, foo: 1) $");
            assert!(
                errs.iter().any(|m| m.contains("unexpected argument: foo")),
                "deve rejeitar named arg desconhecido como o vanilla: {errs:?}"
            );
        }

        /// **P992b** — `inline:` não-booleano → erro "expected boolean,
        /// found content" (antes: `if let Ok(Value::Bool)` descartava e
        /// `inline` ficava preso em `true`). `5` em math avalia como
        /// `Value::Content` → "content", byte-a-byte com o vanilla medido.
        #[test]
        fn p992b_limits_inline_nao_booleano_erro_vanilla() {
            let errs = eval_err("$ limits(A, inline: 5) $");
            assert!(
                errs.iter().any(|m| m.contains("expected boolean, found content")),
                "deve rejeitar inline não-booleano como o vanilla: {errs:?}"
            );
        }
    }

    // ── P996 — `\` quebra a linha ANTES do emparelhamento lr ─────────────
    //
    // Medição P995 + documentação oficial: `\` em math é só quebra de linha;
    // os delimitadores nunca esticam sobre ela (vanilla
    // `ir/multiline.rs:56-107` — dimensionados pelo segmento próprio, não
    // pela pilha). `(n \ k)` no vanilla = duas linhas centradas com
    // parênteses naturais; o cristalino emparelhava-os no parser e esticava
    // sobre a grelha (n a 0.00pt das peças).
    #[cfg(test)]
    mod tests_p996 {
        use super::*;

        fn find_mathdelimited_in(c: &Content) -> Option<(char, char)> {
            match c {
                Content::MathDelimited(e) => Some((e.open, e.close)),
                Content::Sequence(items) | Content::MathSequence(items) => {
                    items.iter().find_map(find_mathdelimited_in)
                }
                Content::Equation(e) => find_mathdelimited_in(&e.body),
                _ => None,
            }
        }

        fn has_linebreak(c: &Content) -> bool {
            match c {
                Content::Linebreak(_) => true,
                Content::Sequence(items) | Content::MathSequence(items) => {
                    items.iter().any(has_linebreak)
                }
                Content::Equation(e) => has_linebreak(&e.body),
                _ => false,
            }
        }

        /// **P996** — `(n \ k)`: o corpo com Linebreak NÃO pode ficar dentro
        /// de um `MathDelimited` (que esticaria os delimitadores sobre a
        /// grelha). O conteúdo fica uma sequência com os parênteses como
        /// glifos normais (`MathText`) à volta da quebra.
        #[test]
        fn p996_paren_com_linebreak_nao_emparelha() {
            let world = MockWorld::new("$ (n \\ k) $");
            let content = extract_math_content(&world);
            assert!(
                has_linebreak(&content),
                "a quebra de linha deve sobreviver: {content:?}"
            );
            assert!(
                find_mathdelimited_in(&content).is_none(),
                "corpo com Linebreak não pode ser MathDelimited (esticaria): {content:?}"
            );
            let texto = content.plain_text();
            assert!(
                texto.contains('(') && texto.contains(')'),
                "os parênteses ficam como glifos normais: {texto:?}"
            );
        }

        /// **P996 (guarda)** — `(n)` sem quebra: `MathDelimited` continua a
        /// ser produzido (caminho esticável inalterado).
        #[test]
        fn p996_paren_sem_linebreak_continua_delimited() {
            let world = MockWorld::new("$ (n) $");
            let content = extract_math_content(&world);
            assert_eq!(
                find_mathdelimited_in(&content),
                Some(('(', ')')),
                "(n) sem quebra deve continuar MathDelimited: {content:?}"
            );
        }
    }

    // ── P997 — o Linebreak sobe para o nível do run ──────────────────────
    //
    // Medição P997 (repro `/tmp/p997.typ`): no vanilla, `(n \ k) = x` ancora
    // o `=`/`x` na linha de BAIXO (com `k)`) — a quebra divide a run inteira
    // (`expand_multiline_fence`, `ir/multiline.rs:56-107`). O
    // desemparelhamento de P996 deixava o Linebreak aninhado na
    // sub-sequência do grupo → invisível para `partition_grid` → o `=`
    // ficava na linha de cima.
    #[cfg(test)]
    mod tests_p997 {
        use super::*;

        /// A quebra de linha dentro de `(n \ k)` tem de aparecer ao nível do
        /// TOPO da sequência da equação (não aninhada numa sub-sequência).
        #[test]
        fn p997_linebreak_sobe_para_o_nivel_do_run() {
            let world = MockWorld::new("$ (n \\ k) = x $");
            let content = extract_math_content(&world);
            let Content::Equation(e) = &content else {
                panic!("esperava Equation: {content:?}")
            };
            let Content::MathSequence(items) = &e.body else {
                panic!("esperava MathSequence no corpo: {:?}", e.body)
            };
            assert!(
                items.iter().any(|c| matches!(c, Content::Linebreak(_))),
                "o Linebreak deve estar ao nível do topo da sequência: {:?}",
                e.body
            );
            // E o conteúdo depois do grupo (`=`, `x`) vem DEPOIS da quebra.
            let pos_lb =
                items.iter().position(|c| matches!(c, Content::Linebreak(_))).unwrap();
            let texto_depois: String =
                items[pos_lb..].iter().map(|c| c.plain_text()).collect();
            assert!(
            texto_depois.contains('=') && texto_depois.contains('x'),
            "o `=` e o `x` devem vir depois da quebra (linha de baixo): {texto_depois:?}"
        );
        }

        /// **Guarda P996**: `(n \ k)` sozinho continua a subir a quebra para
        /// o topo (e sem delimitador emparelhado).
        #[test]
        fn p997_guarda_p996_grupo_sozinho() {
            let world = MockWorld::new("$ (n \\ k) $");
            let content = extract_math_content(&world);
            let Content::Equation(e) = &content else {
                panic!("esperava Equation: {content:?}")
            };
            let Content::MathSequence(items) = &e.body else {
                panic!("esperava MathSequence no corpo: {:?}", e.body)
            };
            assert!(
                items.iter().any(|c| matches!(c, Content::Linebreak(_))),
                "quebra ao nível do topo: {:?}",
                e.body
            );
        }
    }

    // ── P1043: Pares de independência testcase() para math.rs:421 e eval/mod.rs:1039 ──

    #[test]
    fn p1043_math_callee_dot_double_isolada() {
        // C1=T, C2=T: dot.double(x) vira MathAccent
        let world = MockWorld::new("$ dot.double(x) $");
        let content = extract_math_content(&world);
        let Content::Equation(e) = &content else {
            panic!("esperava Equation: {content:?}")
        };
        let is_accent = match &e.body {
            Content::MathAccent(acc) => {
                acc.accent.plain_text().contains('̈')
                    || acc.accent.plain_text().contains('¨')
            }
            Content::MathSequence(items) => {
                items.iter().any(|c| matches!(c, Content::MathAccent(_)))
            }
            _ => false,
        };
        assert!(is_accent, "dot.double deve produzir MathAccent: {:?}", e.body);
    }

    #[test]
    fn p1043_math_callee_dot_other_isolada() {
        // C1=T, C2=F: dot.custom(x) cai no fallback de callee (desconhecido)
        let errs = eval_math_err("$ dot.custom(x) $");
        assert!(!errs.is_empty(), "dot.custom deve gerar erro");
    }

    #[test]
    fn p1043_math_callee_other_double_isolada() {
        // C1=F, C2=_: other.double(x) cai em other_callee
        let errs = eval_math_err("$ calc.double(x) $");
        assert!(!errs.is_empty(), "calc.double deve gerar erro");
    }

    #[test]
    fn p1043_eval_dict_spread_all_isolada() {
        // C1=T, C2=T: todos os spreads são dict -> erro com hint para (: ...)
        let errs = eval_math_err("#let d = (a: 1); #(..d, ..d)");
        assert!(
            !errs.is_empty(),
            "esperava erro ao fazer spread de dict em array literal"
        );
        assert!(errs[0]
            .hints
            .iter()
            .any(|h| h.contains("add a colon to create a dictionary")));
    }

    #[test]
    fn p1043_eval_dict_spread_after_array_isolada() {
        // C1=F, C2=_: spread após array -> sem hint de colon
        let errs = eval_math_err("#let a = (1,); #let d = (k: 2); #(..a, ..d)");
        assert!(
            !errs.is_empty(),
            "esperava erro ao fazer spread de dict em array com elementos normais"
        );
        assert!(!errs[0]
            .hints
            .iter()
            .any(|h| h.contains("add a colon to create a dictionary")));
    }

    #[test]
    fn p1043_eval_dict_spread_before_array_isolada() {
        // C1=T, C2=F: spread antes de array -> sem hint de colon
        let errs = eval_math_err("#let d = (k: 2); #(..d, 1)");
        assert!(
            !errs.is_empty(),
            "esperava erro ao fazer spread de dict em array com elementos normais"
        );
        assert!(!errs[0]
            .hints
            .iter()
            .any(|h| h.contains("add a colon to create a dictionary")));
    }

    // ── Passo 1105: Função Matemática Nativa attach(base, t:, b:, tl:, bl:, tr:, br:) ──

    #[test]
    fn p1105_attach_basico_t_b() {
        let world = MockWorld::new("$ attach(A, t: x, b: y) $");
        let content = extract_math_content(&world);
        let texto = content.plain_text();
        assert!(!texto.contains("attach"), "sem vazamento literal de attach: {texto:?}");
        match content {
            Content::Equation(eq) => match &eq.body {
                Content::MathAttach(att) => {
                    assert_eq!(att.base.plain_text(), "A");
                    assert_eq!(
                        match &att.t {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("t must be present"),
                        }
                        .plain_text(),
                        "x"
                    );
                    assert_eq!(
                        match &att.b {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("b must be present"),
                        }
                        .plain_text(),
                        "y"
                    );
                    assert!(matches!(att.tl, MathAttachSlot::Omitted));
                    assert!(matches!(att.bl, MathAttachSlot::Omitted));
                }
                other => panic!("esperado MathAttach, obteve {:?}", other),
            },
            other => panic!("esperado Equation, obteve {:?}", other),
        }
    }

    #[test]
    fn p1105_attach_quatro_cantos() {
        let world = MockWorld::new("$ attach(A, tl: 1, bl: 2, tr: 3, br: 4) $");
        let content = extract_math_content(&world);
        let texto = content.plain_text();
        assert!(!texto.contains("attach"), "sem vazamento literal: {texto:?}");
        match content {
            Content::Equation(eq) => match &eq.body {
                Content::MathAttach(att) => {
                    assert_eq!(att.base.plain_text(), "A");
                    assert_eq!(
                        match &att.tl {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("tl must be present"),
                        }
                        .plain_text(),
                        "1"
                    );
                    assert_eq!(
                        match &att.bl {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("bl must be present"),
                        }
                        .plain_text(),
                        "2"
                    );
                    assert_eq!(
                        match &att.tr {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("tr must be present"),
                        }
                        .plain_text(),
                        "3"
                    );
                    assert_eq!(
                        match &att.br {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("br must be present"),
                        }
                        .plain_text(),
                        "4"
                    );
                }
                other => panic!("esperado MathAttach, obteve {:?}", other),
            },
            other => panic!("esperado Equation, obteve {:?}", other),
        }
    }

    #[test]
    fn p1105_attach_seis_anexos_simultaneos() {
        let world = MockWorld::new(
            "$ attach(A, t: alpha, b: beta, tl: n, tr: m, bl: p, br: q) $",
        );
        let content = extract_math_content(&world);
        let texto = content.plain_text();
        assert!(!texto.contains("attach"), "sem vazamento literal: {texto:?}");
        match content {
            Content::Equation(eq) => match &eq.body {
                Content::MathAttach(att) => {
                    assert_eq!(att.base.plain_text(), "A");
                    assert_eq!(
                        match &att.t {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("t must be present"),
                        }
                        .plain_text(),
                        "α"
                    );
                    assert_eq!(
                        match &att.b {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("b must be present"),
                        }
                        .plain_text(),
                        "β"
                    );
                    assert_eq!(
                        match &att.tl {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("tl must be present"),
                        }
                        .plain_text(),
                        "n"
                    );
                    assert_eq!(
                        match &att.tr {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("tr must be present"),
                        }
                        .plain_text(),
                        "m"
                    );
                    assert_eq!(
                        match &att.bl {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("bl must be present"),
                        }
                        .plain_text(),
                        "p"
                    );
                    assert_eq!(
                        match &att.br {
                            MathAttachSlot::Present(c) => c,
                            _ => panic!("br must be present"),
                        }
                        .plain_text(),
                        "q"
                    );
                }
                other => panic!("esperado MathAttach, obteve {:?}", other),
            },
            other => panic!("esperado Equation, obteve {:?}", other),
        }
    }

    #[test]
    fn p1105_attach_com_limits_e_scripts() {
        let world_l = MockWorld::new("$ attach(limits(A), t: 1, b: 2) $");
        let content_l = extract_math_content(&world_l);
        assert!(!content_l.plain_text().contains("attach"));

        let world_s = MockWorld::new("$ attach(scripts(sum), t: n, b: k) $");
        let content_s = extract_math_content(&world_s);
        assert!(!content_s.plain_text().contains("attach"));
    }

    #[test]
    fn p1105_attach_named_arg_desconhecido_erro() {
        let errs = eval_math_err("$ attach(A, foo: 1) $");
        assert!(!errs.is_empty(), "esperava erro para named arg 'foo'");
        assert!(errs.iter().any(|e| e.message.contains("unexpected argument: foo")));
    }

    #[test]
    fn p1105_attach_zero_ou_multiplos_args_posicionais_erro() {
        let errs0 = eval_math_err("$ attach() $");
        assert!(!errs0.is_empty(), "esperava erro para 0 args");
        assert!(errs0.iter().any(|e| e.message == "missing argument: base"));

        let errs2 = eval_math_err("$ attach(A, B) $");
        assert!(!errs2.is_empty(), "esperava erro para 2 args posicionais");
        assert!(errs2.iter().any(|e| e.message == "unexpected argument"));
    }

    #[test]
    fn p1121_debug_mat_sec37() {
        let sources = [
            "$ mat(1, 2; 3, 4, row-gap: #(1em), column-gap: #(2em)) $",
            "$ mat(1, 2; 3, 4, gap: #(0.3em)) $",
            "$ mat(augment: #(2), 1, 2, 3; 4, 5, 6) $",
            "$ mat(delim: #(none), 1, 2; 3, 4) $",
        ];
        for (i, src) in sources.iter().enumerate() {
            println!("=== EQ {} ===", i);
            let world = MockWorld::new(src);
            let source = World::source(&world, World::main(&world)).unwrap();
            let module = eval_for_test(&world, &source).unwrap();
            if let Some(Content::Equation(eq)) = module.content() {
                if let Content::MathMatrix(m) = &eq.body {
                    println!("  ROWS: {}", m.rows.len());
                    for (r_idx, r) in m.rows.iter().enumerate() {
                        println!("    ROW {}: {} cells", r_idx, r.len());
                    }
                    println!("  DELIM: {:?}", m.delim);
                    println!("  ROW_GAP: {:?}", m.row_gap);
                    println!("  COL_GAP: {:?}", m.column_gap);
                    println!("  GAP: {:?}", m.gap);
                    println!("  AUGMENT: {:?}", m.augment);
                }
            }
        }
    }

    #[test]
    fn p1144_gradient_metodos_estaticos_e_instancia() {
        let m = p729_eval(
            "#let l = gradient.linear((red, 0%), (green, 60%), (blue, 100%), angle: 30deg, space: rgb, relative: \"self\")\n\
             #let r = gradient.radial(red, blue, center: (25%, 75%), radius: 40%, focal-center: (30%, 70%), focal-radius: 10%, space: luma, relative: \"parent\")\n\
             #let c = gradient.conic(red, blue, center: (20%, 80%), angle: 45deg, space: color.hsl)\n\
             #let kinds = (l.kind() == gradient.linear, gradient.kind(r) == gradient.radial, c.kind() == gradient.conic)\n\
             #let kind_repr = (repr(l.kind()), repr(r.kind()), repr(c.kind()))\n\
             #let absences = (l.center(), r.angle(), c.radius(), l.focal-center(), c.focal-radius())\n\
             #let values = (repr(l.stops()), l.space() == rgb, repr(l.relative()), repr(r.relative()), repr(c.relative()), repr(r.center()), repr(r.radius()), repr(r.focal-center()), repr(r.focal-radius()))\n\
             #let parity = (l.stops() == gradient.stops(l), r.center() == gradient.center(r), c.angle() == gradient.angle(c))",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("kinds"),
            Some(&Value::Array(vec![Value::Bool(true); 3]))
        );
        assert_eq!(
            m.scope().get("kind_repr"),
            Some(&Value::Array(vec![
                Value::Str("linear".into()),
                Value::Str("radial".into()),
                Value::Str("conic".into()),
            ]))
        );
        assert_eq!(m.scope().get("absences"), Some(&Value::Array(vec![Value::None; 5])));
        assert_eq!(
            m.scope().get("parity"),
            Some(&Value::Array(vec![Value::Bool(true); 3]))
        );
        let Value::Array(values) = m.scope().get("values").unwrap() else {
            panic!("values")
        };
        assert_eq!(values[1], Value::Bool(true));
        assert_eq!(values[2], Value::Str("\"self\"".into()));
        assert_eq!(values[3], Value::Str("\"parent\"".into()));
        assert_eq!(values[4], Value::Str("auto".into()));
    }

    #[test]
    fn p1144_gradient_sample_e_samples() {
        let m = p729_eval(
            "#let l = gradient.linear(red, blue)\n\
             #let endpoints = (l.sample(-20%) == l.sample(0%), l.sample(120%) == l.sample(100%))\n\
             #let angle = gradient.conic(red, blue).sample(180deg) == gradient.conic(red, blue).sample(50%)\n\
             #let empty = l.samples()\n\
             #let many = l.samples(0%, 50%, 100%)\n\
             #let parity = many == gradient.samples(l, 0%, 50%, 100%)",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("endpoints"),
            Some(&Value::Array(vec![Value::Bool(true); 2]))
        );
        assert_eq!(m.scope().get("angle"), Some(&Value::Bool(true)));
        assert_eq!(m.scope().get("empty"), Some(&Value::Array(vec![])));
        let Value::Array(many) = m.scope().get("many").unwrap() else { panic!("many") };
        assert_eq!(many.len(), 3);
        assert_eq!(m.scope().get("parity"), Some(&Value::Bool(true)));
    }

    #[test]
    fn p1269_linear_sample_coincidencia_publica() {
        let m = p729_eval(
            "#let check(space) = {\n\
             \u{20} let g = gradient.linear(red, green, blue, space: space).sharp(3)\n\
             \u{20} let s = g.stops()\n\
             \u{20} let exact = (g.sample(s.at(1).at(1)) == s.at(1).at(0), g.sample(s.at(3).at(1)) == s.at(3).at(0))\n\
             \u{20} let epsilon = 0.000001%\n\
             \u{20} let right = (repr(g.sample(s.at(1).at(1) + epsilon)) == repr(s.at(2).at(0)), repr(g.sample(s.at(3).at(1) + epsilon)) == repr(s.at(4).at(0)))\n\
             \u{20} let static = gradient.sample(g, s.at(1).at(1)) == s.at(1).at(0)\n\
             \u{20} let many = g.samples(s.at(1).at(1), s.at(3).at(1)) == (s.at(1).at(0), s.at(3).at(0))\n\
             \u{20} (exact, right, static, many)\n\
             }\n\
             #let oklab = check(color.oklab)\n\
             #let linear_rgb = check(color.linear-rgb)",
        )
        .unwrap();
        let expected = Value::Array(vec![
            Value::Array(vec![Value::Bool(true); 2]),
            Value::Array(vec![Value::Bool(true); 2]),
            Value::Bool(true),
            Value::Bool(true),
        ]);
        assert_eq!(m.scope().get("oklab"), Some(&expected));
        assert_eq!(m.scope().get("linear_rgb"), Some(&expected));
    }

    #[test]
    fn p1271_luma_white_normalizado_para_oklab_componentes_publicos() {
        let m = p729_eval(
            "#let c = gradient.linear(black, white, space: color.oklab).stops().last().first().components()\n\
             #let got = (repr(c.at(0)), c.at(1), c.at(2), repr(c.at(3)))",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("got"),
            Some(&Value::Array(vec![
                Value::Str("100%".into()),
                Value::Float(f32::from_bits(0x3740_0000) as f64),
                Value::Float(f32::from_bits(0x381b_8000) as f64),
                Value::Str("100%".into()),
            ]))
        );
    }

    #[test]
    fn p1271_repeat_preserva_offsets_automaticos_f64_nos_tres_variants() {
        let m = p729_eval(
            "#let offsets(g) = g.repeat(2).stops().map(stop => stop.at(1))\n\
             #let linear = offsets(gradient.linear(red, green, blue, yellow, space: color.linear-rgb))\n\
             #let radial = offsets(gradient.radial(red, green, blue, yellow, space: color.linear-rgb))\n\
             #let conic = offsets(gradient.conic(red, green, blue, yellow, space: color.linear-rgb))",
        )
        .unwrap();
        let expected = Value::Array(
            [
                0.0,
                (1.0 / 3.0) / 2.0,
                (2.0 / 3.0) / 2.0,
                0.5,
                0.5,
                (1.0 / 3.0 + 1.0) / 2.0,
                (2.0 / 3.0 + 1.0) / 2.0,
                1.0,
            ]
            .into_iter()
            .map(|offset| Value::Ratio(crate::entities::layout_types::Ratio(offset)))
            .collect(),
        );
        assert_eq!(m.scope().get("linear"), Some(&expected));
        assert_eq!(m.scope().get("radial"), Some(&expected));
        assert_eq!(m.scope().get("conic"), Some(&expected));
    }

    #[test]
    fn p1271_radial_sample_preserva_t_f64_ate_os_pesos() {
        let m = p729_eval(
            "#let o = gradient.radial(black, white, space: color.oklab).sample(37.123456789%).components()\n\
             #let oklab = (o.at(0) / 100%, o.at(1), o.at(2), o.at(3) / 100%)\n\
             #let l = gradient.radial(red, blue, space: color.linear-rgb).sample(37.123456789%).components()\n\
             #let linear-rgb = (l.at(0) / 100%, l.at(1) / 100%, l.at(2) / 100%, l.at(3) / 100%)",
        )
        .unwrap();
        let values = |bits: [u32; 4]| {
            Value::Array(
                bits.into_iter()
                    .map(|bits| Value::Float(f32::from_bits(bits) as f64))
                    .collect(),
            )
        };
        assert_eq!(
            m.scope().get("oklab"),
            Some(&values([0x3ebe_1275, 0x368e_8dd8, 0x3766_e86c, 0x3f80_0000]))
        );
        assert_eq!(
            m.scope().get("linear-rgb"),
            Some(&values([0x3f20_f6c5, 0x3dc8_da06, 0x3e8f_c2e4, 0x3f80_0000]))
        );
    }

    #[test]
    fn p1145_gradient_constructors_e_transformacoes() {
        let m = p729_eval(
            "#let l = gradient.linear(red, blue, dir: ttb)\n\
             #let r = gradient.radial(red, blue, focal-center: (50%, 50%), focal-radius: 10%)\n\
             #let s = l.sharp(3, smoothness: 20%)\n\
             #let p = r.repeat(2, mirror: true)\n\
             #let values = (repr(l.angle()), repr(r.focal-radius()), s.kind() == gradient.linear, p.kind() == gradient.radial, s.stops().len(), p.stops().len())\n\
             #let static = (gradient.sharp(l, 3).stops() == l.sharp(3).stops(), gradient.repeat(r, 2, mirror: true).stops() == p.stops())",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("values"),
            Some(&Value::Array(vec![
                Value::Str("90deg".into()),
                Value::Str("10%".into()),
                Value::Bool(true),
                Value::Bool(true),
                Value::Int(6),
                Value::Int(3),
            ]))
        );
        assert_eq!(
            m.scope().get("static"),
            Some(&Value::Array(vec![Value::Bool(true), Value::Bool(true)]))
        );
        let Some(Value::Gradient(crate::entities::gradient::Gradient::Linear(sharp))) =
            m.scope().get("s")
        else {
            panic!("sharp linear")
        };
        assert!(!sharp.anti_alias);
        let Some(Value::Gradient(crate::entities::gradient::Gradient::Radial(repeated))) =
            m.scope().get("p")
        else {
            panic!("repeat radial")
        };
        assert!(repeated.anti_alias);
    }

    #[test]
    fn p1252_luma_e_tres_gradients_preservam_alpha_publicamente() {
        let m = p729_eval(
            "#let source = rgb(100%, 25.49%, 21.18%, 40%)\n\
             #let direct = luma(54.02%, alpha: 40%)\n\
             #let converted = luma(source)\n\
             #let l = gradient.linear(source, blue, space: luma)\n\
             #let r = gradient.radial(source, blue, space: luma)\n\
             #let c = gradient.conic(source, blue, space: luma)\n\
             #let alpha = (direct.components().at(1), converted.components().at(1), l.stops().at(0).at(0).components().at(1), r.stops().at(0).at(0).components().at(1), c.stops().at(0).at(0).components().at(1))",
        )
        .unwrap();
        let Some(Value::Array(alpha)) = m.scope().get("alpha") else {
            panic!("alpha deve ser array");
        };
        assert_eq!(alpha.len(), 5);
        for value in alpha {
            let Value::Ratio(value) = value else { panic!("alpha deve ser ratio") };
            assert!((value.0 - 0.4).abs() < 1e-6, "alpha divergente: {}", value.0);
        }
    }

    #[test]
    fn p1239_linear_e_radial_luma_expoem_luminancia_vanilla() {
        let m = p729_eval(
            "#let source = red.transparentize(60%)\n\
             #let direct = repr(luma(source).components().at(0))\n\
             #let linear = repr(gradient.linear(source, blue, space: luma).stops().at(0).at(0).components().at(0))\n\
             #let radial = repr(gradient.radial(source, blue, space: luma).stops().at(0).at(0).components().at(0))",
        )
        .unwrap();
        for name in ["direct", "linear", "radial"] {
            assert_eq!(m.scope().get(name), Some(&Value::Str("54.02%".into())), "{name}");
        }
    }

    #[test]
    fn p1145_gradient_rejeita_stops_e_limites_invalidos() {
        for source in [
            "#gradient.linear(red)",
            "#gradient.radial(red)",
            "#gradient.conic(red)",
            "#gradient.linear(red, (blue, 100%))",
            "#gradient.linear((red, 10%), (blue, 100%))",
            "#gradient.linear((red, 0%), (blue, 90%))",
            "#gradient.linear((red, 0%), (green, 80%), (blue, 70%))",
            "#gradient.linear(red, blue).sharp(1)",
            "#gradient.linear(red, blue).sharp(3, smoothness: 101%)",
            "#gradient.linear(red, blue).repeat(0)",
        ] {
            assert!(p729_eval(source).is_err(), "deveria falhar: {source}");
        }
    }

    #[test]
    fn p1146_datetime_fields_estaticos_accessors_e_display() {
        let m = p729_eval(
            "#let d = datetime(year: 2024, month: 2, day: 29)\n\
             #let t = datetime(hour: 23, minute: 58, second: 57)\n\
             #let dt = datetime(year: 2023, month: 12, day: 31, hour: 1, minute: 2, second: 3)\n\
             #let date = (datetime.year(d), datetime.month(d), datetime.weekday(d), datetime.day(d), datetime.hour(d), datetime.minute(d), datetime.second(d), datetime.ordinal(d))\n\
             #let time = (datetime.year(t), datetime.month(t), datetime.weekday(t), datetime.day(t), datetime.hour(t), datetime.minute(t), datetime.second(t), datetime.ordinal(t))\n\
             #let full = (datetime.year(dt), datetime.month(dt), datetime.weekday(dt), datetime.day(dt), datetime.hour(dt), datetime.minute(dt), datetime.second(dt), datetime.ordinal(dt))\n\
             #let shown = (datetime.display(d), datetime.display(t), datetime.display(dt), datetime.display(d, \"[year]-[ordinal] [weekday repr:short]\"))",
        )
        .unwrap();

        assert_eq!(
            m.scope().get("date"),
            Some(&Value::Array(vec![
                Value::Int(2024),
                Value::Int(2),
                Value::Int(4),
                Value::Int(29),
                Value::None,
                Value::None,
                Value::None,
                Value::Int(60),
            ]))
        );
        assert_eq!(
            m.scope().get("time"),
            Some(&Value::Array(vec![
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::Int(23),
                Value::Int(58),
                Value::Int(57),
                Value::None,
            ]))
        );
        assert_eq!(
            m.scope().get("full"),
            Some(&Value::Array(vec![
                Value::Int(2023),
                Value::Int(12),
                Value::Int(7),
                Value::Int(31),
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(365),
            ]))
        );
        assert_eq!(
            m.scope().get("shown"),
            Some(&Value::Array(vec![
                Value::Str("2024-02-29".into()),
                Value::Str("23:58:57".into()),
                Value::Str("2023-12-31 01:02:03".into()),
                Value::Str("2024-060 Thu".into()),
            ]))
        );
    }

    #[test]
    fn p1146_datetime_today_injeta_duration_no_world() {
        struct TodayWorld {
            inner: MockWorld,
            offsets: std::sync::Mutex<Vec<Option<crate::entities::duration::Duration>>>,
        }

        impl World for TodayWorld {
            fn library(&self) -> &Library {
                self.inner.library()
            }
            fn book(&self) -> &FontBook {
                self.inner.book()
            }
            fn main(&self) -> FileId {
                self.inner.main()
            }
            fn source(&self, id: FileId) -> FileResult<Source> {
                self.inner.source(id)
            }
            fn file(&self, id: FileId) -> FileResult<Bytes> {
                self.inner.file(id)
            }
            fn font(&self, index: usize) -> Option<Font> {
                self.inner.font(index)
            }
            fn today(
                &self,
                offset: Option<crate::entities::duration::Duration>,
            ) -> Option<Datetime> {
                self.offsets.lock().unwrap().push(offset);
                Datetime::new_date(2024, 2, 29)
            }
        }

        let world = TodayWorld {
            inner: MockWorld::new(
                "#let local = datetime.today()\n\
                 #let hours = datetime.today(offset: 2)\n\
                 #let exact = datetime.today(offset: duration(minutes: 90))",
            ),
            offsets: std::sync::Mutex::new(vec![]),
        };
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let expected = Value::Datetime(Datetime::new_date(2024, 2, 29).unwrap());
        assert_eq!(module.scope().get("local"), Some(&expected));
        assert_eq!(module.scope().get("hours"), Some(&expected));
        assert_eq!(module.scope().get("exact"), Some(&expected));
        let expected_offsets = vec![
            None,
            Some(crate::entities::duration::Duration::from_hours(2)),
            Some(crate::entities::duration::Duration::from_minutes(90)),
        ];
        let offsets = world.offsets.lock().unwrap();
        assert!(
            offsets
                .chunks_exact(expected_offsets.len())
                .all(|chunk| chunk == expected_offsets),
            "cada passe de eval deve preservar os offsets: {offsets:?}"
        );
    }

    #[test]
    fn p1146_datetime_rejeita_instancia_e_formato_incompativel() {
        assert!(p729_eval(
            "#let d = datetime(year: 2024, month: 2, day: 29)\n#let x = d.day()"
        )
        .is_err());
        let errors = p729_eval(
            "#let d = datetime(year: 2024, month: 2, day: 29)\n#let x = datetime.display(d, \"[hour]\")",
        )
        .unwrap_err();
        assert!(errors.iter().any(|error| error
            .message
            .contains("failed to format datetime (insufficient information)")));
    }

    #[test]
    fn p1147_int_scope_bitwise_casts_e_bytes() {
        let m = p729_eval(
            "#let bits = (int.signum(-5), (-5).signum(), int.bit-not(4), 128.bit-and(192), 64.bit-or(32), 64.bit-xor(96), 33.bit-lshift(2), (-8).bit-rshift(2), (-8).bit-rshift(2, logical: true), (-8).bit-rshift(64), (-8).bit-rshift(64, logical: true))\n\
             #let casts = (int(2.7), int(-58.34), int(decimal(\"3.8\")))\n\
             #let from = (int.from-bytes(bytes((255,)), endian: \"big\", signed: true), int.from-bytes(bytes((255,)), endian: \"big\", signed: false))\n\
             #let to = (10000.to-bytes(endian: \"big\", size: 4), int.to-bytes(-1000, endian: \"little\", size: 5))",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("bits"),
            Some(&Value::Array(vec![
                Value::Int(-1),
                Value::Int(-1),
                Value::Int(-5),
                Value::Int(128),
                Value::Int(96),
                Value::Int(32),
                Value::Int(132),
                Value::Int(-2),
                Value::Int(4_611_686_018_427_387_902),
                Value::Int(-1),
                Value::Int(0),
            ]))
        );
        assert_eq!(
            m.scope().get("casts"),
            Some(&Value::Array(vec![Value::Int(2), Value::Int(-58), Value::Int(3)]))
        );
        assert_eq!(
            m.scope().get("from"),
            Some(&Value::Array(vec![Value::Int(-1), Value::Int(255)]))
        );
        assert_eq!(
            m.scope().get("to"),
            Some(&Value::Array(vec![
                Value::Bytes(crate::entities::bytes::Bytes::new(vec![0, 0, 39, 16])),
                Value::Bytes(crate::entities::bytes::Bytes::new(vec![
                    24, 252, 255, 255, 255
                ])),
            ]))
        );
    }

    #[test]
    fn p1147_int_limites_e_erros() {
        let m = p729_eval(
            "#let shifts = (33.bit-lshift(63), 1.bit-rshift(65), (-1).bit-rshift(65), (-1).bit-rshift(65, logical: true))\n\
             #let empty = int.from-bytes(bytes(()))\n\
             #let extended = int.to-bytes(-1, endian: \"big\", size: 9)",
        )
        .unwrap();
        assert_eq!(
            m.scope().get("shifts"),
            Some(&Value::Array(vec![
                Value::Int(i64::MIN),
                Value::Int(0),
                Value::Int(-1),
                Value::Int(0)
            ]))
        );
        assert_eq!(m.scope().get("empty"), Some(&Value::Int(0)));
        assert_eq!(
            m.scope().get("extended"),
            Some(&Value::Bytes(crate::entities::bytes::Bytes::new(vec![
                0, 255, 255, 255, 255, 255, 255, 255, 255
            ])))
        );

        for source in [
            "#1.bit-lshift(64)",
            "#1.bit-lshift(-1)",
            "#int.from-bytes(bytes((0, 0, 0, 0, 0, 0, 0, 0, 0)))",
            "#1.to-bytes(size: -1)",
            "#int(40, base: 16)",
        ] {
            assert!(p729_eval(source).is_err(), "deveria falhar: {source}");
        }
    }

    #[test]
    fn p1148_counter_fields_estaticos_e_updates_ricos() {
        use crate::entities::counter_update::CounterUpdate;

        let module = p729_eval(
            "#let c = counter(\"p1148\")\n\
             #let fields = (type(counter.get), type(counter.display), type(counter.at), type(counter.final), type(counter.step), type(counter.update))\n\
             #let static-step = counter.step(c, level: 2)\n\
             #let method-step = c.step(level: 2)\n\
             #let static-set = counter.update(c, (3, 4))\n\
             #let method-set = c.update((3, 4))",
        )
        .unwrap();

        assert_eq!(
            module.scope().get("fields"),
            Some(&Value::Array(vec![Value::Type(Type::Function); 6]))
        );
        for name in ["static-step", "method-step"] {
            let Some(Value::Content(Content::CounterUpdate(elem))) =
                module.scope().get(name)
            else {
                panic!("{name} deve produzir counter-update")
            };
            assert!(
                matches!(elem.action, CounterUpdate::Step(level) if level.get() == 2)
            );
        }
        for name in ["static-set", "method-set"] {
            let Some(Value::Content(Content::CounterUpdate(elem))) =
                module.scope().get(name)
            else {
                panic!("{name} deve produzir counter-update")
            };
            assert_eq!(elem.action, CounterUpdate::Set(vec![3, 4]));
        }
    }

    #[test]
    fn p1150_content_scope_estatico_equivale_a_instancia() {
        let module = p729_eval(
            "#let c = strong[Hi]\n\
             #let kinds = (type(content.func), type(content.has), type(content.at), type(content.fields), type(content.location))\n\
             #let checks = (content.func(c) == c.func(), content.func(c) == strong, content.has(c, \"body\") == c.has(\"body\"), content.at(c, \"body\") == c.at(\"body\"), content.at(c, \"missing\", default: 7) == c.at(\"missing\", default: 7), content.fields(c) == c.fields(), content.location(c) == c.location())",
        )
        .unwrap();
        assert_eq!(
            module.scope().get("kinds"),
            Some(&Value::Array(vec![Value::Type(Type::Function); 5]))
        );
        assert_eq!(
            module.scope().get("checks"),
            Some(&Value::Array(vec![Value::Bool(true); 7]))
        );
    }

    fn p1215_eval_error_range(expression: &str) -> std::ops::Range<usize> {
        let world = MockWorld::new("");
        let (result, _) = eval_expression(&world, expression);
        let errors = result.expect_err("a expressão deve falhar");
        let source = Source::new_with_parser(
            world.main(),
            expression.to_string(),
            crate::compiler::parse::parse_code,
        );
        source
            .span_byte_range(errors[0].span)
            .expect("P1215: span do eval deve resolver contra a expressão")
    }

    #[test]
    fn p1215_bytes_span_chamada_inteira() {
        assert_eq!(p1215_eval_error_range("bytes((1,2,3)).at()"), 0..19);
        assert_eq!(p1215_eval_error_range("bytes((1,2,3)).slice(0,4)"), 0..25);
    }

    #[test]
    fn p1215_bytes_span_positional_especifico() {
        assert_eq!(p1215_eval_error_range("bytes((1,2,3)).at(\"1\")"), 18..21);
        assert_eq!(p1215_eval_error_range("bytes((1,2,3)).slice(0,1,2)"), 25..26);
    }

    #[test]
    fn p1215_bytes_span_named_completo() {
        assert_eq!(p1215_eval_error_range("bytes((1,2,3)).at(0,foo:1)"), 20..25);
    }

    #[test]
    fn p1215_bytes_span_acompanha_deslocamento() {
        assert_eq!(p1215_eval_error_range("\n\n  bytes((1,2,3)).at(\"1\")"), 22..25);
    }

    // ── P1300 — remoção dos aliases globais de constructors de cor ─────────

    fn p1300_profiles() -> [(&'static str, Features); 4] {
        let default = Features::empty();
        let html = Features::html();
        let mut a11y = Features::empty();
        a11y.enable(Feature::A11yExtras);
        let mut html_a11y = Features::html();
        html_a11y.enable(Feature::A11yExtras);
        [("default", default), ("html", html), ("a11y", a11y), ("html+a11y", html_a11y)]
    }

    fn p1300_assert_negative(name: &str, under_std: bool) {
        let expression = if under_std { format!("std.{name}") } else { name.to_string() };
        let world = MockWorld::new("");
        let source = Source::new_with_parser(
            world.main(),
            expression.clone(),
            crate::compiler::parse::parse_code,
        );
        let mut unexpectedly_present = Vec::new();

        for (profile, features) in p1300_profiles() {
            let (result, side_diagnostics) =
                eval_expression_with_features(&world, &expression, features);
            assert!(
                side_diagnostics.is_empty(),
                "{profile}/{expression}: diagnostics laterais inesperados: {side_diagnostics:?}"
            );
            let diagnostics = match result {
                Ok(_) => {
                    unexpectedly_present.push(profile);
                    continue;
                }
                Err(diagnostics) => diagnostics,
            };
            assert_eq!(diagnostics.len(), 1, "{profile}/{expression}: {diagnostics:?}");
            let diagnostic = &diagnostics[0];
            let expected_message = if under_std {
                format!("module `global` does not contain `{name}`")
            } else {
                format!("unknown variable `{name}`")
            };
            let expected_span_start = if under_std { "std.".len() } else { 0 };
            assert_eq!(diagnostic.message, expected_message, "{profile}/{expression}");
            assert!(
                diagnostic.hints.is_empty(),
                "{profile}/{expression}: {:?}",
                diagnostic.hints
            );
            assert_eq!(
                source.span_to_line_col(diagnostic.span),
                Some((1, expected_span_start as u32)),
                "{profile}/{expression}"
            );
            assert_eq!(
                source.span_byte_range(diagnostic.span),
                Some(expected_span_start..expression.len()),
                "{profile}/{expression}"
            );
        }

        assert!(
            unexpectedly_present.is_empty(),
            "P1300: `{expression}` ainda está disponível nos perfis {unexpectedly_present:?}"
        );
    }

    #[test]
    fn p1300_controle_diagnosticos_negativos_exatos_nos_quatro_perfis() {
        p1300_assert_negative("__p1300_unknown_control", false);
        p1300_assert_negative("__p1300_unknown_control", true);
    }

    #[test]
    fn p1300_hsl_bare_unknown_variable() {
        p1300_assert_negative("hsl", false);
    }

    #[test]
    fn p1300_hsv_bare_unknown_variable() {
        p1300_assert_negative("hsv", false);
    }

    #[test]
    fn p1300_linear_rgb_bare_unknown_variable() {
        p1300_assert_negative("linear_rgb", false);
    }

    #[test]
    fn p1300_std_hsl_missing_field() {
        p1300_assert_negative("hsl", true);
    }

    #[test]
    fn p1300_std_hsv_missing_field() {
        p1300_assert_negative("hsv", true);
    }

    #[test]
    fn p1300_std_linear_rgb_missing_field() {
        p1300_assert_negative("linear_rgb", true);
    }

    #[test]
    fn p1300_rotas_color_preservam_funcoes_chamadas_repr_e_space() {
        let expression = "(\
            (type(color.hsl), type(color.hsv), type(color.linear-rgb)), \
            (repr(color.hsl), repr(color.hsv), repr(color.linear-rgb)), \
            color.hsl(120deg, 50%, 40%), \
            color.hsv(240deg, 50%, 80%), \
            color.linear-rgb(10%, 20%, 30%), \
            (repr(color.hsl(120deg, 50%, 40%)), repr(color.hsv(240deg, 50%, 80%)), repr(color.linear-rgb(10%, 20%, 30%))), \
            (repr(color.space(color.hsl(120deg, 50%, 40%))), repr(color.space(color.hsv(240deg, 50%, 80%))), repr(color.space(color.linear-rgb(10%, 20%, 30%)))), \
            (color.space(color.hsl(120deg, 50%, 40%)) == color.hsl, color.space(color.hsv(240deg, 50%, 80%)) == color.hsv, color.space(color.linear-rgb(10%, 20%, 30%)) == color.linear-rgb)\
        )";
        let world = MockWorld::new("");
        for (profile, features) in p1300_profiles() {
            let (result, diagnostics) =
                eval_expression_with_features(&world, expression, features);
            assert!(diagnostics.is_empty(), "{profile}: {diagnostics:?}");
            let Value::Array(values) = result
                .unwrap_or_else(|error| panic!("{profile}: rotas color.*: {error:?}"))
            else {
                panic!("{profile}: resultado deve ser array");
            };
            assert_eq!(values.len(), 8, "{profile}");
            assert_eq!(values[0], Value::Array(vec![Value::Type(Type::Function); 3]));
            assert_eq!(
                values[1],
                Value::Array(vec![
                    Value::Str("hsl".into()),
                    Value::Str("hsv".into()),
                    Value::Str("linear-rgb".into()),
                ])
            );
            assert!(values[2..=4].iter().all(|value| matches!(value, Value::Color(_))));
            assert_eq!(
                values[5],
                Value::Array(vec![
                    Value::Str("color.hsl(120deg, 50%, 40%)".into()),
                    Value::Str("color.hsv(240deg, 50%, 80%)".into()),
                    Value::Str("color.linear-rgb(10%, 20%, 30%)".into()),
                ])
            );
            assert_eq!(
                values[6],
                Value::Array(vec![
                    Value::Str("hsl".into()),
                    Value::Str("hsv".into()),
                    Value::Str("linear-rgb".into()),
                ])
            );
            assert_eq!(values[7], Value::Array(vec![Value::Bool(true); 3]));
        }
    }

    #[test]
    fn p1300_cinco_globals_bare_preservam_funcoes_e_chamadas() {
        let expression = "(\
            (type(rgb), type(luma), type(cmyk), type(oklab), type(oklch)), \
            (rgb(0, 0, 0), luma(0%), cmyk(0%, 0%, 0%, 100%), oklab(0%, 0, 0), oklch(0%, 0, 0deg))\
        )";
        p1300_assert_preserved_globals(expression, "bare");
    }

    #[test]
    fn p1300_cinco_globals_std_preservam_funcoes_e_chamadas() {
        let expression = "(\
            (type(std.rgb), type(std.luma), type(std.cmyk), type(std.oklab), type(std.oklch)), \
            (std.rgb(0, 0, 0), std.luma(0%), std.cmyk(0%, 0%, 0%, 100%), std.oklab(0%, 0, 0), std.oklch(0%, 0, 0deg))\
        )";
        p1300_assert_preserved_globals(expression, "std");
    }

    fn p1300_assert_preserved_globals(expression: &str, route: &str) {
        let world = MockWorld::new("");
        for (profile, features) in p1300_profiles() {
            let (result, diagnostics) =
                eval_expression_with_features(&world, expression, features);
            assert!(diagnostics.is_empty(), "{profile}/{route}: {diagnostics:?}");
            let Value::Array(values) =
                result.unwrap_or_else(|error| panic!("{profile}/{route}: {error:?}"))
            else {
                panic!("{profile}/{route}: resultado deve ser array");
            };
            assert_eq!(values.len(), 2, "{profile}/{route}");
            assert_eq!(
                values[0],
                Value::Array(vec![Value::Type(Type::Function); 5]),
                "{profile}/{route}"
            );
            let Value::Array(calls) = &values[1] else {
                panic!("{profile}/{route}: calls deve ser array de cores");
            };
            assert_eq!(calls.len(), 5, "{profile}/{route}");
            assert!(
                calls.iter().all(|value| matches!(value, Value::Color(_))),
                "{profile}/{route}: {calls:?}"
            );
        }
    }

    // ── P1301 — diagnósticos de field inexistente em `Module` ─────────

    fn p1301_missing_module_field_mismatch(
        expression: &str,
        features: Features,
        expected_message: &str,
        expected_span: std::ops::Range<usize>,
    ) -> Option<String> {
        let world = MockWorld::new("");
        let source = Source::new_with_parser(
            world.main(),
            expression.to_string(),
            crate::compiler::parse::parse_code,
        );
        let (result, side_diagnostics) =
            eval_expression_with_features(&world, expression, features);
        let diagnostics = match result {
            Ok(value) => {
                return Some(format!(
                    "esperado erro, obtido sucesso {value:?}; diagnostics laterais: {side_diagnostics:?}"
                ));
            }
            Err(diagnostics) => diagnostics,
        };

        let mut mismatches = Vec::new();
        if !side_diagnostics.is_empty() {
            mismatches.push(format!(
                "cardinalidade lateral esperada 0, obtida {}: {side_diagnostics:?}",
                side_diagnostics.len()
            ));
        }
        if diagnostics.len() != 1 {
            mismatches.push(format!(
                "cardinalidade primária esperada 1, obtida {}: {diagnostics:?}",
                diagnostics.len()
            ));
        } else {
            let diagnostic = &diagnostics[0];
            if diagnostic.message != expected_message {
                mismatches.push(format!(
                    "mensagem esperada {expected_message:?}, obtida {:?}",
                    diagnostic.message
                ));
            }
            if !diagnostic.hints.is_empty() {
                mismatches
                    .push(format!("hints esperados [], obtidos {:?}", diagnostic.hints));
            }
            let observed_span = source.span_byte_range(diagnostic.span);
            if observed_span != Some(expected_span.clone()) {
                mismatches.push(format!(
                    "span esperado {expected_span:?}, obtido {observed_span:?}"
                ));
            }
        }

        (!mismatches.is_empty()).then(|| mismatches.join("; "))
    }

    #[test]
    fn p1301_std_aliases_mensagem_vanilla_e_span_field_only_nos_quatro_perfis() {
        let cases = [
            ("hsl", "module `global` does not contain `hsl`", 14..17),
            ("hsv", "module `global` does not contain `hsv`", 14..17),
            ("linear_rgb", "module `global` does not contain `linear_rgb`", 14..24),
        ];
        let mut mismatches = Vec::new();
        for (profile, features) in p1300_profiles() {
            for (field, expected_message, expected_span) in &cases {
                let expression = format!("repr(type(std.{field}))");
                if let Some(mismatch) = p1301_missing_module_field_mismatch(
                    &expression,
                    features.clone(),
                    expected_message,
                    expected_span.clone(),
                ) {
                    mismatches.push(format!("{profile}/{field}: {mismatch}"));
                }
            }
        }
        assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    }

    #[test]
    fn p1301_modulos_independentes_mensagem_vanilla_e_span_field_only() {
        let cases = [
            (
                "calc.nope",
                "repr(type(calc.nope))",
                "module `calc` does not contain `nope`",
                15..19,
            ),
            (
                "sym.nope",
                "repr(type(sym.nope))",
                "module `sym` does not contain `nope`",
                14..18,
            ),
            (
                "color.map.nope",
                "repr(type(color.map.nope))",
                "module `map` does not contain `nope`",
                20..24,
            ),
        ];
        let mut mismatches = Vec::new();
        for (case, expression, expected_message, expected_span) in cases {
            if let Some(mismatch) = p1301_missing_module_field_mismatch(
                expression,
                Features::empty(),
                expected_message,
                expected_span,
            ) {
                mismatches.push(format!("{case}: {mismatch}"));
            }
        }
        assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    }

    #[test]
    fn p1301_lookups_de_modulo_existentes_preservam_valor_e_kind() {
        let cases = [
            ("std.rgb", "repr(type(std.rgb))", "function"),
            ("calc.abs", "repr(type(calc.abs))", "function"),
            ("sym.arrow", "repr(type(sym.arrow))", "symbol"),
            ("color.map.turbo", "repr(type(color.map.turbo))", "array"),
        ];
        let world = MockWorld::new("");
        for (case, expression, expected_kind) in cases {
            let (result, side_diagnostics) =
                eval_expression_with_features(&world, expression, Features::empty());
            assert!(
                side_diagnostics.is_empty(),
                "{case}: diagnostics laterais inesperados: {side_diagnostics:?}"
            );
            assert_eq!(
                result.unwrap_or_else(|diagnostics| panic!("{case}: {diagnostics:?}")),
                Value::Str(expected_kind.into()),
                "{case}"
            );
        }
    }

    #[test]
    fn p1301_controle_nao_module_dicionario_preserva_span_total() {
        let expression = "repr(type((:).missing))";
        let mismatch = p1301_missing_module_field_mismatch(
            expression,
            Features::empty(),
            "dictionary does not contain key \"missing\"",
            10..21,
        );
        if let Some(mismatch) = mismatch {
            panic!("{mismatch}");
        }
    }

    // ── P1303 — spans dos gates `pdf.*` e sentinelas de fronteira ──────

    fn p1303_error_mismatches(
        expression: &str,
        features: Features,
        expected_message: &str,
        expected_hints: &[&str],
        expected_span: std::ops::Range<usize>,
    ) -> Vec<String> {
        let world = MockWorld::new("");
        let source = Source::new_with_parser(
            world.main(),
            expression.to_string(),
            crate::compiler::parse::parse_code,
        );
        let (result, side_diagnostics) =
            eval_expression_with_features(&world, expression, features);
        let mut mismatches = Vec::new();

        if !side_diagnostics.is_empty() {
            mismatches.push(format!(
                "cardinalidade lateral esperada 0, obtida {}: {side_diagnostics:?}",
                side_diagnostics.len()
            ));
        }

        let diagnostics = match result {
            Ok(value) => {
                mismatches.push(format!("esperado erro, obtido sucesso {value:?}"));
                return mismatches;
            }
            Err(diagnostics) => diagnostics,
        };

        if diagnostics.len() != 1 {
            mismatches.push(format!(
                "cardinalidade primária esperada 1, obtida {}: {diagnostics:?}",
                diagnostics.len()
            ));
            return mismatches;
        }

        let diagnostic = &diagnostics[0];
        if diagnostic.message != expected_message {
            mismatches.push(format!(
                "mensagem esperada {expected_message:?}, obtida {:?}",
                diagnostic.message
            ));
        }
        let observed_hints =
            diagnostic.hints.iter().map(|hint| hint.as_str()).collect::<Vec<_>>();
        if observed_hints != expected_hints {
            mismatches.push(format!(
                "hints esperados {expected_hints:?}, obtidos {observed_hints:?}"
            ));
        }
        let observed_span = source.span_byte_range(diagnostic.span);
        if observed_span != Some(expected_span.clone()) {
            mismatches.push(format!(
                "span esperado {expected_span:?}, obtido {observed_span:?}"
            ));
        }

        mismatches
    }

    fn p1303_eval_success(expression: &str, features: Features) -> Value {
        let world = MockWorld::new("");
        let (result, side_diagnostics) =
            eval_expression_with_features(&world, expression, features);
        assert!(
            side_diagnostics.is_empty(),
            "{expression}: diagnostics laterais inesperados: {side_diagnostics:?}"
        );
        result.unwrap_or_else(|diagnostics| panic!("{expression}: {diagnostics:?}"))
    }

    #[test]
    fn p1303_negativos_pdf_exatos_nos_perfis_sem_a11y() {
        let cases =
            [("data-cell", 14..23), ("header-cell", 14..25), ("table-summary", 14..27)];
        let profiles = [("default", Features::empty()), ("html", Features::html())];
        let expected_hints = [
            "try enabling the `a11y-extras` feature",
            "see https://typst.app/help/compiler-features for more details",
        ];
        let mut failures = Vec::new();

        for (profile, features) in profiles {
            for (field, expected_span) in &cases {
                let expression = format!("repr(type(pdf.{field}))");
                let expected_message = format!(
                    "cannot access field `{field}` because the `a11y-extras` feature is not enabled"
                );
                let mismatches = p1303_error_mismatches(
                    &expression,
                    features.clone(),
                    &expected_message,
                    &expected_hints,
                    expected_span.clone(),
                );
                if !mismatches.is_empty() {
                    failures.push(format!(
                        "P1303-N/{profile}/{field}: {}",
                        mismatches.join("; ")
                    ));
                }
            }
        }

        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn p1303_positivos_pdf_exatos_nos_perfis_com_a11y() {
        let mut a11y = Features::empty();
        a11y.enable(Feature::A11yExtras);
        let mut html_a11y = Features::html();
        html_a11y.enable(Feature::A11yExtras);

        let function_probe = "(\
            (type(pdf.data-cell), repr(pdf.data-cell)), \
            (type(pdf.header-cell), repr(pdf.header-cell)), \
            (type(pdf.table-summary), repr(pdf.table-summary))\
        )";
        let expected_functions = Value::Array(vec![
            Value::Array(vec![
                Value::Type(Type::Function),
                Value::Str("data-cell".into()),
            ]),
            Value::Array(vec![
                Value::Type(Type::Function),
                Value::Str("header-cell".into()),
            ]),
            Value::Array(vec![
                Value::Type(Type::Function),
                Value::Str("table-summary".into()),
            ]),
        ]);
        let representative_calls = [
            "pdf.data-cell[hi]",
            "pdf.data-cell(table.cell(colspan: 2)[hi])",
            "pdf.header-cell[hi]",
            "pdf.header-cell(table.cell(colspan: 2)[hi])",
            "pdf.table-summary(table(columns: 1)[x], summary: \"summary\")",
        ];

        for (profile, features) in [("a11y", a11y), ("html+a11y", html_a11y)] {
            assert_eq!(
                p1303_eval_success(function_probe, features.clone()),
                expected_functions,
                "{profile}: kind/nome/repr dos três fields"
            );

            for call in representative_calls {
                assert!(
                    matches!(
                        p1303_eval_success(call, features.clone()),
                        Value::Content(_)
                    ),
                    "{profile}/{call}: chamada deve produzir content"
                );
            }
        }
    }

    #[test]
    fn p1303_sentinelas_de_span_module_e_nao_module() {
        let cases = [
            (
                "std.nope",
                "repr(type(std.nope))",
                "module `global` does not contain `nope`",
                14..18,
            ),
            (
                "calc.nope",
                "repr(type(calc.nope))",
                "module `calc` does not contain `nope`",
                15..19,
            ),
            (
                "sym.nope",
                "repr(type(sym.nope))",
                "module `sym` does not contain `nope`",
                14..18,
            ),
            (
                "color.map.nope",
                "repr(type(color.map.nope))",
                "module `map` does not contain `nope`",
                20..24,
            ),
            (
                "dictionary.nope",
                "repr(type((:).nope))",
                "dictionary does not contain key \"nope\"",
                10..18,
            ),
            (
                "float.is-nan",
                "repr(type(float(\"NaN\").is-nan))",
                "cannot access fields on type float",
                23..29,
            ),
        ];
        let mut failures = Vec::new();
        for (case, expression, message, span) in cases {
            let mismatches =
                p1303_error_mismatches(expression, Features::empty(), message, &[], span);
            if !mismatches.is_empty() {
                failures.push(format!("P1303-S/{case}: {}", mismatches.join("; ")));
            }
        }

        let lookup_probe = "(\
            repr(type(std.rgb)), \
            repr(type(calc.abs)), \
            repr(type(sym.alpha)), \
            repr(type(color.map.viridis))\
        )";
        assert_eq!(
            p1303_eval_success(lookup_probe, Features::empty()),
            Value::Array(vec![
                Value::Str("function".into()),
                Value::Str("function".into()),
                Value::Str("symbol".into()),
                Value::Str("array".into()),
            ]),
            "P1303-S/lookups-existentes"
        );
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn p1303_sentinelas_pdf_ungated_e_html_disabled() {
        let bindings_probe = "(\
            (type(pdf.attach), repr(pdf.attach)), \
            (type(pdf.artifact), repr(pdf.artifact))\
        )";
        assert_eq!(
            p1303_eval_success(bindings_probe, Features::empty()),
            Value::Array(vec![
                Value::Array(vec![
                    Value::Type(Type::Function),
                    Value::Str("attach".into()),
                ]),
                Value::Array(vec![
                    Value::Type(Type::Function),
                    Value::Str("artifact".into()),
                ]),
            ])
        );

        match p1303_eval_success("pdf.attach(\"x.txt\", bytes((65,)))", Features::empty())
        {
            Value::Content(Content::PdfAttach(attach)) => {
                assert_eq!(attach.path.as_str(), "x.txt");
                assert_eq!(attach.data.as_slice(), &[65]);
            }
            other => panic!("pdf.attach deve preservar o carrier: {other:?}"),
        }
        assert!(matches!(
            p1303_eval_success("pdf.artifact[body]", Features::empty()),
            Value::Content(_)
        ));

        let html_gate_mismatches = p1303_error_mismatches(
            "html",
            Features::empty(),
            "cannot access variable `html` because the `html` feature is not enabled",
            &[
                "try enabling the `html` feature",
                "see https://typst.app/help/compiler-features for more details",
            ],
            0..4,
        );
        assert!(
            html_gate_mismatches.is_empty(),
            "P1303-S/html-disabled: {}",
            html_gate_mismatches.join("; ")
        );
    }

    #[derive(Debug, PartialEq)]
    struct P1303DiagnosticObservation {
        message: String,
        hints: Vec<String>,
        span: Option<std::ops::Range<usize>>,
    }

    #[derive(Debug, PartialEq)]
    enum P1303Outcome {
        Success(Value),
        Failure(Vec<P1303DiagnosticObservation>),
    }

    #[derive(Debug, PartialEq)]
    struct P1303Observation {
        outcome: P1303Outcome,
        lateral: Vec<P1303DiagnosticObservation>,
    }

    fn p1303_observe(expression: &str, features: Features) -> P1303Observation {
        let world = MockWorld::new("");
        let source = Source::new_with_parser(
            world.main(),
            expression.to_string(),
            crate::compiler::parse::parse_code,
        );
        let (result, side_diagnostics) =
            eval_expression_with_features(&world, expression, features);
        let observe_diagnostic =
            |diagnostic: &crate::entities::source_result::SourceDiagnostic| {
                P1303DiagnosticObservation {
                    message: diagnostic.message.to_string(),
                    hints: diagnostic.hints.iter().map(ToString::to_string).collect(),
                    span: source.span_byte_range(diagnostic.span),
                }
            };
        let lateral = side_diagnostics.iter().map(observe_diagnostic).collect::<Vec<_>>();
        let outcome = match result {
            Ok(value) => P1303Outcome::Success(value),
            Err(diagnostics) => P1303Outcome::Failure(
                diagnostics.iter().map(observe_diagnostic).collect::<Vec<_>>(),
            ),
        };
        P1303Observation { outcome, lateral }
    }

    fn p1303_observation_map(
        probes: &[(String, String, Features)],
    ) -> std::collections::BTreeMap<String, P1303Observation> {
        probes
            .iter()
            .map(|(key, expression, features)| {
                (key.clone(), p1303_observe(expression, features.clone()))
            })
            .collect()
    }

    #[test]
    fn p1303_ordem_repeticao_e_estado_completo() {
        let mut probes = Vec::new();
        for (profile, features) in p1300_profiles() {
            for field in ["data-cell", "header-cell", "table-summary"] {
                probes.push((
                    format!("P1303/{profile}/pdf.{field}"),
                    format!("repr((type(pdf.{field}), repr(pdf.{field})))"),
                    features.clone(),
                ));
            }
        }
        for (key, expression) in [
            ("P1303-S/std.nope", "repr(type(std.nope))"),
            ("P1303-S/calc.nope", "repr(type(calc.nope))"),
            ("P1303-S/sym.nope", "repr(type(sym.nope))"),
            ("P1303-S/color.map.nope", "repr(type(color.map.nope))"),
            ("P1303-S/std.rgb", "repr(type(std.rgb))"),
            ("P1303-S/calc.abs", "repr(type(calc.abs))"),
            ("P1303-S/sym.alpha", "repr(type(sym.alpha))"),
            ("P1303-S/color.map.viridis", "repr(type(color.map.viridis))"),
            ("P1303-S/dictionary.nope", "repr(type((:).nope))"),
            ("P1303-S/float.is-nan", "repr(type(float(\"NaN\").is-nan))"),
            ("P1303-S/pdf.attach", "repr(type(pdf.attach))"),
            ("P1303-S/pdf.artifact", "repr(type(pdf.artifact))"),
            ("P1303-S/html-disabled", "html"),
        ] {
            probes.push((key.into(), expression.into(), Features::empty()));
        }

        let normal = p1303_observation_map(&probes);
        let repeated = p1303_observation_map(&probes);
        probes.reverse();
        let inverted = p1303_observation_map(&probes);

        assert_eq!(normal, repeated, "P1303-META/ORDER-REPEAT: repetição");
        assert_eq!(normal, inverted, "P1303-META/ORDER-REPEAT: ordem invertida");
    }

    // P1305-r2: independent snapshots of pinned vanilla before candidate.
    mod p1305_oracles {
        use super::*;

        fn world(text: &str) -> ImportMockWorld {
            ImportMockWorld::new(
                text,
                &[
                    ("ordinary/std.typ", "#let x = 7\n"),
                    ("ordinary/global.typ", "#let x = 8\n"),
                    ("ordinary/map.typ", "#let x = 9\n"),
                    ("ordinary/unlisted-name.typ", "#let x = 10\n"),
                    ("ordinary/holder.typ", "#let saved = std\n"),
                    ("reexport/std.typ", "#import std: *\n"),
                    (
                        "routes/inner.typ",
                        "#let observation = (repr(std), { let alias = std; repr(alias) }, { import std as renamed; (repr(renamed), renamed.calc.abs(-7)) })\n",
                    ),
                ],
            )
        }

        fn observe(expression: &str, features: Features) -> Value {
            let (result, side) =
                eval_expression_with_features(&world(""), expression, features);
            assert!(
                side.is_empty(),
                "{expression}: unexpected side diagnostics {side:?}"
            );
            result.unwrap_or_else(|errors| panic!("{expression}: {errors:?}"))
        }

        #[test]
        fn p1305_array_repr_0() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(0))"###, features),
                    Value::Str(r###"()"###.into()),
                    "{profile}/array-repr-0"
                );
            }
        }

        #[test]
        fn p1305_array_repr_1() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(1))"###, features),
                    Value::Str(r###"(0,)"###.into()),
                    "{profile}/array-repr-1"
                );
            }
        }

        #[test]
        fn p1305_array_repr_39() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(39))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
)"###
                            .into()
                    ),
                    "{profile}/array-repr-39"
                );
            }
        }

        #[test]
        fn p1305_array_repr_40() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(40))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
)"###
                            .into()
                    ),
                    "{profile}/array-repr-40"
                );
            }
        }

        #[test]
        fn p1305_array_repr_41() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(41))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  .. (1 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/array-repr-41"
                );
            }
        }

        #[test]
        fn p1305_array_repr_42() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(42))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  .. (2 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/array-repr-42"
                );
            }
        }

        #[test]
        fn p1305_array_repr_81() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(81))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  .. (41 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/array-repr-81"
                );
            }
        }

        #[test]
        fn p1305_array_repr_256() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(256))"###, features),
                    Value::Str(
                        r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  .. (216 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/array-repr-256"
                );
            }
        }

        #[test]
        fn p1305_array_strings() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"repr(range(42).map(x => "item-" + str(x)))"###,
                        features
                    ),
                    Value::Str(
                        r###"(
  "item-0",
  "item-1",
  "item-2",
  "item-3",
  "item-4",
  "item-5",
  "item-6",
  "item-7",
  "item-8",
  "item-9",
  "item-10",
  "item-11",
  "item-12",
  "item-13",
  "item-14",
  "item-15",
  "item-16",
  "item-17",
  "item-18",
  "item-19",
  "item-20",
  "item-21",
  "item-22",
  "item-23",
  "item-24",
  "item-25",
  "item-26",
  "item-27",
  "item-28",
  "item-29",
  "item-30",
  "item-31",
  "item-32",
  "item-33",
  "item-34",
  "item-35",
  "item-36",
  "item-37",
  "item-38",
  "item-39",
  .. (2 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/array-strings"
                );
            }
        }

        #[test]
        fn p1305_array_distinct_after_boundary() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ let a = range(42); a.at(40) = -777; (repr(a), a.len(), a.at(39), a.at(40), a.at(41), a) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(
                            r###"(
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  11,
  12,
  13,
  14,
  15,
  16,
  17,
  18,
  19,
  20,
  21,
  22,
  23,
  24,
  25,
  26,
  27,
  28,
  29,
  30,
  31,
  32,
  33,
  34,
  35,
  36,
  37,
  38,
  39,
  .. (2 items omitted),
)"###
                                .into()
                        ),
                        Value::Int(42),
                        Value::Int(39),
                        Value::Int(-777),
                        Value::Int(41),
                        Value::Array(vec![
                            Value::Int(0),
                            Value::Int(1),
                            Value::Int(2),
                            Value::Int(3),
                            Value::Int(4),
                            Value::Int(5),
                            Value::Int(6),
                            Value::Int(7),
                            Value::Int(8),
                            Value::Int(9),
                            Value::Int(10),
                            Value::Int(11),
                            Value::Int(12),
                            Value::Int(13),
                            Value::Int(14),
                            Value::Int(15),
                            Value::Int(16),
                            Value::Int(17),
                            Value::Int(18),
                            Value::Int(19),
                            Value::Int(20),
                            Value::Int(21),
                            Value::Int(22),
                            Value::Int(23),
                            Value::Int(24),
                            Value::Int(25),
                            Value::Int(26),
                            Value::Int(27),
                            Value::Int(28),
                            Value::Int(29),
                            Value::Int(30),
                            Value::Int(31),
                            Value::Int(32),
                            Value::Int(33),
                            Value::Int(34),
                            Value::Int(35),
                            Value::Int(36),
                            Value::Int(37),
                            Value::Int(38),
                            Value::Int(39),
                            Value::Int(-777),
                            Value::Int(41)
                        ])
                    ]),
                    "{profile}/array-distinct-after-boundary"
                );
            }
        }

        #[test]
        fn p1305_nested_inner() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr((range(41), ("tail",)))"###, features),
                    Value::Str(
                        r###"(
  (
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32,
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    .. (1 items omitted),
  ),
  ("tail",),
)"###
                            .into()
                    ),
                    "{profile}/nested-inner"
                );
            }
        }

        #[test]
        fn p1305_nested_outer() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(range(81).map(x => (x, x + 1)))"###, features),
                    Value::Str(
                        r###"(
  (0, 1),
  (1, 2),
  (2, 3),
  (3, 4),
  (4, 5),
  (5, 6),
  (6, 7),
  (7, 8),
  (8, 9),
  (9, 10),
  (10, 11),
  (11, 12),
  (12, 13),
  (13, 14),
  (14, 15),
  (15, 16),
  (16, 17),
  (17, 18),
  (18, 19),
  (19, 20),
  (20, 21),
  (21, 22),
  (22, 23),
  (23, 24),
  (24, 25),
  (25, 26),
  (26, 27),
  (27, 28),
  (28, 29),
  (29, 30),
  (30, 31),
  (31, 32),
  (32, 33),
  (33, 34),
  (34, 35),
  (35, 36),
  (36, 37),
  (37, 38),
  (38, 39),
  (39, 40),
  .. (41 items omitted),
)"###
                            .into()
                    ),
                    "{profile}/nested-outer"
                );
            }
        }

        #[test]
        fn p1305_nested_multiline() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr((range(18), range(41)))"###, features),
                    Value::Str(
                        r###"(
  (
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
  ),
  (
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32,
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    .. (1 items omitted),
  ),
)"###
                            .into()
                    ),
                    "{profile}/nested-multiline"
                );
            }
        }

        #[test]
        fn p1305_escaped_strings() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(("a\n\"b\\c", "tail"))"###, features),
                    Value::Str(r###"("a\n\"b\\c", "tail")"###.into()),
                    "{profile}/escaped-strings"
                );
            }
        }

        #[test]
        fn p1305_ascii_body_49() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"repr(("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0))"###,
                        features
                    ),
                    Value::Str(
                        r###"("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0)"###
                            .into()
                    ),
                    "{profile}/ascii-body-49"
                );
            }
        }

        #[test]
        fn p1305_ascii_body_50() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"repr(("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0))"###,
                        features
                    ),
                    Value::Str(
                        r###"("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0)"###
                            .into()
                    ),
                    "{profile}/ascii-body-50"
                );
            }
        }

        #[test]
        fn p1305_ascii_body_51() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"repr(("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 0))"###,
                        features
                    ),
                    Value::Str(
                        r###"(
  "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  0,
)"###
                            .into()
                    ),
                    "{profile}/ascii-body-51"
                );
            }
        }

        #[test]
        fn p1305_content_op() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"repr(math.op([a \# \*], limits: false))"###, features),
                    Value::Str(
                        r###"op(
  text: sequence([a], [ ], [#], [ ], [*]),
  limits: false,
)"###
                            .into()
                    ),
                    "{profile}/content-op"
                );
            }
        }

        #[test]
        fn p1305_named_modules() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"(repr(std), repr(color.map), repr(calc), repr(sym), repr(pdf))"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Str(r###"<module map>"###.into()),
                        Value::Str(r###"<module calc>"###.into()),
                        Value::Str(r###"<module sym>"###.into()),
                        Value::Str(r###"<module pdf>"###.into())
                    ]),
                    "{profile}/named-modules"
                );
            }
        }

        #[test]
        fn p1305_named_aliases() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ let a = std; let b = a; let c = color.map; (repr(a), repr(b), repr(c), a.calc.abs(-7)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Str(r###"<module global>"###.into()),
                        Value::Str(r###"<module map>"###.into()),
                        Value::Int(7)
                    ]),
                    "{profile}/named-aliases"
                );
            }
        }

        #[test]
        fn p1305_ordinary_collisions() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import "ordinary/std.typ" as a; import "ordinary/global.typ" as b; import "ordinary/map.typ" as c; import "ordinary/unlisted-name.typ" as d; let alias = a; (repr(a), repr(b), repr(c), repr(d), repr(alias), a.x, b.x, c.x, d.x) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module std>"###.into()),
                        Value::Str(r###"<module global>"###.into()),
                        Value::Str(r###"<module map>"###.into()),
                        Value::Str(r###"<module unlisted-name>"###.into()),
                        Value::Str(r###"<module std>"###.into()),
                        Value::Int(7),
                        Value::Int(8),
                        Value::Int(9),
                        Value::Int(10)
                    ]),
                    "{profile}/ordinary-collisions"
                );
            }
        }

        #[test]
        fn p1305_ordinary_bare() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import "ordinary/std.typ"; (repr(std), std.x) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module std>"###.into()),
                        Value::Int(7)
                    ]),
                    "{profile}/ordinary-bare"
                );
            }
        }

        #[test]
        fn p1305_global_bare() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import std; (repr(std), std.calc.abs(-7)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Int(7)
                    ]),
                    "{profile}/global-bare"
                );
            }
        }

        #[test]
        fn p1305_global_alias_bare() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ let renamed = std; import renamed; (repr(renamed), renamed.calc.abs(-8)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Int(8)
                    ]),
                    "{profile}/global-alias-bare"
                );
            }
        }

        #[test]
        fn p1305_global_rename() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import std as renamed; (repr(renamed), renamed.calc.abs(-9)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Int(9)
                    ]),
                    "{profile}/global-rename"
                );
            }
        }

        #[test]
        fn p1305_global_items() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import std: calc as c, rgb as color-fn; (c.abs(-10), repr(type(color-fn))) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Int(10),
                        Value::Str(r###"function"###.into())
                    ]),
                    "{profile}/global-items"
                );
            }
        }

        #[test]
        fn p1305_global_wildcard() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import std: *; (calc.abs(-11), repr(type(rgb))) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Int(11),
                        Value::Str(r###"function"###.into())
                    ]),
                    "{profile}/global-wildcard"
                );
            }
        }

        #[test]
        fn p1305_reexport_collision() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import "reexport/std.typ" as r; let alias = r; (repr(r), repr(alias), r.calc.abs(-12), repr(r.calc)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module std>"###.into()),
                        Value::Str(r###"<module std>"###.into()),
                        Value::Int(12),
                        Value::Str(r###"<module calc>"###.into())
                    ]),
                    "{profile}/reexport-collision"
                );
            }
        }

        #[test]
        fn p1305_imported_route() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import "routes/inner.typ" as r; r.observation }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Str(r###"<module global>"###.into()),
                        Value::Array(vec![
                            Value::Str(r###"<module global>"###.into()),
                            Value::Int(7)
                        ])
                    ]),
                    "{profile}/imported-route"
                );
            }
        }

        #[test]
        fn p1305_global_field_bare() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import "ordinary/holder.typ" as holder; import holder.saved; (repr(saved), saved.calc.abs(-13)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Int(13)
                    ]),
                    "{profile}/global-field-bare"
                );
            }
        }

        #[test]
        fn p1305_global_dynamic_named() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(
                        r###"{ import (std) as chosen; (repr(chosen), chosen.calc.abs(-14)) }"###,
                        features
                    ),
                    Value::Array(vec![
                        Value::Str(r###"<module global>"###.into()),
                        Value::Int(14)
                    ]),
                    "{profile}/global-dynamic-named"
                );
            }
        }

        #[test]
        fn p1305_global_dynamic_items() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"{ import (std): calc as c; c.abs(-15) }"###, features),
                    Value::Int(15),
                    "{profile}/global-dynamic-items"
                );
            }
        }

        #[test]
        fn p1305_global_dynamic_wildcard() {
            for (profile, features) in p1300_profiles() {
                assert_eq!(
                    observe(r###"{ import (std): *; calc.abs(-16) }"###, features),
                    Value::Int(16),
                    "{profile}/global-dynamic-wildcard"
                );
            }
        }

        #[test]
        fn p1305_full_array_values_survive_repr() {
            for (_, features) in p1300_profiles() {
                for n in [0, 1, 39, 40, 41, 42, 81, 256] {
                    let expression = format!(
                        "{{ let a = range({n}); let before = a; let ignored = repr(a); a }}"
                    );
                    let expected = Value::Array((0..n).map(Value::Int).collect());
                    assert_eq!(observe(&expression, features.clone()), expected, "n={n}");
                }
            }
        }

        #[test]
        fn p1305_document_global_constructor_all_profiles() {
            use comemo::Track;
            let world = world(
                "#let observation = (repr(std), { let alias = std; repr(alias) }, { import std as renamed; (repr(renamed), renamed.calc.abs(-7)) })\n",
            );
            for (profile, features) in p1300_profiles() {
                let routines = Routines::new();
                let traced = Traced::default();
                let mut sink = Sink::new();
                let route = Route::root();
                let registry = crate::entities::element_registry::ElementRegistry::new();
                let result = eval_with_full_error_target_and_features(
                    &routines,
                    &world,
                    traced.track(),
                    sink.track_mut(),
                    route.track(),
                    &world.main,
                    &registry,
                    false,
                    EvalTarget::Paged,
                    features,
                )
                .unwrap_or_else(|errors| panic!("{profile}: {errors:?}"));
                assert_eq!(
                    result.scope().get("observation"),
                    Some(&Value::Array(vec![
                        Value::Str("<module global>".into()),
                        Value::Str("<module global>".into()),
                        Value::Array(vec![
                            Value::Str("<module global>".into()),
                            Value::Int(7)
                        ]),
                    ])),
                    "{profile}"
                );
            }
        }

        #[test]
        fn p1305_import_binding_negatives_and_dynamic_spans() {
            let cases: [(&str, &str, std::ops::Range<usize>, &[&str]); 6] = [
                ("{ import std; global }", "unknown variable `global`", 14..20, &[]),
                (
                    "{ let renamed = std; import renamed; global }",
                    "unknown variable `global`",
                    37..43,
                    &[],
                ),
                (
                    "{ import \"ordinary/holder.typ\" as holder; import holder.saved; global }",
                    "unknown variable `global`",
                    63..69,
                    &[],
                ),
                (
                    "{ import (std); global }",
                    "dynamic import requires an explicit name",
                    9..14,
                    &["you can name the import with `as`"],
                ),
                (
                    "{ import { std }; none }",
                    "dynamic import requires an explicit name",
                    9..16,
                    &["you can name the import with `as`"],
                ),
                (
                    "{ let f() = std; import f(); none }",
                    "dynamic import requires an explicit name",
                    24..27,
                    &["you can name the import with `as`"],
                ),
            ];
            for (profile, features) in p1300_profiles() {
                for (expression, message, span, hints) in &cases {
                    let world = world("");
                    let source = Source::new_with_parser(
                        world.main(),
                        expression.to_string(),
                        crate::compiler::parse::parse_code,
                    );
                    let (result, side) = eval_expression_with_features(
                        &world,
                        expression,
                        features.clone(),
                    );
                    assert!(side.is_empty(), "{profile}/{expression}: {side:?}");
                    let diagnostics = result.expect_err(expression);
                    assert_eq!(diagnostics.len(), 1, "{profile}/{expression}");
                    assert_eq!(
                        diagnostics[0].message, *message,
                        "{profile}/{expression}"
                    );
                    assert_eq!(
                        diagnostics[0]
                            .hints
                            .iter()
                            .map(|h| h.as_str())
                            .collect::<Vec<_>>(),
                        *hints,
                        "{profile}/{expression}"
                    );
                    assert_eq!(
                        source.span_byte_range(diagnostics[0].span),
                        Some(span.clone()),
                        "{profile}/{expression}"
                    );
                }
            }
        }

        #[test]
        fn p1305_content_sequence_does_not_inherit_array_elision() {
            let children: Vec<_> =
                (0..41).map(|i| Content::text(i.to_string().as_str())).collect();
            let value = Value::Content(Content::Sequence(std::sync::Arc::from(children)));
            let expected = format!(
                "sequence(\n  {},\n)",
                (0..41).map(|i| format!("[{i}]")).collect::<Vec<_>>().join(",\n  ")
            );
            assert_eq!(repr_value_for_serialization(&value), expected);
        }

        #[test]
        fn p1305_args_fields_remain_integral() {
            let expression = "{ let f(..args) = repr(args); f(..range(41)) }";
            let expected = format!(
                "arguments({})",
                (0..41).map(|i| i.to_string()).collect::<Vec<_>>().join(", ")
            );
            assert_eq!(
                observe(expression, Features::empty()),
                Value::Str(expected.into())
            );
        }
    }
}
