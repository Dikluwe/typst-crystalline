# P1335 — veredito independente

**Preserved: auditoria válida no recorte declarado**, com uma recomendação:
`primitive-instance-field-diagnostic`. Não é atestado de paridade total,
refinamento produtivo, mutation score ou isolamento técnico.

A skill `tekt-materializacao-segregada` orientou a separação de autoria,
o congelamento dos ataques antes do julgamento, a preservação dos antecedentes
e o veto a Unknown como sucesso. Regime: **executado sem atestação técnica de
isolamento**. Este revisor só escreveu novos diagnósticos `p1335-review-*`;
não alterou produto, L0, catálogo, runners, resultados ou ledgers julgados.

## Evidência atual

- A união estrutural de 4708 rotas e dez controles históricos forma 4718
  probes/paths distintos. As 18872 células principais foram recompostas em
  normal/repeat/reverse, sem Unknown ou instabilidade. A igualdade bruta
  continua 18220/18872 (96,5451%); todos os canais são iguais aos de P1322.
- O ajuste documental exclui 156 células C-only de 39 extensões explicitamente
  retidas no L0 atual: 18220/18716 (97,3499%), sem crédito novo no numerador.
  Os bindings calc.deg/rad/log10 continuam dentro do débito.
- O suplemento de 1125 casos/4422 células por ordem permanece separado.
  A preservação histórica foi reconstruída em 1332 comparações literais,
  32 transformações posteriores exatas de mensagem/span e 12 expectativas
  confirmadas no path original por 36 observações. Avanço temporal comparável:
  64 células; outros 12 fechamentos atuais têm fixture relocalizada.
- Os canais transversais não foram apagados por igualdade de JSON/DOM/geometria.
  Query com features é rejeitado pelo cristalino, não pelo vanilla. As duas
  rejeições CLI vanilla de legacy compile/document-id são capacidade ausente,
  nunca MATCH de export/warning. Dezesseis controles negativos próprios da
  exceção permanecem Unknown. Contrato C de warning e paridade são distintos.
- Os quatorze ataques únicos R01–R14 foram executados em cópias de dados e
  rejeitados com controles válidos. Não há mutantes de produto ou crédito de
  certificação por esse resultado; 37 famílias herdadas e P1308 permanecem
  pendentes/separados.
- Build, testes release locked, fmt, lint, V5/V15/V26 e diff-check passaram.
  Permanecem explícitos 240 warnings, 1146 infos e três doctests ignorados.
  A preservação da baseline abrange 3912 arquivos produtivos/L0/lab, 4333
  evidências históricas não rastreadas e nove inputs históricos pinados.

## Decisão e limites

O L0 field_access:320 exige `integer`; o fallback atual usa `int` e origem
agregada. Int e Str compartilham a causa e o helper já importado. A coorte
vence por prioridade 2, um owner, dois caminhos e superfície relativa 1.
Math lexical abs/sqrt empata nos três primeiros critérios, mas tem superfície
2. R13 detectou a troca indevida por math; R14 detectou a retirada de
call_dispatch de loader DataSource mesmo com contagem/rank coerentes.
Nenhuma implementação futura é autorizada por este veredito.

O scanner finito não prova enumeração ilimitada; uma fronteira de profundidade
hipotética pode ser filtrada antes da emissão. A revisão não reexecutou todos
os renderizadores nem certifica layout/PDF/HTML/a11y globais. A QA visual do
operador é amostral. Oito PDFs focais R0 foram sobrescritos e não recuperados:
o erro de retenção permanece registrado; só os exports sucessores preservados
fundamentam o fechamento. Os JSONs e canais anteriores continuam disponíveis.

Também ficam preservadas as calibrações do próprio leitor: ordem de argv de
localização, campo ausente em gate não executado, política de rejeição CLI,
path latente P1322 corrigido antes do gate de preservação e a retificação da
confusão entre união4708 e catálogo4718. Nenhum erro foi escondido por edição
retroativa de resultado julgado.

Os hashes, componentes e controles ativos estão em `p1335-review-final.json`.
O algoritmo `p1335-close.py` foi lido integralmente: só sela diagnóstico após
este veredito, revalida estado/exports e registra a perda dos PDFs R0. O
fechamento só se completa quando o operador executar e conferir esse recibo.
