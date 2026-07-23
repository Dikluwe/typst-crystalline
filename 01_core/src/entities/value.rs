//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/value.md
//! @prompt-hash 5bf2ca7d
//! @layer L1
//! @updated 2026-03-28

use std::sync::Arc;

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::bytes::Bytes;
use crate::entities::counter::Counter;
use crate::entities::decimal::Decimal;
use crate::entities::dir::Dir;
use crate::entities::duration::Duration;
use crate::entities::regex::Regex;
use crate::entities::selector::Selector;
use crate::entities::state::State;
use crate::entities::symbol::Symbol;
use crate::entities::version::Version;

/// Valor em tempo de avaliação do Typst.
///
/// Subset de Passo 15: 9 variantes (5 primitivos + Array, Dict, Module, Datetime).
/// As restantes (~21) são adicionadas quando os tipos dependentes
/// migrarem para L1. Não adicionar variantes sem ADR e tipo migrado.
/// Ver ADR-0017.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // ── Subset inicial (Passo 13) ────────────────────────────────────────
    /// O valor `none` do Typst.
    None,
    /// Valor booleano (`true` / `false`).
    Bool(bool),
    /// Inteiro de 64 bits com semântica de número inteiro Typst.
    Int(i64),
    /// Número de vírgula flutuante IEEE 754.
    Float(f64),
    /// String de texto. EcoString — clone O(1) (ADR-0024).
    Str(EcoString),

    // ── Variantes fáceis (Passo 15) ──────────────────────────────────────
    /// Lista de valores. Vec<Value> — clone O(n); ver DEBT.md.
    Array(Vec<Value>),
    /// Mapa string → Value preservando ordem de inserção (IndexMap, ADR-0023).
    Dict(IndexMap<EcoString, Value, FxBuildHasher>),
    /// Módulo importado. Arc<ModuleInner> internamente — clone O(1).
    Module(crate::entities::module::Module),
    /// Data/hora Typst.
    Datetime(crate::entities::world_types::Datetime),

    // ── Variantes Passo 16 ────────────────────────────────────────────────
    /// Função Typst (closure). Arc<FuncRepr> internamente — clone O(1).
    Func(crate::entities::func::Func),

    // ── Variantes Passo 18 ────────────────────────────────────────────────
    /// Conteúdo tipográfico produzido por eval().
    Content(crate::entities::content::Content),

    // ── Variantes Passo 25 (ADR-0028) ────────────────────────────────────────
    /// O valor `auto` do Typst.
    Auto,
    /// Comprimento tipográfico (pt ou em). Ver ADR-0028.
    Length(crate::entities::layout_types::Length),
    /// Comprimento relativo (`50%`, `50% + 2cm`). P469.
    Relative(crate::entities::rel::Rel<crate::entities::layout_types::Length>),
    /// Rácio (percentagem normalizada). Ver ADR-0028.
    Ratio(crate::entities::layout_types::Ratio),
    /// Ângulo (armazenado em radianos). Ver ADR-0028.
    Angle(crate::entities::layout_types::Angle),
    /// Cor tipográfica. Ver ADR-0028.
    Color(crate::entities::layout_types::Color),
    /// P227 — Stroke parametriza borders Grid/Table e refinos
    /// cosméticos Fase 5 candidata Layout per ADR-0079 PROPOSTO
    /// + ADR-0080 PROPOSTO (Opção γ literal — L0 não tocado;
    /// pattern "L0 minimal para refactors" N=7 → 8).
    Stroke(crate::entities::geometry::Stroke),
    /// Fracção para dimensionamento relativo (ex: 1fr, 2.5fr). Passo 80.
    Fraction(f64),

    // ── Variantes Passo 84.5 (encerra DEBT-36) ──────────────────────────────
    /// Alinhamento 2D — `left`, `center`, `right`, `top`, `horizon`, `bottom`
    /// e composições simbólicas (ex: `center + bottom`).
    /// `Align2D` é `Copy` (8 bytes) — sem `Arc`.
    Align(crate::entities::layout_types::Align2D),

    // ── Variantes P179 (M9 sub-passo 9 — query upgrade) ─────────────────────
    /// **P179** — Location de elemento indexado pelo Introspector.
    /// `Copy` via `Location` (u128 internal). Usado por stdlib `query`
    /// que retorna `Value::Array(Vec<Value::Location>)`.
    Location(crate::entities::location::Location),

    // ── Variantes P262 (ADR-0087) ──────────────────────────────────────────
    /// **P262** — Gradient activado per ADR-0087 (Linear only;
    /// Radial/Conic comentários reserva). User-facing
    /// `#gradient.linear(...)` retorna `Value::Gradient`.
    Gradient(crate::entities::gradient::Gradient),

    /// **P393** — Regex L1 (ADR-0077). User-facing `regex(pattern)`
    /// retorna `Value::Regex`; usável como selector em `#show`.
    Regex(crate::entities::regex::Regex),

    /// **P395** — Tiling L1 (padrão de azulejos). Abertura do portão
    /// ADR-0017; `tiling()` consumer é P396.
    Tiling(Arc<crate::entities::tiling::Tiling>),

    /// **P398** — Bytes binários opacos. Fecha DEBT-62; activa `read`
    /// binário e byte-strings CBOR.
    Bytes(Bytes),

    /// **P399** — Decimal de precisão fixa (28 dígitos). Tipo L1 puro;
    /// operações aritméticas e constructor stdlib são scope-out futuro.
    Decimal(Decimal),

    /// **P400** — Duration (intervalo de tempo). Tipo L1 puro; operações
    /// temporais e constructor stdlib são scope-out futuro.
    Duration(Duration),

    /// **P401/P684** — Version (sequência arbitrária de componentes inteiros).
    /// Tipo L1 puro; `Arc`-wrapped porque contém `Vec<u64>`.
    Version(Arc<Version>),

    /// **P417 (M)** — Selector (predicado para query/show rules).
    /// Tipo L1 puro; representa `heading.where(level: 1)` como valor
    /// de primeira classe.
    Selector(Selector),

    /// **P471** — Símbolo Unicode nomeado. Subset minimal: char + nome canónico.
    /// Modificadores encadeados (`sym.arrow.r.double`) são scope-out futuro.
    Symbol(Symbol),

    /// **P504** — Argumentos de função (`..args`) como valor de primeira
    /// classe. Exposto via `.named` e `.positional`.
    Args(crate::entities::args::Args),

    /// **P506** — Estado documental mutável (`state("key", init)`).
    State(State),

    /// **P506** — Counter documental (`counter(heading)`).
    Counter(Counter),

    /// **P509** — Etiqueta (`<name>`) como valor de primeira classe para
    /// `query(<label>)` / `locate(<label>)`.
    Label(crate::entities::label::Label),

    /// **P576** — Direcção de texto (`ltr`, `rtl`, `ttb`, `btt`).
    Dir(Dir),

    /// **P685** — Nome de tipo como valor de primeira classe (`int`, `length`,
    /// `type`, …). `Type` é `Copy` (sem payload). `type(x)` devolve esta
    /// variante; os nomes de tipo são registados no scope global como ela.
    Type(Type),
    // ── Variantes futuras — NÃO implementar sem ADR e tipo migrado ───────
    // Variantes futuras restantes:
    // Relative(Relative),       // comprimento relativo — já em L1 como tipo separado
    // Tiling(Tiling),           // padrão de azulejos — já em L1 como tipo separado
    // Version(Version),         // versão semântica — já em L1 como tipo separado
    // Bytes(Bytes),             // bytes binários — já em L1 como tipo separado
    // Decimal(Decimal),         // decimal de alta precisão — já em L1 como tipo separado
    // Duration(Duration),       // duração — já em L1 como tipo separado
    // (Content migrado no Passo 18)
    // Styles(Styles),           // estilos encadeados — bloqueia show/set
    // Args(Args),               // argumentos de função
    // Dyn(Dynamic),             // valor dinâmico opaco
}

