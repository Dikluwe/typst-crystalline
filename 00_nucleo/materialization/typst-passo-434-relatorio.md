# Passo 434 — relatório: fecho do subset `rules/stdlib/assert.rs` (DEBT-57)

**Tipo:** documental / especificação L0 (zero código funcional modificado).  
**Data:** 2026-06-24. **HEAD base:** `f08461956`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=8388608`.

## O que se fez

Fechou-se o subset `rules/stdlib/assert.rs` do débito DEBT-57, criando um
prompt L0 dedicado e actualizando a linhagem `@prompt` para que o ficheiro
desse de depender exclusivamente do prompt grosseiro `stdlib/_comum.md`.

- **L0 novo:**
  - `00_nucleo/prompts/engine/stdlib/assert.md` — spec dedicada à função
    `assert`.
- **Cabeçalhos `@prompt` ajustados:**
  - `01_core/src/engine/stdlib/assert.rs` — aponta agora para
    `00_nucleo/prompts/engine/stdlib/assert.md`.
- **Prompt comum actualizado:**
  - `00_nucleo/prompts/engine/stdlib/_comum.md` — removido `assert.rs` da
    lista de ficheiros que apontam para o prompt comum; adicionada nota sobre
    o novo `assert.md`.
- **Débito actualizado:**
  - `00_nucleo/diagnosticos/debt/DEBT.md` — adicionada entrada para
    `rules/stdlib/assert.rs` marcada como fechada em P434; critério de fecho
    actualizado.

`cargo test --workspace` verde; `crystalline-lint` sem novas violações
(mantêm-se apenas os dois warnings pré-existentes: `adr-stub-vs-fallback.md`
e `show-regex.md`).

## Decisão de engenharia

- **Atomização por ficheiro (ADR-0104):** mesmo ficheiros com uma única
  função (`assert.rs`) merecem prompt L0 próprio, separando a convenção
  partilhada da semântica específica.
- **Named args:** `assert` foi a primeira função nativa com named arg
  documentado (Passo 66, DEBT-16); o prompt reflecte isso.
- **Mensagem default:** divergência consciente vs vanilla — mensagem em
  português ("Asserção falhou") em vez de inglês.

## Verificação

| Critério | Resultado |
|----------|-----------|
| `00_nucleo/prompts/engine/stdlib/assert.md` criado com 1 secção | ✓ |
| Secção cobre assinatura, args, semântica, paridade vanilla, limitações e testes canónicos | ✓ |
| `_comum.md` actualizado | ✓ |
| `assert.rs` cabeçalho `@prompt` aponta `assert.md` | ✓ |
| `DEBT.md` actualizado com nota de fecho P434 | ✓ |
| `crystalline-lint` sem novas violações | ✓ |
| `cargo test --workspace` verde | ✓ |
| Zero código funcional modificado | ✓ |

## Artefactos

- L0:
  - `00_nucleo/prompts/engine/stdlib/assert.md`
  - `00_nucleo/prompts/engine/stdlib/_comum.md` (actualizado)
- Código (apenas cabeçalho de linhagem / hashes):
  - `01_core/src/engine/stdlib/assert.rs`
  - `01_core/src/engine/stdlib/gradients.rs` (hash sincronizado)
  - `01_core/src/engine/stdlib/transforms.rs` (hash sincronizado)
- Débito:
  - `00_nucleo/diagnosticos/debt/DEBT.md`
- Plano:
  - `00_nucleo/materialization/typst-passo-434.md`
- este relatório.

## Próximo passo

Com P434 fechado, continuam em aberto em DEBT-57 os subsets `shapes.rs`,
`transforms.rs`, `gradients.rs` e `foundations.rs`. Alternativas fora de
DEBT-57: **DEBT-43** (linter type-level) ou **DEBT-55** (probe hayagriva).
