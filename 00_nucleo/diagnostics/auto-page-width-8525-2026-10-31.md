# Largura automática e divisor — issue 8525

Correção interna implementada na branch Tekt, sem commit. O divisor não amplia
mais a largura intrínseca com infinito; recebe a largura finita do corpo antes
de fechar a página ou compor o container. Subframes, Hide, Block e rollback
preservam os pedidos privados e o conteúdo dos dois lados.

A reprodução oficial produz página de 483.2819437570304 pt e divisor de
282.832 pt, com ambos os parágrafos preservados em PDF/SVG. Não se adotou o
divisor zero do vanilla. L0 proprietários foram atualizados e resselados antes
do código; RED funcional e candidato refutado foram preservados.

Validação final: 8.250 testes do core passaram (zero falhas; cinco ignorados
preexistentes), seis testes específicos passaram, 16 fontes PDF/SVG sem falhas,
`cargo build --locked` passou e crystalline-lint completo ficou sem erros ou
avisos. Compilações, executáveis e renders finais usam o target exclusivo em RAM
`/dev/shm/typst-crystalline-8525-target`.

HEAD, árvore não commitada, diffstat, UTC, fontes, binários, comandos e recibos:
[relatório e proveniência](../../lab/results/construction/diagnostics/auto-page-width-8525-2026-10-31/report.md).
Os números acima pertencem ao estado de `completion.json` e `final.json.gz` nesse
diretório; não certificam paridade geral de páginas automáticas ou outros bugs.
