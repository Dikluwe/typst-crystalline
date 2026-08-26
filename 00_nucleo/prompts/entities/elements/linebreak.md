# Prompt L0 — `entities/elements/linebreak` — `LinebreakElem`
Hash do Código: 2a86c30d

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/linebreak.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320** (quebras/espaços + grid/table
header/footer). Trait e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**
(confirmado P320). **Comando unit** — precedente `Divider`. Comportamento
idêntico ao braço atual do hub.

---

## Struct — P1140.10

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LinebreakElem {
    pub justify: bool,
    pub justify_explicit: bool,
}
```

`Content::Linebreak` → `Content::Linebreak(Arc<LinebreakElem>)`.
`Content::linebreak()` mantém a assinatura e produz `justify = false,
justify_explicit = false`; é a forma usada pela sintaxe markup e pelos
consumidores internos. O caminho da função global usa
`Content::linebreak_with_justify_presence(justify, justify_explicit)`.

**Medição (vanilla pinado `a51e02804`, 2026-08-24):**
`repr(linebreak())` → `"linebreak()"`; `repr(linebreak(justify: false))` →
`"linebreak(justify: false)"`; `.fields()` e `.has("justify")` também
distinguem omissão de false explícito. Logo `justify_explicit` é mecânica
interna necessária a observáveis de linguagem (ADR-0107).

**Divisão materializada para LTR:** P1140.10 materializa e transporta os dois campos,
binding, repr e reflexão. P1140.11 consome `justify` em
`compiler/layout/linebreak.rs`: true expande oportunidades da linha anterior
com fórmula derivada da largura corrente; false e omissão preservam a quebra
normal. Valores empíricos pertencem exclusivamente aos oracles posicionais.

## `impl Element for LinebreakElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `"\n".to_string()` (`content.rs:1683`) |
| `is_empty` | default `false` (não está no match de `is_empty`) |
| `map_content` | **terminal**, preserva os dois campos |
| `map_text` | **terminal** (`content.rs` bloco terminal) |
| `get_field("justify")` | `Value::Bool` somente quando `justify_explicit`; o dispatcher de reflexão distingue unset de undeclared |
| `element_kind`/`to_payload` | default `None` |

## `eq`

`#[derive(PartialEq, Hash)]` inclui `justify + justify_explicit`, pois os campos
alteram `repr` e reflexão.

## Critério

`plain_text` `"\n"`; map_* terminais e preservam campos; igualdade/hash por
`justify+justify_explicit`; omissão distinta de false explícito; layout true
expande a linha anterior e false/markup não expandem. P1140.12 preserva a
fronteira explícita até o reflow bidi; RTL true/false coincide sem constante
empírica.
