//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f6cc2443
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas fundamentais (type, len, rgb, luma, range, str, int, float).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use ecow::EcoString;
use crate::entities::file_id::FileId;

use super::{err, expect_no_named};

use crate::entities::args::Args;
use crate::entities::layout_types::Length;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

/// `type(v)` → nome do tipo como string Typst.
pub fn native_type(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Str(v.type_name().into())),
        _   => err(format!("type() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `len(v)` → comprimento de Str, Array ou Dict.
pub fn native_len(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)]   => Ok(Value::Int(s.chars().count() as i64)),
        [Value::Array(a)] => Ok(Value::Int(a.len() as i64)),
        [Value::Dict(d)]  => Ok(Value::Int(d.len() as i64)),
        [other]           => err(format!("len() não suporta {}", other.type_name())),
        _                 => err(format!("len() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `rgb(r, g, b)` ou `rgb(r, g, b, a)` → Color.
///
/// Args em Int 0–255. Quatro args incluem canal alpha.
/// Fora de 0–255 → Err.
pub fn native_rgb(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn check(v: i64, name: &str) -> SourceResult<u8> {
        if (0..=255).contains(&v) {
            Ok(v as u8)
        } else {
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("rgb(): componente {} fora de 0–255: {}", name, v),
            )])
        }
    }
    match args.items.as_slice() {
        [Value::Int(r), Value::Int(g), Value::Int(b)] => {
            Ok(Value::Color(Color::rgb(check(*r, "r")?, check(*g, "g")?, check(*b, "b")?)))
        }
        [Value::Int(r), Value::Int(g), Value::Int(b), Value::Int(a)] => {
            Ok(Value::Color(Color::rgba(check(*r, "r")?, check(*g, "g")?, check(*b, "b")?, check(*a, "a")?)))
        }
        _ => err(format!("rgb() requer 3 ou 4 Int, recebeu {} args", args.items.len())),
    }
}

/// `luma(l)` → Color::Rgb { r: l, g: l, b: l } (escala de cinzentos).
pub fn native_luma(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(l)] => {
            if !(0..=255).contains(l) {
                return err(format!("luma(): componente fora de 0–255: {}", l));
            }
            let l = *l as u8;
            Ok(Value::Color(Color::rgb(l, l, l)))
        }
        _ => err(format!("luma() requer 1 Int, recebeu {} args", args.items.len())),
    }
}

/// `range(n)` → Array de 0..n; `range(start, end)` → Array de start..end.
pub fn native_range(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n)] => {
            if *n < 0 {
                return err("range() requer argumento não-negativo");
            }
            Ok(Value::Array((0..*n).map(Value::Int).collect()))
        }
        [Value::Int(start), Value::Int(end)] => {
            let items = if start <= end {
                (*start..*end).map(Value::Int).collect()
            } else {
                vec![]
            };
            Ok(Value::Array(items))
        }
        _ => err(format!("range() requer 1 ou 2 Int, recebeu {} args", args.items.len())),
    }
}

// ── Funções de conversão de tipo (Passo 27) ─────────────────────────────────

