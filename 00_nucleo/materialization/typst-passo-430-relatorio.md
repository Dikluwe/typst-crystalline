# Passo 430 — relatório: fecho do subset `rules/stdlib/structural.rs` (DEBT-57)

**Tipo:** documental / especificação L0 (zero código modificado).  
**Data:** 2026-06-23. **HEAD base:** `d2be53465`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=8388608`.

## O que se fez

Fechou-se o subset `rules/stdlib/structural.rs` do débito DEBT-57, criando um prompt L0 dedicado e actualizando a linhagem `@prompt` para que o ficheiro deixe de depender exclusivamente do prompt grosseiro `stdlib/_comum.md`.

- **L0 novo:**
  - `00_nucleo/prompts/engine/stdlib/structural.md` — spec dedicada ao módulo `structural.rs`.
- **Cabeçalhos `@prompt` ajustados:**
  - `01_core/src/engine/stdlib/structural.rs` — `model/document.md`, `model/asset.md`, `stdlib/structural.md` (esta última em último lugar, conforme convenção do linter para prompt "dono" do ficheiro).
  - `01_core/src/engine/stdlib/mod.rs` — mantém `_comum.md` e `model/document.md` (último).
  - `01_core/src/engine/layout/mod.rs` — adicionada referência a `model/asset.md` (último), garantindo que o prompt `asset.md` continue referenciado por um ficheiro L1 depois de `structural.rs` passar a listar `structural.md` em último.
- **Prompt comum actualizado:**
  - `00_nucleo/prompts/engine/stdlib/_comum.md` — removido `structural.rs` da lista de ficheiros que apontam para o prompt comum; adicionada nota sobre o novo `structural.md`.
- **Débito actualizado:**
  - `00_nucleo/diagnosticos/debt/DEBT.md` — anotado que o subset `structural.rs` foi fechado em P430.

`cargo test --workspace` verde; `crystalline-lint` sem novas violações (mantêm-se apenas os dois warnings pré-existentes: `adr-stub-vs-fallback.md` e `show-regex.md`).

## Nota sobre a contagem de funções

O plano original (P430) listava 21 funções nativas no ficheiro. O inventário real de `structural.rs` contém **24** funções `native_*`:

`strong`, `emph`, `raw`, `heading`, `divider`, `terms`, `quote`, `table`, `table_cell`, `table_header`, `table_footer`, `grid_cell`, `grid_header`, `grid_footer`, `bibliography`, `cite`, `link`, `footnote`, `accent`, `cancel`, `underover`, `op`, `document`, `asset`.

O prompt `structural.md` cobre todas as 24 secções. As secções `document` e `asset` são resumos com remissão para os prompts dedicados `model/document.md` e `model/asset.md`, criados no Passo 397.

## Decisão de engenharia

- **Atomização por ficheiro (ADR-0104):** cada módulo `stdlib/*.rs` merece o seu próprio prompt L0 fino. `structural.rs` era o maior subset sem spec dedicada em DEBT-57.
- **Scope-outs preservados:** CSL styling real (`bibliography` / `cite`), OpenType shaping real (`accent` / `cancel` / `underover` / `op`) e multi-region grid/table continuam explicitamente fora de scope, conforme ADR-0054 / ADR-0060 / ADR-0078.
- **Linhagem `@prompt` e linter:** o `crystalline-lint` considera o *último* `@prompt` de cada ficheiro como a referência efectiva para efeitos de detecção de órfãos. Por isso `structural.md` foi colocado em último em `structural.rs`, e `asset.md` foi re-referenciado em `layout/mod.rs` para não ficar órfão.

## Verificação

| Critério | Resultado |
|----------|-----------|
| `00_nucleo/prompts/engine/stdlib/structural.md` criado | ✓ 24 secções |
| Cada secção cobre assinatura, args, semântica, paridade vanilla, limitações e testes canónicos | ✓ |
| `_comum.md` actualizado | ✓ |
| `DEBT.md` actualizado com nota de fecho P430 | ✓ |
| `crystalline-lint` sem novas violações | ✓ |
| `cargo test --workspace` verde | ✓ |
| Zero código funcional modificado | ✓ |

## Artefactos

- L0:
  - `00_nucleo/prompts/engine/stdlib/structural.md`
  - `00_nucleo/prompts/engine/stdlib/_comum.md` (actualizado)
  - `00_nucleo/prompts/engine/model/document.md` (referenciado)
  - `00_nucleo/prompts/engine/model/asset.md` (referenciado)
- Código (apenas cabeçalhos de linhagem):
  - `01_core/src/engine/stdlib/structural.rs`
  - `01_core/src/engine/stdlib/mod.rs`
  - `01_core/src/engine/layout/mod.rs`
- Débito:
  - `00_nucleo/diagnosticos/debt/DEBT.md`
- Plano:
  - `00_nucleo/materialization/typst-passo-430.md`
- este relatório.

## Próximo passo

Com P430 fechado, o subset `structural.rs` desaparece da lista de DEBT-57. Continua em aberto o fecho dos restantes ficheiros `stdlib/*.rs` sem spec L0 dedicada, bem como os débitos técnicos dependentes (DEBT-55 hayagriva/CSL, DEBT-50 bake-in, etc.).
