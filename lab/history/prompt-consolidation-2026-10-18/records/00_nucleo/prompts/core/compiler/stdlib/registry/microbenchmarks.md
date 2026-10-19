# L0 — compiler/stdlib/registry/microbenchmarks
Hash do Código: bc076d40

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/microbenchmarks.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own an ignored cfg(test) Criterion runner under the private registry; do not
expose factories/helpers or production APIs. Require release profile. Measure
Scope construction/destruction across Html/A11yExtras/Bundle combinations,
empty and scaled inputs, present/absent global lookup, calc lookup, and calls
through pre-resolved calc.abs/upper pointers for valid and invalid type/arity/
named arguments. Keep fixtures, native resolution, World and EvalContext outside
timed sections. World performs no I/O and fails source/file requests. Exclude
parser, eval dispatch, cache, layout and export from these benchmark names.
Include output/diagnostic construction and destruction. Use black_box and fixed
semantic assertions before timing. Use 30 samples, 150 ms warmup and 300 ms
measurement windows. Reject debug execution. Controls verify feature gates and
nonempty diagnostics on invalid calls; results do not establish vanilla parity
or improvements without a comparable repeated baseline.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Fluxo de controle e verificações executáveis — 2026-10-13

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. 01_core/src/compiler/stdlib/registry/microbenchmarks.rs:203-206 contém o trecho medido. O inventário Clippy e a leitura dos owners distinguem código sem efeito, dados inválidos deliberados e verificações vazias. Fontes, L0, UTC, status, diffstat e comandos estão em `lab/results/construction/diagnostics/controle-fluxo-ci-2026-10-13/before.json.gz` e nas evidências do mesmo diretório. Não é nova medição de paridade nem inferência sobre intenção upstream.

### Decisão e aceitação

Conservar exatamente a guarda que rejeita a execução explícita do runner em debug. O assert de cfg!(debug_assertions) é uma pré-condição intencional de perfil, não um teste de comportamento do produto. Usar expect(clippy::assertions_on_constants) exclusivamente sobre essa guarda, com justificativa; não silenciar o módulo nem outros avisos. Confirmar por execução ignorada selecionada que debug falha com a mensagem run with cargo test --release antes de Criterion. As asserções semânticas de fixtures e as medições release permanecem intactas.

Preservar APIs públicas, comportamento padrão, fase e expectativas de linguagem existentes. Revalidar a suíte apropriada, Clippy, build e linhagem. Correção interna em fluxo contínuo ADR-0127.