/// `str(v)` → representação textual do valor.
pub fn native_str(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let s: String = match v {
                Value::None        => "none".into(),
                Value::Bool(b)     => if *b { "true" } else { "false" }.into(),
                Value::Int(i)      => i.to_string(),
                Value::Float(f)    => format_float(*f),
                Value::Str(s)      => return Ok(Value::Str(s.clone())),
                Value::Auto        => "auto".into(),
                Value::Length(l)   => format_length(l),
                Value::Ratio(r)    => format!("{}%", r.to_percent()),
                Value::Angle(a)    => format!("{}deg", a.to_deg()),
                Value::Color(_)    => return err("str() não suporta color"),
                other => return err(format!("str() não suporta {}", other.type_name())),
            };
            Ok(Value::Str(EcoString::from(s)))
        }
        _ => err(format!("str() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// Formata f64 de forma compacta — sem trailing zeros desnecessários.
fn format_float(f: f64) -> String {
    let s = format!("{}", f);
    if s.contains('.') || s.contains('e') { s } else { format!("{s}.0") }
}

/// Formata Length como string (ex: "12pt", "1.5em", "6pt + 1em").
fn format_length(l: &Length) -> String {
    let abs = l.abs.to_pt();
    let em  = l.em;
    match (abs == 0.0, em == 0.0) {
        (true,  true)  => "0pt".into(),
        (false, true)  => format!("{abs}pt"),
        (true,  false) => format!("{em}em"),
        (false, false) => format!("{abs}pt + {em}em"),
    }
}

/// `int(v)` → inteiro. Aceita Int, Str (decimal), Bool.
/// Float → Err (semântica vanilla: Float não é `ToInt`).
pub fn native_int(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]    => Ok(Value::Int(*i)),
        [Value::Bool(b)]   => Ok(Value::Int(if *b { 1 } else { 0 })),
        [Value::Str(s)]    => s.parse::<i64>()
            .map(Value::Int)
            .map_err(|_| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("int() não consegue parsear {:?}", s.as_str()),
            )]),
        [Value::Float(f)]  => err(format!(
            "int() não converte float {f} — usar int(calc.round(x)) ou int(calc.floor(x))"
        )),
        [other] => err(format!("int() não suporta {}", other.type_name())),
        _ => err(format!("int() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `float(v)` → float. Aceita Float, Int (coerção), Str.
pub fn native_float(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Float(f)] => Ok(Value::Float(*f)),
        [Value::Int(i)]   => Ok(Value::Float(*i as f64)),
        [Value::Str(s)]   => s.parse::<f64>()
            .map(Value::Float)
            .map_err(|_| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("float() não consegue parsear {:?}", s.as_str()),
            )]),
        [other] => err(format!("float() não suporta {}", other.type_name())),
        _ => err(format!("float() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `metadata(value)` — embeber valor opaco no documento para query
/// via `Introspector::query_metadata`. P169 (M9 sub-passo 1).
///
/// Vanilla: `metadata(value)` em `introspection/metadata.rs`. Cristalino
/// minimal: 1 argumento posicional; sem named args; produz
/// `Content::Metadata { value: Box<Value> }` que é zero-size em layout.
pub fn native_metadata(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn crate::contracts::world::World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Content(crate::entities::content::Content::Metadata {
            value: Box::new(v.clone()),
        })),
        _ => err(format!(
            "metadata() requer 1 argumento, recebeu {}",
            args.items.len()
        )),
    }
}

/// `state(key, init)` — define runtime mutable state. P171 (M9 sub-passo 3).
///
/// Vanilla: `state(key, init)` em `introspection/state.rs`. Cristalino
/// minimal: 2 argumentos posicionais (key: Str, init: Value); produz
/// `Content::State { key, init: Box<Value> }`. Invisível em layout.
pub fn native_state(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn crate::contracts::world::World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), init] => Ok(Value::Content(
            crate::entities::content::Content::State {
                key:  key.to_string(),
                init: Box::new(init.clone()),
            },
        )),
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

/// `state_update(key, value)` — actualiza runtime state. P171 (M9 sub-passo 3).
///
/// Forma funcional cristalina (vanilla expõe como `state.update(key, fn)`
/// método; cristalino não suporta methods em values em P171). `value`
/// é o novo valor (Set variant); callbacks `Func` adiados.
pub fn native_state_update(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn crate::contracts::world::World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), value] => Ok(Value::Content(
            crate::entities::content::Content::StateUpdate {
                key:    key.to_string(),
                update: crate::entities::state_update::StateUpdate::Set(
                    Box::new(value.clone()),
                ),
            },
        )),
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

