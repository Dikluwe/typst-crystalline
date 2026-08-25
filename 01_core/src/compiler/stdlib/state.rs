//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/state.md
//! @prompt-hash b6537895
//! @layer L1
//! @updated 2026-08-13
//!
//! `state(key, init)` como valor de primeira classe + métodos `.update()`,
//! `.get()` e `.display()`. P506 — runtime state via `context`.

use ecow::EcoString;

use crate::compiler::eval::call_dispatch::apply_func;
use crate::compiler::eval::EvalContext;
use crate::compiler::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::file_id::FileId;
use crate::entities::introspector::Introspector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::state::State;
use crate::entities::state_update::StateUpdate;
use crate::entities::value::Value;

use super::err;

/// `state(key, init)` → `Value::State`.
pub fn native_state(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), init] => {
            Ok(Value::State(State { key: key.clone(), init: Box::new(init.clone()) }))
        }
        [other, _] => err(format!(
            "state() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state() requer 2 argumentos (key, init), recebeu {}",
            args.items.len()
        )),
    }
}

/// Resolve `.update(value)` num `Content::StateUpdate`.
pub fn state_update(key: EcoString, value: Value) -> Value {
    Value::Content(Content::state_update(
        key.to_string(),
        StateUpdate::Set(Box::new(value)),
    ))
}

/// Resolve `.get()` dentro ou fora de context.
/// Fora de context: erro descritivo. Dentro: consulta o introspector.
pub fn state_get(state: &State, ctx: &EvalContext, span: Span) -> SourceResult<Value> {
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "state.get() can only be used inside context".to_string(),
        )]);
    }
    let Some(location) = ctx.current_location else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "state.get() requer uma localização de contexto".to_string(),
        )]);
    };
    let value = ctx
        .introspector
        .state_value(state.key.as_str(), location)
        .unwrap_or(state.init.as_ref())
        .clone();
    Ok(value)
}

/// Resolve `.display([callback])` dentro ou fora de context.
pub fn state_display(
    state: &State,
    args: &Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "state.display() can only be used inside context".to_string(),
        )]);
    }
    let Some(location) = ctx.current_location else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "state.display() requer uma localização de contexto".to_string(),
        )]);
    };

    let value = ctx
        .introspector
        .state_value(state.key.as_str(), location)
        .unwrap_or(state.init.as_ref())
        .clone();

    match args.items.as_slice() {
        [] => Ok(Value::Content(value_to_content(&value))),
        [Value::Func(callback)] => {
            let result = apply_func(
                callback.clone(),
                Args::positional(vec![value]),
                scopes,
                ctx,
                engine,
            )?;
            Ok(Value::Content(value_to_content(&result)))
        }
        [other] => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "state.display() requer função como argumento, recebeu {}",
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            span,
            "state.display() requer 0 ou 1 argumentos".to_string(),
        )]),
    }
}

/// **P844** (achado #49 de P831) — Resolve `.at(location)` dentro de
/// context. Paridade vanilla `State::at` (`introspection/state.rs`):
/// valor do state na `Location` indicada; se o state nunca foi
/// actualizado até lá, devolve o init (medido no vanilla 0.15.0:
/// `state("s", 7).final()` sem updates → `7`; `at` antes do primeiro
/// update → init). A validação de argumentos corre no dispatch
/// (`bindings.rs`) antes deste gate — ordem medida no vanilla:
/// `s.at(1)` fora de contexto → erro de tipo, não gate de contexto.
pub fn state_at_location(
    state: &State,
    location: crate::entities::location::Location,
    ctx: &EvalContext,
    span: Span,
) -> SourceResult<Value> {
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "can only be used when context is known".to_string(),
        )]);
    }
    let value = ctx
        .introspector
        .state_value(state.key.as_str(), location)
        .unwrap_or(state.init.as_ref())
        .clone();
    Ok(value)
}

/// **P844** (achado #49 de P831) — Resolve `.final()` dentro de
/// context. Paridade vanilla `State::final`: valor no fim do documento;
/// sem updates, o init (medido: `state("s", 7).final()` → `7`).
/// Reusa `Introspector::state_final_value` (P171/P240 — two-pass real
/// pós-fixpoint).
pub fn state_final(state: &State, ctx: &EvalContext, span: Span) -> SourceResult<Value> {
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            span,
            "can only be used when context is known".to_string(),
        )]);
    }
    let value = ctx
        .introspector
        .state_final_value(state.key.as_str())
        .cloned()
        .unwrap_or_else(|| state.init.as_ref().clone());
    Ok(value)
}

