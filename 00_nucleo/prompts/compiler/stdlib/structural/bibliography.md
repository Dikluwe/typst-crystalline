# Prompt L0 — `compiler/stdlib/structural/bibliography` — bibliografia e citação
Hash do Código: 23a7b25e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/bibliography.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/{bibliography,cite}.rs`. Co-mudança: P159a/c/d/e/g movem `extract_bib_entries`, `native_bibliography` e `native_cite` sempre juntos.

---

## Contexto

`bibliography` e `cite` são um par com acoplamento semântico no próprio vanilla: a
citação só resolve contra as entradas registadas pela bibliografia.

**Fronteira de camada**: este nó **não lê ficheiros**. As entradas chegam já parseadas
como `Value::Array<Value::Dict>` — o parsing de BibTeX vive em `compiler/eval/bibtex.rs`
e o carregamento em L3. É o que mantém o nó em L1.

## Instrução

| Nativa | Assinatura |
|---|---|
| `bibliography` | `bibliography(entries: array, title: ?, style: ?, locale: ?)` → `Content::Bibliography` |
| `cite` | `cite(key, supplement: ?, form: ?, style: ?)` → `Content::Cite` |

### Extractores (privados ao nó)

- `extract_bib_entries` — coage `Array<Dict>` para `Vec<BibEntry>` conforme P159A §5 +
  P159D/E/G §5.1.
- `extract_citation_form` (P159C) e `extract_citation_style` (P468) — parsing de
  `Value::Str` com **matching estrito e sensível a maiúsculas**; `auto`/`none` → `None`.

## Restrições Estruturais

- L1 puro: zero I/O, zero acesso a `World` para ficheiros `.bib`.
- O matching de forma/estilo é estrito por decisão registada — não aceitar variantes
  case-insensitive nem abreviaturas.

## Critérios de Verificação

```
#bibliography(entries)                  → Content::Bibliography
#cite("key")                            → Content::Cite
#cite("k", form: "prose")               → form reconhecida
#cite("k", form: "Prose")               → Err (matching sensível a maiúsculas)
#cite("k", form: auto)                  → None
#bibliography(entries, style: "ieee")   → estilo reconhecido
```
