# Prompt L0 — `entities/elements/math_sequence` — `MathSequenceElem`
Hash do Código: (NÃO MATERIALIZADO — aguardando decisão de primitivos de AST)

> **⏸️ NÃO MATERIALIZAR.** Decisão do dono no checkpoint P317: L0 **guardado mas
> não materializado**. `MathSequence` é **contentor de AST** (largura 21;
> paralelo direto a `Sequence`); a guideline de elegibilidade do P317 exclui-o.
> Grupo `{MathSequence, MathText, MathIdent}` + `{Sequence, Empty, Block}` →
> **decisão futura própria** (ver `math_ident.md`).

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_sequence.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait e regras
partilhadas: ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub. Preserva clone O(1) via `Arc`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathSequenceElem {
    pub nodes: Arc<[Content]>,
}
```

`Content::MathSequence(Arc<[Content]>)` → `Content::MathSequence(Arc<MathSequenceElem>)`.
Construtor ergonómico: `Content::math_sequence(nodes: Vec<Content>)` (`Arc::from`).

## `impl Element for MathSequenceElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.nodes.iter().map(|n| n.plain_text()).collect()` (`content.rs:1610`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em cada nó (`content.rs:2091`): mapeia `nodes` e devolve `Content::MathSequence(Arc::new(MathSequenceElem { nodes: Arc::from(new_nodes?) }))` |
| `map_text` | **terminal** (math structural; `content.rs:2620`): `Content::MathSequence(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `nodes` por conteúdo (`Arc<[Content]>: PartialEq`
desreferencia; paridade `content.rs:1812` `a.as_ref() == b.as_ref()`).

## Critério

`plain_text` concatena filhos; `map_content` recurse e re-`Arc`; `map_text`
terminal; igualdade por conteúdo dos nós.