/// `state_update_with(key, fn)` — actualiza runtime state via callback.
/// P172 (M9 sub-passo 4).
///
/// **Stub em P172**: a callback é capturada na variant
/// `StateUpdate::Func(fn)` mas **NÃO é avaliada** em from_tags
/// (eval requer `Engine + EvalContext` que não estão disponíveis após
/// eval — pipeline restructuring deferido para passo dedicado).
/// Usar este stdlib func em P172 cria uma `StateUpdate::Func` que é
/// silenciosamente ignorada pelo `StateRegistry::apply_update`. O
/// state continua a refletir apenas as `Set` updates.
///
/// Para uso real, aguardar passo M7+ ou refactor de pipeline que
/// permita threading de Engine para from_tags.
pub fn native_state_update_with(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn crate::contracts::world::World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Func(func)] => Ok(Value::Content(
            crate::entities::content::Content::StateUpdate {
                key:    key.to_string(),
                update: crate::entities::state_update::StateUpdate::Func(func.clone()),
            },
        )),
        [_, other] => err(format!(
            "state_update_with() requer função como segundo argumento, recebeu {}",
            other.type_name()
        )),
        [other, _] => err(format!(
            "state_update_with() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_update_with() requer 2 argumentos (key, fn), recebeu {}",
            args.items.len()
        )),
    }
}

/// `counter_at(key_str, label_str)` — valor do counter `key` na
/// `Location` associada à `label_str`. P177 (M9 sub-passo 7).
///
/// **Forma minimal P177**: retorna `Value::Str` formatado
/// hierárquicamente (e.g. `"1.2.3"`). Reusa
/// `Introspector::query_by_label` (P165) +
/// `Introspector::formatted_counter_at` (P177).
///
/// Casos de borda (todos retornam `Value::Str("")`):
/// - Label inexistente.
/// - Counter sem update prévia à Location do label.
/// - Counter inexistente.
pub fn native_counter_at(
    ctx:                &mut EvalContext,
    args:               &Args,
    _world:             &dyn crate::contracts::world::World,
    _current_file:      FileId,
    _figure_numbering:  Option<&str>,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Str(label_str)] => {
            let label = Label(label_str.to_string());
            let formatted = ctx
                .introspector
                .query_by_label(&label)
                .and_then(|loc| ctx.introspector.formatted_counter_at(key.as_str(), loc))
                .unwrap_or_default();
            Ok(Value::Str(formatted.into()))
        }
        [_, other] => err(format!(
            "counter_at() requer string como segundo argumento (label), recebeu {}",
            other.type_name()
        )),
        [other, _] => err(format!(
            "counter_at() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter_at() requer 2 argumentos (key, label), recebeu {}",
            args.items.len()
        )),
    }
}

/// `counter_final(key_str)` — consulta o valor final do counter `key`
/// no `Introspector` da iteração de fixpoint anterior. P176 (M9
/// sub-passo 6).
///
/// **Forma minimal P176** (Opção β): retorna `Value::Str` com o
/// formato hierárquico (e.g. `"1.2.3"`) reusando
/// `Introspector::formatted_counter` (P170). Sem `Value::Counter`
/// rich type — refino futuro.
///
/// Iteração 0 (sem fixpoint, ou primeira iter de
/// `introspect_to_fixpoint`): `ctx.introspector` está vazio →
/// retorna `Value::Str("")`. Iterações seguintes vêem counter
/// populado pela iter anterior.
pub fn native_counter_final(
    ctx:                &mut EvalContext,
    args:               &Args,
    _world:             &dyn crate::contracts::world::World,
    _current_file:      FileId,
    _figure_numbering:  Option<&str>,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => {
            let formatted = ctx
                .introspector
                .formatted_counter(key.as_str())
                .unwrap_or_default();
            Ok(Value::Str(formatted.into()))
        }
        [other] => err(format!(
            "counter_final() requer string como argumento, recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter_final() requer 1 argumento (key), recebeu {}",
            args.items.len()
        )),
    }
}

