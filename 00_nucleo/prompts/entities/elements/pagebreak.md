# Prompt L0 — `entities/elements/pagebreak` — `PagebreakElem`
Hash do Código: ba6ff09f

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/pagebreak.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P320). **Comando
unit** (flags `weak`/`to`) — precedente `Divider`. Comportamento idêntico.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PagebreakElem {
    pub weak: bool,
    pub to:   Option<Parity>,
}
```

`Content::Pagebreak { weak, to }` → `Content::Pagebreak(Arc<PagebreakElem>)`.
Construtor ergonómico preservado: `Content::pagebreak(weak: bool, to: Option<Parity>)`.
`Parity` de `entities::parity`.

> **`Hash` por derive — dependência do lote** (decisão do dono no checkpoint
> P320, opção a): `Parity` não implementava `Hash` (derivava `Debug, Clone,
> Copy, PartialEq, Eq`). Como é enum **`Copy + Eq` sem floats**, o `Hash`
> canónico é trivialmente correto → **adicionar `Hash` ao derive de `Parity`**
> (`entities/parity.rs`) e `PagebreakElem` deriva `Hash` normalmente. **Não é
> conserto oportunista**: é dependência directa do lote (registar no relatório).
> Contraste com `HSpace`/`VSpace`, que carregam `Length`/`f64` → mantêm o
> Debug-hash (precedente Lote 4). Regra geral no modelo (Fase B, junto ao
> derive): Debug-hash só quando o `Hash` canónico é inseguro (f64/Length); tipo
> `Copy+Eq` sem floats recebe o derive.

## `impl Element for PagebreakElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1788`) |
| `is_empty` | default `false` — **nunca vazio** (`content.rs:1614`) |
| `map_content` | **terminal** |
| `map_text` | **terminal** |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq`

`#[derive(PartialEq)]` compara `weak + to` (paridade `content.rs:1969`).

## Critério

`plain_text` vazio; `is_empty` `false`; map_* terminais; igualdade por `weak+to`;
`Hash` manual via Debug.
