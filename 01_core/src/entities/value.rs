//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/value.md
//! @prompt-hash 02423035
//! @layer L1
//! @updated 2026-03-28

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

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
    /// Fracção para dimensionamento relativo (ex: 1fr, 2.5fr). Passo 80.
    Fraction(f64),

    // ── Variantes Passo 84.5 (encerra DEBT-36) ──────────────────────────────
    /// Alinhamento 2D — `left`, `center`, `right`, `top`, `horizon`, `bottom`
    /// e composições simbólicas (ex: `center + bottom`).
    /// `Align2D` é `Copy` (8 bytes) — sem `Arc`.
    Align(crate::entities::layout_types::Align2D),

    // ── Variantes futuras — NÃO implementar sem ADR e tipo migrado ───────
    // Variantes futuras (~13 restantes após Passo 80):
    // Relative(Relative),       // comprimento relativo
    // Gradient(Gradient),       // gradiente
    // Tiling(Tiling),           // padrão de azulejos
    // Symbol(Symbol),           // símbolo Unicode
    // Version(Version),         // versão semântica
    // Bytes(Bytes),             // bytes binários — já em L1 como tipo separado
    // Decimal(Decimal),         // decimal de alta precisão
    // Duration(Duration),       // duração
    // (Content migrado no Passo 18)
    // Styles(Styles),           // estilos encadeados — bloqueia show/set
    // Args(Args),               // argumentos de função
    // Type(Type),               // tipo como valor (int, str, etc.)
    // Dyn(Dynamic),             // valor dinâmico opaco
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
            Self::Fraction(_)  => "fraction",
            Self::Align(_)     => "alignment",
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