/// `query(kind_str)` — consulta o `Introspector` da iteração de fixpoint
/// anterior por elementos do kind indicado.
///
/// **P175 (M9 sub-passo 5)**: forma original retornava `Value::Int(count)`.
/// **P179 upgrade**: retorna `Value::Array(Vec<Value::Location>)` com
/// as Locations dos elementos matched, em ordem de aparecimento.
/// Cliente que precise apenas de count usa `len(query("heading"))`.
///
/// `kind_str` válidos: `"heading"`, `"figure"`, `"citation"`,
/// `"metadata"`, `"state"`, `"state_update"`, `"outline"` (P178).
///
/// Iteração 0 (sem fixpoint, ou primeira iter de
/// `introspect_to_fixpoint`): `ctx.introspector` está vazio →
/// retorna `Value::Array(vec![])`. Iterações seguintes vêem
/// introspector populado pela iter anterior.
pub fn native_query(
    ctx:                &mut EvalContext,
    args:               &Args,
    _world:             &dyn crate::contracts::world::World,
    _current_file:      FileId,
    _figure_numbering:  Option<&str>,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    expect_no_named(&args.named)?;
    let selector = parse_selector_arg(&args.items, "query")?;
    let locations = ctx.introspector.query(&selector);
    let values: Vec<Value> = locations
        .into_iter()
        .map(Value::Location)
        .collect();
    Ok(Value::Array(values))
}

/// **P209B (M9c)** — Parse selector arg para `native_query` +
/// `native_locate`. Dispatch por `Value` variant:
///
/// - `Value::Str("<name>")` (entre `<` e `>`) → `Selector::Label(Label(name))`.
/// - `Value::Str("kind")` → `Selector::Kind(ElementKind::from_name(kind))`.
/// - `Value::Location(loc)` → `Selector::Location(loc)`.
///
/// `func_name` é usado nas mensagens de erro para diferenciar
/// `query()` vs `locate()`.
fn parse_selector_arg(
    items:     &[Value],
    func_name: &str,
) -> SourceResult<crate::entities::selector::Selector> {
    use crate::entities::element_kind::ElementKind;
    use crate::entities::label::Label;
    use crate::entities::selector::Selector;
    let msg = |s: String| -> SourceResult<Selector> {
        Err(vec![SourceDiagnostic::error(Span::detached(), s)])
    };
    match items {
        [Value::Str(s)]
            if s.len() >= 2 && s.starts_with('<') && s.ends_with('>') =>
        {
            // P209B: <name> syntax → Selector::Label.
            let name = &s[1..s.len() - 1];
            Ok(Selector::Label(Label(name.to_string())))
        }
        [Value::Str(kind_str)] => {
            match ElementKind::from_name(kind_str.as_str()) {
                Some(kind) => Ok(Selector::Kind(kind)),
                None => msg(format!(
                    "{}(): kind '{}' não reconhecido (válidos: \
                     heading, figure, citation, metadata, state, \
                     state_update, outline). Para label, use \
                     `<nome>` syntax.",
                    func_name, kind_str
                )),
            }
        }
        [Value::Location(loc)] => {
            // P209B: Value::Location dispatch.
            Ok(Selector::Location(*loc))
        }
        [other] => msg(format!(
            "{}() requer string ou location, recebeu {}. \
             Tipos suportados: \"kind\", \"<label>\", \
             Value::Location. (Regex requer P209D; And/Or \
             ainda só Rust API.)",
            func_name, other.type_name()
        )),
        _ => msg(format!(
            "{}() requer 1 argumento (selector), recebeu {}",
            func_name, items.len()
        )),
    }
}

