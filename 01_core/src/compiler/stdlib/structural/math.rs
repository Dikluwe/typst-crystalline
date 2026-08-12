//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math.md
//! @prompt-hash f7ea72b4
//! @layer L1
//! @updated 2026-08-12
//!
//! Nativas de matemática: `accent`, `cancel`, `class`, `underover`, `op`,
//! e a montagem do módulo `math`.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::entities::file_id::FileId;
use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// `accent(base, accent)` — emite `Content::MathAccent { base, accent }`.
/// Ambos posicionais obrigatórios (content ou string).
pub fn native_accent(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let base = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "accent() base espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "accent() exige base como 1.º argumento posicional".to_string(),
            )])
        }
    };
    let accent = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "accent() accent espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "accent() exige accent como 2.º argumento posicional".to_string(),
            )])
        }
    };

    // Validar ausência de named args (P296 scope-out cosméticos).
    for k in args.named.keys() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("accent(): argumento nomeado '{}' não suportado em P296 (size/dotless scope-out per ADR-0054 graded)", k),
        )]);
    }

    Ok(Value::Content(Content::math_accent(base, accent)))
}

/// `cancel(body)` — emite `Content::MathCancel { body }`.
/// Body posicional obrigatório (content ou string).
pub fn native_cancel(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "cancel() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "cancel() exige body como argumento posicional".to_string(),
            )])
        }
    };

    // Validar ausência de named args (P296 scope-out cosméticos).
    for k in args.named.keys() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("cancel(): argumento nomeado '{}' não suportado em P296 (length/inverted/cross/angle/stroke scope-out per ADR-0054 graded)", k),
        )]);
    }

    Ok(Value::Content(Content::math_cancel(body)))
}

// ── P772y — `math.class(class, body)` ────────────────────────────────────
//
// Força a `MathClass` de `body`, override do valor inferido automaticamente
// por `default_math_class`/`spacing::node_math_class`. Afecta apenas o
// espaçamento automático (`rules/math/layout/spacing.rs`); `body` é
// layoutado normalmente (`MathLayouter::layout_node`). Vanilla `ClassElem`
// (`math/mod.rs`).

/// **P825** — nome do tipo no formato longo do vanilla para mensagens de
/// cast (`found integer` etc.): `str`→`string`, `int`→`integer`,
/// `bool`→`boolean`; os restantes coincidem com `type_name()` (mesma
/// convenção de `loading.rs:561`/`pdf.rs:88`).
fn vanilla_type_name_class(v: &Value) -> &'static str {
    match v {
        Value::Str(_) => "string",
        Value::Int(_) => "integer",
        Value::Bool(_) => "boolean",
        other => other.type_name(),
    }
}

/// `class(class, body)` — emite `Content::MathClassOverride { class, body }`.
/// `class` é obrigatório e uma das **10 strings do cast vanilla**
/// (P825 — `foundations/cast.rs:502-520`; as outras 5 variantes do enum —
/// `alphabetic`, `diacritic`, `glyph-part`, `space`, `special` — são
/// internas e rejeitadas no cast); `body` é obrigatório (content ou
/// string).
pub fn native_math_class(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // **P825** — domínio do cast do vanilla + mensagem verbatim.
    const CAST_DOMAIN: [&str; 10] = [
        "normal", "punctuation", "opening", "closing", "fence", "large", "relation",
        "unary", "binary", "vary",
    ];
    const CAST_MSG: &str = "expected \"normal\", \"punctuation\", \"opening\", \
         \"closing\", \"fence\", \"large\", \"relation\", \"unary\", \"binary\", \
         or \"vary\"";

    let class_name = match args.items.first() {
        Some(Value::Str(s)) => s.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}, found {}", CAST_MSG, vanilla_type_name_class(other)),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "class() exige o nome da classe como 1.º argumento posicional"
                    .to_string(),
            )])
        }
    };
    if !CAST_DOMAIN.contains(&class_name.as_str()) {
        return Err(vec![SourceDiagnostic::error(Span::detached(), CAST_MSG)]);
    }
    // Domínio verificado acima — `parse_math_class` reconhece necessariamente.
    let class = crate::entities::math_class::parse_math_class(class_name.as_str())
        .expect("domínio do cast vanilla é subset dos 15 nomes reconhecidos");

    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        // P772y — `math.class("relation", sym.suit.heart)`: símbolo
        // Unicode como body (paridade com a conversão de markup, P471).
        Some(Value::Symbol(s)) => Content::Text(EcoString::from(s.ch)),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "class() body espera content, string ou symbol, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "class() exige body como 2.º argumento posicional".to_string(),
            )])
        }
    };

    for k in args.named.keys() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("class(): argumento nomeado '{}' não suportado", k),
        )]);
    }

    Ok(Value::Content(Content::math_class_override(class, body)))
}

