# P1340 — auditoria adversarial anterior ao candidato

Executor: `/root/p1340_adversary`. Regime: executado sem atestação de isolamento.
Papel: adversário; não escreve solução, contrato, oráculos ou veredito final.
Leituras: L0s de eval/pipeline/estabilização, fontes L1 já materializadas no
P1339, pipeline ainda sem integração seletiva e artefatos canônicos indicados.
Não houve leitura de candidato P1340 nem acesso a materialization/context.
Escrita autorizada: somente `diagnosticos/p1340-adversary-*`; mutantes futuros
apenas em `/tmp` pinado, depois de recebido o contrato aplicável.

## Proveniência

Auditoria estática em 2026-09-10, leitura de estado às 13:26:14Z. HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado.
O antecedente integral, inclusive `before.diff_stat`, `before.modified`,
novos fontes e diff binário, está em `p1340-baseline.json`, SHA-256
`0869202e774bd7b76278365aac7292d45a1c930be42cf83270845c985cb5000a`.
As fontes L1 abaixo ainda têm os hashes desse antecedente. Individualização
L0 P1340 é posterior ao baseline e está identificada separadamente.

| Entrada | SHA-256 |
|---|---|
| prompts/infra/pipeline/context_stabilization.md | 62f27ec604a0b9b0d4121ecf43ac556a18a59cd53947744c29187c4d49c1781c |
| prompts/infra/pipeline.md | e69691e7ced9e1cc26ddc0b752ff64a418322cad68c21a4b5bc68bbae6f66c1d |
| prompts/compiler/eval.md | 321d6747749a72488e13ee4f4996ae0d8b8a3c7df1c227a521431f02ae4a0cb1 |
| 01_core/src/compiler/eval/mod.rs | 8cac57e3d1d1f1401a8e0d63ede40d4fc0772d1df60134478a73f37d37b47e6a |
| 01_core/src/compiler/stdlib/counter.rs | 31dfc8413e5b90901c6c8fc313bc254c2d0794df87b2d7df61661f6166d65f62 |
| 01_core/src/compiler/stdlib/state.rs | 23c9b683a5beb25778aeee7672fd561943656da69434aff9573a549b8c569f49 |
| 01_core/src/compiler/introspect.rs | f71589c6594c99bd5d4cdb2d3c2b521ad2789a388e6bc13126f744f02035fd5c |
| diagnosticos/p1339-contract-r3.json | c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17 |
| diagnosticos/p1339-seal.json | 35f00c4b9e15a010692017f5083ea4f451f4104a3730022136972e151ac0a8ee |

Skill e referências lidos integralmente. ADR-0127 e ADR-0129 lidas; busca
dirigida nas ADRs não encontrou ADR de materialização segregada neste repo.
ADR-0118 foi consultada como contexto e está PROPOSTO, sem autoridade para
substituir o L0 atual.

## Medições e conclusões limitadas

1. `eval/mod.rs:5797-5804` compara a relação privada com Same e devolve
   `Ok(false)` tanto para Different como para Unproven. Não propaga um enum
   público. `:5853-5870` exclui Query, Locate, Here e projeções de Location dos
   detalhes; `:5877-5881` também não emite detalhe quando a relação não é
   Different. Portanto `false` com detalhes vazios não é prova de opacidade;
   detalhes presentes de outra leitura tampouco provam ausência de opacidade.

2. Isso **não demonstra** que a API aprovada seja insuficiente. Os únicos
   produtores efetivos de Unproven no comparador são Func::Element/Plugin
   (`:5551-5552`) e DynElement (`:5566-5569`). Arc genérico recursa no valor
   (`:5357-5360`), não usa identidade para encobrir esses casos. As folhas
   opacas não podem se comparar reflexivamente como Same. A validação de uma
   tentativa contra seu snapshot original, preservados os recursos, pode
   fornecer uma precondição verificável adicional à validação candidata.

   Evidência favorável: state_get/final clonam o valor armazenado, não
   reexecutam StateUpdate::Func (`state.rs:73-79,180-185,413-430`); query
   clona os carriers do snapshot (`foundations/query.rs:59-65`); o replay de
   CounterFold produz somente Locations e arrays de inteiros
   (`counter.rs:315-334,396-400`). A hipótese de que state recriaria uma
   closure durante esse replay foi refutada por essa leitura.

   Limite: esta é uma auditoria de suficiência possível, não certificação de
   um algoritmo inexistente. O registro completo e o replay determinístico
   perante World/métricas/styles preservados continuam precondições. Uma
   testemunha apoiada em fonte que produza falso contra o snapshot original
   sem opacidade/recurso alterado refutaria a inferência e exigiria reabrir
   a conclusão. Não foi encontrada tal testemunha nesta revisão. Não há
   fundamento aqui para exigir nova API pública por causa do bool sozinho.

