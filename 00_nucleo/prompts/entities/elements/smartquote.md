# Prompt L0 — `entities/elements/smartquote` — `SmartQuoteElem`
Hash do Código: 3882f22f

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/smartquote.rs`
**Origem**: modelo D (ADR-0105), **Lote 9 P324** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável**. Leaf — `map_*` terminais.

---

## Struct vigente

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuoteElem {
    pub double: bool,
}
```

`Content::SmartQuote { double }` → `Content::SmartQuote(Arc<SmartQuoteElem>)`.
Construtor ergonómico: `Content::smartquote(double: bool)`. **Deriva `Hash`/`Eq`**
(`bool`).

## P1286 — configuração explícita (GATE ADR-0127)

### Medição anterior à decisão

O vanilla (`text/smartquote.rs:51-89,201-317,335-419`) distingue argumento
omitido, `auto` explícito e override single/double. A auditoria cristalina
mostrou que o carrier alternativo `Content::Styled` é transparente em
`repr_content`, mas não em `content.func()`; portanto não preserva toda a
morfologia pública da folha. O carrier local deve permanecer na própria
`Content::SmartQuote`.

### Contrato público proposto

Após confirmação, os tipos canônicos são:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuotePair {
    pub open: EcoString,
    pub close: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuoteOverrides {
    pub single: Option<SmartQuotePair>,
    pub double: Option<SmartQuotePair>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmartQuoteQuotes {
    Auto,
    Custom(SmartQuoteOverrides),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuoteElem {
    pub double: bool,
    pub alternative: Option<bool>,
    pub quotes: Option<SmartQuoteQuotes>,
}
```

`None` externo significa argumento omitido e herança da StyleChain;
`Some(false)` é override explícito de `alternative`; `Some(Auto)` é
`quotes:auto` explícito e apaga quotes herdadas. Em `Custom`, `None` por membro
significa usar o par localizado para esse membro. String/array simples
preenchem apenas `double`; o dicionário pode preencher `single` e `double`.

`Content::smartquote(double)` permanece com a assinatura pública vigente e
constrói ambos os campos novos como `None`. Igualdade/hash incluem os três
campos; `plain_text`, `is_empty` e `map_*` conservam a semântica vigente e
preservam a configuração. A forma de `content.fields()` para estes novos
argumentos não foi medida e permanece `Unknown`; não a inventar neste lote.

Adicionar campos e tipos públicos ativa **PARAGEM OBRIGATÓRIA** ADR-0127.

## `impl Element for SmartQuoteElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `if self.double { "\"".to_string() } else { "'".to_string() }` (`content.rs:1722`) |
| `is_empty` | default `false` (`content.rs:1662`) |
| `map_content`/`map_text` | **terminais** (leaf) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

No estado vigente, `#[derive(PartialEq)]` compara `double` (paridade
`content.rs:1992`). Após confirmação/materialização P1286, compara também
`alternative` e `quotes`, conforme o contrato público proposto acima.

> **Arm `|`-combinado**: `SmartQuote` está no arm terminal combinado de
> `map_content`/`map_text` com outros leaves. Como é terminal (sem binding),
> fica **no mesmo arm** (`{ .. }` → `(_)`) — **sem split**.
