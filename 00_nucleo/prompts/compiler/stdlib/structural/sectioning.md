# Prompt L0 — `compiler/stdlib/structural/sectioning` — seccionamento e sumários
Hash do Código: a63d5a41

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/sectioning.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/{heading,outline,title,divider}.rs`. `lof`/`lot` são extensão cristalina (P472) — aliases de `outline(target:)`.

---

## Contexto

Cabeçalhos e as listagens que os consomem. `outline`, `lof` e `lot` partilham a mesma
entidade (`OutlineElem`) com `OutlineTarget` diferente — é o que os mantém no mesmo nó
(co-mudança confirmada em P763a-P765a: `native_outline` e `native_title` movem-se juntos).

## Instrução

| Nativa | Assinatura | Emite |
|---|---|---|
| `heading` | `heading(level, body, numbering: ?)` | `Content::Heading`; também selector (`#show heading:`) |
| `outline` | `outline(title: content?, depth: int?, indent: bool?)` | `Content::Outline(OutlineElem)` |
| `title` | `title(body?)` | `Content::Title` — paridade com o CLI vanilla 0.15.0 (P765a) |
| `lof` | `lof(title: ?)` | `Content::Outline` com `target = Figures` (P472) |
| `lot` | `lot(title: ?)` | `Content::Outline` com `target = Tables` (P472) |
| `divider` | `divider()` | `Content::Divider`; **não aceita argumentos** |

`heading` valida `level` dentro do intervalo e distingue `outlined` de `bookmarked`
(P605/P606 — são campos separados, não sinónimos).

## Restrições Estruturais

- L1 puro.
- `divider()` com qualquer argumento é erro — o vanilla não tem parâmetros aqui.

## Critérios de Verificação

```
= T                          → Heading level 1
#heading(level: 7)[x]        → Err (level fora do intervalo)
#heading(outlined: 1)[x]     → Err (não bool)
#heading(outlined: false)    → marca o campo sem tocar em bookmarked
#outline(depth: 2)           → Outline com depth
#lof()                       → Outline target=Figures
#divider()                   → Divider
#divider(x: 1)               → Err
```
