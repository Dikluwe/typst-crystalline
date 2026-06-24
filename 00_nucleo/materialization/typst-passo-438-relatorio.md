# Passo 438 — relatório: fecho do subset `rules/stdlib/foundations.rs` e encerramento completo de DEBT-57

**Tipo:** documental / especificação L0 (zero código funcional modificado).  
**Data:** 2026-06-24. **HEAD base:** `f78ce4ce4`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=8388608`.

## O que se fez

Fechou-se o último subset de DEBT-57 (`rules/stdlib/foundations.rs`), criando
um prompt L0 dedicado completo. Com este fecho, **DEBT-57 está encerrado** —
todos os ficheiros stdlib de L1 possuem spec L0 dedicada.

- **L0 novo/expandido:**
  - `00_nucleo/prompts/rules/stdlib/foundations.md` — spec dedicada às
    ~30 funções de `foundations.rs`, organizada em 8 grupos lógicos:
    1. Utilitários gerais (`type`, `repr`, `len`).
    2. Conversões de tipo (`range`, `str`, `int`, `float`).
    3. Construtores de cor (`rgb`, `luma`, `oklab`, `oklch`, `linear_rgb`,
       `cmyk`, `hsl`, `hsv`).
    4. Metadados (`metadata`).
    5. Estado runtime (`state`, `state_update`, `state_update_with`,
       `state_display`, `state_final`, `state_at`).
    6. Contadores (`counter_display`, `counter_at`, `counter_final`,
       `counter_step`).
    7. Query e localização (`query`, `locate`, `here`).
    8. Nota de fecho de DEBT-57.

- **Cabeçalho `@prompt` ajustado:**
  - `01_core/src/rules/stdlib/foundations.rs` — data de actualização e hash
    sincronizados; continua a apontar para `foundations.md`.

- **Prompt comum actualizado:**
  - `00_nucleo/prompts/rules/stdlib/_comum.md` — adicionada nota sobre
    `foundations.md` e declaração de que DEBT-57 está encerrado.

- **Débito actualizado:**
  - `00_nucleo/diagnosticos/debt/DEBT.md` — DEBT-57 reclassificado de
    **EM ABERTO** para **FECHADO (Passo 438)**; entrada de
    `rules/stdlib/foundations.rs` marcada como fechada em P438; critério de
    fecho actualizado para refletir o encerramento completo.

`cargo test --workspace` verde; `crystalline-lint` sem novas violações
(mantêm-se apenas os dois warnings pré-existentes: `adr-stub-vs-fallback.md`
e `show-regex.md`).

## Decisão de engenharia

- **Atomização por ficheiro (ADR-0104):** `foundations.rs` era o último subset
  pendente de DEBT-57. A spec existente (`foundations.md`) era um resíduo do
  prompt grosso de P314; foi substituída por uma especificação completa e
  honesta do estado actual.
- **Honestidade epistémica:** o prompt documenta exactamente o que está
  implementado (por exemplo, `str()` não suporta `Color`; `int()` rejeita
  `Float`; `state_update_with` aceita callback; `query` suporta
  `Value::Selector` de primeira classe). Não inventa funções não presentes em
  `foundations.rs` (ex.: `datetime`, `bytes`, `module` como nativa global).
- **Agrupamento lógico:** em vez de ~30 secções totalmente separadas, as
  funções foram agrupadas por domínio (cores, estado, contadores, query),
  mantendo subsecções individuais com assinatura, args, semântica, paridade,
  limitações e testes canônicos.
- **Scope-outs preservados:** render PDF nativo para CMYK/gradientes, métodos
  avançados de `str`/`array`/`dict`, e introspection completa de módulos
  continuam fora do scope.

## Verificação

| Critério | Resultado |
|----------|-----------|
| `00_nucleo/prompts/rules/stdlib/foundations.md` criado/expandido com ~30 funções | ✓ |
| Cada função/grupo cobre assinatura, args, semântica, paridade vanilla, limitações e testes canónicos | ✓ |
| `_comum.md` actualizado com nota de fecho completo DEBT-57 | ✓ |
| `foundations.rs` cabeçalho `@prompt` aponta `foundations.md` | ✓ |
| `DEBT.md` reclassificado: DEBT-57 FECHADO (P438) | ✓ |
| `crystalline-lint` sem novas violações | ✓ |
| `cargo test --workspace` verde | ✓ |
| Zero código funcional modificado | ✓ |

## Artefactos

- L0:
  - `00_nucleo/prompts/rules/stdlib/foundations.md`
  - `00_nucleo/prompts/rules/stdlib/_comum.md` (actualizado)
- Código (apenas cabeçalho de linhagem / hash):
  - `01_core/src/rules/stdlib/foundations.rs`
- Débito:
  - `00_nucleo/diagnosticos/debt/DEBT.md`
- Plano:
  - `00_nucleo/materialization/typst-passo-438.md`
- este relatório.

## Próximo passo

Com DEBT-57 encerrado, os débitos em aberto restantes são:
- **DEBT-43** (linter type-level);
- **DEBT-55** (probe hayagriva);
- **DEBT-42** (`get_unchecked` bloqueado por benchmark).