/// **P685** — Enumerador fechado dos tipos Typst visíveis como valor.
///
/// `Copy` (sem payload). `Type::name()` coincide com `Value::type_name()`
/// para a variante correspondente. Usado por `Value::Type`, por `type(x)`
/// (`Value::type_of`) e pelos bindings globais de nomes de tipo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    None,
    Auto,
    Bool,
    Int,
    Float,
    Str,
    Array,
    Dictionary,
    Module,
    Datetime,
    Function,
    Content,
    Length,
    Ratio,
    Relative,
    Angle,
    Color,
    Stroke,
    Fraction,
    Alignment,
    Location,
    Gradient,
    Regex,
    Tiling,
    Bytes,
    Decimal,
    Duration,
    Version,
    Selector,
    Symbol,
    Arguments,
    State,
    Counter,
    Label,
    Direction,
    Type,
}

impl Type {
    /// Nome textual do tipo, idêntico a `Value::type_name()` da variante
    /// correspondente (e ao `repr` do valor-tipo no Typst).
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Auto => "auto",
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Float => "float",
            Self::Str => "str",
            Self::Array => "array",
            Self::Dictionary => "dictionary",
            Self::Module => "module",
            Self::Datetime => "datetime",
            Self::Function => "function",
            Self::Content => "content",
            Self::Length => "length",
            Self::Ratio => "ratio",
            Self::Relative => "relative",
            Self::Angle => "angle",
            Self::Color => "color",
            Self::Stroke => "stroke",
            Self::Fraction => "fraction",
            Self::Alignment => "alignment",
            Self::Location => "location",
            Self::Gradient => "gradient",
            Self::Regex => "regex",
            Self::Tiling => "tiling",
            Self::Bytes => "bytes",
            Self::Decimal => "decimal",
            Self::Duration => "duration",
            Self::Version => "version",
            Self::Selector => "selector",
            Self::Symbol => "symbol",
            Self::Arguments => "arguments",
            Self::State => "state",
            Self::Counter => "counter",
            Self::Label => "label",
            Self::Direction => "direction",
            Self::Type => "type",
        }
    }

    /// `true` para os tipos que são chamáveis como construtor no Typst
    /// (`int`, `float`, `str`, `type`). Medido na sonda P685: `int("5")`,
    /// `str(5)`, `float("3.5")`, `type(1)` funcionam; `bool(1)`, `array(1,2)`,
    /// `dictionary(a: 1)` → "type X does not have a constructor".
    pub fn is_callable(&self) -> bool {
        matches!(self, Self::Int | Self::Float | Self::Str | Self::Type)
    }
}

