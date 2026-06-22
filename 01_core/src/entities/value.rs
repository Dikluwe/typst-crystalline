//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/value.md
//! @prompt-hash c9faa625
//! @layer L1
//! @updated 2026-03-28

use std::sync::Arc;

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::bytes::Bytes;
use crate::entities::decimal::Decimal;
use crate::entities::duration::Duration;
use crate::entities::regex::Regex;
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

    /// **P401** — Version (semver). Tipo L1 puro; `Arc`-wrapped porque contém
    /// `Vec<EcoString>`; constructor stdlib e comparações são scope-out futuro.
    Version(Arc<Version>),

    // ── Variantes futuras — NÃO implementar sem ADR e tipo migrado ───────
    // Variantes futuras (~9 restantes após P262):
    // Relative(Relative),       // comprimento relativo
    // Tiling(Tiling),           // padrão de azulejos
    // Symbol(Symbol),           // símbolo Unicode
    // Version(Version),         // versão semântica — já em L1 como tipo separado
    // Bytes(Bytes),             // bytes binários — já em L1 como tipo separado
    // Decimal(Decimal),         // decimal de alta precisão — já em L1 como tipo separado
    // Duration(Duration),       // duração — já em L1 como tipo separado
    // (Content migrado no Passo 18)
    // Styles(Styles),           // estilos encadeados — bloqueia show/set
    // Args(Args),               // argumentos de função
    // Type(Type),               // tipo como valor (int, str, etc.)
    // Dyn(Dynamic),             // valor dinâmico opaco
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
            Self::None       => "none",
            Self::Bool(_)    => "bool",
            Self::Int(_)     => "int",
            Self::Float(_)   => "float",
            Self::Str(_)     => "str",
            Self::Array(_)   => "array",
            Self::Dict(_)    => "dictionary",
            Self::Module(_)  => "module",
            Self::Datetime(_)=> "datetime",
            Self::Func(_)    => "function",
            Self::Content(_) => "content",
            Self::Auto         => "auto",
            Self::Length(_)    => "length",
            Self::Ratio(_)     => "ratio",
            Self::Angle(_)     => "angle",
            Self::Color(_)     => "color",
            Self::Stroke(_)    => "stroke",
            Self::Fraction(_)  => "fraction",
            Self::Align(_)     => "alignment",
            Self::Location(_)  => "location",
            Self::Gradient(_)  => "gradient",
            Self::Regex(_)     => "regex",
            Self::Tiling(_)    => "tiling",
            Self::Bytes(_)     => "bytes",
            Self::Decimal(_)   => "decimal",
            Self::Duration(_)  => "duration",
            Self::Version(_)   => "version",
        }
    }

    /// Retorna true se o valor é `none`.
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Converte para bool, se for Bool.
    pub fn cast_bool(&self) -> Option<bool> {
        match self { Self::Bool(b) => Some(*b), _ => None }
    }

    /// Converte para i64, se for Int.
    pub fn cast_int(&self) -> Option<i64> {
        match self { Self::Int(i) => Some(*i), _ => None }
    }

    /// Converte para f64 (aceita Int e Float — coerção implícita do Typst).
    pub fn cast_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Int(i)   => Some(*i as f64),
            _ => None,
        }
    }

    /// Converte para &str, se for Str.
    pub fn cast_str(&self) -> Option<&str> {
        match self { Self::Str(s) => Some(s.as_str()), _ => None }
    }

    /// Converte para slice de Value, se for Array.
    pub fn cast_array(&self) -> Option<&[Value]> {
        match self { Self::Array(a) => Some(a), _ => None }
    }

    /// Converte para Dict, se for Dict.
    pub fn cast_dict(&self) -> Option<&IndexMap<EcoString, Value, FxBuildHasher>> {
        match self { Self::Dict(d) => Some(d), _ => None }
    }

    /// Converte para `Align2D`, se for `Align`. Passo 84.5.
    pub fn cast_align(&self) -> Option<crate::entities::layout_types::Align2D> {
        match self { Self::Align(a) => Some(*a), _ => None }
    }

    /// Converte para `&Bytes`, se for `Bytes`. Passo 398.
    pub fn cast_bytes(&self) -> Option<&Bytes> {
        match self { Self::Bytes(b) => Some(b), _ => None }
    }

    /// Converte para `Decimal`, se compatível. Passo 399.
    ///
    /// Aceita `Decimal` (identidade), `Int` (preciso), `Float` (com perda;
    /// rejeita NaN/Inf) e `Str` (parse decimal).
    pub fn cast_decimal(&self) -> Option<Decimal> {
        match self {
            Self::Decimal(d) => Some(*d),
            Self::Int(i)     => Some(Decimal::from_i64(*i)),
            Self::Float(f)   => Decimal::from_f64(*f),
            Self::Str(s)     => Decimal::from_str(s),
            _                => None,
        }
    }

    /// Converte para `Duration`, se compatível. Passo 400.
    ///
    /// Aceita `Duration` (identidade), `Int` (nanossegundos; rejeita
    /// negativos) e `Float` (segundos → nanos; rejeita negativos e overflow).
    pub fn cast_duration(&self) -> Option<Duration> {
        match self {
            Self::Duration(d) => Some(*d),
            Self::Int(i) if *i >= 0 => Some(Duration::from_nanos(*i as u64)),
            Self::Float(f) if *f >= 0.0 => {
                let max_seconds = u64::MAX as f64 / 1e9;
                if *f > max_seconds {
                    return None;
                }
                Some(Duration::from_nanos((*f * 1e9) as u64))
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
impl From<bool>      for Value { fn from(v: bool)      -> Self { Self::Bool(v) } }
impl From<i64>       for Value { fn from(v: i64)       -> Self { Self::Int(v) } }
impl From<i32>       for Value { fn from(v: i32)       -> Self { Self::Int(v as i64) } }
impl From<f64>       for Value { fn from(v: f64)       -> Self { Self::Float(v) } }
impl From<EcoString> for Value { fn from(v: EcoString) -> Self { Self::Str(v) } }
impl From<&str>      for Value { fn from(v: &str)      -> Self { Self::Str(v.into()) } }
impl From<String>    for Value { fn from(v: String)    -> Self { Self::Str(v.into()) } }
impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self { Self::Array(v) }
}
impl From<IndexMap<EcoString, Value, FxBuildHasher>> for Value {
    fn from(v: IndexMap<EcoString, Value, FxBuildHasher>) -> Self { Self::Dict(v) }
}
impl From<crate::entities::module::Module> for Value {
    fn from(m: crate::entities::module::Module) -> Self { Self::Module(m) }
}
impl From<crate::entities::world_types::Datetime> for Value {
    fn from(d: crate::entities::world_types::Datetime) -> Self { Self::Datetime(d) }
}
impl From<crate::entities::func::Func> for Value {
    fn from(f: crate::entities::func::Func) -> Self { Self::Func(f) }
}
impl From<crate::entities::content::Content> for Value {
    fn from(c: crate::entities::content::Content) -> Self { Self::Content(c) }
}
impl From<crate::entities::layout_types::Length> for Value {
    fn from(v: crate::entities::layout_types::Length) -> Self { Self::Length(v) }
}
impl From<crate::entities::layout_types::Ratio> for Value {
    fn from(v: crate::entities::layout_types::Ratio) -> Self { Self::Ratio(v) }
}
impl From<crate::entities::layout_types::Angle> for Value {
    fn from(v: crate::entities::layout_types::Angle) -> Self { Self::Angle(v) }
}
impl From<crate::entities::layout_types::Color> for Value {
    fn from(v: crate::entities::layout_types::Color) -> Self { Self::Color(v) }
}
impl From<crate::entities::geometry::Stroke> for Value {
    fn from(v: crate::entities::geometry::Stroke) -> Self { Self::Stroke(v) }
}
impl From<crate::entities::tiling::Tiling> for Value {
    fn from(v: crate::entities::tiling::Tiling) -> Self { Self::Tiling(Arc::new(v)) }
}
impl From<crate::entities::bytes::Bytes> for Value {
    fn from(v: crate::entities::bytes::Bytes) -> Self { Self::Bytes(v) }
}
impl From<crate::entities::decimal::Decimal> for Value {
    fn from(v: crate::entities::decimal::Decimal) -> Self { Self::Decimal(v) }
}
impl From<crate::entities::duration::Duration> for Value {
    fn from(v: crate::entities::duration::Duration) -> Self { Self::Duration(v) }
}
impl From<crate::entities::version::Version> for Value {
    fn from(v: crate::entities::version::Version) -> Self { Self::Version(Arc::new(v)) }
}
impl From<crate::entities::regex::Regex> for Value {
    fn from(v: crate::entities::regex::Regex) -> Self { Self::Regex(v) }
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
        let v2 = v1.clone();  // clone O(1)
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
        let v2 = v1.clone();  // O(1) via Arc
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
        assert_eq!(Value::Ratio(Ratio(0.5)).type_name(),        "ratio");
        assert_eq!(Value::Angle(Angle::deg(90.0)).type_name(),  "angle");
        assert_eq!(Value::Color(Color::rgb(0, 0, 0)).type_name(), "color");
        assert_eq!(Value::Auto.type_name(),                     "auto");
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
        let b = Value::from(Decimal::new(10, 1));  // 1.0
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
    fn value_duration_cast_from_int_negative_rejected() {
        let v = Value::Int(-1);
        assert_eq!(v.cast_duration(), None);
    }

    #[test]
    fn value_duration_cast_from_float() {
        let v = Value::Float(1.5);
        assert_eq!(v.cast_duration(), Some(Duration::from_nanos(1_500_000_000)));
    }

    #[test]
    fn value_duration_cast_from_float_negative_rejected() {
        let v = Value::Float(-1.0);
        assert_eq!(v.cast_duration(), None);
    }

    #[test]
    fn value_duration_repr_canonical() {
        let d = Duration::from_days(3)
            .nanos
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

    // ── Passo 401 — Version (tipo S puro) ────────────────────────────────────

    use crate::entities::version::Version;

    fn version_ids(parts: &[&str]) -> Vec<EcoString> {
        parts.iter().map(|s| EcoString::from(*s)).collect()
    }

    #[test]
    fn value_version_type_name() {
        let v = Value::from(Version::new(1, 2, 3));
        assert_eq!(v.type_name(), "version");
    }

    #[test]
    fn value_version_cast_identity() {
        let ver = Version::new(1, 2, 3).with_pre(version_ids(&["alpha", "1"]));
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
    fn value_version_cast_from_str_with_pre_build() {
        let v = Value::Str("1.2.3-alpha.1+build.2".into());
        let got = v.cast_version().unwrap();
        assert_eq!(got.major, 1);
        assert_eq!(got.pre, version_ids(&["alpha", "1"]));
        assert_eq!(got.build, version_ids(&["build", "2"]));
    }

    #[test]
    fn value_version_cast_from_str_invalid() {
        let v = Value::Str("abc".into());
        assert_eq!(v.cast_version(), None);
    }

    #[test]
    fn value_version_repr_canonical() {
        let v = Value::from(Version::new(1, 2, 3).with_pre(version_ids(&["alpha", "1"])));
        assert_eq!(v.cast_version().unwrap().to_string(), "1.2.3-alpha.1");
    }

    #[test]
    fn value_version_partial_eq() {
        let a = Value::from(Version::new(1, 0, 0));
        let b = Value::from(Version::new(1, 0, 0));
        let c = Value::from(Version::new(1, 0, 1));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn value_version_ordering_via_cast() {
        let a = Value::from(Version::new(1, 0, 0).with_pre(version_ids(&["alpha"])));
        let b = Value::from(Version::new(1, 0, 0));
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
}
