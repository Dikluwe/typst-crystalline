# Prompt L0 — `compiler/stdlib/structural/heading` — `heading`
Hash do Código: b692aa47

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/heading.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/heading.rs` — ficheiro próprio.

**Fronteira medida**: nó de uma nativa só, e a de maior churn da família — **7** commits de
corpo reais (`1aea00338`, `d2faea55d`, `fdd39b892`, `f14f71b0c`, `e01e39f68`, `439d4dbd6`,
`971d9e3b6`). Nenhum deles toca `title`, `divider`, `outline`, `lof` ou `lot`: os commits
partilhados com esses cinco são **todos** ruído ou lote — `0661aef91` (`cargo fmt` global),
`0f5575cd0` e `6636c5ea6` (renames de módulo), `c4978547e` (lote CSL, 8 nós), `e1f09cc24`
(de-bake de fonte única, 7 nós).

> Os dois clusters que antes a mantinham no nó `sectioning` — `heading`+`outline` em
> `3197135b0` e `divider`+`heading` em `fbc806c15` — eram **artefacto de atribuição de
> fronteira**: em ambos o corpo de `heading` ficou intacto. Medido e desfeito em
> 2026-08-13.

Critério 3 diz que `heading` anda sozinha; critério 4 (ficheiro próprio no vanilla)
concorda. Nó próprio, com as duas fontes a apontar no mesmo sentido.

---

## Contexto

`heading(...)` cria um cabeçalho e é também **selector** de show rules
(`#show heading: it => …`). A superfície é a mais tolerante do módulo: o vanilla aceita
quatro formas de chamada, e a paridade é com a **linguagem** — todas têm de funcionar.

Dois eixos que este nó tem de manter separados e visíveis:

1. **`outlined` vs `bookmarked`** são flags distintas — a primeira controla o índice do
   documento (`#outline()`), a segunda a árvore `/Outlines` do PDF. `bookmarked` ausente
   (`None`) **segue** `outlined`, que é o `auto` do vanilla.
2. **Campos assentes** (`set_fields`): o elemento registra quais campos foram dados
   explicitamente, porque `has`/`at`/`fields` do vanilla distinguem "assente com o valor
   default" de "não assente".

## Instrução

Quatro formas de chamada, todas válidas:

```
#heading(1, [b])            level posicional + body posicional
#heading[b]                 body posicional, level = 1
#heading(level: 2)[b]       body posicional, level named
#heading(level: 2, body: [b])   ambos named
```

- 1.º posicional `Int` → `level`, com `body` no 2.º posicional. `level` fora de `1..=6` →
  `heading(): level deve estar entre 1 e 6, recebeu {n}`. `Int` sozinho → `heading() exige
  body`.
- 1.º posicional `Content | Str` → é o `body`, e `level` default é `1`.
- 1.º posicional de outro tipo → `heading(): primeiro argumento deve ser int (level) ou
  content/string (body), recebeu {tipo}`.
- Sem posicionais → `body` vem do named; ausente também aí → `heading() exige body`.
- `level:` named **sobrepõe-se** ao default `1` das formas body-only; tipo errado →
  `heading(level:): espera int, recebeu {tipo}`; fora de `1..=6` → mesma mensagem de range.

| Nomeado | Tipo | Default | Semântica |
|---|---|---|---|
| `numbering` | `Str \| none` | `None` | pattern (`"1."`, `"I."`, `"(a)"`) |
| `outlined` | `Bool \| none` | `true` | entra no `#outline()` |
| `bookmarked` | `Bool \| none` | `None` = segue `outlined` | entra em `/Outlines` do PDF |

**`set_fields`** marca `HEADING_SET_LEVEL` quando `level` vem como posicional `Int` **ou**
como named, e `HEADING_SET_OUTLINED` quando `outlined` aparece como named (mesmo com valor
`true`). `bookmarked` é auto-rastreado pelo `Option`.

Emissão: com `numbering` → `Content::heading_numbered_native(level, body, pattern, outlined,
bookmarked, set_fields)`; sem → `Content::heading_native(level, body, outlined, bookmarked,
set_fields)`.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados. O contador de
  cabeçalhos é do layout/introspecção; numerar aqui seria mudança de fase (ADR-0127).
- `set_fields` é **contrato de introspecção**, não detalhe interno: mudar quando um campo
  conta como assente muda `has`/`at`/`fields` observáveis pelo documento.
- Nenhum helper privado.

## Critérios de Verificação

```
#heading(1, [b])              → Heading{level:1, body:b}, SET_LEVEL assente
#heading[b]                   → level 1, SET_LEVEL não assente
#heading(level: 2)[b]         → level 2, SET_LEVEL assente
#heading(level: 2, body: [b]) → level 2 via named
#heading(7, [b])              → Err "level deve estar entre 1 e 6, recebeu 7"
#heading(0, [b])              → Err (mesma mensagem, recebeu 0)
#heading(2)                   → Err "heading() exige body"
#heading()                    → Err "heading() exige body"
#heading(1.5, [b])            → Err "primeiro argumento deve ser int (level) ou content/string (body)"
#heading(level: "2")[b]       → Err "heading(level:): espera int, recebeu string"
#heading(numbering: "1.")[b]  → heading numerado
#heading(numbering: 1)[b]     → Err "heading(numbering:): espera string, recebeu integer"
#heading(outlined: false)[b]  → outlined false, SET_OUTLINED assente
#heading(outlined: true)[b]   → outlined true, SET_OUTLINED assente (named conta)
#heading(bookmarked: true)[b] → bookmarked Some(true)
#heading(bookmarked: 1)[b]    → Err "heading(bookmarked:): espera bool, recebeu integer"
#show heading: it => it       → selector resolve
```
