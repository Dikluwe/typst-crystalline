# Passo 1331 — diagnósticos dos tipos rejeitados por calc.abs

## Recorte medido

O baseline `00_nucleo/diagnosticos/p1331-baseline.json`, SHA-256
`7287bfac86e29e8ff0e86955ce96f63e140b745fb781eb4b4b7410215c794b45`,
mostra que tipos não numéricos continuam rejeitados com mensagem antiga
e span detached. Vanilla a51e02804 informa os tipos aceitos, o tipo real
encontrado e a origem do argumento. `string` e `boolean` são nomes de
diagnóstico diferentes dos nomes curtos `str` e `bool` do cristalino.
Path e comprimento relativo também foram confirmados em
`00_nucleo/diagnosticos/p1331-domain-probes.json`.

Corrigir somente o fallback de um argumento em
`01_core/src/compiler/stdlib/calc.rs`, após os guards atuais. Nenhum tipo
passa a ser aceito. Conteúdo P1328, dimensões P1329, overflow P1330,
guards, nomes das funções e outros consumers permanecem intactos.
Este passo é plano de execução; o L0 proprietário é
`00_nucleo/prompts/compiler/stdlib/calc.md`.

## Implementação e prova

1. Atualizar L0 primeiro; preflight V15/V26 e resselo. Correção de
   paridade ADR-0127 em fluxo contínuo, sem API/default/fase novos.
2. A/B: autor de testes sem ler runtime/patch calc; revisor somente lê
   produto/oráculos. Congelar norma, baseline, testes e expectativas.
3. Migrar somente os controles históricos string/symbol para diagnóstico
   completo novo. Preservar todos os demais testes, inclusive módulos
   P1329 e P1330 inteiros. Evidências históricas não são reescritas.
4. RED compilado antes de C, depois GREEN dos mesmos testes. Cobrir
   famílias rejeitadas, string numérica sem coerção, bool/string versus
   nomes curtos, origens sintéticas/conflitantes, alias/With/spread,
   UTF-8/linhas/warnings, quatro perfis e controles de guards/números.
5. Repetir corpus anterior mais casos novos nas ordens normal/repetida/
   invertida. Congelar literalmente dívidas de trace, parsing e guards;
   não tratar construções que falham antes de abs como cobertura de abs.
6. Build/workspace, fmt/diff, lint, V5/V15/V26, linhagem recíproca e
   histórico íntegro. Relatório substancial em diagnosticos, com ganho,
   limites e proveniência, antes do fechamento independente.

Skill Tekt em A/B sem atestação técnica de isolamento, sem selo de
refinamento. Unknown obrigatório bloqueia; duas revisões sem ganho
exigem rever o recorte. Outro consumer necessário refuta a suficiência
antes de ampliar. Temporários no target dedicado /tmp por limite de RAM.
Sem stage, commit, push ou criação do próximo passo.
