# Prompt L0 — `compiler/stdlib/structural/divider` — `divider`
Hash do Código: a86e3178

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/structural/divider.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/structural.md` — dono de
`structural/mod.rs` e da história acumulada. Este L0 especifica **a superfície do nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/model/divider.rs` — ficheiro próprio. (Não confundir com
`visualize/line.rs`, que é a primitiva de desenho `line()`; `divider` é semântico.)

**Fronteira medida**: nó de uma nativa só. `native_divider` tem **2** commits de corpo reais
(`fbc806c15`, `8a59bae93`) e nenhum toca `heading`, `title` ou o núcleo `outline`: os
partilhados são os dois renames de módulo, mais `c4978547e` e `e1f09cc24` (lotes
transversais de 8 e 7 nós).

> O cluster `divider`+`heading` de `fbc806c15`, que a mantinha no nó `sectioning`, era
> **artefacto**: nesse commit só o corpo de `divider` mudou. Medido e desfeito em
> 2026-08-13. O outro par aparente do mesmo commit, `divider`+`terms` (nó `lists`), é do
> mesmo lote de duas features e também não é sinal de fronteira.

---

## Contexto

`divider()` emite um separador horizontal semântico. É a nativa com a menor superfície do
módulo: **zero** argumentos, de qualquer espécie. A ausência de argumentos é deliberada e é
o que este L0 tem a fixar — o vanilla tem cosméticos que aqui estão fora de escopo.

## Instrução

`divider()` — zero posicionais, zero nomeados.

- Emite `Content::divider()`.
- Qualquer nomeado → erro de `expect_no_named`.
- Qualquer posicional → `divider() não aceita argumentos posicionais`.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados.
- `expect_no_named` vem de `compiler/stdlib/mod.rs` (`pub(super)`); não reimplementar.
- A espessura, a cor e o espaçamento do separador são do layout. Se um passo futuro
  acrescentar cosméticos (`stroke`, `length`), a superfície muda e o L0 reescreve-se antes
  do código — não se acrescenta um nomeado silenciosamente.

## Critérios de Verificação

```
#divider()          → Content::Divider
#divider(1)         → Err "divider() não aceita argumentos posicionais"
#divider[x]         → Err (idem, posicional)
#divider(zz: 1)     → Err (nomeado inesperado, via expect_no_named)
```
