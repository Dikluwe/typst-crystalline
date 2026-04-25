//! `ValueDTO` — representação neutra de `Value` para comparação
//! de paridade P2 (eval).
//!
//! Materializado no Passo 153. Conversões:
//! - `from_cristalino(&typst_core::Value) -> ValueDTO` real.
//! - `from_vanilla_stub() -> ValueDTO` placeholder; vanilla
//!   integration é DEBT-54 → fecho DEBT-53.
//!
//! Variants 1:1 com `Value` cristalino actual (18 variants
//! per inventário 148, pós-149). Casos de divergência
//! arquitectural (ADR-0058 `Value::Type` simplificado;
//! ADR-0059 `Value::Args` não-variant) tratados via:
//! - `ValueDTO::Type(String)` — nome textual (cristalino devolve
//!   `Value::Str(type_name)`; aqui mantemos como `Str` por falta
//!   de contexto, divergência observável quando vanilla integrar).
//! - `ValueDTO::Args` — **inexistente**; vanilla `Value::Args`
//!   mapearia para `Other("args")`.
//!
//! Comparação P2 (per `typst-paridade-definicoes.md` §P2):
//! - `Float`: bits idênticos via `f64::to_bits()`.
//! - `Func`: por nome (closures sem nome → `"<closure>"`).
//! - `Array`/`Dict`: estrutural; `Dict` preserva ordem de
//!   inserção (IndexMap em cristalino).
//! - `Content`: `plain_text` extraído (sub-DTO completo é
//!   trabalho futuro).

use typst_core::entities::value::Value;

/// Representação neutra de `Value`. Variants em ordem
/// alfabética para fácil verificação visual.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueDTO {
    /// Valor de alinhamento (`Align2D`). Serializado como Debug.
    Align(String),
    /// Comprimento. Serializado como Debug.
    Angle(String),
    /// Array de valores.
    Array(Vec<ValueDTO>),
    /// Auto. Singleton.
    Auto,
    /// Boolean.
    Bool(bool),
    /// Cor. Serializada como Debug.
    Color(String),
    /// Conteúdo. Texto plano extraído via `Content::plain_text`.
    Content(String),
    /// Data/hora. Serializada como Debug.
    Datetime(String),
    /// Dicionário preservando ordem de inserção.
    Dict(Vec<(String, ValueDTO)>),
    /// Float — comparação por bits (`f64::to_bits()`).
    Float(u64),
    /// Fracção (`fr`). Numero raw.
    Fraction(u64),
    /// Função identificada por nome. `"<closure>"` para sem-nome.
    Func(String),
    /// Inteiro.
    Int(i64),
    /// Comprimento. Serializada como Debug.
    Length(String),
    /// Módulo identificado por nome.
    Module(String),
    /// Valor `none`.
    None,
    /// Catch-all. Variants vanilla sem equivalente cristalino
    /// (ex: vanilla `Args`, `Bytes`, `Decimal`, etc.) mapeiam
    /// para `Other("variant_name")`.
    Other(String),
    /// Razão / percentagem.
    Ratio(String),
    /// String. Em cristalino também é o resultado de `type(x)`
    /// (ADR-0058) — divergência só visível com vanilla DTO.
    Str(String),
    /// Tipo como valor. **Vazio em cristalino** (ADR-0058);
    /// reservado para vanilla via DEBT-54.
    Type(String),
}

impl ValueDTO {
    /// Conversão a partir do `Value` cristalino. Cobre os 18
    /// variants existentes em `01_core/src/entities/value.rs`.
    pub fn from_cristalino(v: &Value) -> Self {
        match v {
            Value::None         => ValueDTO::None,
            Value::Auto         => ValueDTO::Auto,
            Value::Bool(b)      => ValueDTO::Bool(*b),
            Value::Int(i)       => ValueDTO::Int(*i),
            Value::Float(f)     => ValueDTO::Float(f.to_bits()),
            Value::Str(s)       => ValueDTO::Str(s.to_string()),
            Value::Array(arr)   => ValueDTO::Array(
                arr.iter().map(Self::from_cristalino).collect(),
            ),
            Value::Dict(d)      => ValueDTO::Dict(
                d.iter()
                    .map(|(k, v)| (k.to_string(), Self::from_cristalino(v)))
                    .collect(),
            ),
            Value::Module(m)    => ValueDTO::Module(m.name().to_string()),
            Value::Datetime(d)  => ValueDTO::Datetime(format!("{d:?}")),
            Value::Func(f)      => ValueDTO::Func(
                f.name().unwrap_or("<closure>").to_string(),
            ),
            Value::Content(c)   => ValueDTO::Content(c.plain_text()),
            Value::Length(l)    => ValueDTO::Length(format!("{l:?}")),
            Value::Ratio(r)     => ValueDTO::Ratio(format!("{r:?}")),
            Value::Angle(a)     => ValueDTO::Angle(format!("{a:?}")),
            Value::Color(c)     => ValueDTO::Color(format!("{c:?}")),
            Value::Fraction(fr) => ValueDTO::Fraction(fr.to_bits()),
            Value::Align(al)    => ValueDTO::Align(format!("{al:?}")),
        }
    }

    /// Stub. Vanilla integration é DEBT-54.
    pub fn from_vanilla_stub() -> Self {
        ValueDTO::Other("vanilla_stub".into())
    }

    /// Comparação semântica. Usa `PartialEq` derivado.
    pub fn compare(&self, other: &Self) -> ValueComparison {
        if self == other {
            ValueComparison::Equal
        } else {
            ValueComparison::Differ {
                crist:   format!("{self:?}"),
                vanilla: format!("{other:?}"),
            }
        }
    }

    /// Nome canónico do tipo — útil para sumarizar matriz.
    pub fn type_name(&self) -> &'static str {
        match self {
            ValueDTO::None       => "none",
            ValueDTO::Auto       => "auto",
            ValueDTO::Bool(_)    => "bool",
            ValueDTO::Int(_)     => "int",
            ValueDTO::Float(_)   => "float",
            ValueDTO::Str(_)     => "str",
            ValueDTO::Array(_)   => "array",
            ValueDTO::Dict(_)    => "dict",
            ValueDTO::Module(_)  => "module",
            ValueDTO::Datetime(_) => "datetime",
            ValueDTO::Func(_)    => "function",
            ValueDTO::Content(_) => "content",
            ValueDTO::Length(_)  => "length",
            ValueDTO::Ratio(_)   => "ratio",
            ValueDTO::Angle(_)   => "angle",
            ValueDTO::Color(_)   => "color",
            ValueDTO::Fraction(_) => "fraction",
            ValueDTO::Align(_)   => "alignment",
            ValueDTO::Type(_)    => "type",
            ValueDTO::Other(_)   => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueComparison {
    Equal,
    Differ {
        crist:   String,
        vanilla: String,
    },
}
