# Passo 960 — Relatório (quebra visível na curva do assembly: NÃO reproduz pós-P957 — fechado sem código)

**Data**: 2026-08-04
**Estado da árvore**: commit base `54639027c` (P958); zero código neste passo.

---

## Veredicto

**O sintoma já estava corrigido por P957.** A auditoria externa (secção 5.3,
datada 2026-08-04) usou um build anterior a P957 (fechado em 2026-08-03).

## Verificação (mesmo método da auditoria)

- Reproduzida a matriz 3×3 da secção 5 isoladamente (`temp/p960/m33.typ`)
  com o binário actual (pós-P957, commit `d472b523a`+), render a **400dpi e
  800dpi** — o mesmo método declarado pela auditoria.
- Comparação visual directa com o vanilla nas duas juntas (peça de
  canto ↔ extensor, superior e inferior — `temp/p960/*-junta*.png`):
  **curva contínua e lisa nos dois lados** — nenhuma quebra visível em
  nenhuma junta, a 800dpi.
- Contexto: P957 corrigiu exactamente a posição vertical das peças de
  assembly (baseline no fundo do slot; sequência de passos rodada). A
  "quebra" que a auditoria viu é consistente com o estado pré-P957 (peças
  deslocadas `advance_i` acima do sítio certo — a junta entre peças ficava
  desalinhada verticalmente, lendo-se como quebra de ângulo).

## Nota de processo

A dúvida do passo ("residual pós-P957 ou build anterior?") resolveu-se pela
data: P957 fechou em 2026-08-03, a auditoria é de 2026-08-04 mas o PDF que
ela analisou (`test_crystalline.pdf`) foi gerado antes da correcção. A
verificação acima confirma com o build actual. Nenhuma acção de código; as
Fases B/C do passo não se aplicam (condição de não-reprodução da Fase A.3).
