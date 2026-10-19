# Passo 1140.20.1 — Geometria lógica de página

**Estado:** executado  
**Data:** 2026-08-24  
**Continua:** P1140.20  
**Gate:** ADR-0127 obrigatório antes de código

## Objetivo

Implementar `paper`, `flipped`, `binding` e completar `margin` em set-rules e
page-runs, sem expor ainda o binding global `page`.

## Medição vinculante

Usar a matriz e proveniência de
`diagnosticos/typst-p1140.20-propriedades-page.md`. Antes de decidir tipos,
revalidar `page.rs:54-168,598-801`, `pages/run.rs:101-155` e o L0 vigente.

## Fase A — L0 e gate

1. Medir os módulos cristalinos tocados e confirmar seus hashes.
2. Definir tipos fechados para `Paper`, `Binding` e lados lógicos; a tabela de
   papel deve ser normativa e nomeada, nunca derivada de valores de um PDF.
3. Especificar precedência paper→width/height, flip posterior, fórmula de
   margem automática, fold e paridade inside/outside.
4. Atualizar L0s de entities/content/page_run, eval/stdlib e layout.
5. Declarar que o constructor continua incompleto até P1140.20.2–.4 e que
   `page` só aparece em P1140.21.
6. Resselar hashes e **parar para confirmação**.

## Fase B — RED→GREEN, após confirmação

- RED para papéis aceitos/inválidos, overrides por eixo e flipped;
- RED para binding auto sob LTR/RTL e troca ímpar/par;
- RED para margin scalar/dict/auto, precedência rest/x/y/lado, fold e conflito
  left/right versus inside/outside;
- RED para isolamento/restauração em page-run e rejeição em containers;
- implementar tipos, parsing e free functions atomizadas;
- estender `SetPage`, `PageConfig` e `PageRunElem` sem mapas dinâmicos.

## Aceitação

Sem defaults empíricos; erros inválidos explícitos; dimensões e região do body
batem semanticamente com o vanilla; restauração LIFO; testes completos;
`cargo build`, testes relevantes, `crystalline-lint .` e `git diff --check`.

## Resultado parcial

Fase A executada em 2026-08-24. O L0 novo
`entities/page_geometry.md` e os seis L0s consumidores foram escritos e
resselados. A tabela de papel passa a ser normativa em milímetros; os literais
A4 em pontos ficam mandatados para remoção na Fase B. Nenhum teste RED nem
comportamento foi implementado. Relatório do gate:
`00_nucleo/diagnosticos/typst-p1140.20.1-gate-geometria-page.md`.

## Resultado final

Gate confirmado pelo dono. Fase B concluída com tabela normativa completa,
parsing, transporte, aplicação atomizada, restauração e resolução de margens
lógicas por paridade. Relatório:
`00_nucleo/diagnosticos/typst-p1140.20.1-geometria-page.md`.
