# Prompt L0 — `Regex` — wrapper L1 sobre `regex::Regex`
Hash do Código: b57030cc

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/regex.rs`, `01_core/src/entities/value.rs`
**Criado em**: 2026-05-12 (P209D sub-passo — wrapper L1 sobre
`regex::Regex` crate para `Selector::Regex` per ADR-0077).
**Refinado em**: 2026-06-22 (P402 — refino para tipo de primeiro-cidadão:
`EcoString` + `Arc<regex::Regex>`, integração `Value::Regex`, cast).
**ADRs relevantes**: ADR-0077 (regex em L1); ADR-0017 (portão aberto);
ADR-0107 (paridade linguagem); ADR-0029 (pureza L1); ADR-0054 (operações
regex scope-out); DEBT-52 (`text.font` dict desbloqueado para futuro).

---

## Contexto

P209D introduziu `Selector::Regex(Regex)` com wrapper L1 sobre `regex::Regex`.
P393 activou `Value::Regex(Regex)` para `regex(pattern)` em `#show`. P402
refina o tipo para primeiro-cidadão pleno: `pattern` passa a `EcoString`,
`compiled` passa a `Arc<regex::Regex>` (clone barato), e adiciona-se cast
`Str → Regex`.

## Restrições estruturais

- Camada **L1**: struct puro, sem I/O.
- `Hash` + `PartialEq` + `Eq` manuais via `pattern`.
- `Clone` via derive sobre `Arc<regex::Regex>` — clone O(1).
- `Debug` manual (`compiled` é opaco do crate).
- Dep `regex` autorizada via ADR-0077 + `crystalline.toml` + `01_core/Cargo.toml`.

## Interface pública

```rust
use ecow::EcoString;
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum RegexError {
    #[error("regex inválida: {0}")]
    Invalid(String),
}

#[derive(Clone)]
pub struct Regex {
    pattern: EcoString,
    compiled: Arc<regex::Regex>,
}

pub struct RegexMatch {
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub captures: Vec<Option<String>>,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self, RegexError>;
    pub fn pattern(&self) -> &str;
    pub fn is_match(&self, text: &str) -> bool;
    /// **P689** — primeiro match: posições em **bytes** (paridade vanilla),
    /// texto do match e capturas dos grupos (em ordem; grupo não-participante → None (P1075)).
    pub fn captures_first(&self, text: &str) -> Option<RegexMatch>;
}

impl Hash for Regex      { /* via pattern */ }
impl PartialEq for Regex { /* via pattern */ }
impl Eq for Regex        {}
impl Debug for Regex     { /* Regex { pattern: "..." } */ }
impl Default for Regex   { /* pattern vazia */ }
```

## Semântica

- `Regex::new(pattern)`: valida pattern; erro contextual.
- `pattern()`: pattern original.
- `is_match(text)`: delega ao `regex::Regex` compilado.
- `captures_first(text)` (**P689**): delega a `regex::Regex::captures`; devolve o
  primeiro match com índices em **bytes**, o texto e as capturas dos grupos em ordem
  (grupo opcional não participante → `None` (P1075: paridade estrita com vanilla `str.rs:936-940`)).
  Base de `str.match` e de `str.position(regex)`. Grupos nomeados entram na ordem
  posicional (paridade vanilla).

> **Fonte de paridade (P1031)** — vanilla ratificado (`e0e8ca4d`). A superfície de linguagem
> correspondente é `str.match`, cujo doc comment `#[func]`
> (`crates/typst-library/src/foundations/str.rs:426-437`, publicado em
> `typst.app/docs/reference/foundations/str/#definitions-match`) diz:
>
> *"Searches for the specified pattern in the string and returns a dictionary with details
> about the first match or `{none}` if there is no match. The returned dictionary has the
> following keys: - `start`: The start offset of the match - `end`: The end offset of the
> match - `text`: The text that matched. - `captures`: An array containing a string for each
> matched capturing group. **The first item of the array contains the first matched
> capturing, not the whole match!** This is empty unless the `pattern` was a regex with
> capturing groups."*
>
> Isto sustenta literalmente: (a) a forma do resultado; (b) que as capturas **excluem** o
> match completo — o `skip(1)` de `captures_to_dict` (`str.rs:930-941`); (c) a ordem
> posicional.
>
> **Offsets em bytes — confirmado por medição (2026-08-13).** Documento
> `#let m = "ação: (x)".match(regex("\((.)\)"))`:
>
> | Binário | Resultado |
> |---|---|
> | Vanilla `/usr/local/bin/typst` (`typst 0.15.1 (e0e8ca4d)`) | `start=8 end=11 text=(x) caps=1` |
> | Cristalino `target/release/typst` (fonte em HEAD `4f64e4e69`) | `start=8 end=11 text=(x) caps=1` |
>
> `"ação: "` tem 8 bytes UTF-8 e 6 codepoints; ambos dão 8 → **bytes**, em paridade. ✅
>
> **ACHADO CORRIGIDO (P1075) — grupo não participante devolve `none` (`Value::None`).**
> O vanilla mapeia explicitamente para `Value::None` (`str.rs:936-940`):
> ```rust
> "captures" => cap.iter().skip(1)
>     .map(|opt| opt.map_or(Value::None, |m| m.as_str().into_value()))
>     .collect::<Array>(),
> ```
> Medição, documento `#let m = "ab".match(regex("a(x)?(b)"))` + `#repr(m.captures)`:
>
> | Binário | `captures` |
> |---|---|
> | Vanilla | `(none, "b")` |
> | Cristalino | `("", "b")` |
>
> É superfície de linguagem (o valor observável de um campo do dicionário devolvido), logo
> paridade — e é diferença semântica real: `none` e `""` distinguem-se em comparações e em
> `type()`. Mudança de comportamento por defeito → gate ADR-0127 e passo próprio.
> **Não implementado aqui.**
- `Hash`/`PartialEq`/`Eq`: por `pattern` (valor linguagem).
- `Clone`: partilha `Arc<regex::Regex>`; não recompila.

## Consumers

- `entities::selector::Selector::Regex(Regex)` — P209D.
- `entities::value::Value::Regex(Regex)` — P393 + P402 refino.
- `cast_regex()` em `Value`: `Regex` identidade; `Str` → compile fallible.

## Tests obrigatórios

- `regex_new_valido_ok`, `regex_new_invalido_err`.
- `regex_is_match_basico`.
- `regex_eq_via_pattern`, `regex_clone_preserva_semantica`, `regex_hash_determinismo`.
- `value_regex_type_name`, `value_regex_cast_identity`, `value_regex_cast_from_str`,
  `value_regex_cast_from_str_invalid`, `value_regex_partial_eq`.

## Não-objectivos

- `text.font` dict — DEBT-52, depende de refactor futuro.
- Flags regex — scope-out ADR-0054.
- `str.replace` com regex — scope-out (P689 cobre apenas `str.match`/`str.position`).
- Query `Selector::Regex` sobre Content text — continua stub.


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — identidade pública do tipo `regex`

O nome global `regex` é o valor `Value::Type(Type::Regex)`, chamável
pelo dispatcher estático e delegado ao construtor L1 já existente. Assim,
`type(regex) == type` e `repr(type(regex)) == "type"`, enquanto valores
construídos continuam com `type_name() == "regex"`. A representação interna
da entidade e suas operações não mudam.
