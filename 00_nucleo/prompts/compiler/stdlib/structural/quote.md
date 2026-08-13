# Prompt L0 — `compiler/stdlib/structural/quote` — `quote`
Hash do Código: 8ea15dc9

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/quote.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/quote.rs` — ficheiro próprio.

**Fronteira medida**: nó de uma nativa só. `native_quote` tem **2** commits de corpo reais
(`91a07d9bc`, `ac60d6012`), e nenhum deles toca `par` ou `footnote`. Os commits partilhados
com essas duas são todos ruído ou lote transversal: `0f5575cd0` e `6636c5ea6` (renames de
módulo), `c4978547e` (lote CSL, 8 nós), `e1f09cc24` (de-bake de fonte única, 7 nós).
Critério 3 mudo, critério 4 separa.

---

## Contexto

`quote(...)` produz uma citação em bloco ou inline. Ao contrário de `par`, emite variant
próprio (`Content::Quote`), porque a atribuição e a decisão bloco/inline têm de sobreviver
até ao layout.

## Instrução

`quote(body, attribution: ?, block: ?, quotes: ?)` — 1 posicional `Content | Str`
obrigatório.

| Nomeado | Tipo | Default | Semântica |
|---|---|---|---|
| `attribution` | `Content \| Str \| none` | `none` | fonte da citação |
| `block` | `Bool` | `false` | citação em bloco em vez de inline |
| `quotes` | `Bool` | `true` | aspas em volta do corpo |

- Emite `Content::Quote`.
- Sem body → erro; tipo errado no body → erro citando o tipo recebido; nomeado
  desconhecido → erro.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados.
- A decisão de bloco vs inline e o desenho das aspas são do layout; este nó só transporta
  os campos.
- Nenhum helper privado.

## Critérios de Verificação

```
#quote[x]                        → Content::Quote com body
#quote("x")                      → body coagido de Str
#quote(attribution: "A")[x]      → Quote com attribution
#quote(block: true)[x]           → Quote em bloco
#quote(quotes: false)[x]         → Quote sem aspas
#quote()                         → Err (body obrigatório)
#quote(zz: 1)[x]                 → Err (nomeado desconhecido)
```
