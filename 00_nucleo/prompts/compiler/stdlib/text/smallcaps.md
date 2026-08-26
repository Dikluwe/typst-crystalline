# Prompt L0 — `compiler/stdlib/text/smallcaps` — `smallcaps`
Hash do Código: 7769a522

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/text/smallcaps.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/text.md` — dono de `text/mod.rs`
e da história por marco (`0b97d2d78`, `e7ae938f6`). Este L0 especifica **a superfície do
nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/text/smallcaps.rs` — `SmallcapsElem`, ficheiro próprio.

**Fronteira medida**: nó de uma nativa só. A co-mudança cristalina não liga `smallcaps` a
nenhuma outra nativa de `text` fora de lotes transversais (`87bc1c64d` `Value::Symbol`/
`StyleDelta`, `04eda8179` span de `Args`, `0661aef91` `cargo fmt` global, `6636c5ea6` e
`0f5575cd0` renames de módulo). Os dois clusters aparentes — `overline`+`smallcaps` em
`0b97d2d78` (2026-06-22) e `smallcaps`+`sub`/`super` em `f28fba77d` (2026-06-24) — são
**artefacto de atribuição de fronteira** da ferramenta de co-mudança, não sinal: nos dois
commits o corpo de `smallcaps` (respectivamente de `overline`) ficou intacto; só o banner
de comentário da função *nova* caiu no intervalo da função anterior. Com a atribuição
corrigida os dois clusters desaparecem e o critério 3 fica em **vácuo** — a fronteira
vanilla (ficheiro próprio) fica de pé por não haver evidência contra ela.

---

## Contexto

`smallcaps(body)` transforma o body em versaletes. É a nativa mais simples deste
módulo: sem argumentos nomeados, sem parser interno, sem eixo de variação. Emite um
variant próprio (`Content::SmallCaps`) — ao contrário de `sub`/`super`/`highlight`, que
viajam como `Content::Styled` — porque o layout precisa de reconhecer o run inteiro para
segmentar minúsculas de maiúsculas.

## Instrução

`smallcaps(body)` — exactamente 1 posicional `Content | Str`; **zero** nomeados.

- `Content` → usado directamente; `Str` → `Content::text(s)`.
- Emite `Content::smallcaps(body)`.
- Erros: 0 posicionais → `smallcaps() exige body como argumento posicional`; tipo
  errado → `smallcaps() espera content ou string, recebeu {tipo}`; mais de 1 →
  `smallcaps() recebeu {n} argumentos posicionais (espera 1)`; qualquer nomeado →
  erro de `expect_no_named`.

O consumer de layout (fallback por escala: minúsculas em maiúsculas a `0.8×`,
maiúsculas e não-letras no tamanho corrente, flag herdada por conteúdo aninhado) é
propriedade de `compiler/layout/text.md`, não deste nó.

> **Correcção de deriva**: o comentário de `0b97d2d78` no código descreve o consumer de
> layout como "stub transparente (output byte-idêntico ao body)". Isso deixou de ser
> verdade em `e7ae938f6` (2026-06-24), que materializou o fallback por escala. Ao mover a
> função para este nó, a afirmação sobre o consumer sai do comentário — quem a quer, lê o
> L0 do layout.

**Scope-out (ADR-0054 graded)**: shaping OpenType nativo (`smcp`/`c2sc`) continua fora
de escopo; o fallback por escala é funcionalmente equivalente na maioria das fontes.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — recebidos pela assinatura
  uniforme das nativas e nunca usados.
- Nenhum helper privado neste nó: se um passo futuro precisar de um parser de
  argumentos aqui, é sinal de que a superfície cresceu e o L0 tem de ser reescrito
  antes do código.
- `expect_no_named` vem de `compiler/stdlib/mod.rs` (`pub(super)`); não reimplementar.

## Critérios de Verificação

```
#smallcaps[abc]            → Content::SmallCaps { body: "abc" }
#smallcaps("abc")          → Content::SmallCaps { body: text("abc") }
#smallcaps()               → Err "smallcaps() exige body como argumento posicional"
#smallcaps(1)              → Err "smallcaps() espera content ou string, recebeu integer"
#smallcaps[a][b]           → Err "recebeu 2 argumentos posicionais (espera 1)"
#smallcaps(foo: 1)[a]      → Err (nomeado inesperado)
#show smallcaps: it => it  → selector resolve (NodeKind::Smallcaps)
```
