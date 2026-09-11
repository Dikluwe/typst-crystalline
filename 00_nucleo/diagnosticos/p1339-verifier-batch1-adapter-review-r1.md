# P1339 — revisão estática dos adapters do lote 1

2026-09-10, verificador `/root/p1311_review`, executado sem atestação de
isolamento. Mesma fase e estado da revisão successor-r1: antes do manifesto,
sem candidato ou execução semântica. Nenhum input julgado foi alterado.

Entradas em `00_nucleo/diagnosticos/`, pins reconferidos:

- `p1339-mutant-closed-state-projection-harness.rs`:
  `27c1e4d8ee3f0bbfe689eb2af82dd0451d7e34cfdaf900b80f37462dae6bfcc2`.
- `p1339-mutant-closed-state-public-opaque-harness-r2.rs`:
  `6838a1c00c0004bb3fb0cd282efa196c3c58c85b91c4ea64fb345de10558e8d7`.
- `p1339-mutant-closed-state-lifecycle-collector.rs`:
  `d8e07c060162976895e54ebf55dc12e5b51e5234aa173f09f234f82a2cc38f1b`.
- `p1339-mutant-closed-state-core-wrapper.rs`:
  `1b316c6a1d00feca89d5d2c1894caac6a2ba2acae212b3d2c4820b95cbadfa5c`.

Projection traduz os oito inputs, incluindo PageStore cru com os mesmos
Func bindings, snapshots separados e APIs reais. Os efeitos vêm de um port
passivo begin/phase/finish que não recebe expected. As assertions propagam
falha; identidade de allocation usada aqui verifica apenas que o resultado
é o binding já construído, não substitui a relação produtiva. No final,
invocação de callback e gravação de operação devem vir dos caminhos reais
auditados, nunca de contadores preenchidos conforme o cenário.

Opaco-r2 preserva a construção das mesmas instâncias para o par privado e
query pública e traduz o veto de todos os canais. O diff do predecessor
audita a extensão; nenhuma relação alternativa foi introduzida. O predicado
Python independente continua obrigatório no driver externo.

O collector lifecycle recebe somente fonte/coordenadas/projeções de entrada,
invoca um port cuja obrigação é a compilação paginada real e imprime a
observação. Não possui ciclo de tentativas, decisão de retenção nem geração
de eventos. Preenche apenas coordenadas do input. Seu sucesso é declarado
COLLECTION_ONLY_NOT_PASS, corretamente: predicado independente e auditoria
de ligação continuam obrigatórios. O port não pode fabricar eventos,
implementar minipipeline ou trocar FallbackFontMetrics.

O core-wrapper chama as APIs, exige a matriz privada completa e failures
vazio, depois o opaco público, styles e projeções. Fecha o risco anterior
de somente imprimir falhas privadas sem reprovar o teste. O filtro final
deve excluir o teste standalone herdado para não duplicar linhas.

Aceite estático destes componentes, sujeito às pendências já registradas
na revisão successor-r1 e ao driver ainda não auditado. O driver deve
verificar conjunto exato de IDs/perfis/ordens em cada prefixo, contagens,
exit/timeout/sinal reais, pins de todos os includes/predicados e resultados
dos checkers originais. Hash de test-bin fornecido por ambiente só é
proveniência quando comparado ao hash calculado pelo executor do arquivo
efetivamente lançado; um campo JSON ou env autodeclarado não basta.

Não executado em C: as APIs futuras e hooks reais continuam NotDue sem
crédito. Compilação falhada futura é falha do gate, nunca mutante morto ou
RED semântico. Nenhuma aprovação de manifesto, selo, RED ou candidato é
emitida por esta revisão.