// P204B (M8): impl Hash via Debug formatting. Necessária para
// `#[comemo::track]` no trait `Introspector` per ADR-0073 — métodos
// como `query_metadata`, `state_value`, `state_final_value` exigem
// `Value: Hash`. Estratégia: delega à serialização Debug (mesmo
// padrão de `hash_content` em P162); aceita potenciais colisões
// estruturais por simplicidade — comemo trata colisões de hash como
// cache miss (sem prejuízo correção). Variantes `Float` e `Fraction`
// (f64) seriam impossíveis de hash via derive devido a NaN; Debug
// formato literal resolve.
impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{:?}", self).hash(state);
    }
}

impl Value {
    /// Retorna o nome do tipo Typst deste valor.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Bool(_) => "bool",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Str(_) => "str",
            Self::Array(_) => "array",
            Self::Dict(_) => "dictionary",
            Self::Module(_) => "module",
            Self::Datetime(_) => "datetime",
            Self::Func(_) => "function",
            Self::Content(_) => "content",
            Self::Auto => "auto",
            Self::Length(_) => "length",
            Self::Relative(_) => "relative length",
            Self::Ratio(_) => "ratio",
            Self::Angle(_) => "angle",
            Self::Color(_) => "color",
            Self::Stroke(_) => "stroke",
            Self::Fraction(_) => "fraction",
            Self::Align(_) => "alignment",
            Self::Location(_) => "location",
            Self::Gradient(_) => "gradient",
            Self::Regex(_) => "regex",
            Self::Tiling(_) => "tiling",
            Self::Bytes(_) => "bytes",
            Self::Decimal(_) => "decimal",
            Self::Duration(_) => "duration",
            Self::Version(_) => "version",
            Self::Selector(_) => "selector",
            Self::Symbol(_) => "symbol",
            Self::Args(_) => "arguments",
            Self::State(_) => "state",
            Self::Counter(_) => "counter",
            Self::Label(_) => "label",
            Self::Dir(_) => "direction",
            Self::Type(_) => "type",
        }
    }

    /// **P685** — O `Type` deste valor (usado por `type(x)` e pela igualdade
    /// de tipos).
    ///
    /// **P842 (achado #32 de P831)** — `Value::Relative` mapeia para
    /// `Type::Relative` e `Value::Ratio` para `Type::Ratio`. Paridade vanilla
    /// medida nos dois binários (`temp/p842/l1_type_*.typ`): `type(50%)` →
    /// ratio (literal percentual é `Value::Ratio` desde P842), `type(50% +
    /// 0pt)` → relative (Ratio + Length constrói `Rel`, mesmo com a parte
    /// absoluta zero — o tipo depende da construção, não do valor),
    /// `type(30% + 1em)` → relative. O comentário pré-P842 que afirmava
    /// `type(50% + 1pt) == length` como paridade estava errado — refutado
    /// pela medição.
    pub fn type_of(&self) -> Type {
        match self {
            Self::None => Type::None,
            Self::Auto => Type::Auto,
            Self::Bool(_) => Type::Bool,
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::Str(_) => Type::Str,
            Self::Array(_) => Type::Array,
            Self::Dict(_) => Type::Dictionary,
            Self::Module(_) => Type::Module,
            Self::Datetime(_) => Type::Datetime,
            Self::Func(_) => Type::Function,
            Self::Content(_) => Type::Content,
            Self::Length(_) => Type::Length,
            Self::Relative(_) => Type::Relative,
            Self::Ratio(_) => Type::Ratio,
            Self::Angle(_) => Type::Angle,
            Self::Color(_) => Type::Color,
            Self::Stroke(_) => Type::Stroke,
            Self::Fraction(_) => Type::Fraction,
            Self::Align(_) => Type::Alignment,
            Self::Location(_) => Type::Location,
            Self::Gradient(_) => Type::Gradient,
            Self::Regex(_) => Type::Regex,
            Self::Tiling(_) => Type::Tiling,
            Self::Bytes(_) => Type::Bytes,
            Self::Decimal(_) => Type::Decimal,
            Self::Duration(_) => Type::Duration,
            Self::Version(_) => Type::Version,
            Self::Selector(_) => Type::Selector,
            Self::Symbol(_) => Type::Symbol,
            Self::Args(_) => Type::Arguments,
            Self::State(_) => Type::State,
            Self::Counter(_) => Type::Counter,
            Self::Label(_) => Type::Label,
            Self::Dir(_) => Type::Direction,
            Self::Type(_) => Type::Type,
        }
    }

    /// Retorna true se o valor é `none`.
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Retorna true se o valor é "truthy" (semântica Typst minimal).
    ///
    /// Falsy: `none`, `false`, `0`/`0.0`, string vazia, array vazia.
    /// Todos os outros são truthy.
    pub fn truthy(&self) -> bool {
        match self {
            Self::None => false,
            Self::Bool(b) => *b,
            Self::Int(i) => *i != 0,
            Self::Float(f) => *f != 0.0,
            Self::Str(s) => !s.is_empty(),
            Self::Array(a) => !a.is_empty(),
            _ => true,
        }
    }

    /// Converte para bool, se for Bool.
    pub fn cast_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Converte para i64, se for Int.
    pub fn cast_int(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Converte para f64 (aceita Int e Float — coerção implícita do Typst).
    pub fn cast_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Converte para &str, se for Str.
    pub fn cast_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Converte para slice de Value, se for Array.
    pub fn cast_array(&self) -> Option<&[Value]> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Converte para Dict, se for Dict.
    pub fn cast_dict(&self) -> Option<&IndexMap<EcoString, Value, FxBuildHasher>> {
        match self {
            Self::Dict(d) => Some(d),
            _ => None,
        }
    }

    /// Converte para `Align2D`, se for `Align`. Passo 84.5.
    pub fn cast_align(&self) -> Option<crate::entities::layout_types::Align2D> {
        match self {
            Self::Align(a) => Some(*a),
            _ => None,
        }
    }

    /// Converte para `&Bytes`, se for `Bytes`. Passo 398.
    pub fn cast_bytes(&self) -> Option<&Bytes> {
        match self {
            Self::Bytes(b) => Some(b),
            _ => None,
        }
    }

    /// Converte para `Decimal`, se compatível. Passo 399.
    ///
    /// Aceita `Decimal` (identidade), `Int` (preciso), `Float` (com perda;
    /// rejeita NaN/Inf) e `Str` (parse decimal).
    pub fn cast_decimal(&self) -> Option<Decimal> {
        match self {
            Self::Decimal(d) => Some(*d),
            Self::Int(i) => Some(Decimal::from_i64(*i)),
            Self::Float(f) => Decimal::from_f64(*f),
            Self::Str(s) => Decimal::from_str(s),
            _ => None,
        }
    }

    /// Converte para `Duration`, se compatível. Passo 400; Passo 850 —
    /// durações negativas suportadas.
    ///
    /// Aceita `Duration` (identidade), `Int` (nanossegundos) e `Float`
    /// (segundos → nanos; rejeita overflow).
    pub fn cast_duration(&self) -> Option<Duration> {
        match self {
            Self::Duration(d) => Some(*d),
            Self::Int(i) => Some(Duration::from_nanos(*i as i128)),
            Self::Float(f) => {
                let max_seconds = i64::MAX as f64 / 1e9;
                let min_seconds = i64::MIN as f64 / 1e9;
                if *f > max_seconds || *f < min_seconds {
                    return None;
                }
                Some(Duration::from_nanos((*f * 1e9) as i128))
            }
            _ => None,
        }
    }

    /// Converte para `Arc<Version>`, se compatível. Passo 401.
    ///
    /// Aceita `Version` (identidade) e `Str` (parse semver).
    pub fn cast_version(&self) -> Option<Arc<Version>> {
        match self {
            Self::Version(v) => Some(Arc::clone(v)),
            Self::Str(s) => Version::from_str(s).map(Arc::new),
            _ => None,
        }
    }

    /// Converte para `Regex`, se compatível. Passo 402.
    ///
    /// Aceita `Regex` (identidade) e `Str` (compile; rejeita pattern inválido).
    pub fn cast_regex(&self) -> Option<Regex> {
        match self {
            Self::Regex(r) => Some(r.clone()),
            Self::Str(s) => Regex::new(s).ok(),
            _ => None,
        }
    }
}