/// **P208B (M9c)** — `here()` — retorna a Location "actual" disponível
/// no `EvalContext`.
///
/// Paridade vanilla: `here(context: Tracked<Context>) -> HintedStrResult<Location>`.
/// Cristalino diverge per P205A.div-1: lê `ctx.current_location`
/// directamente (sem `Tracked<Context>` envolvendo, pattern P174 +
/// `native_query` P175/P179). Retorna `Value::Location(loc)` quando
/// `current_location` está populated; erro contextual coerente caso
/// contrário.
///
/// **Mecanismo de população** (P208B infra minimal): `current_location`
/// é `None` por defeito. Caller que conhece a Location actual
/// (futuro show-rule para `Content::Context` block análogo a vanilla
/// `ContextElem`; tests sintéticos) seta o field antes de invocar
/// eval. Sub-mecanismo de captura automática no eval walk é deferred
/// per P208B C1 (zero consumers production confirmado em P208A A5 +
/// P208B C1.3).
///
/// **Sem args** (vanilla recebe Tracked<Context> só; cristalino sem
/// args explícitos).
pub fn native_here(
    ctx:               &mut EvalContext,
    args:              &Args,
    _world:            &dyn crate::contracts::world::World,
    _current_file:     FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if !args.items.is_empty() {
        return err(format!(
            "here() não aceita argumentos, recebeu {}",
            args.items.len()
        ));
    }
    match ctx.current_location {
        Some(loc) => Ok(Value::Location(loc)),
        None => err(
            "here() chamado fora de contexto locatable — \
             current_location não populado (P208B: infra minimal; \
             captura automática no walk é deferred)".to_string()
        ),
    }
}

/// **P210B (M9c)** — `counter_step(key)` — emite
/// `Content::CounterUpdate { key, action: Step }` que aplica
/// em layout time.
///
/// Paridade vanilla: `counter.step()` → emite `CounterUpdateElem`.
/// Cristalino devolve `Value::Content(Content::CounterUpdate {...})`
/// que, quando inserido no documento, faz o Layouter aplicar
/// `CounterAction::Step` ao counter `key`.
///
/// **Não depende de `current_location`** (per P210A A3) — emite
/// Content estático; layout-time semantics. Distinto de
/// `counter.display`/`state.get` (deferred per P210A C3).
///
/// Q1=β subset minimal — apenas `counter.step()` materializado
/// nesta passada; display/get aguardam walk advance
/// implementação.
pub fn native_counter_step(
    _ctx:              &mut EvalContext,
    args:              &Args,
    _world:            &dyn crate::contracts::world::World,
    _current_file:     FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    use crate::entities::content::Content;
    use crate::entities::counter_update::CounterUpdate as CounterAction;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => {
            let content = Content::CounterUpdate {
                key:    key.to_string(),
                action: CounterAction::Step,
            };
            Ok(Value::Content(content))
        }
        [other] => err(format!(
            "counter_step() requer string como argumento (key), \
             recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "counter_step() requer 1 argumento (key), recebeu {}",
            args.items.len()
        )),
    }
}

/// **P208C (M9c)** — `locate(kind)` — retorna a **primeira** Location
/// de um elemento do `kind` indicado.
///
/// Paridade vanilla: `locate(selector) -> Location` retorna primeira
/// match. Cristalino P208C aceita **apenas `kind-as-string`** (paridade
/// com `native_query` P175 minimal). `locate(<label>)` requer
/// `Selector::Label` que será materializado em P209 (per
/// `P207A.div-1` Q-decisões).
///
/// Reusa pattern literal de `native_query`:
/// - 1 arg `Value::Str(kind)`.
/// - `ElementKind::from_name(kind)` → `Selector::Kind`.
/// - `ctx.introspector.query(&selector).first().copied()`.
///
/// Retorno:
/// - `Value::Location(loc)` se kind tem ≥1 match.
/// - `Value::None` se kind válido mas sem matches (`Vec::first` →
///   `None`).
/// - `SourceResult::Err` se kind inválido ou arg não-string.
pub fn native_locate(
    ctx:               &mut EvalContext,
    args:              &Args,
    _world:            &dyn crate::contracts::world::World,
    _current_file:     FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    expect_no_named(&args.named)?;
    let selector = parse_selector_arg(&args.items, "locate")?;
    let first = ctx.introspector.query(&selector).first().copied();
    Ok(match first {
        Some(loc) => Value::Location(loc),
        None      => Value::None,
    })
}
