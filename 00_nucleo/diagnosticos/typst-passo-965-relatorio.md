# Passo 965 — Relatório (gate de L0 formalizado: ADR-0127)

**Data**: 2026-08-04
**Estado da árvore**: commit base `0c01589b4` (P964); só documentação.

---

## Fase A — critério confirmado contra os passos executados

O critério proposto no enunciado foi verificado contra P893–P964 e
**separa limpo todos os casos** (tabela na ADR §3). Um caso de fronteira
foi discutido e fechado na redacção: correções de paridade que mudam o
output visível (P958: `Gamma(z)` → Γ(𝑧)) pareciam colidir com "mudança de
comportamento por defeito" — a distinção adoptada é **intenção de produto
vs convergência para o vanilla**: a primeira para (P956); a segunda flui
(P958/961/962/963/964). Sem essa cláusula, o critério exigiria paragem em
cada bug fix — inviável e não era o espírito da queixa de P952 (que era
sobre contrato, não sobre paridade).

**Não-colisão (varredura real)**: `paragem|confirma|gate|PARAR` em
`00_nucleo/adr/` — nenhuma ADR decide quando parar após editar L0;
ADR-0114/0117 são de método (complementares). Número por listagem real:
último `0126` → **0127 livre**.

## Fase B — ADR escrita + Regra de Ouro actualizada

- `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md` — critério em
  4+4 pontos, tabela de verificação contra os passos, regra de bolso ("em
  dúvida, parar") e registo operacional da confirmação no relatório do
  passo.
- `CLAUDE.md` (Regra de Ouro): nova cláusula "Quando parar para
  confirmação (ADR-0127)" — a paragem deixa de ser ambígua; tabela de
  ADRs vigentes ganha as linhas 0126 (estava em falta desde P954) e 0127.
- `adr/README.md`: linha na tabela + nota no ledger.
