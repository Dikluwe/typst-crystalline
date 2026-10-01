# L0 — leituras filtradas e replay de counter
Hash do Código: 8263637b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/counter/filtered_reads.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência antes da decisão

Em `01_core/src/compiler/stdlib/counter.rs:317-632`, a identificação de chave filtrada, resolução de localização, reconstrução de histórico, métodos `*_owned` e replay contextual compartilham a política de leitura, separada do construtor e da exibição. `compiler/eval/context_reads.rs:268` chama `replay_context_read` pela fachada. Isto mede propriedade mecânica da árvore no commit `320c2149b`; não prova intenção do vanilla.

## Contrato

Possuir apenas a política de leitura filtrada e replay. Preservar a ordem de validação, origem dos diagnósticos, marcação de leituras filtradas, `observe_context_read` e a ordem causal dos eventos. Quando a chave não é filtrada, delegar aos métodos existentes da fachada. Reexecutar somente as variantes `Counter*` já atribuídas a este owner. Manter as assinaturas da fachada e a representação de `Counter` inalteradas. A lógica de `.display()` pertence ao prompt irmão `compiler/stdlib/counter/display.md`.

## Aceitação

Resultados e erros observáveis idênticos para `.get()`, `.at()`, `.final()` e replay, incluindo selectors filtrados, labels e ausência de context; despacho estático, build e lint válidos. Não alegar ganho de desempenho sem medição.
