# L0 — wiring/query
Hash do Código: 4a2e0651

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/query.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/src/query.rs:16` — `run_query`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/query.rs`.

## Owned contract

- Own run_query composition. Emit the specific deprecation warning after successful serialization or after compilation diagnostics; use the registered stdin Source versus physical root/main/font/CA paths.
- Call L3 query_compiled_elements; drain compilation errors then warnings; retrieval emits its standalone error, and delegate field/one/pretty/format output to L2 serialize_query_with_format. Selectors/cardinality are not reimplemented in L4.
- Success exits 0; semantic/serialization errors exit 1; I/O/argument errors exit 2, except measured missing physical input exits 1. Preserve diagnostic origins through the dedicated drain owner.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Ordem e hint de query — gate confirmado ADR-0127

Evidência: before.json.gz no diagnóstico plano-debitos-2026-10-06 mede a ordem
error → warnings do documento → depreciação no pin, e o hint específico.
Upstream query.rs:43,51-64 e compile.rs:779 ordenam explicitamente esses dados.
O adendo vigente da ADR-0045 §8 preserva warnings → errors; este gate foi confirmado pelo dono em 2026-10-06; a ordem vigente é
substituída conforme o novo adendo da ADR-0045.

Usar query_deprecation_hint do owner 00_nucleo/prompts/shell/cli.md.
Falhas de aquisição antes de compilação não emitem depreciação. Falha
semântica drena primeiro errors, depois warnings e por último depreciação.
Sucesso serializa/escreve stdout e só depois drena warnings/depreciação.
Falha de cardinalidade/campo/serialização retorna o erro existente sem
acrescentar depreciação, conforme upstream retrieve/format com ?.
Não alterar SourceDiagnostic público ou ordem dentro de cada coleção.
Para input -, compor for_stdin com texto adquirido e root cwd absoluto;
obter world.source(main()) em vez de fabricar Source com id existente.
Manter stdin como origem causal e base de imports/read, fora das dependências
físicas. As APIs existentes bastam; sem nova execução do documento.

Gate confirmado pelo humano em 2026-10-06 (resposta confirmo), com candidatos
e hashes preservados em 00_nucleo/materialization/warnings-cli-aprovacao-2026-10-06.json.

## Entrada física inexistente — correção de paridade em fluxo contínuo

Medição antes da decisão: query-residuos-2026-10-06/before.json.gz, controles
query-missing-file/query-dot-missing-file/query-absolute-missing-file. Oracle
pinado rejeita com status 1, stdout vazio, uma linha:
error: input file not found (searched at INPUT)
Tekt retorna 2 e MainNotFound Display. World::new em infra/world.rs:150–155
já discrimina MainNotFound da classe Io; não inferir pela mensagem ou refazer
stat antes/depois. No match tipado da composição de query, MainNotFound
recebe o envelope medido, input.display() exato, e exit 1, sem depreciação.
Outros erros mantêm seus contratos; não alterar World nem outros comandos.
Não há nova assinatura pública ou fase. RED→GREEN é o gate ADR-0127.


## Query CLI compilada — gate ADR-0127 confirmado

### Medição anterior à decisão

query-residuos-2026-10-06/before.json.gz registra HEAD/working tree, UTC,
pin/binary hashes, fontes, comandos e stdout/stderr/status. query-stdin-callback
retorna sucesso onde vanilla reporta erro e trace. query-context-success perde
metadata produzido; nested-error também desaparece. query-selector-quotes
apresenta rejeição P206C antes do erro do documento. where-level é rejeitado,
mas vanilla retorna [2]. Document-error-invalid-selector demonstra precedência
da compilação; valid-document-invalid/text-selector demonstra retrieval error
sem depreciação. eval-context-reference e eval-where-reference já coincidem
usando a compilação paginada atual. Upstream typst-cli/src/query.rs:34–66
compila antes de retrieve; :86–105 avalia selector em Scope default e depois
converte para LocatableSelector. Fontes locais: query_helpers.rs:488–522 consulta
somente Module; pipeline.rs:182–214 já usa compilação com introspector final.

### Decisão confirmada

Correção da linguagem, não cópia da mecânica upstream. CLI query passa a
compilar documento paginado completo antes da avaliação de selector, sem
exportar arquivo. A execução comum do programa permanece única, conforme os
owners vigentes de realização/contexto. Reutilizar o pipeline paged completo:
nenhuma expansão avulsa, índice pré-contexto ou ciclo de estabilização paralelo.
Distinguir falha de compilação e de retrieval por enum explícito, nunca por
mensagens, spans destacados ou busca de substrings. A nova API e o caminho de query compilada foram confirmados pelo humano
em 2026-10-06 (confirmo); recibo query-compilada-aprovacao-2026-10-06.json.
Os candidatos/manifesto permanecem como recibo histórico; este é o L0 vigente.

No run_query, substituir query_elements por query_compiled_elements. Compilation:
drenar errors, warnings e depreciação, exit 1. Retrieval: emitir uma linha
error: MENSAGEM, exit 1, sem warnings ou depreciação (upstream early ?).
Sucesso: serialização/publicação L2 vigente, depois warnings/depreciação.
Manter aquisição stdin por for_stdin e regra de MainNotFound já corrigida.
Nenhum algoritmo de seletor ou realização no wiring, nenhuma flag/target novo.
Não construir um PDF para extrair informação de query; usar introspector final.