3. IDs não constituem identidade causal suficiente. EvalContext::new zera
   next_context_id (`eval/mod.rs:6058`); Contextual usa o contador local
   (`:7263-7266`); populate_intr insere Location num mapa por id
   (`introspect.rs:1427-1434`). Logo avaliar dois produtores com contextos
   frescos e sem coordenação pode gerar o mesmo id de filho. Exemplo estático:

   ```typst
   #let c = counter(strong.where(delta: 1))
   #context { c.get(); context [esquerda] }
   #context { c.get(); context [direita] }
   ```

   Os dois corpos de descoberta podem criar filho id 0; o mapa sozinho
   perde uma ocorrência. Isto é testemunha do risco do baseline, não
   execução ou achado em candidato. O campo next_context_id já é público;
   não falta capacidade de coordenação L3. A Location é alocada pela travessia
   (`introspect.rs:1648-1649`), não por hash do id. Preservar/reconstruir a
   topologia continua necessário independentemente do esquema de IDs.

4. Retenção exige mais que IDs: substituir produtor com mesmo body/id e
   captura diferente deve invalidar filho e seu transcript. O comparador
   ContextBlock já observa id **e** closure (`eval/mod.rs:2728-2734`), mas
   a pipeline não recebe por isso autorização de comparar Debug/hash nem
   de aceitar id igual como testemunha de geração conservada.

## Ataques congelados nesta revisão, ainda não executados

As causas abaixo ficam escolhidas antes de acesso ao candidato. O contrato
e o verificador decidirão sua elegibilidade e o binding real; este documento
não substitui oráculos nem autoriza predicado adicional permissivo.

| ID | Desvio negativo | Testemunha exigida |
|---|---|---|
| A01 | Aceitar teto porque uma consulta gera detalhe, ignorando outra opaca | Mesmo bloco tem counter realmente alterado e state/query com folha deliberadamente opaca; não pode receber sucesso por detalhes do counter |
| A02 | Interpretar todo false sem detalhes como Unproven | Consulta query ou Location comprovadamente diferente, com leitura Element estável; distinguir falta de detalhe público de opacidade real |
| A03 | Reiniciar IDs locais por produtor e resolver por id global | Dois produtores reais criam filhos distintos, ou pai e filho colidem; ambos conteúdos e identidades causais devem sobreviver |
| A04 | Reter descendente pelo mesmo id/body após substituir captura do pai | Filho fecha sobre valor alterado; páginas/body textual podem ficar iguais, mas o valor produzido deve refletir a nova captura |
| A05 | Reter contribuição antiga quando a tentativa atual falha | Sucesso anterior gera update; tentativa invalidada seguinte falha; update anterior não pode permanecer em C_k |
| A06 | Reiniciar budget em descoberta nested ou relayout | Transcript deve manter uma única sequência A_k e devolver D5 lido de I4, sem execução A6 |
| A07 | Reexecutar irmão não selecionado junto do selecionado | Irmão legado conserva saída/erro/sink ordinário embora snapshot posterior mude |
| A08 | Comparar só páginas ou publicar sinks descartados | Leituras/posições mudam com páginas iguais; warning da execução substituída e de validação não é publicado |

Reuso: os F07/F08/F09 do contrato R3 e o fixture selado
`p1339-ab-batch1-retention-lifecycle-typed-r3.json` já cobrem famílias A04–A08.
Os M01–M20 selados de P1339 eram majoritariamente mutantes de superfície,
executados contra vanilla; seu score não demonstra por si só discriminação
da nova coordenação seletiva L3. Nenhum corpus global foi executado aqui.

Estado: aguardando contrato/escopo focal antes de gerar ou executar mutantes.
Este relatório não é selo nem veredito final.
