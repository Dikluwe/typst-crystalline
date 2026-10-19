---
# P695 — `repr` de dict vazio e array vazio são ambíguos?

> **Passo:** 695
> **Data:** 2026-07-10
> **Foco:** P694 encontrou que `repr((:))` (dict vazio) dá `()` no cristalino, diferente do vanilla (`(:)`), e classificou como diferença "mecânica" de formatação. Confirmar directamente se `repr(())` (array vazio) também dá `()` — se sim, isto não é só formatação, é uma ambiguidade real: dois tipos diferentes a produzir texto idêntico, o que pode enganar quem lê o `repr` para depurar código.
> **Tipo:** Verificação directa. Correcção se confirmada a ambiguidade.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P694 (onde a diferença foi encontrada e classificada como mecânica).

---

## Verificação

```bash
cat > /tmp/p695-repr.typ <<'EOF'
#repr(())
#repr((:))
#(() == (:))
EOF
lab/typst-original/target/release/typst compile /tmp/p695-repr.typ /tmp/p695-vanilla.pdf
pdftotext /tmp/p695-vanilla.pdf -

./target/release/typst /tmp/p695-repr.typ /tmp/p695-cristalino.pdf
pdftotext /tmp/p695-cristalino.pdf -
```

Confirmar se `repr(())` e `repr((:))` dão o mesmo texto no cristalino, e se o vanilla os distingue (`()` vs `(:)`).

---

## Decisão

Se confirmada a ambiguidade: corrigir `repr` de dict vazio para `(:)`, distinto de array vazio (`()`), seguindo o vanilla — isto deixa de ser "diferença mecânica de formatação" e passa a ser uma correcção de clareza necessária, já que dois tipos diferentes não devem produzir o mesmo texto de depuração.

Se não confirmada (por exemplo, se o cristalino já distinguir de outra forma que a sonda anterior não testou): manter como estava, com a classificação de P694 confirmada, não só assumida.

---

## Critério de fecho do passo

- [ ] Testado directamente, ambiguidade confirmada ou refutada.
- [ ] Se confirmada: `repr` de dict vazio corrigido para `(:)`.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p695.md`, com hash do commit.
