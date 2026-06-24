# Passo 435 — relatório: fecho do subset `rules/stdlib/shapes.rs` (DEBT-57)

**Tipo:** documental / especificação L0 (zero código funcional modificado).  
**Data:** 2026-06-24. **HEAD base:** `2cfb0d030`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=8388608`.

## O que se fez

Fechou-se o subset `rules/stdlib/shapes.rs` do débito DEBT-57, criando um
prompt L0 dedicado e actualizando a linhagem `@prompt` para que o ficheiro
desse de depender exclusivamente do prompt grosseiro `stdlib/_comum.md`.

- **L0 novo:**
  - `00_nucleo/prompts/rules/stdlib/shapes.md` — spec dedicada às 6 funções
    geométricas do subset (`rect`, `ellipse`, `circle`, `line`, `polygon`,
    `curve`).
- **Cabeçalhos `@prompt` ajustados:**
  - `01_core/src/rules/stdlib/shapes.rs` — `_comum.md`, `shapes.md`,
    `square.md` (este último em último lugar, conforme convenção do linter
    para prompt "dono" do ficheiro). `square.md` continua a documentar o
    helper morfológico `square`.
  - `01_core/src/rules/stdlib/gradients.rs` — adicionada referência a
    `shapes.md` em último lugar, garantindo que `shapes.md` não fique órfão
    (paridade P430: re-referenciar prompt dedicado noutro ficheiro L1 quando
    o ficheiro original já tem outro prompt como "dono").
- **Prompt comum actualizado:**
  - `00_nucleo/prompts/rules/stdlib/_comum.md` — removido `shapes.rs` da
    lista de ficheiros que apontam para o prompt comum; adicionada nota sobre
    o novo `shapes.md`.
- **Débito actualizado:**
  - `00_nucleo/diagnosticos/debt/DEBT.md` — adicionada entrada para
    `rules/stdlib/shapes.rs` marcada como fechada em P435; critério de fecho
    actualizado.

`cargo test --workspace` verde; `crystalline-lint` sem novas violações
(mantêm-se apenas os dois warnings pré-existentes: `adr-stub-vs-fallback.md`
e `show-regex.md`).

## Decisão de engenharia

- **Atomização por ficheiro (ADR-0104):** `shapes.rs` era o próximo subset
  mais fechável depois de `assert.rs` (P434).
- **Manutenção de `square.md`:** o helper `square` já tinha prompt dedicado
  (`square.md`). Em vez de absorvê-lo ou marcá-lo como órfão, mantivemo-lo
  como prompt auxiliar referenciado por `shapes.rs`.
- **Linhagem `@prompt` e linter:** como `shapes.rs` já hospeda `square.md`
  como prompt "dono" (último), `shapes.md` foi re-referenciado em
  `gradients.rs` para evitar órfão. Este padrão segue o estabelecido no
  P430 (`asset.md` re-referenciado em `layout/mod.rs`).
- **Scope-outs preservados:** `cmyk`/`oklab`/`oklch`/`linear_rgb`/`hsl`/
  `hsv`, gradientes como `Value::Gradient` (fora do scope de `shapes.rs`,
  apesar de `parse_paint` os aceitar), e API de constructores por scope do
  `curve` continuam scope-out.

## Verificação

| Critério | Resultado |
|----------|-----------|
| `00_nucleo/prompts/rules/stdlib/shapes.md` criado com 6 secções | ✓ |
| Cada secção cobre assinatura, args, semântica, paridade vanilla, limitações e testes canónicos | ✓ |
| `_comum.md` actualizado | ✓ |
| `shapes.rs` cabeçalho `@prompt` aponta `shapes.md` | ✓ |
| `shapes.md` não ficou órfão (re-referenciado em `gradients.rs`) | ✓ |
| `DEBT.md` actualizado com nota de fecho P435 | ✓ |
| `crystalline-lint` sem novas violações | ✓ |
| `cargo test --workspace` verde | ✓ |
| Zero código funcional modificado | ✓ |

## Artefactos

- L0:
  - `00_nucleo/prompts/rules/stdlib/shapes.md`
  - `00_nucleo/prompts/rules/stdlib/_comum.md` (actualizado)
- Código (apenas cabeçalhos de linhagem / hashes):
  - `01_core/src/rules/stdlib/shapes.rs`
  - `01_core/src/rules/stdlib/gradients.rs`
  - `01_core/src/rules/stdlib/transforms.rs` (hash sincronizado)
- Débito:
  - `00_nucleo/diagnosticos/debt/DEBT.md`
- Plano:
  - `00_nucleo/materialization/typst-passo-435.md`
- este relatório.

## Próximo passo

Com P435 fechado, continuam em aberto em DEBT-57 os subsets `transforms.rs`,
`gradients.rs` e `foundations.rs`. Alternativas fora de DEBT-57: **DEBT-43**
(linter type-level) ou **DEBT-55** (probe hayagriva).
