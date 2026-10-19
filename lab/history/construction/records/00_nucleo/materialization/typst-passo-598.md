---
# P598 — Confirmar a fórmula exacta de margem do vanilla

> **Passo:** 598
> **Data:** 2026-07-05
> **Foco:** P597 mediu que 70,87 ÷ 595,28 ≈ 0,119, a mesma proporção encontrada na margem do vanilla para uma página de 200pt de altura. Isto sugere que a margem fixa do cristalino é, por coincidência, o resultado de uma fórmula proporcional nunca implementada como tal — só calculada uma vez para A4 e gravada como número fixo. Este passo confirma a fórmula exacta com mais pontos de dados, antes de decidir se a correcção é pequena (trocar um número por uma conta) ou realmente precisa da escala que P597 sugeriu.
> **Tipo:** Sonda directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P597 (onde a coincidência numérica apareceu, sem ser confirmada como fórmula).

---

## Sonda

### Testar mais tamanhos de página no vanilla

```bash
for altura in 100 150 200 300 500 841.89 1200; do
  cat > /tmp/p598-h$altura.typ <<EOF
#set page(height: ${altura}pt)
teste
EOF
  lab/typst-original/target/release/typst compile /tmp/p598-h$altura.typ /tmp/p598-vanilla-h$altura.pdf
  mutool draw -F stext /tmp/p598-vanilla-h$altura.pdf 2>/dev/null | grep -o 'bbox="[0-9.]*' | head -1
done
```

Para cada altura testada, extrair a margem esquerda medida (o primeiro valor do `bbox`), e calcular a proporção `margem ÷ menor_dimensão_da_página`.

### Testar também variando a largura, não só a altura

```bash
for largura in 200 300 400 595.28 800; do
  cat > /tmp/p598-w$largura.typ <<EOF
#set page(width: ${largura}pt, height: 841.89pt)
teste
EOF
  lab/typst-original/target/release/typst compile /tmp/p598-w$largura.typ /tmp/p598-vanilla-w$largura.pdf
  mutool draw -F stext /tmp/p598-vanilla-w$largura.pdf 2>/dev/null | grep -o 'bbox="[0-9.]*' | head -1
done
```

Confirmar se a fórmula usa sempre a menor dimensão, ou se é sempre baseada na largura, ou sempre na altura, independentemente de qual é menor.

### Confirmar directamente no código fonte do vanilla, se disponível

```bash
grep -rn "margin\|2.5cm\|DEFAULT_MARGIN" lab/typst-original/crates/typst-library/src/layout/page.rs 2>/dev/null | head -20
```

Se o código fonte do vanilla estiver acessível, a fórmula exacta pode estar ali escrita directamente, sem precisar de inferir a partir de medições.

### Critério de fecho da sonda

- [ ] Proporção confirmada com pelo menos 5 tamanhos de página diferentes, não só um.
- [ ] Confirmado se a fórmula usa a menor dimensão, a maior, a largura, ou a altura especificamente.
- [ ] Se o código fonte estiver acessível: fórmula exacta confirmada por leitura directa, não só inferida por medição.

---

## Decisão

Se a fórmula for confirmada como simples e consistente (por exemplo, "12% da menor dimensão, com um mínimo de X pontos"): a correcção é pequena — trocar o valor fixo em `PageConfig::default` por esta conta, sem precisar de reestruturar nada. Escrever o L0 pequeno que documenta a fórmula, e implementar directamente.

Se a fórmula for mais complexa, ou não for consistente entre os testes: aceitar a hesitação de P597 como correcta, e tratar como mudança maior, precisando de mais sonda antes de implementar.

---

## Critério de fecho do passo

- [ ] Fórmula confirmada com múltiplos pontos de dados.
- [ ] Decisão registada: correcção pequena e directa, ou mudança maior a adiar com razão.
- [ ] Se for pequena: implementada neste mesmo passo, com teste de regressão para A4 (confirmando que o caso comum não muda) e para o caso de P597 (confirmando que `height: 200pt` passa a bater com o vanilla).
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p598.md`, com hash do commit.
