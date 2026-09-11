# Passo 1340 — completar a estabilização contextual seletiva

## Objetivo e fronteira

Implementar somente a pendência de coordenação da pipeline especificada no
P1339: contextos dependentes de contadores filtrados devem estabilizar suas
leituras e contribuições antes da saída paginada. Este passo é plano de
execução, não Prompt L0, nem declaração de que P1339 foi concluído.

Antecedente: HEAD `2f42d64253547734564513a1159ee6b584c1c4b4` com working tree
P1339 não commitada. Congelar diff integral, fontes novas, binários e L0 antes
de editar produção; não atribuir o estado àquele commit limpo.

Exclusões: repr de floats, Array, novas rotas públicas, correção geral de
show rules/HTML, atomização global de eval, commits e fechamento global do
1339. Falhas nessas superfícies não serão mascaradas nem incorporadas por
conveniência. Dependência necessária fora desta fronteira deve ser relatada.

## Execução com a skill

Regime completo Tekt, **executado sem atestação de isolamento**: capacidades
são declaradas, mas o workspace é compartilhado. Root escreve intenção e
implementação; autor independente seleciona contrato/oráculos; adversário
escreve ataques; verificador não modifica material que julga.

1. Congelar baseline e mapear as obrigações já aprovadas em
   `00_nucleo/prompts/infra/pipeline.md` e `compiler/eval.md`. Ler os L0,
   ADR-0109/0127/0129 e confirmar V15/V26 antes de resselo.
2. Individualizar a coordenação em owner privado L3 com L0 próprio, mantendo
   a pipeline como integração. Não mover semântica de leituras para L3 nem
   introduzir assinatura pública, fase ou default novos. A especificação
   deve preceder o código; transferência de ownership invalida somente a
   parte afetada da cadeia anterior e exige selo sucessor explícito.
3. Reaproveitar testes e oráculos históricos imutáveis quando o contrato
   independente demonstrar pertinência. Congelar seleção, expectativas e
   política de Unknown antes da implementação. Demonstrar RED funcional,
   sem confundir erro de build/runner com falha de linguagem.
4. Validar poder discriminatório com ataques reais e testemunhas causais.
   Implementar somente após autorização do verificador baseada nesse gate.
5. Executar primeiro focais, depois os controles e repetições pertinentes.
   Não repetir corpora globais a cada alteração de hash. Preservar evidência
   das falhas e não editar expectativas para acomodar o candidato.
6. Validar build, testes de integração, fmt/diff e lint; produzir diagnóstico
   com comandos, estado exato, hashes, resultados e veredito independente.

## Obrigações de conclusão

- Descoberta por execução real, seleção por demanda efetiva e preservação
  de erros/contribuições de irmãos não selecionados.
- Seed observacional vazio para gerações selecionadas, com origem e
  Location preservadas; descoberta não contada como tentativa.
- Registro/resultado/sink por tentativa, revalidação por L1 e reexecução
  somente dos blocos invalidados; mudança de produtor invalida descendentes.
- Descoberta de contextos produzidos pela árvore real, sem colisão silenciosa,
  duplicação de marcadores, acumulação de updates ou sucesso antigo após erro.
- Integração de snapshots, posições e PageStore no orçamento único aprovado;
  saída final corresponde à tentativa efetiva, inclusive ao atingir o teto.
- Erros finais impedem exportação; warnings descartados não escapam nem
  duplicam; Unknown/opacidade não recebem crédito ou warning linguístico falso.
- Caminhos sem seleção conservam comportamento anterior; math continua com
  seu ciclo próprio e HTML fora do layout paginado.

Budget de calibração: no máximo três revisões de contrato e duas execuções
completas pré-selo; duas revisões sem ganho causal exigem revisão de desenho.
Esses limites não autorizam reduzir obrigações ou fabricar PASS.

Relatório: `00_nucleo/diagnosticos/p1340-estabilizacao-contextual.md`.
Veredito estreito: concluído somente com implementação e gates correspondentes;
pendência externa ou pré-condição insuficiente permanece explícita.

## Estado da execução

