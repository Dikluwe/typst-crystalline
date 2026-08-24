# Passo 1140.20.2 — Canvas físico e camadas de página

**Estado:** executado — gate confirmado e implementação fechada por P1140.23  
**Data:** 2026-08-24  
**Continua:** P1140.20.1  
**Gate:** ADR-0127 obrigatório antes de código

## Objetivo

Implementar `bleed`, `fill`, `background` e `foreground` de ponta a ponta:
entidade, parsing, layout, `Page` e exporters, sem expor `page`.

## Fase A — L0 e gate

1. Revalidar `page.rs:170-219,257-278,452-492`, `pages/run.rs:132-140,218-244`,
   `document.rs:88-121` e consumers PDF/SVG/raster.
2. Especificar bleed por lado e lógico, canvas trimado versus físico e caixas
   de export. Não exigir igualdade de bytes PDF.
3. Preservar `fill` ternário: auto, transparente e paint; resolver auto no
   target, não em L1.
4. Especificar ordem fill→background→body→foreground e resolução percentual
   contra página incluindo bleed.
5. Atomizar domínio, composição de página e consumers L3 nos seus owners.
6. Atualizar L0, declarar incompletude até .3/.4, resselo e **parar**.

## Fase B — RED→GREEN, após confirmação

- RED para bleed uniforme/dict, zero/omitido, inside/outside e restauração;
- RED estrutural para MediaBox/TrimBox quando bleed é não zero;
- RED por target para fill auto/none/paint;
- RED para ordem visual e referencial de background/foreground;
- RED para page-runs aninhados e multipágina;
- implementar sem importar L3 em L1 e sem assar frames no eval.

## Aceitação

Todo argumento aceito chega ao consumer final; caixas e ordem de pintura têm
oracles semânticos; acessibilidade não duplica layers decorativos; testes,
build, lint e diff check passam.

## Resultado parcial

Fase A executada. O L0 `entities/page_canvas.md` e nove consumers foram
escritos/atualizados e resselados. Nenhum teste RED ou código funcional desta
fase foi iniciado. Relatório:
`00_nucleo/diagnosticos/typst-p1140.20.2-gate-canvas-page.md`.

Após a confirmação, a implementação iniciou somente o owner de domínio em
`entities/page_canvas.rs`. O trabalho ainda aberto foi materializado em
`typst-passo-1140.22.md`; P1140.20.2 permanece não fechado até esse passo
passar integralmente.
