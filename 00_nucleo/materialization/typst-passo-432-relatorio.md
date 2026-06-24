# Passo 432 — relatório: fecho do subset `rules/stdlib/layout.rs` (DEBT-57)

**Tipo:** documental / especificação L0 (zero código funcional modificado).  
**Data:** 2026-06-24. **HEAD base:** `76b48bc46`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=8388608`.

## O que se fez

Fechou-se o subset `rules/stdlib/layout.rs` do débito DEBT-57, criando um
prompt L0 dedicado e actualizando a linhagem `@prompt` para que o ficheiro
desse de depender exclusivamente do prompt grosseiro `stdlib/_comum.md`.

- **L0 novo:**
  - `00_nucleo/prompts/rules/stdlib/layout.md` — spec dedicada ao módulo
    `layout.rs`.
- **Cabeçalhos `@prompt` ajustados:**
  - `01_core/src/rules/stdlib/layout.rs` — `_comum.md`, `stdlib/layout.md`
    (esta última em último lugar, conforme convenção do linter para prompt
    "dono" do ficheiro).
- **Prompt comum actualizado:**
  - `00_nucleo/prompts/rules/stdlib/_comum.md` — removido `layout.rs` da
    lista de ficheiros que apontam para o prompt comum; adicionada nota sobre
    o novo `layout.md`.
- **Débito actualizado:**
  - `00_nucleo/diagnosticos/debt/DEBT.md` — anotado que o subset
    `layout.rs` foi fechado em P432; corrigida a contagem de funções de 17
    para 16 (`native_page` foi removido no P335).

`cargo test --workspace` verde; `crystalline-lint` sem novas violações
(mantêm-se apenas os dois warnings pré-existentes: `adr-stub-vs-fallback.md`
e `show-regex.md`).

## Nota sobre a contagem de funções

O plano original (P432) listava 17 funções nativas no ficheiro, incluindo
`page`. O inventário real de `layout.rs` contém **16** funções `native_*`:

`align`, `place`, `grid`, `pad`, `hide`, `h`, `v`, `block`, `stack`, `box`,
`repeat`, `columns`, `colbreak`, `measure`, `stroke`, `pagebreak`.

A forma-função `page(...)` foi removida no P335; o caminho canónico em Typst
é `#set page(...)` via `eval_set_rule`. O prompt `layout.md` documenta as 16
funções e inclui uma nota sobre a ausência de `native_page`.

## Decisão de engenharia

- **Atomização por ficheiro (ADR-0104):** cada módulo `stdlib/*.rs` merece o
  seu próprio prompt L0 fino. `layout.rs` era o próximo subset mais
  fechável depois de `structural.rs` (P430).
- **Scope-outs preservados:**
  - `measure(body)` runtime queries genuínas — continua diferido per
    ADR-0066.
  - `columns()` / `colbreak()` multi-region flow real — scope-out per
    ADR-0078.
  - `place()` float real — scope-out já materializado no P245.
- **Linhagem `@prompt` e linter:** o `crystalline-lint` considera o *último*
  `@prompt` de cada ficheiro como a referência efectiva para efeitos de
  detecção de órfãos. `layout.md` foi colocado em último em `layout.rs`.
  `asset.md` continua referenciado por `layout/mod.rs` (último), portanto
  não ficou órfão.

## Verificação

| Critério | Resultado |
|----------|-----------|
| `00_nucleo/prompts/rules/stdlib/layout.md` criado | ✓ 16 secções |
| Cada secção cobre assinatura, args, semântica, paridade vanilla, limitações e testes canónicos | ✓ |
| `_comum.md` actualizado | ✓ |
| `DEBT.md` actualizado com nota de fecho P432 | ✓ |
| `crystalline-lint` sem novas violações | ✓ |
| `cargo test --workspace` verde | ✓ |
| Zero código funcional modificado | ✓ |

## Artefactos

- L0:
  - `00_nucleo/prompts/rules/stdlib/layout.md`
  - `00_nucleo/prompts/rules/stdlib/_comum.md` (actualizado)
- Código (apenas cabeçalho de linhagem):
  - `01_core/src/rules/stdlib/layout.rs`
  - `01_core/src/rules/stdlib/assert.rs` (hash sincronizado)
  - `01_core/src/rules/stdlib/gradients.rs` (hash sincronizado)
  - `01_core/src/rules/stdlib/transforms.rs` (hash sincronizado)
- Débito:
  - `00_nucleo/diagnosticos/debt/DEBT.md`
- Plano:
  - `00_nucleo/materialization/typst-passo-432.md`
- este relatório.

## Próximo passo

Com P432 fechado, continuam em aberto em DEBT-57 os subsets `calc.rs`,
`shapes.rs`, `transforms.rs`, `gradients.rs`, `assert.rs` e `foundations.rs`.
Alternativas fora de DEBT-57: **DEBT-43** (linter type-level) ou **DEBT-55**
(probe hayagriva).
