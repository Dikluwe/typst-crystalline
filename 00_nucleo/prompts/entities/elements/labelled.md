# Prompt L0 — `entities/elements/labelled` — `LabelledElem`
Hash do Código: d24c96d1

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/labelled.rs`
**Origem**: modelo D (ADR-0105), **Lote 14 P329** (reclassificado da triagem
DEBT-58 — wrapper denso, não primitivo). Trait: ver `entities/elements/_comum.md`.
Contentor — `map_*` recursam no `target`.

> **Não-locatável no sentido do trait** (sem `element_kind`/`to_payload`): o
> `extract_payload` **não** matcheia `Labelled` (não é pre-recursion). Mas a
> introspecção **consome `Labelled` directamente** num arm de walk
> (`introspect.rs:944`) que emite `ElementPayload::Labelled` em **pós-recursão**
> (P195B — `resolved_text` depende de state mutado durante o walk recursivo).
> A migração **não muda esse mecanismo**; só converte o lado `Content::Labelled`
> dos arms (walk, materialize, fixpoint) de struct-pattern para `(e)` — sites no
> plano de toque.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LabelledElem {
    pub target: Content,           // era Box<Content>
    pub label:  Label,
}
```

`Content::Labelled { target, label }` → `Content::Labelled(Arc<LabelledElem>)`.
Construtor ergonómico: `Content::labelled(target, label)`. **Deriva `Hash`**
(`Label(String)` deriva `Eq + Hash`; `Content` tem `impl Hash` manual).

## `impl Element for LabelledElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.target.plain_text()` (`content.rs:1700`) |
| `is_empty` | **override**: `self.target.is_empty()` (`content.rs:1573`) |
| `map_content` | **recursivo** no `target`, preserva `label` (`content.rs:1981`) |
| `map_text` | **recursivo** no `target`, preserva `label` (`content.rs:2184`) |
| `get_field`/`element_kind`/`to_payload` | default (o payload é emitido pelo walk arm, não pelo trait) |

## `eq`

`#[derive(PartialEq)]` compara `target`/`label` (paridade `content.rs:1818`).