/// Converte um `Value` resolvido em `Content` para display.
pub fn value_to_content(value: &Value) -> Content {
    match value {
        Value::Content(c) | Value::LocatedContent(c, _) => c.clone(),
        Value::Str(s) => Content::text(s.clone()),
        Value::Int(i) => Content::text(i.to_string()),
        Value::Float(f) => Content::text(format_float(*f)),
        Value::Bool(b) => Content::text(b.to_string()),
        // **P821** (colateral do achado #8 de P810) — display de um tipo é o
        // seu nome curto (medido no vanilla: `#context type(1)` → "int",
        // `#context type("abc")` → "str"). Antes caía no braço `_` → Empty.
        Value::Type(t) => Content::text(t.name().to_string()),
        // **P842** (achado #35 de P831) — tipos numéricos/geométricos com o
        // display já usado em `repr` (para unidades coincide com o display
        // vanilla). Medido nos dois binários (`temp/p842/l4_ctx_*.typ`):
        // `#context (10pt)` → "10pt", `(50%)` → "50%", `(30% + 1em)` →
        // "30% + 1em", `(45deg)` → "45deg", `(2fr)` → "2fr". Pré-P842
        // caíam no braço `_` → `Content::Empty` (página vazia).
        Value::Length(_)
        | Value::Ratio(_)
        | Value::Relative(_)
        | Value::Angle(_)
        | Value::Fraction(_) => {
            Content::text(crate::compiler::eval::repr::repr_value(value))
        }
        // **P844** (achado #51 de P831) — display de array dentro de
        // `#context` é o repr (`(3,)`, `(1, 2)`), medido no vanilla
        // 0.15.0 (`temp/p844/a5_context_array.typ`). Reusa a rotina de
        // repr já corrigida em P801 para o caminho directo. Antes:
        // join próprio com `.` (só Int/Str, resto → "").
        Value::Array(_) => Content::text(crate::compiler::eval::repr::repr_value(value)),
        // **P886** (achado 2 de P885) — `#context measure[...]` rendia
        // página vazia: `measure()` (P712) devolve `Value::Dict`, que caía
        // aqui. Medido no vanilla (`vanilla-07-context.pdf`, P872):
        // `(width: 42.85pt, height: 7.24pt)`. Mesmo padrão do braço
        // `Array` acima — reusa `repr_value`, já produz esse formato.
        Value::Dict(_) => Content::text(crate::compiler::eval::repr::repr_value(value)),
        // intencional: funções, módulos, estilos e metadados não produzem conteúdo visual em state display
        Value::None
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Auto
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_) => Content::Empty,
    }
}

fn format_float(f: f64) -> String {
    if f == f.trunc() {
        format!("{:.1}", f)
    } else {
        f.to_string()
    }
}

// ── Nativas globais absorvidas de `foundations.rs` no Passo 1032 ────────────

/// `state_update(key, value)` — actualiza runtime state. P171.
///
/// Forma funcional cristalina (vanilla expõe como `state.update(key, fn)`
/// método). `value` é o novo valor (Set variant).
pub fn native_state_update(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), value] => {
            Ok(Value::Content(crate::entities::content::Content::state_update(
                key.to_string(),
                crate::entities::state_update::StateUpdate::Set(Box::new(value.clone())),
            )))
        }
        [other, _] => err(format!(
            "state_update() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_update() requer 2 argumentos (key, value), recebeu {}",
            args.items.len()
        )),
    }
}

/// `state_update_with(key, fn)` — actualiza runtime state via callback. P172.
pub fn native_state_update_with(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Func(func)] => {
            Ok(Value::Content(crate::entities::content::Content::state_update(
                key.to_string(),
                crate::entities::state_update::StateUpdate::Func(func.clone()),
            )))
        }
        [_, other] => err(format!(
            "state_update_with() requer função como segundo argumento, recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_update_with() requer 2 argumentos (key, fn), recebeu {}",
            args.items.len()
        )),
    }
}

/// `state_display(key, [callback])` — render-mediated state display. P240.
pub fn native_state_display(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => Ok(Value::Content(
            crate::entities::content::Content::state_display(key.to_string(), None),
        )),
        [Value::Str(key), Value::Func(callback)] => {
            Ok(Value::Content(crate::entities::content::Content::state_display(
                key.to_string(),
                Some(callback.clone()),
            )))
        }
        [Value::Str(_), other] => err(format!(
            "state_display() requer função como segundo argumento (callback), recebeu {}",
            other.type_name()
        )),
        [other, ..] => err(format!(
            "state_display() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_display() requer 1-2 argumentos (key, [callback]), recebeu {}",
            args.items.len()
        )),
    }
}

/// `state_final(key)` — valor final do state `key` pós-walk. P236.
pub fn native_state_final(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => {
            let value = ctx
                .introspector
                .state_final_value(key.as_str())
                .cloned()
                .unwrap_or(Value::None);
            Ok(value)
        }
        [other] => err(format!(
            "state_final() requer string como argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_final() requer 1 argumento (key), recebeu {}",
            args.items.len()
        )),
    }
}

