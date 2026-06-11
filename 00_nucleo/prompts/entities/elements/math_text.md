# Prompt L0 — `entities/elements/math_text` — `MathTextElem`
Hash do Código: (NÃO MATERIALIZADO — aguardando decisão de primitivos de AST)

> **⏸️ NÃO MATERIALIZAR.** Decisão do dono no checkpoint P317: L0 **guardado mas
> não materializado**. `MathText` é **folha de AST** (largura 50); a guideline de
> elegibilidade do P317 exclui-o (risco de `Arc` em folha quente, ADR-0029/0030).
> Grupo `{MathSequence, MathText, MathIdent}` + `{Sequence, Empty, Block}` →
> **decisão futura própria** (ver `math_ident.md`).

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_text.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait e regras
partilhadas: ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathTextElem {
    pub text: EcoString,
}
```

`Content::MathText(EcoString)` → `Content::MathText(Arc<MathTextElem>)`.
Construtor ergonómico: `Content::math_text(s: impl Into<EcoString>)`.

## `impl Element for MathTextElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.text.to_string()` (`content.rs:1612`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **terminal** (folha): `Ok(Content::MathText(Arc::new(self.clone())))` |
| `map_text` | **terminal** (math structural; `content.rs:2616`): `Content::MathText(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `text` (paridade `content.rs:1814`).

## Critério

`plain_text` devolve o literal; map_* terminais; igualdade por `text`.
