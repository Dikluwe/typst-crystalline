# L0 — Generic axis pair
Hash do Código: 61b5e1a0

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/axes.rs`

## Contract

`Axes<T>` has public `x: T` and `y: T`. `new(x, y)` preserves both values.
Debug, Clone, Copy and PartialEq are derived when supported by T; Eq requires
T: Eq. This pure generic pair does not acquire map/with helpers merely because
the upstream `lab/typst-original/crates/typst-library/src/layout/axes.rs`
has them. The assigned scope is the base carrier and constructor (ADR-0080).

Facade exposure belongs to `00_nucleo/prompts/entities/mod.md`. The former
`pub use axes::Axes` obligation is retained as an unresolved ownership/public
surface discrepancy in the sanitation diagnostic; absence in code does not
revoke it. This cleanup neither adds nor removes a reexport.

## Cobertura de Copy e Clone — 2026-10-13

### Medição anterior à decisão

`01_core/src/entities/axes.rs:43-49` cobre Copy e chama Clone somente com
f64, que já é Copy. A assinatura derivada em :17 deve suportar também
T: Clone sem Copy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working
tree não commitado; fontes, L0, UTC, status e diffstat ficam em
`00_nucleo/diagnosticos/simplificacoes-ci-2026-10-13/before.json.gz`.
O filtro axes anterior passa; o aviso Clippy expõe a redundância da fixture,
não um defeito confirmado na implementação de Clone.

### Aceitação

Manter o caso Copy de Axes<f64>. No mesmo teste, exercitar Clone com
Axes<String>, tipo não Copy: preservar ambos os valores e verificar que
modificar o x da cópia não altera o original nem o y. Não substituir o teste
de Clone por uma atribuição Copy, não alterar derives, fields, construtor,
API ou reexportações. Esta é cobertura do contrato genérico já vigente,
sem decisão nova de paridade ou fase. Revalidar suíte core, build e linhagem.