// ── Passo 297 — `underover()` math (P296.1) ──────────────────────────────
//
// **HV'.a (A.0.0 N=5)**: vanilla typst NÃO tem `UnderoverElem`
// unificado — fragmenta em 12 elementos (UnderlineElem/OverlineElem/
// UnderbraceElem/OverbraceElem/etc.). Cristalino agrega num único
// variant `MathUnderover { base, under?, over? }` per ADR-0054
// graded.
//
// **A.2 → (b) Option `Box<Content>` estrutural**: primeira
// qualificação genuína "variant rico" N=5 desde P287 refutação.
// Promoção ADR meta adiada per P273.17 §0.

/// `underover(base, under: ?, over: ?)` — emite
/// `Content::MathUnderover`. Base posicional; under/over named
/// opcionais.
pub fn native_underover(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let base = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "underover() base espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "underover() exige base como argumento posicional".to_string(),
            )])
        }
    };

    // Validar named args só "under"/"over" permitidos.
    for k in args.named.keys() {
        if !["under", "over"].contains(&k.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("underover(): argumento nomeado inesperado '{}' (válidos: under, over)", k),
            )]);
        }
    }

    let under = args.named.get("under").and_then(|v| match v {
        Value::Content(c) => Some(c.clone()),
        Value::Str(s) => Some(Content::text(s.as_str())),
        Value::None => None,
        other => Some(Content::text(other.type_name())),
    });
    let over = args.named.get("over").and_then(|v| match v {
        Value::Content(c) => Some(c.clone()),
        Value::Str(s) => Some(Content::text(s.as_str())),
        Value::None => None,
        other => Some(Content::text(other.type_name())),
    });

    Ok(Value::Content(Content::math_underover(base, under, over)))
}

// ── Passo 298 — `op()` math (P296.2 fecho cluster math 4/4) ───────────────
//
// **HV'' adaptado (A.0.0 N=6 magnitude alta)**: cristalino tinha
// heurística limits-style hardcoded em `attach.rs` via
// `symbols::is_limit_function`/`is_large_operator`. P298 estende
// para suportar `MathOp { limits: true }` user-customizable.
//
// **Cross-variant interaction inaugural**: `MathOp.limits` afecta
// layout de `MathAttach` (modificação `is_limits` em `attach.rs`).
// Heurística pré-P298 preservada — fallback `MathIdent("lim")`
// continua a funcionar.

