# P1322 — congelamento C após correção local

Conferência explícita SHA-256 em **2026-09-08T22:43:05Z**, HEAD e working tree produtiva do baseline P1322. Estes são os nove paths passados ao segundo `apply_patch` pelo gerador C. A primeira emissão não foi arquivada integralmente; seus hashes anteriores não foram registrados por C. Os TSVs/cohorts logicamente não mudaram, mas foram passados novamente ao patch; não se afirma um histórico de bytes anterior sem recibo. A correção dos dois campos de resumo e os novos metadados de hora/hash do gerador são descritos em `p1322-classification-correction.md`.

Root já havia acessado a primeira emissão. D informou que duas verificações estavam em execução ao receber o aviso. Logo a revisão local ocorreu antes do hand-off formal/veredito, **não antes de qualquer leitura externa**. Recibos de revisão que cruzaram essa escrita devem ser invalidados e reexecutados contra este congelamento. Não se alega preservação de R1. Nenhum produto, L0, histórico P1309 ou recibo de medição foi sobrescrito.

| Path exato | SHA-256 atual |
| --- | --- |
| `00_nucleo/diagnosticos/p1322-classification-owner-ledger.tsv` | `6fdf058c92dda6f3ac11dd691c1a53b4edf8c4fc3d5529cb32b9cd23bc1cce3a` |
| `00_nucleo/diagnosticos/p1322-classification-supplemental-ledger.tsv` | `3d499a3405534e7501948919868cf569f42930a7d85daed868adec869be200d6` |
| `00_nucleo/diagnosticos/p1322-classification-transversal-ledger.tsv` | `1c0a367036458bb04205143d82151b1c0ca53009f9e4c5e067b4009b3a4402f4` |
| `00_nucleo/diagnosticos/p1322-classification-transitions.tsv` | `2204911c65914d344697604323d5dd71752fc46b53ceea5957adf5a73179bd01` |
| `00_nucleo/diagnosticos/p1322-classification-summary.json` | `98b98e0debaf5d5fc92f84b7f5b4dbe1973ee1a95f18b521ff039c65304d706b` |
| `00_nucleo/diagnosticos/p1322-classification-selection.json` | `3d8ff907008cb4a380ca730b59e6c52a127696842d859c88482f0ae59fe83bc8` |
| `00_nucleo/diagnosticos/p1322-classification-source-lineage.json` | `cce656408b4185828a38fb15f3b5854c6f10630628f2d91236a7ce248992b2f1` |
| `00_nucleo/diagnosticos/p1322-classification-functional-reconciliation.json` | `875ddb0a4ae27c661253331c1e6cdbf2b050a57b397599e7dfecb7feb8678268` |
| `00_nucleo/diagnosticos/p1322-classification-cohorts.md` | `f38eaad510172aad9bdb9b2d440055b55bde440aefb51b4d6d007baa9c45375a` |

Não editar estes nove inputs julgados após este congelamento. Qualquer correção posterior cria sucessor nomeado e preserva esta versão. Classificador C não aprova sua saída; somente revisão D separada pode aceitá-la no regime sem atestação técnica de isolamento.
