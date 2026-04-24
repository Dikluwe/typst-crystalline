# Prompt L0 — entities/font-list
Hash do Código: 95949e33

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/font_list.rs`
**ADRs relevantes**: ADR-0053 (materialização), ADR-0033 (paridade vanilla),
ADR-0037 (coesão por domínio)

## Contexto

`FontList` é a lista priorizada de famílias de fonte usada por
`#set text(font: ...)`. Materializado no Passo 132B para obter
paridade (parcial) com o Typst vanilla.

**Paridade parcial**: 2 das 3 forms do vanilla são aceites:
- ✅ String simples: `#set text(font: "Arial")`.
- ✅ Array de strings: `#set text(font: ("A", "B"))`.
- ❌ Dict com covers: **rejeitado** com mensagem clara até
  `regex` ser autorizado em L1 (ADR-0054 futura).

Diagnóstico prévio:
`00_nucleo/diagnosticos/diagnostico-font-list-passo-132a.md`.

## Interface pública

```rust
/// Enum inabitado — reserva forma estrutural.
pub enum Covers {}

pub struct FontFamily {
    pub name: EcoString,          // lowercased
    pub covers: Option<Covers>,   // sempre None (Covers inabitado)
}

impl FontFamily {
    pub fn new(name: EcoString) -> Self;  // normaliza para lowercase
}

pub struct FontList(Vec<FontFamily>);     // non-empty por construção

impl FontList {
    pub fn new(families: Vec<FontFamily>) -> Option<Self>;  // None se vazio
    pub fn single(name: EcoString) -> Self;                 // 1 elemento
    pub fn as_slice(&self) -> &[FontFamily];
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;       // sempre false (invariante)
}
```

## Semântica

- **Name lowercased**: `FontFamily::new("Arial")` →
  `name = "arial"`. Paridade vanilla.
- **Non-empty invariante**: `FontList::new(vec![])` devolve
  `None`. Clients não podem construir lista vazia.
- **Ordem preservada**: prioridade de fallback chain.
- **`Covers` inabitado**: `enum Covers {}` sem variantes.
  `Option<Covers>` só pode ser `None` por construção.
  Adição futura de variantes é mudança additive.

## Validação no eval (arm `"font"`)

Arm em `eval_set_text` (`rules/eval/rules.rs`) aceita:
- `Value::Str(s)` → `FontList::single(s)`.
- `Value::Array(arr)` de `Value::Str` → `FontList::new(...)`.

Rejeita (erro hard):
- `Value::Dict(_)` → `"dict form of font not yet supported —
  use string or array of strings"`.
- `Value::Array` com item não-string → `"font array must contain
  only strings"`.
- Array vazio → `"font array must not be empty"`.
- Outros tipos → `"font expects a string or array of strings"`.

## Critérios de Verificação

```
Dado FontFamily::new("Arial")
Quando called as_str()
Então "arial"                     (lowercase normalizado)

Dado FontList::single("X")
Quando called len()
Então 1

Dado FontList::new(vec![])
Quando called
Então None                        (non-empty obrigatório)

Dado FontList::new(vec![f1, f2])
Quando chamado as_slice()
Então [f1, f2]                    (ordem preservada)

Dado Covers
Quando try to match on variant
Então compile error               (enum inabitado)
```

## Não incluído (deferido)

- `covers` concreto (keyword `LatinInCjk` ou `Regex`) —
  requer `regex` crate authorization (nova ADR).
- Parser completo `Value → FontList` (vive no arm do eval).
- Integração com `FontBook` lookup / selecção de fontes.
- `Display` / formatação user-facing.
- Iteração `IntoIterator` — pode adicionar on-demand.
