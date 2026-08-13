# Prompt L0 — `compiler/stdlib/structural/footnote` — `footnote` (construção)
Hash do Código: c43a00aa

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/footnote.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/footnote.rs` — ficheiro próprio.

**Fronteira medida**: nó de uma nativa só. `native_footnote` tem **3** commits de corpo
reais (`c823fca5b`, `dfef736af`, `fac57c4d1`) e nenhum toca `par`; com `quote` partilha só
`c4978547e` e `e1f09cc24` — os dois lotes transversais. Critério 3 mudo, critério 4 separa.

**Fronteira de fase — verificada, não presumida**: existe um `compiler/layout/footnote.rs`
(L0 próprio: `compiler/layout/footnote.md`). **Não há duplicação de responsabilidade**:

| Nível | Ficheiro | O que faz |
|---|---|---|
| construção (este nó) | `stdlib/structural/footnote.rs` | valida argumentos e emite `Content::footnote_with_numbering(body, numbering)` |
| render | `compiler/layout/footnote.rs` | `layout(...)`, estado `pending_footnote_bodies`, marcador inline via `format_pattern` + `Content::superscript` |

A nativa não sabe numerar nem posicionar; o layout não valida argumentos. A fronteira é a
fase do pipeline, e mexer nela é gate ADR-0127.

---

## Contexto

`footnote(...)` marca o ponto de chamada e transporta o corpo da nota. Quem decide o
número, o marcador e a posição na página é o layout — este nó só constrói o elemento.

## Instrução

`footnote(body, numbering: ?)` — 1 posicional `Content | Str` obrigatório.

- `Content` → usado directamente; `Str` → `Content::text(s)`.
- `numbering: Str | none` → pattern de numeração; `none` remove-o. Tipo diferente de
  string → `footnote(numbering:): espera string, recebeu {tipo}`.
- Emite `Content::footnote_with_numbering(body, numbering)`.
- Sem body → `footnote() exige body como argumento posicional`; tipo errado →
  `footnote() espera content ou string, recebeu {tipo}`.
- **Qualquer outro nomeado** → erro que cita o scope-out graded (ADR-0054), não
  "argumento inesperado" genérico: os cosméticos do vanilla estão deliberadamente fora.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados. Em especial: o
  contador de notas **não** vive aqui (é estado do layout), e introduzi-lo neste nó seria
  mudança de fase do pipeline — paragem obrigatória (ADR-0127).
- Nenhum helper privado.

## Critérios de Verificação

```
#footnote[x]                   → Content::Footnote com body, numbering None
#footnote("x")                 → body coagido de Str
#footnote(numbering: "a")[x]   → Footnote com pattern "a"
#footnote(numbering: none)[x]  → Footnote sem pattern
#footnote(numbering: 1)[x]     → Err "footnote(numbering:): espera string, recebeu integer"
#footnote()                    → Err "footnote() exige body como argumento posicional"
#footnote(1)                   → Err "footnote() espera content ou string, recebeu integer"
#footnote(zz: 1)[x]            → Err citando o scope-out graded (ADR-0054)
```
