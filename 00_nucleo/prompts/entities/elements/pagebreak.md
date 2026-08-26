# Prompt L0 — `entities/elements/pagebreak` — `PagebreakElem`
Hash do Código: 92ccb6c5

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/pagebreak.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P320). **Comando
unit** (flags `weak`/`to`) — precedente `Divider`. Comportamento idêntico.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PagebreakElem {
    pub weak:          bool,
    pub weak_explicit: bool,
    pub to:            Option<Parity>,
}
```

`Content::Pagebreak { weak, to }` → `Content::Pagebreak(Arc<PagebreakElem>)`.
Construtor ergonómico preservado: `Content::pagebreak(weak: bool, to: Option<Parity>)`.
`Parity` de `entities::parity`.

**P1140.9 — contrato público e presença.** `weak_explicit` conserva se o named
foi fornecido para que `pagebreak()` e `pagebreak(weak: false)` tenham `repr`
distinto. O construtor existente mantém a assinatura e trata `true` como
explícito e `false` como omitido. Eval usa
`pagebreak_with_weak_presence(weak, weak_explicit, to)`. Layout usa somente
`weak`; `to` permanece inalterado.

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

`#[derive(PartialEq)]` compara `weak + weak_explicit + to`. O `Hash` derivado
inclui os três campos.

## Critério

`plain_text` vazio; `is_empty` `false`; map_* terminais; igualdade por
`weak+weak_explicit+to`;
`Hash` **por derive** (não Debug-hash).

> **Correcção de contradição interna** (2026-08-13). Esta linha dizia "`Hash` manual via
> Debug", em contradição com a secção `Struct` do próprio prompt e com o código. Medição:
> `entities/elements/pagebreak.rs:20` é `#[derive(Debug, Clone, PartialEq, Hash)]`, sem
> `impl Hash` manual no ficheiro; `entities/parity.rs:26` deriva `Hash` também
> (`Copy + Eq` sem floats), que é a dependência de lote registada na nota acima. O lado
> errado era **o L0**, e era esta linha — a intenção está escrita na nota da secção
> `Struct` e bate com o código. O Debug-hash é o precedente de `HSpace`/`VSpace`, que
> carregam `Length`/`f64`; não se aplica aqui.