Baseline, individualização L0, revisão independente e RED local executados.
O dono aprovou a política terminal para estabilidade não comprovável com
“Faça as correções”; o recibo `p1340-terminal-policy-approval.json` e o
contrato sucessor R2 preservam essa decisão. Não há repetição do gate humano.
O selo R3 liberou a implementação estreita após C/D. A sessão privada L3
foi criada e integrada; os testes focais da pipeline, build normal e check
da instrumentação passaram. O diagnóstico registra os recibos e o lint
com zero erros, mas warnings remanescentes — não zero violations.

**Execução parcial; R4 entregue NOT_SEALED.** A decisão independente
`p1340-verifier-gate-pause-r1.json` registra conflito de features no predicado,
identidade de callback e span de atualização não disponíveis nas portas
atuais. O orçamento de três revisões foi consumido; a resposta posterior
“Autorizado” concedeu uma revisão adicional delimitada de contrato de testes
e instrumentação cfg nos callsites necessários. A autoridade prospectiva
`p1340-authority-successor-r4.json` fixa orçamento e parada; inventário
somente leitura precede grant de owners exatos. Não há F/PASS global ou
commit; a aprovação humana da política terminal continua válida.

A instrumentação terminal cfg-only foi ligada ao compilador real. O primeiro
caso abortou porque seu corpo produz `cannot join array with content`, antes
do término pretendido. As fontes exatas T01/T02/T03 também falham no vanilla;
T01 falha no binário preservado anterior ao P1340. O diagnóstico atual conserva
fontes, recibos e a distinção entre defeito de input e defeito do produto.

`p1340-verifier-terminal-input-gate-r4.json` acionou a parada por necessidade
de outra revisão de oráculo. A autorização posterior concedeu somente essa
revisão: R5 acrescentou quatro descartes explícitos `let _ =` a T01–T03,
preservando avaliação, ordem, propagação de erro, T04–T06 e expectativas.

O input R5 foi selado independentemente após 45 rejeições de mutantes. No
candidato, um teste Rust executou os seis casos em ordem normal, repetida e
inversa: **18 observações passaram**. O veredito independente é
`PASS_TERMINAL_R5_CANDIDATE_FRAGMENT_ONLY`, sem `Unknown` nesse fragmento.
O adapter somente de teste aponta agora para o harness R5; produção não teve
mudança semântica nessa correção.

R4 foi preservada como entrega documental, sem calibração/selo ou
implementação de novas portas L1. NT01–NT06, provenance/lifecycle e a matriz
pública/lifecycle continuam pendentes; T05 valida o ramo terminal, não a
descoberta de topologia impossível. Portanto não há F/PASS global, fechamento
do P1339 ou commit. Os recibos, hashes e controles finais estão em
`p1340-estabilizacao-contextual.md`.

## Fecho do próprio passo: não forçar F

A cadeia R6/R6b tentou fechar as pendências acima sem alterar o candidato.
Feasibility confirmou que hooks `cfg(test)` bastariam e que não há novo gate
ADR-0127. Porém o contrato não atingiu poder discriminatório: após duas
revisões, o adversário matou 19 de 41 mutantes e deixou 22 sobreviventes em
oito classes. As expectations externas também não são autoráveis ex ante no
formato R6b, pois misturam identidades dinâmicas por execução com uma entrada
única por caso.

O veredito `p1340-verifier-final-r6.json` é
`NO_SEAL_STOP_REQUIRES_PROTOCOL_REDESIGN`. O budget 2/2 acabou; nenhum F foi
executado ou aceito e não se autoriza R6c silencioso.

Assim, o Passo 1340 encerra como **parcial medido**, com o fragmento terminal
R5 aprovado e o protocolo lifecycle rejeitado antes da implementação. A
continuação correta é um passo novo: separar expectativas semânticas estáveis,
identidades runtime e auditoria estática de callsites; selar esse contrato;
depois implementar os hooks cfg-only e executar lifecycle, NT01–NT06 e gates
finais. Isso evita transformar o 1340 num lote ainda maior ou declarar êxito
com um oráculo que aceita mutações.
