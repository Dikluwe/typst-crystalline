# Passo 436 — relatório: fecho do subset `rules/stdlib/transforms.rs` (DEBT-57)

**Tipo:** documental / especificação L0 (zero código funcional modificado).  
**Data:** 2026-06-24. **HEAD base:** `929530090`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=8388608`.

## O que se fez

Fechou-se o subset `rules/stdlib/transforms.rs` do débito DEBT-57, criando um
prompt L0 dedicado e actualizando a linhagem `@prompt` para que o ficheiro
desse de depender exclusivamente do prompt grosseiro `stdlib/_comum.md`.

- **L0 novo:**
  - `00_nucleo/prompts/rules/stdlib/transforms.md` — spec dedicada às 4 funções
    de transformação do subset (`move`, `rotate`, `scale`, `skew`).
- **Cabeçalhos `@prompt` ajustados:**
  - `01_core/src/rules/stdlib/transforms.rs` — `_comum.md` e `transforms.md`
    (este último em último lugar, conforme convenção do linter para prompt
    "dono" do ficheiro).
- **Prompt comum actualizado:**
  - `00_nucleo/prompts/rules/stdlib/_comum.md` — removido `transforms.rs` da
    lista de ficheiros que apontam para o prompt comum; adicionada nota sobre
    o novo `transforms.md`.
- **Configuração de linhagem:**
  - `crystalline.toml` — adicionada excepção de órfão para
    `00_nucleo/prompts/rules/stdlib/_comum.md`, uma vez que, após P436, nenhum
    ficheiro L1 o tem como último `@prompt`. O prompt continua referenciado por
    múltiplos ficheiros stdlib e é, portanto, comum por definição.
- **Débito actualizado:**
  - `00_nucleo/diagnosticos/debt/DEBT.md` — adicionada entrada para
    `rules/stdlib/transforms.rs` marcada como fechada em P436; critério de fecho
    actualizado; removida redundância na lista de ficheiros pendentes
    (`shapes.rs`, `transforms.rs`, `assert.rs` já constavam como fechados).

`cargo test --workspace` verde; `crystalline-lint` sem novas violações
(mantêm-se apenas os dois warnings pré-existentes: `adr-stub-vs-fallback.md`
e `show-regex.md`).

## Decisão de engenharia

- **Atomização por ficheiro (ADR-0104):** `transforms.rs` era o próximo subset
  mais fechável depois de `shapes.rs` (P435), com apenas 4 funções nativas e
  consumers reais em L1.
- **Linhagem `@prompt` e linter:** o linter considera o último `@prompt` de cada
  ficheiro L1 como a referência efectiva. Como `transforms.rs` passou a ter
  `transforms.md` como prompt "dono", `_comum.md` deixou de ser contado como
  referenciado. Em vez de forçar `_comum.md` a ser último noutro ficheiro
  (o que criaria órfãos em prompts dedicados), registou-se a excepção em
  `crystalline.toml`, reflectindo o seu papel de prompt comum partilhado.
- **Scope-outs preservados:** argumento `origin` em `rotate`/`scale`/`skew`,
  transformações 3D, e aplicação interactiva de transformações via introspector
  continuam fora do scope de paridade vanilla.

## Verificação

| Critério | Resultado |
|----------|-----------|
| `00_nucleo/prompts/rules/stdlib/transforms.md` criado com 4 secções | ✓ |
| Cada secção cobre assinatura, args, semântica, paridade vanilla, limitações e testes canónicos | ✓ |
| `_comum.md` actualizado | ✓ |
| `transforms.rs` cabeçalho `@prompt` aponta `transforms.md` | ✓ |
| `transforms.md` não ficou órfão | ✓ |
| `DEBT.md` actualizado com nota de fecho P436 | ✓ |
| `crystalline.toml` actualizado com excepção para `_comum.md` | ✓ |
| `crystalline-lint` sem novas violações | ✓ |
| `cargo test --workspace` verde | ✓ |
| Zero código funcional modificado | ✓ |

## Artefactos

- L0:
  - `00_nucleo/prompts/rules/stdlib/transforms.md`
  - `00_nucleo/prompts/rules/stdlib/_comum.md` (actualizado)
- Código (apenas cabeçalhos de linhagem / hashes):
  - `01_core/src/rules/stdlib/transforms.rs`
- Configuração:
  - `crystalline.toml` (excepção de órfão para `_comum.md`)
- Débito:
  - `00_nucleo/diagnosticos/debt/DEBT.md`
- Plano:
  - `00_nucleo/materialization/typst-passo-436.md`
- este relatório.

## Próximo passo

Com P436 fechado, continuam em aberto em DEBT-57 os subsets `gradients.rs` e
`foundations.rs`. Alternativas fora de DEBT-57: **DEBT-43** (linter type-level)
ou **DEBT-55** (probe hayagriva).
