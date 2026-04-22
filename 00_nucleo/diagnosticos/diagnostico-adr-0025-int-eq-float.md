# Diagnóstico prévio à ADR-0025

**Contexto**: registo do diagnóstico executado antes da decisão
formalizada em `00_nucleo/adr/typst-adr-0025-int-eq-float.md`.

**Data do diagnóstico**: 2026-03-28 (obtida por `git blame` ao
cabeçalho original da secção movida).

**Natureza**: este ficheiro é registo histórico. Os comandos
abaixo foram executados antes da decisão do ADR-0025 ser tomada;
o estado do código pode ter mudado desde então. Consultar este
ficheiro para entender o contexto factual da decisão, não para
re-executar.

---

```bash
# Confirmar a semântica exacta do original para Eq com tipos mistos
grep -n "Int.*Float\|Float.*Int\|Eq\|eq.*value" \
  lab/typst-original/crates/typst-library/src/foundations/ops.rs \
  | head -30

# Como PartialEq está implementado no Value original (manual ou derive?)
grep -n "PartialEq\|fn eq\b" \
  lab/typst-original/crates/typst-library/src/foundations/value.rs \
  | head -15
```
