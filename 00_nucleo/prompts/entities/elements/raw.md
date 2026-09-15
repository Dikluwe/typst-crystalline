# Prompt L0 — `entities/elements/raw` — `RawElem`
Hash do Código: 14b01220

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/raw.rs`
**Origem**: modelo D (ADR-0105), **Lote 7 P322** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P322). Comportamento
idêntico ao braço atual.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RawElem {
    pub text:  EcoString,
    pub lang:  Option<EcoString>,
    pub block: bool,
}
```

`Content::Raw { text, lang, block }` → `Content::Raw(Arc<RawElem>)`.
Construtor ergonómico preservado: `Content::raw(text, lang, block)`.
(P502 — `native_raw` passa a aceitar `lang:` e `block:` named; syntax highlighting real continua scope-out per ADR-0054.)
**Deriva `Hash`** (`EcoString`/`Option<EcoString>`/`bool`).

## `impl Element`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.text.to_string()` (`content.rs:1683`) |
| `is_empty` | default `false` |
| `map_content`/`map_text` | **terminais** (folha) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `text + lang + block` (paridade `content.rs:1857`).

## P1413 — `RawLineElem` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No baseline `e7b2ddd44`, o cristalino não possui `raw.line`. No vanila
ratificado, cada raw inline/bloco sintetiza uma instância por linha, inclusive
linhas vazias, e o constructor público aceita quatro argumentos requeridos.

### Contrato proposto

O mesmo owner declara `RawLineElem { number: i64, count: i64, text: EcoString,
body: Content }`. `plain_text` devolve `text`; `map_content` e `map_text`
atravessam `body`; `get_field` expõe exatamente `number`, `count`, `text` e
`body`. A identidade dinâmica estável é `raw.line`, reutilizando
`Content::Dynamic` sem ampliar o enum fechado `Content`. Igualdade e hash são
estruturais. O constructor rejeita cardinalidade ou tipos incompatíveis.

Esta ampliação de contrato público aguarda confirmação humana explícita antes
de L1.
