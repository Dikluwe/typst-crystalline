# Prompt L0 — `entities/elements/math_ident` — `MathIdentElem`
Hash do Código: (NÃO MATERIALIZADO — aguardando decisão de primitivos de AST)

> **⏸️ NÃO MATERIALIZAR.** Decisão do dono no checkpoint P317: este L0 fica
> **guardado mas não materializado**. `MathIdent` é **primitivo de AST** (folha
> quente, largura de uso 108 — a maior do enum a seguir a `Sequence`/`Empty`) e
> a própria guideline de elegibilidade do P317 exclui-o; o override pré-tabela
> caiu. Risco de alocação `Arc` em folha quente (ADR-0029/0030). Faz parte do
> grupo `{MathSequence, MathText, MathIdent}` + `{Sequence, Empty, Block}` cuja
> migração é **decisão futura própria** (migrar c/ medição de performance,
> manter inline por design via nota na ADR-0105, ou forma terceira).

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_ident.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait e regras
partilhadas: ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317: zero refs `ElementKind`/introspecção para `Math*`). Comportamento idêntico
ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathIdentElem {
    pub ident: EcoString,
}
```

`Content::MathIdent(EcoString)` → `Content::MathIdent(Arc<MathIdentElem>)`.
Construtor ergonómico: `Content::math_ident(s: impl Into<EcoString>)`.

## `impl Element for MathIdentElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.ident.to_string()` (`content.rs:1611`) |
| `is_empty` | default `false` (math cai no `_ => false`, `content.rs:1568`) |
| `map_content` | **terminal** (folha; sem filhos): `Ok(Content::MathIdent(Arc::new(self.clone())))` |
| `map_text` | **terminal** (math structural; `content.rs:2616`): `Content::MathIdent(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `ident` (paridade `content.rs:1813`: `a == b`).

## Critério

`plain_text` devolve o identificador; map_* terminais; igualdade por `ident`.
