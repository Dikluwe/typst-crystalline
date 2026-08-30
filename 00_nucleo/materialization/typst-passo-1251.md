# P1251 — fechamento noturno e próxima fronteira

**Estado:** FECHADO — CONSOLIDAÇÃO SANEADA E GATES PASS  
**Predecessor:** P1250  
**Saída:** estado reproduzível para retomada humana.

Consolidar todos os vereditos P1232–P1250 sem promover item rejeitado ou apenas
diagnosticado. Atualizar mapa DSM, fila P1213 e relatório de paridade SVG com
proveniência completa. Distinguir `Preserved`, `Unknown`, `Violated`,
`CONTRACT-GAP` e `EXTERNAL-LAYOUT`.

Rodar build, testes afetados, SVG completo, core gradient, fmt, diff-check,
V5/V15/V26, linter completo e lente contra o vanilla pinado. Não fazer commit,
merge, mudança pública ou decisão ADR sem autorização explícita. Deixar um
resumo contendo: último passo realmente executado, passos pulados e por quê,
primeiro blocker, hashes das provas e próxima decisão que exige o dono.

Em filesystem compartilhado, declarar `EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`.

## Critério saneado de execução

O relatório histórico que classificava P1245–P1250 como rejeitados foi
invalidado pelos certificados posteriores. O fechamento deve consumir o estado
terminal efetivo de cada passo, sem reescrever recibos históricos de falha e
sem converter sucesso de fragmento em equivalência geral.

O P1251 só fecha após:

1. reconciliar o relatório noturno, a fila P1213 e o mapa DSM com os
   certificados P1245–P1250;
2. manter `Unknown` para paint multi-space não provado, links cross-page,
   imagens opacas e wrappers sem fontes;
3. executar testes focais P1236–P1250, suíte do workspace, build, formatação,
   `git diff --check`, V1/V5/V15/V26, lint completo e lente DSM;
4. registrar HEAD, árvore não commitada, horário, comandos e hashes das provas;
5. publicar veredito limitado e declarar a ausência de isolamento ambiental
   forte do filesystem compartilhado.

Até a conclusão dos gates, nenhuma promoção adicional é alegada.

## Resultado executado — 2026-08-28

O relatório noturno, a fila P1213 e o mapa DSM foram reconciliados com os
certificados efetivos P1245–P1250. A fotografia rejeitada anterior foi mantida
apenas nos seus recibos históricos; o estado terminal agora reconhece os
fragmentos materializados sem promover equivalência SVG geral.

`cargo test --workspace`, `cargo build --workspace`, formatação,
`git diff --check`, V1/V5/V15/V26 e lint completo passaram. A lente DSM passou
duas vezes com saída byte-idêntica. O lint normal registrou V16=211, V19=358 e
V20=635, todos warnings sob a política vigente; nenhum deles foi convertido em
silêncio ou alegado como zero. O gate arquitetural focal terminou sem violações.

A execução ocorreu no HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`
com working tree não commitada. O certificado P1251 registra hashes, contagens,
comandos e limitações. Veredito: `PASS_DIAGNOSTIC_CLOSURE`, limitado à
consolidação e aos fragmentos previamente certificados.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
