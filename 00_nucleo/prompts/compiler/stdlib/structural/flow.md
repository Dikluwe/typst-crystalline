# Prompt L0 — `compiler/stdlib/structural/flow` — fluxo de bloco
Hash do Código: 02503a10

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/flow.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/{par,quote,footnote}.rs`. `native_par`/`native_quote` co-mudam (P806).

---

## Contexto

Nativas que produzem blocos no fluxo vertical.

## Instrução

| Nativa | Assinatura | Nota |
|---|---|---|
| `par` | `par(body, leading: ?)` | **P806** — devolve o body como `Content` (parágrafo implícito); `leading` embrulha em `Styled`; `justify` é aceite e ignorado |
| `quote` | `quote(body, attribution: ?, block: false, quotes: true)` | body posicional obrigatório |
| `footnote` | `footnote(body, numbering: ?)` | body posicional obrigatório (content ou string) |

## Restrições Estruturais

- L1 puro.
- `par` sem body é erro com a mensagem **literal do vanilla** — a mecânica é o
  observável aqui (ADR-0108), logo a string conta.

## Critérios de Verificação

```
#par[x]                      → Content devolvido directamente
#par("x")                    → string convertida em Content
#par(leading: 2pt)[x]        → Styled com leading custom
#par(justify: true)[x]       → aceite, sem efeito
#par()                       → Err (mensagem literal do vanilla)
#par(1)                      → Err "expected content"
#par(zz: 1)                  → Err (nomeado desconhecido)
#quote[x]                    → Quote
#footnote[x]                 → Footnote
```