/// `state_at(key, label)` — valor do state `key` na Location associada
/// ao `label`. P237.
pub fn native_state_at(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;

    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Str(label_str)] => {
            let label = Label(label_str.to_string());
            let value = ctx
                .introspector
                .query_by_label(&label)
                .and_then(|loc| ctx.introspector.state_value(key.as_str(), loc).cloned())
                .unwrap_or(Value::None);
            Ok(value)
        }
        [_, other] => err(format!(
            "state_at() requer string como segundo argumento (label), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_at() requer 2 argumentos (key, label), recebeu {}",
            args.items.len()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    struct NullWorld {
        library: Library,
        book: FontBook,
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
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn null_world() -> NullWorld {
        NullWorld { library: Library::new(), book: FontBook::new() }
    }
    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    #[test]
    fn native_state_cria_valor() {
        let s = native_state(
            &mut EvalContext::new(),
            &Args::positional(vec![Value::Str("x".into()), Value::Int(0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(matches!(s, Value::State(_)));
    }

    #[test]
    fn native_state_rejeita_key_nao_string() {
        let r = native_state(
            &mut EvalContext::new(),
            &Args::positional(vec![Value::Int(1), Value::Int(0)]),
            &null_world(),
            test_file_id(),
        );
        assert!(r.is_err());
    }

    #[test]
    fn p821_value_to_content_type_usa_nome_do_tipo() {
        // P821 (colateral do achado #8 de P810): `#context type(1)` rendia
        // vazio — `value_to_content` caía no braço `_ => Content::Empty`.
        // Medido no vanilla: display de um tipo é o seu nome curto
        // (`type(1)` → "int", `type("abc")` → "str").
        use crate::entities::value::Type;
        assert_eq!(value_to_content(&Value::Type(Type::Int)).plain_text(), "int");
        assert_eq!(value_to_content(&Value::Type(Type::Str)).plain_text(), "str");
    }

    #[test]
    fn p844_a5_value_to_content_array_usa_repr() {
        // P844 (achado #51 de P831) — `#context ((3,))` mostrava join
        // ("3") em vez do repr ("(3,)"). Medido nos dois binários
        // (`temp/p844/a5_context_array.typ`): vanilla `(3,) (3,)`;
        // cristalino `3 (3,)` — fora de `#context` o repr já estava
        // correto (P801, achado #4); o achado é só o caminho
        // `value_to_content` (display dentro de `#context`).
        assert_eq!(
            value_to_content(&Value::Array(vec![Value::Int(3)])).plain_text(),
            "(3,)"
        );
        assert_eq!(
            value_to_content(&Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Str("ab".into())
            ]))
            .plain_text(),
            "(1, 2, \"ab\")"
        );
    }

    #[test]
    fn p842_l4_value_to_content_tipos_numericos_geometricos() {
        // P842 (achado #35 de P831): `#context (10pt)` e outros tipos
        // numéricos/geométricos rendiam página vazia (`_ => Content::Empty`).
        // Medido nos dois binários (`temp/p842/l4_ctx_*.typ`): vanilla exibe
        // "10pt", "50%", "30% + 1em", "45deg", "2fr" (Int/Float já
        // funcionavam pré-P842).
        use crate::entities::layout_types::{Angle, Length, Ratio};
        use crate::entities::rel::Rel;
        assert_eq!(
            value_to_content(&Value::Length(Length::pt(10.0))).plain_text(),
            "10pt"
        );
        assert_eq!(
            value_to_content(&Value::Ratio(Ratio::from_percent(50.0))).plain_text(),
            "50%"
        );
        assert_eq!(
            value_to_content(&Value::Relative(Rel::from_percent(30.0) + Length::em(1.0)))
                .plain_text(),
            "30% + 1em"
        );
        assert_eq!(
            value_to_content(&Value::Angle(Angle::deg(45.0))).plain_text(),
            "45deg"
        );
        assert_eq!(value_to_content(&Value::Fraction(2.0)).plain_text(), "2fr");
        // Não-regressão: Int/Float mantêm o display pré-P842.
        assert_eq!(value_to_content(&Value::Int(3)).plain_text(), "3");
        assert_eq!(value_to_content(&Value::Float(2.5)).plain_text(), "2.5");
    }

    #[test]
    fn p886_value_to_content_dict_usa_repr() {
        // P886 (achado 2 de P885) — `#context measure[...]` rendia página
        // vazia: `measure()` (P712, `eval/closures.rs`) devolve
        // `Value::Dict`, e `value_to_content` caía no braço
        // `_ => Content::Empty`. Medido no vanilla (`pdftotext` em
        // `vanilla-07-context.pdf`, fonte `07-context.typ` de P872):
        // `(width: 42.85pt, height: 7.24pt)`. `repr_value` para
        // `Value::Dict` já produz esse formato (`eval/repr.rs`); falta só
        // o braço em `value_to_content`.
        use crate::entities::layout_types::Length;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;

        let mut dict: IndexMap<ecow::EcoString, Value, FxBuildHasher> =
            IndexMap::default();
        dict.insert("width".into(), Value::Length(Length::pt(42.85)));
        dict.insert("height".into(), Value::Length(Length::pt(7.24)));
        assert_eq!(
            value_to_content(&Value::Dict(dict)).plain_text(),
            "(width: 42.85pt, height: 7.24pt)"
        );

        // P695 — dict vazio é `(:)` (paridade com `repr_value`), não vazio.
        let empty: IndexMap<ecow::EcoString, Value, FxBuildHasher> = IndexMap::default();
        assert_eq!(value_to_content(&Value::Dict(empty)).plain_text(), "(:)");
    }
}
