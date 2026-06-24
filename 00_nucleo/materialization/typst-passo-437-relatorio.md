# Passo 437 — relatório: fecho do subset `rules/stdlib/gradients.rs` (DEBT-57)

**Tipo:** documental / especificação L0 (zero código funcional modificado).  
**Data:** 2026-06-24. **HEAD base:** `280a26e13`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=8388608`.

## O que se fez

Fechou-se o subset `rules/stdlib/gradients.rs` do débito DEBT-57, criando um
prompt L0 dedicado e actualizando a linhagem `@prompt` para que o ficheiro
desse de depender exclusivamente do prompt grosseiro `stdlib/_comum.md`.

- **L0 novo:**
  - `00_nucleo/prompts/rules/stdlib/gradients.md` — spec dedicada às 3 funções
    de gradiente (`gradient.linear`, `gradient.radial`, `gradient.conic`).
    O prompt documenta o estado real: os construtores existem e produzem
    `Value::Gradient`, mas o render PDF real está em scope-out (fallback para a
    cor do primeiro stop).
- **Cabeçalhos `@prompt` ajustados:**
  - `01_core/src/rules/stdlib/gradients.rs` — `_comum.md` e `gradients.md`
    (este último em último lugar, conforme convenção do linter para prompt
    "dono" do ficheiro). A referência a `shapes.md` (adicionada no P435 para
    evitar órfão) foi removida.
  - `01_core/src/rules/stdlib/shapes.rs` — reordenado para `_comum.md`,
    `square.md`, `shapes.md` (último), tornando `shapes.md` o prompt dono do
    ficheiro. `square.md` passou a ser uma excepção de órfão.
- **Prompt comum actualizado:**
  - `00_nucleo/prompts/rules/stdlib/_comum.md` — removido `gradients.rs` da
    lista de ficheiros que apontam para o prompt comum; adicionada nota sobre
    o novo `gradients.md`.
- **Configuração de linhagem:**
  - `crystalline.toml` — adicionada excepção de órfão para
    `00_nucleo/prompts/rules/stdlib/square.md`. O prompt continua referenciado
    por `shapes.rs`, mas `shapes.md` é o seu prompt "dono".
- **Débito actualizado:**
  - `00_nucleo/diagnosticos/debt/DEBT.md` — adicionada entrada para
    `rules/stdlib/gradients.rs` marcada como fechada em P437 (documental,
    scope-out); removido `gradients.rs` da lista de ficheiros pendentes.

`cargo test --workspace` verde; `crystalline-lint` sem novas violações
(mantêm-se apenas os dois warnings pré-existentes: `adr-stub-vs-fallback.md`
e `show-regex.md`).

## Decisão de engenharia

- **Atomização por ficheiro (ADR-0104):** `gradients.rs` era o próximo subset
  mais fechável depois de `transforms.rs` (P436).
- **Honestidade epistémica:** o prompt L0 não inventa comportamento. Documenta
  que `Value::Gradient` existe, que `shapes.rs` aceita gradientes como `Paint`,
  mas que o output PDF usa fallback sólido (`first_stop_color`) e que
  `tiling(gradient)` é rejeitado (scope-out ADR-0054).
- **Linhagem `@prompt` e linter:** como `gradients.rs` passou a ter
  `gradients.md` como prompt "dono", a referência anterior a `shapes.md`
  deixaria este último órfão. Em vez de re-referenciar `shapes.md` noutro
  ficheiro (o que deslocaria o problema), reordenou-se `shapes.rs` para que
  `shapes.md` seja o seu próprio prompt dono. `square.md` (prompt auxiliar do
  helper `square`) foi registado como excepção de órfão, dado que é um detalhe
  local de `shapes.rs` e não tem ficheiro próprio.
- **Scope-outs preservados:** render PDF de gradientes via shading operators,
  `tiling(gradient)`, e `anti_alias` continuam fora do scope.

## Verificação

| Critério | Resultado |
|----------|-----------|
| `00_nucleo/prompts/rules/stdlib/gradients.md` criado com 3 secções | ✓ |
| Cada secção cobre assinatura, args, semântica, paridade vanilla, scope-out e testes canónicos | ✓ |
| `_comum.md` actualizado | ✓ |
| `gradients.rs` cabeçalho `@prompt` aponta `gradients.md` | ✓ |
| `gradients.md` não ficou órfão | ✓ |
| `shapes.md` não ficou órfão (prompt dono em `shapes.rs`) | ✓ |
| `square.md` registado como excepção de órfão | ✓ |
| `DEBT.md` actualizado com nota de fecho P437 | ✓ |
| `crystalline-lint` sem novas violações | ✓ |
| `cargo test --workspace` verde | ✓ |
| Zero código funcional modificado | ✓ |

## Artefactos

- L0:
  - `00_nucleo/prompts/rules/stdlib/gradients.md`
  - `00_nucleo/prompts/rules/stdlib/_comum.md` (actualizado)
- Código (apenas cabeçalhos de linhagem / hashes):
  - `01_core/src/rules/stdlib/gradients.rs`
  - `01_core/src/rules/stdlib/shapes.rs`
- Configuração:
  - `crystalline.toml` (excepção de órfão para `square.md`)
- Débito:
  - `00_nucleo/diagnosticos/debt/DEBT.md`
- Plano:
  - `00_nucleo/materialization/typst-passo-437.md`
- este relatório.

## Próximo passo

Com P437 fechado, **DEBT-57** reduz-se ao subset `foundations.rs` (~30 funções,
o último ficheiro stdlib sem spec L0 dedicada). Alternativas fora de DEBT-57:
**DEBT-43** (linter type-level) ou **DEBT-55** (probe hayagriva).
