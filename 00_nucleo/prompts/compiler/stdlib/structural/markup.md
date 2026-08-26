# Prompt L0 — `compiler/stdlib/structural/markup` — nativas de markup inline
Hash do Código: bdfe2355

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/markup.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada destas nativas (marcos P69…P962). Este L0
especifica **a superfície do nó**; o detalhe por marco vive no pai.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/{strong,emph,link}.rs` — um ficheiro por elemento; aqui agregados por co-mudança (P101, P109, P96.5, P97-99 movem-nos sempre juntos).

---

## Contexto

Markup inline: as nativas que envolvem conteúdo sem quebrar o fluxo. Todas são
também **selectores** em show rules — `#show strong: …` chama a mesma função sem
argumentos.

## Instrução

| Nativa | Assinatura | Emite |
|---|---|---|
| `strong` | `strong(body)` | `Content::Styled([Bold(true)], body)` |
| `emph` | `emph(body)` | `Content::Styled([Italic(true)], body)` |
| `raw` | `raw(text, lang: ?, block: ?)` | `Content::Raw` — aceita **só** string, sem coerção de content |
| `link` | `link(url, body?)` | hiperligação (P422); 1.º posicional `Str`, 2.º opcional `Content` |

## Restrições Estruturais

- L1 puro. Sem `Engine`, sem I/O.
- `raw` não coage `Content` para string — a recusa é deliberada (paridade vanilla).
- Todas devem continuar utilizáveis como selector: chamada sem argumentos devolve o
  valor-sentinela que o matching de show rules reconhece.

## Critérios de Verificação

```
*a*                      → Styled([Bold(true)], "a")
_a_                      → Styled([Italic(true)], "a")
#raw("x", lang: "rs")    → Raw com lang
#raw([x])                → Err (content não aceite)
#link("u")[t]            → link com body
#show strong: it => it   → selector resolve
```
