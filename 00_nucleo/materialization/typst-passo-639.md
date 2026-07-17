---
# P639 — `table.numbering` deve existir no cristalino?

> **Passo:** 639
> **Data:** 2026-07-09
> **Foco:** P638 encontrou, de passagem, que o vanilla não tem propriedade `numbering` em `table` — a mensagem de erro é "unexpected argument: numbering", não um erro de tipo. O cristalino aceita `#set table(numbering: "1")` com tipo certo. Confirmar o que essa propriedade faz de facto no cristalino hoje, e se deve continuar a existir.
> **Tipo:** Sonda directa. Implementação só se a sonda confirmar que é preciso mudar algo.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P636 (onde o tipo de `table.numbering` foi validado, sem se questionar se a propriedade devia existir), P638 (onde a ausência no vanilla foi encontrada).

---

## Sonda

### Confirmar o que `table.numbering` faz hoje no cristalino

```bash
grep -n "table.*numbering\|numbering.*table" 01_core/src/rules/eval/rules.rs 01_core/src/rules/layout/*.rs 01_core/src/entities/elements/table*.rs 2>/dev/null
```

Confirmar se esta propriedade é lida nalgum sítio depois de ser definida, ou se fica guardada sem nunca ser usada — o que confirmaria código morto.

```bash
cat > /tmp/p639-table-numbering.typ <<'EOF'
#set table(numbering: "1")
#table(
  columns: 2,
  [A], [B],
)
EOF
./target/release/typst /tmp/p639-table-numbering.typ /tmp/p639.pdf
pdftotext /tmp/p639.pdf -
```

Confirmar visualmente se `numbering` tem algum efeito observável no output.

### Confirmar se `table` deve ter numeração no vanilla através de outro mecanismo

Tabelas no Typst costumam ser numeradas através de `#figure(table(...), caption: [...])`, não directamente na função `table`. Confirmar se é isso que está a acontecer — a propriedade `numbering` pertence a `figure`, não a `table`, e o cristalino pode tê-la posto no sítio errado por engano.

```bash
grep -n "figure.*numbering\|numbering.*figure" lab/typst-original/crates/typst-library/src/model/figure.rs 2>/dev/null | head -10
```

### Critério de fecho da sonda

- [ ] Confirmado se `table.numbering` no cristalino tem algum efeito, ou é código morto.
- [ ] Confirmado se a numeração de tabelas pertence a `figure`, não a `table`, no vanilla.

---

## Decisão

Se `table.numbering` não tiver efeito nenhum hoje: remover a propriedade de `#set table(...)`, produzindo o mesmo erro "unexpected argument" que o vanilla já dá, em vez de aceitar silenciosamente um valor que não faz nada.

Se `table.numbering` tiver efeito real, e for uma extensão genuína do cristalino além do vanilla: manter, mas registar explicitamente como capacidade extra, não como algo a validar "para bater com o vanilla" — a mensagem de erro de tipo (já corrigida em P636) fica correcta, mas o enquadramento muda.

---

## Implementação, condicional ao resultado da sonda

Depende inteiramente da sonda.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa.
- [ ] Decisão registada — remover, manter como extensão, ou corrigir para o sítio certo (`figure`, não `table`).
- [ ] Se implementado: testado sem regressão.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p639.md`, com hash do commit.
- [ ] Entrada 11 de P633/P636 actualizada com a decisão final, não deixada como estava.