/// `op(text, limits: false)` — emite `Content::MathOp { text, limits }`.
/// Text posicional obrigatório; `limits` named opcional (default `false`).
pub fn native_op(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let text = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "op() text espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "op() exige text como argumento posicional".to_string(),
            )])
        }
    };

    // Validar named args só "limits" permitido.
    for k in args.named.keys() {
        if k.as_str() != "limits" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("op(): argumento nomeado inesperado '{}' (válido: limits)", k),
            )]);
        }
    }

    let limits = match args.named.get("limits") {
        Some(Value::Bool(b)) => *b,
        Some(Value::None) => false,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("op(limits:) espera bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    Ok(Value::Content(Content::math_op(text, limits)))
}

// ── Passo 299 — `math` module: operadores pré-definidos (P298.X) ───────────
//
// **A.0.0 N=7 magnitude baixa-modesta**: cristalino tem `calc`
// module precedente claro (P283) via `make_calc_module()` paralelo
// `make_math_module()`. **41 operadores vanilla** registados como
// `Value::Content(Content::MathOp { ... })` — 1ª aplicação prática
// de MathOp pós-materialização P298.
//
// Lista canónica vanilla `lab/.../math/op.rs:62-105`:
// - 29 scripts-style.
// - 12 limits-style.
//
// Acesso user-facing: `math.sin x`, `math.lim_(x→0) f` (namespaced).
// Heurística pré-P299 preservada — `MathIdent("lim")` literal
// continua a funcionar via fallback `is_limit_function`.

fn op_value(text: &str, limits: bool) -> Value {
    Value::Content(Content::math_op(Content::text(text), limits))
}

/// Constrói o módulo `math` como `Value::Dict` com 41 operadores
/// vanilla pré-definidos (paralelo `make_calc_module()` P283).
pub fn make_math_module() -> Value {
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();

    // Scripts-style operators (29) — limits: false.
    for name in [
        "arccos", "arcsin", "arctan", "arg", "cos", "cosh", "cot", "coth", "csc", "csch",
        "ctg", "deg", "dim", "exp", "hom", "id", "im", "ker", "lg", "ln", "log", "mod",
        "sec", "sech", "sin", "sinc", "sinh", "tan", "tanh", "tg", "tr",
    ] {
        dict.insert(name.into(), op_value(name, false));
    }

    // Limits-style operators (12) — limits: true. Multi-word usam
    // text literal vanilla (e.g. `liminf` → "lim inf").
    for (name, text) in [
        ("det", "det"),
        ("gcd", "gcd"),
        ("lcm", "lcm"),
        ("inf", "inf"),
        ("lim", "lim"),
        ("liminf", "lim inf"),
        ("limsup", "lim sup"),
        ("max", "max"),
        ("min", "min"),
        ("Pr", "Pr"),
        ("sup", "sup"),
    ] {
        dict.insert(name.into(), op_value(text, true));
    }

    // P480 — alias `equation` no módulo math para paridade de namespace vanilla.
    // Vanilla expõe `math.equation` como selector; cristalino regista aqui para
    // que `parse_selector("math.equation")` e `scope.get("math").equation`
    // resolvam. Value::None porque não existe função nativa `equation` em L1.
    dict.insert("equation".into(), Value::None);

    // P795 — dif e Dif operadores em modo math (expostos no modulo math)
    // **P962** — com wrapper `upright` (`MathStyled { italic: Some(false) }`,
    // o mesmo que `upright(d)` produz): sem ele, `apply_math_default`
    // italicava o "d" para 𝑑 (U+1D451) — o vanilla define
    // `dif = HElem(THIN, weak) + ClassElem(Unary, upright(SymbolElem('d')))`
    // (`math/op.rs:52-56`) e o "d" do diferencial é reto. Ver
    // `stdlib/structural.md` §P962 (o espaço fino fraco + classe Unary do
    // vanilla ficam registados como scope-out nessa secção).
    dict.insert(
        "dif".into(),
        Value::Content(Content::math_styled(
            None,
            None,
            Some(false),
            Content::MathText("d".into()),
            None,
        )),
    );
    dict.insert(
        "Dif".into(),
        Value::Content(Content::math_styled(
            None,
            None,
            Some(false),
            Content::MathText("D".into()),
            None,
        )),
    );

    // **P772y** — `math.class(class, body)`: override manual de `MathClass`
    // para efeitos de espaçamento automático. Vive no scope do módulo
    // `math` (não no scope global, ao contrário de `cancel`/`accent`).
    dict.insert(
        "class".into(),
        Value::Func(crate::entities::func::Func::native("class", native_math_class)),
    );

    // **P895** — `math.op(...)`: mesma função nativa `native_op` já registada
    // no scope global (`eval/mod.rs::scope.define("op", ...)`) também
    // acessível via `math.op(...)`, paridade vanilla (namespace `math`
    // reexpõe as funções de operador — achado do catálogo de terceiros em
    // `typst-passo-895-relatorio.md`, Parte B).
    dict.insert("op".into(), Value::Func(crate::entities::func::Func::native("op", native_op)));

    // **P895** — espaçamentos nomeados de modo math, nunca registados
    // (paridade vanilla `math/mod.rs:36-40,98-102`: `THIN`/`MEDIUM`/`THICK`
    // = mesmas fracções de em já usadas em `spacing.rs` para o espaçamento
    // automático por `MathClass`; `QUAD`/`WIDE` são 1em/2em). `lookup_math_op`
    // (`eval/math.rs`) resolve identificadores bare em modo math via
    // `Value::Content` no scope do módulo `math` — o mesmo mecanismo que já
    // resolve `dif`/`Dif` acima serve estes cinco sem mudança de código.
    for (name, em) in [
        ("thin", 1.0 / 6.0),
        ("med", 2.0 / 9.0),
        ("thick", 5.0 / 18.0),
        ("quad", 1.0),
        ("wide", 2.0),
    ] {
        dict.insert(
            name.into(),
            Value::Content(Content::h_space(crate::entities::layout_types::Length::em(em), false)),
        );
    }

    // **P731** — `Value::Module` (paridade vanilla — medido: `type(math)` →
    // `module`), não `Value::Dict`. `eval/math.rs::lookup_math_op` lê o
    // scope do módulo.
    let mut scope = crate::entities::scope::Scope::new();
    for (name, value) in dict {
        scope.define(name.as_str(), value);
    }
    Value::Module(crate::entities::module::Module::new("math", scope))
}

// ── `figure()` — migrada de eval.rs (Passo 64, DEBT-16) ─────────────────────

// ── Passo 397: `document(...)` e `asset(...)` ────────────────────────────────

