# Passo 1333 — erro do primeiro valor de `calc.abs` antes das sobras

## Problema medido

O baseline público em `00_nucleo/diagnosticos/p1333-baseline-public.json`
registra comandos, horários, binários e a árvore não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`. `calc.abs([x], 2)`
produz erro de aridade no cristalino, mas erro de tipo no vanilla ratificado
`a51e02804`. O mesmo desvio atinge overflow e comprimento misto.

## Entrega

Atualizar primeiro o L0 proprietário `00_nucleo/prompts/compiler/stdlib/calc.md`.
Depois corrigir somente `01_core/src/compiler/stdlib/calc.rs`: se houver
primeiro posicional e seu processamento falhar, conservar seu diagnóstico
e origem, mesmo com argumentos adicionais. Quando esse valor for válido,
ou estiver ausente, preservar os guards atuais. Não mudar avaliação eager,
API, dispatcher, tipos aceitos, identidade ou nome da função.

O recorte não fecha a validação completa de argumentos: mensagens/ordem
das sobras, named `value:` e origem da chamada ausente ficam para P1334.
Este passo é coordenação; somente o L0 legitima a implementação.

## Verificação e conclusão

Regime A/B sem atestação técnica de isolamento: autor independente congela
testes e migração explícita das expectativas históricas antes do candidato;
revisor separado julga escopo e evidências. Demonstrar RED compilado,
implementar, obter GREEN, build/workspace, fmt, linhagem e linter sem
violações. Repetir corpus CLI congelado em ordem normal e inversa.
Unknown obrigatório bloqueia conclusão; duas revisões focais sem ganho
reabrem o desenho. Relatório substantivo e proveniência em
`00_nucleo/diagnosticos/p1333-final-report.md`. Preservar mudanças anteriores.
Sem commit, stage ou push neste pedido.
