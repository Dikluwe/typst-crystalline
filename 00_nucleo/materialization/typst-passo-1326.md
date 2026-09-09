# Passo 1326 — diagnóstico de campo em closures

## Problema e decisão

A medição fresca `00_nucleo/diagnosticos/p1326-baseline.json` confirma:
`{ let f() = 1; f.nope }` acusa genericamente `type function` e marca
`f.nope` inteiro. Vanilla ratificado `a51e02804` identifica funções
definidas pelo usuário e marca só `nope`. Alias, função anônima e With
confirmam a mesma causa local. Positivos preservam chamadas e valores.

A fila P1322 é hipótese, não aprovação herdada. P1325 fechou o recorte
Dict/Content raw/Float; identidade nativa de `math.join` continua pendente
conforme seu diagnóstico independente. O próximo elegível é
`closure-missing-field-diagnostic`, agora medido no baseline P1325.

## Execução

1. L0 primeiro: `00_nucleo/prompts/compiler/eval/bindings/field_access.md`,
   com sucessão explícita de Closure nas proteções/testes P1311/P1324.
2. Congelar norma, baseline, escopo e expectativas antes de C. Autor de
   testes separado, incluindo atualização estrita dos controles antigos;
   revisão ex ante e RED real. Não adaptar expectations ao candidato.
3. Implementar somente mensagem e span de Closure/With no owner
   `01_core/src/compiler/eval/bindings/field_access.rs`, por categoria já
   existente. Sem nova API, entidade, default ou fase: ADR-0127 contínua.
4. GREEN, build e testes workspace, fmt, lint/linhagem e comparação
   bilateral integral normal/repetida/inversa. Relatório substantivo e
   fechamento em `00_nucleo/diagnosticos/p1326-*`, preservando evidências.

Não alterar Plugin/Element, nativas, lookup, args ou método/gate antecipado.
`f.nope(panic("arg"))` permanece débito de ordem fora do recorte; preservar
baseline nesse caso não vale paridade. O foco é mensagem e origem, não
paridade geral de funções. Nenhuma autorização para corrigir call_dispatch.

Regime da skill tekt-materializacao-segregada: ensaio A/B, com testador e
revisor separados, ambiente compartilhado sem atestação técnica de isolamento
ou selo de refinamento. Unknown obrigatório bloqueia. Duas revisões sem
ganho na mesma causa exigem reabertura; focal antes de corpus completo.
Target exclusivo `/tmp/p1326-target.6Vi4Km`, cópia sem hardlinks do anterior.
Preservar todos os demais arquivos e artefatos. Sem staging, commit ou push.
O passo coordena; somente o L0 legitima código.
