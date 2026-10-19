---
# P758 — Aspas CJK ainda no início de linha em espaço apertado

> **Passo:** 758
> **Data:** 2026-07-14
> **Foco:** P756 implementou tailoring de aspas CJK (impedindo `U+201C` de ser um ponto de quebra válido), mas em páginas muito estreitas a aspa ainda pode acabar no início de linha, porque o layout greedy decide linha a linha sem olhar para o fragmento seguinte. O vanilla evita isto porque o Knuth-Plass optimiza globalmente. Este passo investiga se há uma correcção local (sem precisar de Knuth-Plass completo) que resolva o caso específico, dado que o tailoring de breakpoints já impede a aspa de ser separada do carácter anterior — o problema agora é só a decisão de "cabe ou não cabe" na fronteira.
> **Tipo:** Sonda + Implementação, condicional ao que a sonda revelar.
> **Tamanho:** S-M, se houver correcção local; caso contrário, converge com P759 (Knuth-Plass).
> **ADR-0108 EM VIGOR.**
> **Dependências:** P756/P757 (onde o problema foi confirmado e registado como scope-out).

---

## Sonda

### Confirmar exactamente o mecanismo do problema

```bash
cat > /tmp/p758-aspas.typ <<'EOF'
#set page(width: 100pt, margin: 5pt)
#set text(font: "Noto Serif CJK SC", size: 12pt)
测试文本，"测试引号的位置"。这是一段很长的中文文字用来测试换行的效果如何。
EOF
./target/release/typst /tmp/p758-aspas.typ /tmp/p758-antes.pdf
pdftotext /tmp/p758-antes.pdf -
```

Confirmar exactamente em que ponto o fragmento `"测试引号的位置"` (que começa com a aspa já impedida de ser ponto de quebra) é maior do que o espaço restante na linha corrente, forçando o greedy a mover o fragmento inteiro para a linha seguinte — com a aspa a acabar no início dessa linha, porque é o primeiro carácter do fragmento.

### Confirmar se o vanilla resolve isto com Knuth-Plass, ou com outra heurística local

```bash
grep -n "fn.*kinsoku\|prevent.*start\|leading.*punct" lab/typst-original/crates/typst-layout/src/inline/*.rs 2>/dev/null | head -20
```

Confirmar se o vanilla tem alguma heurística específica para este caso (por exemplo, "se o próximo fragmento começar com uma aspa proibida de iniciar linha, tentar encaixá-la à força, mesmo que ultrapasse ligeiramente a margem, ou comprimir a linha anterior"), independente do Knuth-Plass global.

### Avaliar uma correcção local possível

Uma hipótese: quando o layout greedy decide mover um fragmento para a linha seguinte, e esse fragmento começa com um carácter proibido de iniciar linha (a mesma classe já usada no tailoring de P756), tentar puxar esse carácter específico de volta para o fim da linha anterior, mesmo que isso ultrapasse ligeiramente a margem (efeito de compressão/hanging punctuation), replicando uma técnica tipográfica comum sem precisar de Knuth-Plass completo.

### Critério de fecho da sonda

- [ ] Mecanismo exacto do problema confirmado.
- [ ] Confirmado se o vanilla usa uma heurística local para este caso específico, ou só o Knuth-Plass global.
- [ ] Avaliada a viabilidade de uma correcção local (hanging punctuation) sem Knuth-Plass completo.

---

## Implementação, condicional ao resultado da sonda

Se houver uma correcção local viável: implementar, testada com o caso de P756/P757.

Se não houver (o problema só se resolve com optimização global): registar como dependente de P759 (Knuth-Plass), sem forçar uma correcção parcial que não resolveria o caso geral.

---

## Validação

```bash
./target/release/typst /tmp/p758-aspas.typ /tmp/p758-depois.pdf
pdftotext /tmp/p758-depois.pdf -
```

Comparar com o vanilla 0.15.0 já obtido em P757.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, mecanismo confirmado.
- [ ] Corrigido, se viável localmente; ou dependência de P759 registada explicitamente, se não.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p758.md`, com hash do commit.