// Conversões From para ergonomia em eval() e testes
impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Int(v)
    }
}
impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::Int(v as i64)
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}
impl From<EcoString> for Value {
    fn from(v: EcoString) -> Self {
        Self::Str(v)
    }
}
impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self::Str(v.into())
    }
}
impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::Str(v.into())
    }
}
impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Self::Array(v)
    }
}
impl From<IndexMap<EcoString, Value, FxBuildHasher>> for Value {
    fn from(v: IndexMap<EcoString, Value, FxBuildHasher>) -> Self {
        Self::Dict(v)
    }
}
impl From<crate::entities::module::Module> for Value {
    fn from(m: crate::entities::module::Module) -> Self {
        Self::Module(m)
    }
}
impl From<crate::entities::world_types::Datetime> for Value {
    fn from(d: crate::entities::world_types::Datetime) -> Self {
        Self::Datetime(d)
    }
}
impl From<crate::entities::selector::Selector> for Value {
    fn from(s: crate::entities::selector::Selector) -> Self {
        Self::Selector(s)
    }
}
impl From<crate::entities::func::Func> for Value {
    fn from(f: crate::entities::func::Func) -> Self {
        Self::Func(f)
    }
}
impl From<crate::entities::content::Content> for Value {
    fn from(c: crate::entities::content::Content) -> Self {
        Self::Content(c)
    }
}
impl From<crate::entities::layout_types::Length> for Value {
    fn from(v: crate::entities::layout_types::Length) -> Self {
        Self::Length(v)
    }
}
impl From<crate::entities::rel::Rel<crate::entities::layout_types::Length>> for Value {
    fn from(v: crate::entities::rel::Rel<crate::entities::layout_types::Length>) -> Self {
        Self::Relative(v)
    }
}
impl From<crate::entities::layout_types::Ratio> for Value {
    fn from(v: crate::entities::layout_types::Ratio) -> Self {
        Self::Ratio(v)
    }
}
impl From<crate::entities::layout_types::Angle> for Value {
    fn from(v: crate::entities::layout_types::Angle) -> Self {
        Self::Angle(v)
    }
}
impl From<crate::entities::layout_types::Color> for Value {
    fn from(v: crate::entities::layout_types::Color) -> Self {
        Self::Color(v)
    }
}
impl From<crate::entities::geometry::Stroke> for Value {
    fn from(v: crate::entities::geometry::Stroke) -> Self {
        Self::Stroke(v)
    }
}
impl From<crate::entities::tiling::Tiling> for Value {
    fn from(v: crate::entities::tiling::Tiling) -> Self {
        Self::Tiling(Arc::new(v))
    }
}
impl From<crate::entities::bytes::Bytes> for Value {
    fn from(v: crate::entities::bytes::Bytes) -> Self {
        Self::Bytes(v)
    }
}
impl From<crate::entities::decimal::Decimal> for Value {
    fn from(v: crate::entities::decimal::Decimal) -> Self {
        Self::Decimal(v)
    }
}
impl From<crate::entities::duration::Duration> for Value {
    fn from(v: crate::entities::duration::Duration) -> Self {
        Self::Duration(v)
    }
}
impl From<crate::entities::version::Version> for Value {
    fn from(v: crate::entities::version::Version) -> Self {
        Self::Version(Arc::new(v))
    }
}
impl From<crate::entities::regex::Regex> for Value {
    fn from(v: crate::entities::regex::Regex) -> Self {
        Self::Regex(v)
    }
}
impl From<crate::entities::symbol::Symbol> for Value {
    fn from(v: crate::entities::symbol::Symbol) -> Self {
        Self::Symbol(v)
    }
}

