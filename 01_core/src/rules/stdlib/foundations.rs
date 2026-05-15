//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash 68fc3823
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

/// `luma(l)` → Color::Luma (paridade vanilla D65Gray pós-P257).
///
/// **P257 (ADR-0083 PROPOSTO)** — refactor: constrói `Color::Luma`
/// dedicado (em vez de `Color::Srgb` cinza). Aceita Int [0, 255]
/// como paridade construtor anterior; converte para f32 [0.0, 1.0]
/// internamente. PDF output bit-equivalente via `to_srgb()` que
/// expande Luma para sRGB cinza.
pub fn native_luma(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(l)] => {
            if !(0..=255).contains(l) {
                return err(format!("luma(): componente fora de 0–255: {}", l));
            }
            Ok(Value::Color(Color::luma(*l as f32 / 255.0)))
        }
        _ => err(format!("luma() requer 1 Int, recebeu {} args", args.items.len())),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `oklab(l, a, b[, alpha])` →
/// `Color::Oklab`. Componentes f32 (Float ou Int).
pub fn native_oklab(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i)   => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("oklab({}): espera Float/Int, recebeu {}", name, other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [l, a, b] => Ok(Value::Color(Color::oklab(
            as_f32(l, "l")?, as_f32(a, "a")?, as_f32(b, "b")?, 1.0,
        ))),
        [l, a, b, alpha] => Ok(Value::Color(Color::oklab(
            as_f32(l, "l")?, as_f32(a, "a")?, as_f32(b, "b")?, as_f32(alpha, "alpha")?,
        ))),
        _ => err(format!("oklab() requer 3 ou 4 Float/Int, recebeu {} args", args.items.len())),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `oklch(l, c, h[, alpha])` →
/// `Color::Oklch`. `h` em graus.
pub fn native_oklch(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i)   => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("oklch({}): espera Float/Int, recebeu {}", name, other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [l, c, h] => Ok(Value::Color(Color::oklch(
            as_f32(l, "l")?, as_f32(c, "c")?, as_f32(h, "h")?, 1.0,
        ))),
        [l, c, h, alpha] => Ok(Value::Color(Color::oklch(
            as_f32(l, "l")?, as_f32(c, "c")?, as_f32(h, "h")?, as_f32(alpha, "alpha")?,
        ))),
        _ => err(format!("oklch() requer 3 ou 4 Float/Int, recebeu {} args", args.items.len())),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `linear_rgb(r, g, b[, alpha])`
/// → `Color::LinearRgb`. Componentes f32 [0.0, 1.0].
pub fn native_linear_rgb(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i)   => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("linear_rgb({}): espera Float/Int, recebeu {}", name, other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [r, g, b] => Ok(Value::Color(Color::linear_rgb(
            as_f32(r, "r")?, as_f32(g, "g")?, as_f32(b, "b")?, 1.0,
        ))),
        [r, g, b, a] => Ok(Value::Color(Color::linear_rgb(
            as_f32(r, "r")?, as_f32(g, "g")?, as_f32(b, "b")?, as_f32(a, "a")?,
        ))),
        _ => err(format!("linear_rgb() requer 3 ou 4 Float/Int, recebeu {} args", args.items.len())),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `cmyk(c, m, y, k)` →
/// `Color::Cmyk`. Componentes f32 [0.0, 1.0].
/// PDF native `/DeviceCMYK` scope-out P257 (converte para sRGB
/// via `Color::to_srgb()` no exporter; ADR-0083 §"Scope-out
/// PDF native CMYK").
pub fn native_cmyk(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i)   => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("cmyk({}): espera Float/Int, recebeu {}", name, other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [c, m, y, k] => Ok(Value::Color(Color::cmyk(
            as_f32(c, "c")?, as_f32(m, "m")?, as_f32(y, "y")?, as_f32(k, "k")?,
        ))),
        _ => err(format!("cmyk() requer 4 Float/Int, recebeu {} args", args.items.len())),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `hsl(h, s, l[, alpha])` →
/// `Color::Hsl`. `h` em graus.
pub fn native_hsl(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i)   => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("hsl({}): espera Float/Int, recebeu {}", name, other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [h, s, l] => Ok(Value::Color(Color::hsl(
            as_f32(h, "h")?, as_f32(s, "s")?, as_f32(l, "l")?, 1.0,
        ))),
        [h, s, l, a] => Ok(Value::Color(Color::hsl(
            as_f32(h, "h")?, as_f32(s, "s")?, as_f32(l, "l")?, as_f32(a, "a")?,
        ))),
        _ => err(format!("hsl() requer 3 ou 4 Float/Int, recebeu {} args", args.items.len())),
    }
}

/// **P257 (ADR-0083 PROPOSTO)** — `hsv(h, s, v[, alpha])` →
/// `Color::Hsv`. `h` em graus.
pub fn native_hsv(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
        match v {
            Value::Float(f) => Ok(*f as f32),
            Value::Int(i)   => Ok(*i as f32),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("hsv({}): espera Float/Int, recebeu {}", name, other.type_name()),
            )]),
        }
    }
    match args.items.as_slice() {
        [h, s, v] => Ok(Value::Color(Color::hsv(
            as_f32(h, "h")?, as_f32(s, "s")?, as_f32(v, "v")?, 1.0,
        ))),
        [h, s, v, a] => Ok(Value::Color(Color::hsv(
            as_f32(h, "h")?, as_f32(s, "s")?, as_f32(v, "v")?, as_f32(a, "a")?,
        ))),
        _ => err(format!("hsv() requer 3 ou 4 Float/Int, recebeu {} args", args.items.len())),
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

/// `state_display(key, [callback])` — render-mediated state display.
/// P240 (M9d/M7+1; per ADR-0081 PROPOSTO P239 Opção γ).
///
/// **Vanilla `state.display(callback)`**: durante walk, captura
/// `(key, callback)`; pós-fixpoint, `apply_state_displays`
/// (em `from_tags.rs`) chama `apply_func(callback, [state.value_at(loc)],
/// ctx, engine)` com Engine+ctx disponíveis e armazena Content
/// resultado em `intr.state_displays[(key, loc)]`. Layout arm
/// `Content::StateDisplay` consome via `state_display_value`.
/// Layouter permanece puro (sem Engine+ctx em signature).
///
/// **Forma 1-arg (`state_display(key)`)**: callback ausente; valor é
/// renderizado directamente — `Value::Content` passa-through;
/// `Value::Str` via `Content::text`; outros tipos fallback
/// `Content::Empty`.
///
/// **Forma 2-arg (`state_display(key, callback)`)**: callback aplicada
/// ao valor; resultado convertido para Content por mesma regra.
pub fn native_state_display(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn crate::contracts::world::World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        // 1-arg: sem callback (renderiza Value→Content directo pós-fixpoint).
        [Value::Str(key)] => Ok(Value::Content(
            crate::entities::content::Content::StateDisplay {
                key:      key.to_string(),
                callback: None,
            },
        )),
        // 2-arg: com callback.
        [Value::Str(key), Value::Func(callback)] => Ok(Value::Content(
            crate::entities::content::Content::StateDisplay {
                key:      key.to_string(),
                callback: Some(callback.clone()),
            },
        )),
        // 2-arg com segundo arg não-Func.
        [Value::Str(_), other] => err(format!(
            "state_display() requer função como segundo argumento (callback), recebeu {}",
            other.type_name()
        )),
        // Primeiro arg não-string (1 OR 2 args).
        [other, ..] => err(format!(
            "state_display() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        // Arity errada (0 ou 3+).
        _ => err(format!(
            "state_display() requer 1-2 argumentos (key, [callback]), recebeu {}",
            args.items.len()
        )),
    }
}

/// `counter_display(key, [callback])` — render-mediated counter display.
/// P241 (M9d/M7+2; per ADR-0081 IMPLEMENTADO parcial M7+2 paralelo
/// absoluto M7+1 P240).
///
/// **Vanilla `counter.display(callback)`**: durante walk, captura
/// `(key, callback)`; pós-fixpoint, `apply_counter_displays` (em
/// `from_tags.rs`) converte `intr.counters.value_at(key, loc)` para
/// `Value::Array(Vec<Value::Int>)` representando counter state
/// (paridade vanilla `CounterState = SmallVec<[u64; 3]>`) e chama
/// `apply_func(callback, [array], ctx, engine)`. Resultado Content
/// armazenado em `intr.counter_displays[(key, loc)]`. Layout arm
/// `Content::CounterDisplayCallback` consome via
/// `counter_display_value`.
///
/// **Forma 1-arg (`counter_display(key)`)**: callback ausente;
/// formato default "1.2.3" via join "." (paridade
/// `formatted_counter_at` P177). Counter inexistente: `Content::Empty`.
///
/// **Forma 2-arg (`counter_display(key, callback)`)**: callback
/// aplicada ao Value::Array; resultado convertido para Content
/// (paridade `native_state_display` P240). Counter inexistente:
/// callback recebe `Value::Array(vec![])`.
///
/// **Distinto de `Content::CounterDisplay { kind }` legacy
/// single-pass** — variant nova paralela coexiste (Decisão 1 P241
/// Opção α: variant nova vs refino legacy).
pub fn native_counter_display(
    _ctx:                &mut EvalContext,
    args:                &Args,
    _world:              &dyn crate::contracts::world::World,
    _current_file:       FileId,
    _figure_numbering:   Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        // 1-arg: sem callback (formato default "1.2.3" pós-fixpoint).
        [Value::Str(key)] => Ok(Value::Content(
            crate::entities::content::Content::CounterDisplayCallback {
                key:      key.to_string(),
                callback: None,
            },
        )),
        // 2-arg: com callback.
        [Value::Str(key), Value::Func(callback)] => Ok(Value::Content(
            crate::entities::content::Content::CounterDisplayCallback {
                key:      key.to_string(),
                callback: Some(callback.clone()),
            },
        )),
        // 2-arg com segundo arg não-Func.
        [Value::Str(_), other] => err(format!(
            "counter_display() requer função como segundo argumento (callback), recebeu {}",
            other.type_name()
        )),
        // Primeiro arg não-string.
        [other, ..] => err(format!(
            "counter_display() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        // Arity errada (0 ou 3+).
        _ => err(format!(
            "counter_display() requer 1-2 argumentos (key, [callback]), recebeu {}",
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

/// `state_final(key)` — valor final do state `key` pós-walk.
///
/// **P236 (Fase 5 Layout candidata Categoria D 1/? — refino aditivo)**.
///
/// State runtime mutable foi materializado em **P171/M9 + P172**
/// (`Content::State`, `Content::StateUpdate`, `native_state`,
/// `native_state_update`, `native_state_update_with`,
/// `StateRegistry`, `state_update::StateUpdate`); pipeline activo
/// via `Introspector::state_final_value` (P171). P236 expõe `state_final`
/// user-facing — paralelo absoluto a `counter_final` (P176) +
/// `counter_at` (P177).
///
/// Subset minimal: argumento posicional Str `key` → `Value` (init se
/// state nunca actualizado; valor da última update caso contrário).
/// Iteração 0 de fixpoint (`introspector` vazio) → `Value::None`.
///
/// **Divergência factual** `P236.div-1`: spec P236 assumia ADR-0066
/// PROPOSTO + state runtime ausente; audit C1 confirmou ADR-0066
/// SUPERSEDED-BY 0073 + state runtime já materializado P171+M9+M9c.
/// Materialização P236 limitada a refino aditivo `state_final` per
/// directiva humana pós-divergência.
///
/// **P240 (M9d/M7+1) — two-pass real confirmado**: audit P239 C3.3
/// identificou sobreposição grande bloqueador A (walk-time Func
/// dispatch) + D (state.final two-pass). Audit P240 C7 cenário α
/// confirmou empíricamente: `state_final_value` baseline retorna
/// `state.final_value(key)` que delega a `history.last()` em
/// `StateRegistry`. Após fixpoint convergência, `apply_state_funcs`
/// (P191B) avaliou cumulativamente `StateUpdate::Func` updates;
/// `history.last()` reflete o valor final two-pass real. Portanto
/// **`state_final` semantic já é two-pass real pós-P240** —
/// paridade vanilla `state.final()` sem refactor adicional.
pub fn native_state_final(
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
/// ao `label`. P237 (Fase 5 Layout candidata Categoria D 1/? refino
/// estendido).
///
/// **Paralelo absoluto a `counter_at(key, label)` P177** — pattern
/// "stdlib func runtime para label-based lookup" N=1 inaugurado P237
/// (counter_at baseline P177 não conta no N novo por ser anterior à
/// série Categoria D refino).
///
/// Reusa `Introspector::query_by_label` (P139+P140) +
/// `Introspector::state_value` (P171) — wrapper trivial paralelo
/// pattern counter_at literal.
///
/// Retorna `Value::None` se: key inexistente, label inexistente, ou
/// state nunca actualizado antes da Location resolved (paridade
/// `counter_at` que retorna `Value::Str("")` em ambos os casos —
/// state `Value::None` distinto pois state pode ter qualquer Value
/// type, paridade `state_final` P236).
pub fn native_state_at(
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
            // Paralelo absoluto counter_at P177: query_by_label →
            // Option<Location>; state_value(key, loc) → Option<&Value>;
            // chain via and_then; default Value::None.
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
        [other, _] => err(format!(
            "state_at() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_at() requer 2 argumentos (key, label), recebeu {}",
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
