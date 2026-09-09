# Passo 1330 — rejeitar overflow inteiro em calc.abs

## Problema medido e recorte

O fechamento P1329 deixou uma lacuna concreta: abs do menor inteiro satura
no cristalino e falha no vanilla ratificado a51e02804. A nova medição está
em `00_nucleo/diagnosticos/p1330-baseline.json`, SHA-256
`a3f732bfb2fda2f177dbf3b33caddcd84b219af6fac22f96eae9644e784b6987`,
com HEAD, working tree não commitado, diff/stat, inventários e binários.
`calc.abs(-9223372036854775807 - 1)` devolve um inteiro saturado no baseline;
vanilla devolve `the result is too large` no argumento.

Este passo muda somente o caso de overflow de um Int já construído, após
os guards vigentes, no owner `01_core/src/compiler/stdlib/calc.rs`.
Não corrige o literal direto -9223372036854775808 (divergência de parsing),
guards, nomes de trace, construção dimensional NaN ou outras funções.
Os resultados dimensionais e diagnósticos P1328/P1329 são preservados.
Este documento coordena execução; a norma é exclusivamente o Prompt L0
`00_nucleo/prompts/compiler/stdlib/calc.md`.

## Execução autorizada

1. Auditar L0/ownership e escrever a obrigação de overflow antes do código.
   Correção de paridade ADR-0127 em fluxo contínuo, sem API ou fase nova.
2. Congelar norma, baseline e capacidades A/B. Autor de testes sem acesso
   ao runtime/patch calc; revisor somente lê produto e escreve pareceres.
3. Testes independentes, com migração restrita das duas expectativas antigas
   de saturação (nativa e expressão avaliada); RED compilado antes de candidato. Cobrir origens reais
   e sintéticas, limites inteiros e preservação de tipos/guards.
4. Implementar apenas a rejeição de overflow; GREEN dos mesmos testes.
5. Repetir corpus CLI anterior com migração exclusiva do overflow; ampliar
   rotas alias/With/spread e fronteiras. Quatro perfis, ordens normal,
   repetida e invertida; dívidas explícitas, nenhuma normalização de traces.
6. Build/workspace, fmt/diff, lint geral e V5/V15/V26 estritos, linhagem
   recíproca, verificação do baseline e evidências históricas preservadas.
7. Relatório em diagnosticos com ganho efetivo, pendências e proveniência.
   Sem commit, push ou novo passo. Temporários em target dedicado /tmp;
   a RAM disponível não comporta o cache, e targets anteriores ficam intactos.

Regime skill Tekt: A/B sem atestação técnica de isolamento, sem selo de
refinamento ou alegação de paridade geral. Unknown obrigatório bloqueia
fechamento. Duas revisões sem ganho na mesma causa exigem rever o recorte;
necessidade de outro owner ou mudança de fase exige parar antes de ampliar.
