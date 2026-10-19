:warning: **Nota de correção retroativa — P453, C5 (contagem de ADRs)**

**Data:** 2026-06-25  
**Autor:** Auditoria pós-P454  
**Referência:** Spec P453, Secção C5; Relatório P453, sumário executivo.

---

## Divergência

O spec de P453 (C5) mandava actualizar o documento de cobertura para **67 ADRs**.
O relatório de P453 declarou **68 ADRs**.

## Causa

O spec foi escrito sem contar o efeito do próprio passo. P453 criou a **ADR-0117**
(mecanismo da sonda A.0), elevando o total de 67 → 68. O spec pediu 67,
esquecendo que o passo ia adicionar uma ADR.

## Correção

- O **documento de cobertura** ficou com 68 (correcto, via relatório P453).
- O **spec de P453** deveria ter declarado:
  "Actualizar para **68 ADRs** (67 pré-existentes + ADR-0117 criada neste passo)."

## Lição

Este é o mesmo tipo de descuido de número que a auditoria persegue: o spec foi
escrito sem contar o efeito do próprio passo sobre o contador que ele próprio
estava a actualizar. A sonda A.0 (ADR-0117) deve incluir:
"O próprio passo altera contadores que a sonda mede? Se sim, ajustar a contagem
final."

## Aplicação

Marcar o spec de P453 como contendo errata documentada. Não reescrever o
ficheiro (histórico preservado), mas anexar esta nota como
`typst-passo-453-nota-adr68.md` em `00_nucleo/materialization/`.