/// **P685** — `Type` → `Value::Type`. Ergonomia para registar bindings de
/// nomes de tipo (`scope.define("length", Value::from(Type::Length))`).
impl From<Type> for Value {
    fn from(t: Type) -> Self {
        Self::Type(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::Color;
    use ecow::EcoString;

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn type_names() {
        assert_eq!(Value::None.type_name(), "none");
        assert_eq!(Value::Bool(true).type_name(), "bool");
        assert_eq!(Value::Int(42).type_name(), "int");
        assert_eq!(Value::Float(3.14).type_name(), "float");
        assert_eq!(Value::Str(EcoString::from("hi")).type_name(), "str");
        assert_eq!(Value::Type(Type::Int).type_name(), "type");
    }

    // ── P685 — Value::Type e Type ────────────────────────────────────────────

    #[test]
    fn type_name_coincide_com_value_type_name() {
        // Type::name() == Value::type_name() para cada variante correspondente.
        assert_eq!(Type::Int.name(), Value::Int(0).type_name());
        assert_eq!(Type::Float.name(), Value::Float(0.0).type_name());
        assert_eq!(Type::Str.name(), Value::Str(EcoString::from("")).type_name());
        assert_eq!(Type::Length.name(), "length");
        assert_eq!(Type::Bool.name(), Value::Bool(false).type_name());
        assert_eq!(Type::Function.name(), "function");
        assert_eq!(Type::Type.name(), "type");
    }

    #[test]
    fn type_of_mapeia_variantes() {
        assert_eq!(Value::Int(1).type_of(), Type::Int);
        assert_eq!(Value::Float(1.0).type_of(), Type::Float);
        assert_eq!(Value::Bool(true).type_of(), Type::Bool);
        assert_eq!(Value::None.type_of(), Type::None);
        assert_eq!(Value::Auto.type_of(), Type::Auto);
        // P842 (#32) — relative length mapeia para Type::Relative (paridade
        // vanilla medida: `type(30% + 1em)` → relative; `50%` puro é Ratio).
        let rel = crate::entities::rel::Rel::from_percent(50.0);
        assert_eq!(Value::Relative(rel).type_of(), Type::Relative);
        // tipo de um tipo é `type`
        assert_eq!(Value::Type(Type::Int).type_of(), Type::Type);
    }

    #[test]
    fn type_equality_por_identidade() {
        // type(1) == int  (mesma variante Type)
        assert_eq!(Value::Type(Type::Int), Value::Type(Type::Int));
        // length == ratio -> false
        assert_ne!(Value::Type(Type::Length), Value::Type(Type::Ratio));
        assert_ne!(Value::Type(Type::Int), Value::Type(Type::Float));
    }

    #[test]
    fn type_is_callable() {
        assert!(Type::Int.is_callable());
        assert!(Type::Float.is_callable());
        assert!(Type::Str.is_callable());
        assert!(Type::Type.is_callable());
        assert!(!Type::Bool.is_callable());
        assert!(!Type::Length.is_callable());
        assert!(!Type::Array.is_callable());
        assert!(!Type::Dictionary.is_callable());
    }

    #[test]
    fn from_type_para_value() {
        let v: Value = Type::Length.into();
        assert_eq!(v, Value::Type(Type::Length));
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn cast_float_aceita_int() {
        assert_eq!(Value::Int(3).cast_float(), Some(3.0));
        assert_eq!(Value::Float(3.14).cast_float(), Some(3.14));
        assert_eq!(Value::Bool(true).cast_float(), None);
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn from_primitivos() {
        assert_eq!(Value::from(true), Value::Bool(true));
        assert_eq!(Value::from(42i64), Value::Int(42));
        assert_eq!(Value::from(3.14f64), Value::Float(3.14));
        assert_eq!(Value::from("hello"), Value::Str("hello".into()));
    }

    #[test]
    fn ecostring_clone_e_eq() {
        let v1 = Value::Str(EcoString::from("test"));
        let v2 = v1.clone(); // clone O(1)
        assert_eq!(v1, v2);
        assert_ne!(Value::Str("a".into()), Value::Str("b".into()));
    }

    #[test]
    fn scope_com_value_real() {
        use crate::entities::scope::Scope;
        let mut scope = Scope::new();
        scope.define("x", Value::Int(42));
        scope.define("s", Value::Str("hello".into()));
        assert_eq!(scope.get("x"), Some(&Value::Int(42)));
        assert_eq!(scope.get("s"), Some(&Value::Str("hello".into())));
    }

    #[test]
    fn value_none_is_none() {
        assert!(Value::None.is_none());
        assert!(!Value::Int(0).is_none());
    }

    #[test]
    fn array_type_name_e_cast() {
        let v = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(v.type_name(), "array");
        assert_eq!(v.cast_array().unwrap().len(), 2);
    }

    #[test]
    fn array_from_vec() {
        let v = Value::from(vec![Value::Bool(true)]);
        assert!(matches!(v, Value::Array(_)));
    }

    #[test]
    fn array_clone_is_independent() {
        // Vec<Value> clone é O(n) — verificar que são independentes
        let v1 = Value::Array(vec![Value::Int(1)]);
        let mut v2 = v1.clone();
        if let Value::Array(ref mut a) = v2 {
            a.push(Value::Int(2));
        }
        assert_eq!(v1.cast_array().unwrap().len(), 1);
    }

    #[test]
    fn dict_type_name() {
        let d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        assert_eq!(Value::Dict(d).type_name(), "dictionary");
    }

    #[test]
    fn dict_from_indexmap() {
        let mut m: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        m.insert("k".into(), Value::Int(1));
        let v = Value::from(m);
        assert!(matches!(v, Value::Dict(_)));
        assert_eq!(v.cast_dict().unwrap().get("k"), Some(&Value::Int(1)));
    }

    #[test]
    fn module_em_value_clone_barato() {
        use crate::entities::{module::Module, scope::Scope};
        let m = Module::new("test", Scope::new());
        let v1 = Value::from(m);
        let v2 = v1.clone(); // O(1) via Arc
        assert_eq!(v1.type_name(), v2.type_name());
        assert_eq!(v1.type_name(), "module");
    }

    #[test]
    fn datetime_em_value() {
        let dt = crate::entities::world_types::Datetime::new_date(2026, 3, 27).unwrap();
        assert_eq!(Value::from(dt).type_name(), "datetime");
    }

    // ── Passo 25 — tipos tipográficos (ADR-0028) ─────────────────────────────

    #[test]
    fn value_type_names_novos() {
        use crate::entities::layout_types::{Angle, Color, Length, Ratio};
        assert_eq!(Value::Length(Length::pt(12.0)).type_name(), "length");
        assert_eq!(Value::Ratio(Ratio(0.5)).type_name(), "ratio");
        assert_eq!(Value::Angle(Angle::deg(90.0)).type_name(), "angle");
        assert_eq!(Value::Color(Color::rgb(0, 0, 0)).type_name(), "color");
        assert_eq!(Value::Auto.type_name(), "auto");
    }

    // ── Passo 395 — Tiling (ADR-0017) ────────────────────────────────────────

    #[test]
    fn value_tiling_type_name() {
        use crate::entities::tiling::{Tiling, TilingBody};
        let t = Tiling::new(TilingBody::Color(Color::rgb(255, 0, 0)));
        let v = Value::from(t);
        assert_eq!(v.type_name(), "tiling");
    }

    #[test]
    fn value_tiling_partial_eq() {
        use crate::entities::tiling::{Tiling, TilingBody};
        let a = Value::from(Tiling::new(TilingBody::Color(Color::rgb(1, 2, 3))));
        let b = Value::from(Tiling::new(TilingBody::Color(Color::rgb(1, 2, 3))));
        let c = Value::from(Tiling::new(TilingBody::Color(Color::rgb(4, 5, 6))));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // ── Passo 398 — Bytes (fecha DEBT-62) ────────────────────────────────────

    #[test]
    fn value_bytes_type_name() {
        let b = Bytes::new(vec![0xDE, 0xAD]);
        assert_eq!(Value::from(b).type_name(), "bytes");
    }

    #[test]
    fn value_bytes_cast_identity() {
        let b = Bytes::new(vec![1, 2, 3]);
        let v = Value::from(b.clone());
        assert_eq!(v.cast_bytes(), Some(&b));
        assert_eq!(v.cast_int(), None);
    }

    #[test]
    fn value_bytes_partial_eq() {
        let a = Value::from(Bytes::new(vec![1, 2]));
        let b = Value::from(Bytes::new(vec![1, 2]));
        let c = Value::from(Bytes::new(vec![2, 1]));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // ── Passo 399 — Decimal (tipo S puro) ────────────────────────────────────

    #[test]
    fn value_decimal_type_name() {
        let d = Decimal::new(12345, 2);
        assert_eq!(Value::from(d).type_name(), "decimal");
    }

    #[test]
    fn value_decimal_cast_identity() {
        let d = Decimal::new(12345, 2);
        let v = Value::from(d);
        assert_eq!(v.cast_decimal(), Some(d));
        assert_eq!(v.cast_int(), None);
    }

    #[test]
    fn value_decimal_cast_from_int() {
        let v = Value::Int(42);
        assert_eq!(v.cast_decimal(), Some(Decimal::from_i64(42)));
    }

    #[test]
    fn value_decimal_cast_from_float() {
        let v = Value::Float(1.5);
        assert_eq!(v.cast_decimal(), Some(Decimal::from_f64(1.5).unwrap()));
    }

    #[test]
    fn value_decimal_cast_from_float_nan_rejected() {
        let v = Value::Float(f64::NAN);
        assert_eq!(v.cast_decimal(), None);
    }

    #[test]
    fn value_decimal_cast_from_str() {
        let v = Value::Str("3.14".into());
        assert_eq!(v.cast_decimal(), Some(Decimal::from_str("3.14").unwrap()));
    }

    #[test]
    fn value_decimal_cast_from_str_invalid() {
        let v = Value::Str("abc".into());
        assert_eq!(v.cast_decimal(), None);
    }

    #[test]
    fn value_decimal_partial_eq() {
        let a = Value::from(Decimal::new(100, 2)); // 1.00
        let b = Value::from(Decimal::new(10, 1)); // 1.0
        let c = Value::from(Decimal::new(2, 0));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // ── Passo 400 — Duration (tipo S puro) ───────────────────────────────────

    #[test]
    fn value_duration_type_name() {
        let d = Duration::from_seconds(270180);
        assert_eq!(Value::from(d).type_name(), "duration");
    }

    #[test]
    fn value_duration_cast_identity() {
        let d = Duration::from_seconds(270180);
        let v = Value::from(d);
        assert_eq!(v.cast_duration(), Some(d));
        assert_eq!(v.cast_int(), None);
    }

    #[test]
    fn value_duration_cast_from_int() {
        let v = Value::Int(1_000_000_000);
        assert_eq!(v.cast_duration(), Some(Duration::from_seconds(1)));
    }

    #[test]
    fn value_duration_cast_from_int_negative() {
        let v = Value::Int(-1_000_000_000);
        assert_eq!(v.cast_duration(), Some(-Duration::from_seconds(1)));
    }

    #[test]
    fn value_duration_cast_from_float() {
        let v = Value::Float(1.5);
        assert_eq!(v.cast_duration(), Some(Duration::from_nanos(1_500_000_000)));
    }

    #[test]
    fn value_duration_cast_from_float_negative() {
        let v = Value::Float(-1.5);
        assert_eq!(v.cast_duration(), Some(-Duration::from_nanos(1_500_000_000)));
    }

    #[test]
    fn value_duration_repr_canonical() {
        let d = Duration::from_days(3).nanos
            + Duration::from_hours(2).nanos
            + Duration::from_minutes(30).nanos;
        let v = Value::from(Duration::from_nanos(d));
        assert_eq!(v.cast_duration().unwrap().to_string(), "3d2h30m");
    }

    #[test]
    fn value_duration_repr_zero() {
        let v = Value::from(Duration::ZERO);
        assert_eq!(v.cast_duration().unwrap().to_string(), "0s");
    }

    #[test]
    fn value_duration_partial_eq() {
        let a = Value::from(Duration::from_seconds(60));
        let b = Value::from(Duration::from_minutes(1));
        let c = Value::from(Duration::from_seconds(1));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // ── Passo 401 / P684 — Version (componentes arbitrários, zero-pad) ───────

    use crate::entities::version::Version;

    #[test]
    fn value_version_type_name() {
        let v = Value::from(Version::new(1, 2, 3));
        assert_eq!(v.type_name(), "version");
    }

    #[test]
    fn value_version_cast_identity() {
        let ver = Version::new(1, 2, 3);
        let v = Value::from(ver.clone());
        let got = v.cast_version().unwrap();
        assert_eq!(*got, ver);
    }

    #[test]
    fn value_version_cast_from_str() {
        let v = Value::Str("1.2.3".into());
        let got = v.cast_version().unwrap();
        assert_eq!(*got, Version::new(1, 2, 3));
    }

    #[test]
    fn value_version_cast_from_str_arbitrary() {
        let v = Value::Str("1.2.3.4.5".into());
        let got = v.cast_version().unwrap();
        assert_eq!(*got, Version::from_components(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn value_version_cast_from_str_rejects_pre_build() {
        // P684 — `pre`/`build` textuais não existem: cast falha.
        let v = Value::Str("1.2.3-alpha.1+build.2".into());
        assert_eq!(v.cast_version(), None);
    }

    #[test]
    fn value_version_cast_from_str_invalid() {
        let v = Value::Str("abc".into());
        assert_eq!(v.cast_version(), None);
    }

    #[test]
    fn value_version_repr_canonical() {
        let v = Value::from(Version::from_components(vec![1, 2, 3, 4, 5]));
        assert_eq!(v.cast_version().unwrap().to_string(), "1.2.3.4.5");
    }

    #[test]
    fn value_version_partial_eq() {
        let a = Value::from(Version::new(1, 0, 0));
        let b = Value::from(Version::new(1, 0, 0));
        let c = Value::from(Version::new(1, 0, 1));
        assert_eq!(a, b);
        assert_ne!(a, c);
        // zero-pad: `version(1, 2, 3) == version(1, 2, 3, 0)`.
        let d = Value::from(Version::new(1, 2, 3));
        let e = Value::from(Version::from_components(vec![1, 2, 3, 0]));
        assert_eq!(d, e);
    }

    #[test]
    fn value_version_ordering_via_cast() {
        let a = Value::from(Version::from_components(vec![1, 2, 3]));
        let b = Value::from(Version::from_components(vec![1, 2, 3, 4]));
        assert!(a.cast_version().unwrap() < b.cast_version().unwrap());
    }

    // ── Passo 402 — Regex (refino de tipo L1) ────────────────────────────────

    #[test]
    fn value_regex_type_name() {
        let r = Regex::new("a.*b").unwrap();
        assert_eq!(Value::from(r).type_name(), "regex");
    }

    #[test]
    fn value_regex_cast_identity() {
        let r = Regex::new("a.*b").unwrap();
        let v = Value::from(r.clone());
        let got = v.cast_regex().unwrap();
        assert_eq!(got, r);
        assert!(got.is_match("axxxb"));
    }

    #[test]
    fn value_regex_cast_from_str() {
        let v = Value::Str("a.*b".into());
        let got = v.cast_regex().unwrap();
        assert_eq!(got.pattern(), "a.*b");
        assert!(got.is_match("axxxb"));
    }

    #[test]
    fn value_regex_cast_from_str_invalid() {
        let v = Value::Str("[".into());
        assert_eq!(v.cast_regex(), None);
    }

    #[test]
    fn value_regex_partial_eq() {
        let a = Value::from(Regex::new("a+").unwrap());
        let b = Value::from(Regex::new("a+").unwrap());
        let c = Value::from(Regex::new("b+").unwrap());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // ── P471 — Value::Symbol ─────────────────────────────────────────────────

    #[test]
    fn p471_symbol_type_name() {
        use crate::entities::symbol::Symbol;
        let s = Symbol::new('α', "alpha");
        assert_eq!(Value::Symbol(s).type_name(), "symbol");
    }

    #[test]
    fn p471_symbol_from() {
        use crate::entities::symbol::Symbol;
        let s = Symbol::new('→', "arrow");
        let v: Value = s.into();
        assert!(matches!(v, Value::Symbol(_)));
    }
}
