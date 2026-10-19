# Execução — reaproveitamento dos produtos Auto de células

Continuação autorizada pelo usuário, após o reuso do primeiro layout. Escopo: cache local consumível para células comprovadamente simples, na mesma fase, sem API pública/flag/unsafe. L0 dos owners alterado antes do código e helper com owner exclusivo. Fluxo contínuo ADR-0127.

RED: duas células Auto construíam quatro subframes. GREEN: dois subframes, conteúdo/métricas preservados. Fixed, alinhamento e placement explícito conservam fallback; testes locais verificam invalidação/consumo único e alteração do estilo efetivo. Suítes core/infra, release build, ownership gate, lint e controles CLI/PDF concluídos. Sem commit nesta sessão.

O [diagnóstico](../diagnosticos/frames-celulas-2026-10-05.md) registra ganhos modestos da CLI, redução causal de construções, fontes/binários/horários exatos, avisos herdados e limites. Não foi implementado cache geral de conteúdo transformado ou persistente entre layouts.
