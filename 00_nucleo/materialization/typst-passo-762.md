---
# P762 — Mudar o modelo de avanço entre linhas para `cap-height + leading`

> **Passo:** 762
> **Data:** 2026-07-14
> **Foco:** P761 confirmou, com matemática exacta, que o cristalino avança `line_height` (ascender+descender+lineGap, métricas da fonte) entre linhas, enquanto o vanilla avança `cap-height + par.leading` (default 0,65em) — um modelo diferente, não um valor mal calculado. Isto acumula ~2,304pt de deslocamento por linha em DejaVu Sans 11pt, explicando a maior parte do resíduo de pixels em qualquer parágrafo com mais de uma linha. Afecta praticamente todos os documentos de texto. Prioridade máxima.
> **Tipo:** Sonda + Implementação. Mudança de modelo arquitectural, exige alteração ao L0.
> **Tamanho:** L — mecanismo central de layout de texto, usado por todo o documento.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mudança fundamental no modelo de avanço de linha; sonda obrigatória e cuidado redobrado.
> **Dependências:** P761 (onde a causa foi confirmada com matemática exacta), P750-752 (correcções irmãs de baseline/cap-height, mesma família de mecanismo).

---

## Sonda

### Confirmar o conceito `top-edge`/`bottom-edge` no vanilla, não assumir

```bash
grep -rn "top_edge\|bottom_edge\|TextEdge\|leading" lab/typst-original/crates/typst-library/src/text/mod.rs lab/typst-original/crates/typst-layout/src/inline/*.rs 2>/dev/null | head -30
```

Confirmar a fórmula exacta usada pelo vanilla para o avanço entre linhas, incluindo se `top_edge`/`bottom_edge` são configuráveis (o utilizador pode mudar `#set text(top-edge: ..., bottom-edge: ...)`) — se forem, a correcção não pode ser um valor fixo, tem de respeitar essas opções.

### Confirmar o valor por defeito de `par.leading` e se é configurável

```bash
cat > /tmp/p762-leading.typ <<'EOF'
#set par(leading: 1em)
Linha um linha um linha um linha um linha um linha um.
Segunda linha aqui.
EOF
lab/typst-original/target/release/typst compile /tmp/p762-leading.typ /tmp/p762-vanilla.pdf
mutool show /tmp/p762-vanilla.pdf 4 2>&1 | grep -E "Td|Tm" | head -10
```

Confirmar que `leading` explícito muda o avanço entre linhas de forma previsível, para confirmar a fórmula completa.

### Confirmar todos os call-sites do avanço de linha actual no cristalino

```bash
grep -n "line_height\|fn flush_line" 01_core/src/rules/layout/cursor.rs 01_core/src/rules/layout/*.rs | head -20
```

### Critério de fecho da sonda

- [ ] Fórmula exacta do vanilla confirmada, incluindo configurabilidade de `top-edge`/`bottom-edge`.
- [ ] Comportamento de `leading` explícito confirmado.
- [ ] Todos os call-sites do avanço actual localizados.

---

## Implementação

Mudar `flush_line()` (e qualquer outro ponto de avanço de linha) para usar `cap_height + leading`, seguindo a fórmula confirmada pela sonda, incluindo suporte para `top-edge`/`bottom-edge`/`leading` configuráveis pelo utilizador, se confirmado que existem.

Actualizar o L0 correspondente (`00_nucleo/prompts/rules/layout.md`) para reflectir a nova fórmula.

### Critério de fecho da implementação

- [ ] Avanço entre linhas usa `cap_height + leading`, testado contra o vanilla.
- [ ] `leading` explícito (`#set par(leading: ...)`) funciona correctamente.
- [ ] `top-edge`/`bottom-edge`, se confirmados como configuráveis, respeitados.
- [ ] L0 actualizado, hashes recalculados.

---

## Validação

```bash
cat > /tmp/p762-fixo.typ <<'EOF'
#set page(width: 350pt, margin: 40pt)
#set text(font: "DejaVu Sans", size: 11pt)
#lorem(50)
EOF
lab/typst-original/target/release/typst compile /tmp/p762-fixo.typ /tmp/p762-fixo-vanilla.pdf
./target/release/typst /tmp/p762-fixo.typ /tmp/p762-fixo-depois.pdf
mutool show /tmp/p762-fixo-vanilla.pdf 4 2>&1 | grep -E "Td|Tm" | head -20
mutool show /tmp/p762-fixo-depois.pdf 4 2>&1 | grep -E "Td|Tm" | head -20
```

Confirmar que as coordenadas Y de todas as linhas (não só a primeira) coincidem agora, sem deslocamento crescente.

```bash
mutool draw -o /tmp/p762-fixo-vanilla.png -r 300 /tmp/p762-fixo-vanilla.pdf
mutool draw -o /tmp/p762-fixo-depois.png -r 300 /tmp/p762-fixo-depois.pdf
compare -metric AE /tmp/p762-fixo-vanilla.png /tmp/p762-fixo-depois.png /tmp/p762-diff.png
```

Confirmar redução substancial (não 2,2%) na diferença de pixels face à medição de P761.

```bash
cargo test --workspace
crystalline-lint .
```

Dado o alcance universal (qualquer parágrafo com mais de uma linha), correr o corpus completo com atenção máxima a regressão silenciosa.

### Repetir a reprodução final de `cetz`

```bash
cat > /tmp/p762-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p762-cetz.typ /tmp/p762-cetz.pdf
```

---

## Critério de fecho do passo

- [ ] Sonda completa, fórmula confirmada com configurabilidade.
- [ ] Implementado, coordenadas Y de todas as linhas confirmadas idênticas ao vanilla.
- [ ] Redução substancial da diferença de pixels confirmada com números.
- [ ] Sem regressão em `cargo test --workspace`, corpus completo verificado com atenção máxima.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` sem regressão.
- [ ] L0 actualizado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p762.md`, com hash do commit.
