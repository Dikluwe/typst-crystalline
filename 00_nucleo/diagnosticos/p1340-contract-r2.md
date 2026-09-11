# P1340 — sucessor contratual estreito R2

Proposta congelada anterior ao candidato, para revisão independente. Autor
`/root/p1340_contract`; executado sem atestação de isolamento. O contrato JSON
registra os hashes dos predecessores, baseline, aprovação humana, L0 vigente,
harness e binding. Não altera nenhum artefato histórico.

Mantém os 40 casos públicos e os 12 lifecycle da proposta anterior, com os
mesmos predicados, perfis, ordens e controles de preservação. Acrescenta apenas
seis cenários da política terminal aprovada por `Faça as correções`:

| Caso | Observação exigida |
|---|---|
| T01 opaco | Após A5, Error exato com root span e sem exportação |
| T02 diferença + opaco | Detalhe do contador não permite ignorar opacidade |
| T03 query muda e estabiliza | False sem detalhe não causa erro terminal antecipado |
| T04 erro original | Vetor original completo prevalece; warnings retidos uma vez |
| T05 impossibilidade comprovada | Operação terminal real produz Error exato; causa exige testemunha separada |
| T06 crescimento fechado | D5 lê I4, metadata 4 e política linguística anterior preservada |

O erro próprio é exatamente `contextual stability could not be verified`,
Severity::Error, `source.root().span()`, sem hints ou trace fabricados. Sinks
global e final retido permanecem; descoberta selecionada, validação e execuções
substituídas não vazam. O harness também compara todos os campos do erro
original e observa chamadas reais de exportação.

O harness Rust compila contra o artefato existente de typst-core; o JSON fixa
comando, compilador e hash. Isso verifica tipos/sintaxe, sem crédito semântico.
A ligação tardia deve executar a sessão/decisão produtiva real. Construção opaca
usa Func::element real em Metadata da entrada, antes da descoberta; não injeta
resposta de query ou resultado de validação. Regras completas estão em
`p1340-contract-binding-r2.md`.

T05 é deliberadamente teste da operação terminal com estado privado comprovado,
não afirma que dois filhos com id local igual sejam impossíveis. A03 continua
controle negativo obrigatório: os dois filhos válidos devem sobreviver. A causa
real da impossibilidade e sua passagem à operação terminal exigem testemunha
estrutural independente; não há GREEN global por chamar somente o formatador.

Pendente antes de selo: revisão independente da sucessão/binding, elegibilidade
dos ataques focais e demais gates históricos. Nenhum patch candidato foi lido;
nenhuma produção, API, L0, oráculo histórico ou veredito foi escrito.
