# Diagnóstico de realização textual — 2026-10-18

A investigação confirmou falhas de precedência, escopo herdado/local e proteção
de produtos. Está pronta a [revisão R3-T1](/repos/Antigravity/typst-crystalline/00_nucleo/materialization/textual-realization-2026-10-18/review.md); nenhuma correção produtiva
ou atualização de L0 vigente foi realizada neste lote de diagnóstico.

Medição em HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, working tree não commitado, arquivos exatos
em [before-sources.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/textual-realization-2026-10-18/before-sources.json.gz), com UTC/status/diffstat.
Pin vanilla `bd5ebec285505889e8ff1d4f68804ab147a699f3`; ambos os executáveis,
fontes/argv/stderr e horários estão nas três coletas .json.gz. A referência
ratificada está em [vanilla-resync](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/vanilla-resync-2026-10-17/relatorio.md).

Foram executadas **68 linhas de sondas, 66
fontes distintas**. Isto não equivale a essa quantidade de bugs. **25** linhas
têm texto extraído igual com ambos os compiladores aceitando, **24** têm texto
divergente, **13** só o Tekt rejeita, **1** só o vanilla rejeita e **5** rejeitam
nos dois. As categorias vêm de [classification.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/textual-realization-2026-10-18/classification.json)
e não certificam estilo/morfologia/render nem igualdade de diagnósticos.

Os 13 casos de rejeição só do Tekt incluem show-set e retornos de callback:
não são todos o mesmo defeito. O caso rejeitado só pelo vanilla envolve query
que retorna o próprio heading e permanece investigação separada. Entre os
cinco rejeitados em ambos há dois efeitos leftmost (primeiro panic diverge
em um), uma sonda inválida e dois controles Sequence com erros distintos.
Nenhum desses últimos é considerado feature GREEN por ambos rejeitarem.

Pico máximo dos três cgroups: **75120640 bytes (71.64 MiB)**,
MemoryMax **12884901888 bytes**, High **8589934592 bytes**, swap **0**;
eventos high/max/oom/oom_kill zero. [verification.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/textual-realization-2026-10-18/verification.json)
e os recursos survey/scope/refuters preservam limites e comandos. Não houve
compilação neste lote: esses picos medem sondas nos binários congelados.
Não são comparação de desempenho nem pico de uma implementação nova.

A conferência final verificou 2092 arquivos produtivos/L0 do
freeze sem mudança e 11 L0s-base dos candidatos com SHA-256 intacto. A última
suíte implementada continua sendo [R3 anterior](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/sequence-r3-2026-10-18/relatorio.md),
com 8979 testes verdes, 7 ignorados e 5 doctests, em seu estado congelado.
Esses testes não validam código candidato, que ainda não existe. A baseline
SARIF herdada conserva 106 V16:error, 1120 V19:note e 876 V20:note; gate global
continua vermelho. Não executar testes novos de código inexistente nem usar
supressões para dizer que o objetivo geral terminou.

O próximo passo dependente é a decisão do gate ADR-0127 §2.3 para R3-T1.
R3-T2/retornos e os demais lotes independentes continuam no objetivo geral;
não houve commit/push/release nem marcação do objetivo como concluído.

## Lista completa de sondas

| Coleta/caso | Classificação | Destino |
|---|---|---|
| before/constant-chain | texto divergente | R3-T1/controle |
| before/constant-chain-inverse | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/function-chain | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/function-chain-inverse | texto divergente | R3-T1/controle |
| before/alias-chain | texto divergente | R3-T1/controle |
| before/alias-chain-inverse | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/cycle | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/constant-cycle | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/revoked-cross | texto divergente | R3-T1/controle |
| before/revoked-cross-inverse | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/local-cross | texto divergente | R3-T1/controle |
| before/leftmost | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/unicode-overlap | texto divergente | R3-T1/controle |
| before/multiple-matches | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/show-set | só Tekt rejeita | R3-T1/controle |
| before/none-definition | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/none-assert | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/none-explicit | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/none-only | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/identical-literals | texto divergente | R3-T1/controle |
| before/mixed-regex-literal | texto divergente | R3-T1/controle |
| before/mixed-literal-regex | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/overlap | texto divergente | R3-T1/controle |
| before/leftmost-effect | ambos rejeitam; diagnóstico a classificar | R3-T1/controle |
| before/leftmost-regex-effect | ambos rejeitam; diagnóstico a classificar | R3-T1/controle |
| before/same-guard-adjacent | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/same-guard-regex-adjacent | texto divergente | R3-T1/controle |
| before/different-guard-adjacent | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/product-whole-group | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/product-separate-nodes | texto divergente | R3-T1/controle |
| before/constant-revoked-cross | texto divergente | R3-T1/controle |
| before/new-rule-on-completed | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/new-rule-cross-completed | texto divergente | R3-T1/controle |
| before/func-alias-raw-output | texto divergente | R3-T1/controle |
| before/white-text | texto divergente | R3-T1/controle |
| before/white-text-callback | texto divergente | R3-T1/controle |
| before/white-input-normalization | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/product-parent-hides-text | só Tekt rejeita | R3-T1/controle |
| before/product-parent-hides-constant | texto divergente | R3-T1/controle |
| before/local-cjk | texto divergente | R3-T1/controle |
| before/identity-shared-rule | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| before/constant-style | só Tekt rejeita | R3-T1/controle |
| before/function-style | só Tekt rejeita | R3-T1/controle |
| before/constant-invalid-output | só Tekt rejeita | R3-T2/retornos ou paridade query: separado |
| scope/captured-outer-precedence | texto divergente | R3-T1/controle |
| scope/captured-local-precedence | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| scope/captured-none-then-new | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| scope/captured-unmatched-then-new | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| scope/captured-outer-chain | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| scope/captured-latest-inside | texto divergente | R3-T1/controle |
| scope/hidden-generic-child | só Tekt rejeita | R3-T1/controle |
| scope/hidden-sequence-in-generic | ambos rejeitam; diagnóstico a classificar | inválido como teste da feature: identificador não definido |
| scope/white-text-middle | texto divergente | R3-T1/controle |
| scope/white-content-middle | texto divergente | R3-T1/controle |
| scope/white-callback-middle | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| scope/callback-int-literal | só Tekt rejeita | R3-T2/retornos ou paridade query: separado |
| scope/callback-int-node | só Tekt rejeita | R3-T2/retornos ou paridade query: separado |
| scope/callback-float | só Tekt rejeita | R3-T2/retornos ou paridade query: separado |
| scope/callback-bool | só Tekt rejeita | R3-T2/retornos ou paridade query: separado |
| scope/callback-none | só Tekt rejeita | R3-T2/retornos ou paridade query: separado |
| scope/callback-array | só Tekt rejeita | R3-T2/retornos ou paridade query: separado |
| scope/callback-auto | só Tekt rejeita | R3-T2/retornos ou paridade query: separado |
| scope/callback-content-location | só vanilla rejeita | R3-T2/retornos ou paridade query: separado |
| refuters/hidden-sequence-in-generic-valid | ambos rejeitam; diagnóstico a classificar | controle diagnóstico pendente: ambos rejeitam, ordem causal ainda não estabelecida |
| refuters/hidden-sequence-product-valid | ambos rejeitam; diagnóstico a classificar | controle diagnóstico pendente: ambos rejeitam, ordem causal ainda não estabelecida |
| refuters/capture-with-local-text | texto divergente | R3-T1/controle |
| refuters/captured-rule-binding-return | texto igual; sem prova geral de render/morfologia | R3-T1/controle |
| refuters/captured-global-binding-return | texto divergente | R3-T1/controle |
