# P1321 — classificação da reabertura R2

Concordo com reabrir o escopo para incluir exclusivamente a identidade nativa
CSV no transporte agregado já existente em call_dispatch. A classe continua
fluxo contínuo ADR-0127: correção interna de paridade e entrada de mapeamento,
sem API pública nova, entidade/trait, flag/default ou mudança eval/layout.
Não exige nova autorização humana por si só. Isto não autoriza alterações
antes dos dois L0 proprietários atualizados e do gate causal R2.

Li integralmente `00_nucleo/prompts/compiler/eval/call_dispatch.md`, 698 linhas,
SHA-256 `256407665bd0892a6b8675de89a1bd68169e938bd3eaca7b22b112966dd15add`.
Fonte antes de R2 SHA-256
`fd34727de8570bb3dfa65ec988dcd118c4e96adac0e5528de37c373ef13dcae2`.
Recalculei A efetivo d0d2343e e B reverso 9fe53cf5, ambos coincidentes com os
headers; Núcleo é o mesmo content-snapshot já auditado. P1307-R4 especifica
o transporte por identidade de function pointer, recursão With, nenhuma
inspeção de argumentos/mensagem e mesma rota em math; P1308 inclui panic.
O precedente resolve exatamente a perda observada, mantendo decisão
diagnóstica no consumer. Não é necessário inferir ranges ou adicionar dado.

O L0 atual restringe expressamente o conjunto, e loading P1321 proíbe outro
owner: ambos precisam da cláusula sucessora causal R2 antes do patch. A norma
deve declarar que chamadas CSV sintéticas sem AST conservam o Args.span
recebido e que as outras identidades não mudam. Os owners continuam 1:1.

Refreeze R2 deve acrescentar a norma de transporte e corrigir a fronteira
de loading, conservando exatamente casos e expectativas A/B originais.
Manter freeze, candidato e falhas R1 intactos, com vínculo predecessor;
essa reabertura melhora dados disponíveis sem ajustar o oráculo ao código.
RED local no transportador deve falhar antes da entrada CSV, incluindo With,
e proteger função usuária chamada csv contra identificação nominal.
Controles de read/decoders e encoders/panic protegem alcance da allowlist.

Validar primeiro os oito missing que falharam, mais controles de fronteira;
somente depois do focal passar repetir corpus integral/repeat/reverse e
gates finais nos dois owners. A primeira reabertura tem causa nova medida;
duas revisões focais consecutivas sem ganho exigiriam nova revisão causal.
Regime continua sem atestação de isolamento, sem selo e sem paridade geral.
