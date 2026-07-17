---
# P749 — Validar correctamente o círculo após P748, e confirmar (não assumir) a causa da diferença de texto

> **Passo:** 749
> **Data:** 2026-07-10
> **Foco:** P748 comparou o topo do círculo (cristalino) com o centro do círculo (vanilla) — dois pontos de referência diferentes do mesmo objecto, o que não prova nada sobre se a posição está correcta. Convertendo para o mesmo ponto de referência (centro), o cristalino ficaria em ~100,867pt e o vanilla em ~84,066pt — uma diferença de quase 17pt, nunca calculada nem confirmada como resolvida ou persistente. Este passo corrige a validação. Também confirma, em vez de assumir, a causa da diferença de ~3,8pt no texto "X" (P748 atribuiu a "métricas de fonte diferentes" sem verificar).
> **Tipo:** Verificação directa. Correcção se confirmado que o círculo ainda diverge.
> **Tamanho:** S-M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P748 (onde a comparação foi feita incorrectamente, com pontos de referência diferentes).

---

## Verificação

### Recalcular a posição do círculo na mesma referência (centro) dos dois lados

```bash
cat > /tmp/p749-circle.typ <<'EOF'
#circle(radius: 30pt, fill: rgb(255, 200, 0))
EOF
lab/typst-original/target/release/typst compile /tmp/p749-circle.typ /tmp/p749-vanilla.pdf
mutool show /tmp/p749-vanilla.pdf 4 2>&1 | head -10
./target/release/typst /tmp/p749-circle.typ /tmp/p749-cristalino.pdf
mutool show /tmp/p749-cristalino.pdf 4 2>&1 | head -10
```

Extrair a posição real do centro do círculo (não do topo) dos dois PDFs — se o content stream expressar o círculo por curvas de Bézier a partir de um ponto específico, calcular o centro a partir dos pontos de controlo, na mesma referência (topo da página) nos dois lados.

### Confirmar visualmente

```bash
mutool draw -o /tmp/p749-vanilla.png -r 150 /tmp/p749-vanilla.pdf
mutool draw -o /tmp/p749-cristalino.png -r 150 /tmp/p749-cristalino.pdf
python3 /tmp/pngdiff.py /tmp/p749-vanilla.png /tmp/p749-cristalino.png
```

Se a diferença calculada (≈17pt) for real, isto deve aparecer claramente no diff de pixels (um deslocamento desta magnitude não seria "anti-aliasing", seria um desalinhamento óbvio) — confirmar directamente, não assumir.

### Confirmar a causa real da diferença de texto

```bash
cat > /tmp/p749-texto.typ <<'EOF'
X
EOF
lab/typst-original/target/release/typst compile /tmp/p749-texto.typ /tmp/p749-texto-vanilla.pdf
./target/release/typst /tmp/p749-texto.typ /tmp/p749-texto-cristalino.pdf
```

Confirmar qual fonte cada um está de facto a usar por defeito (não assumir "CrystallineFont vs LibertinusSerif" sem verificar), e se a diferença de ~3,8pt é explicável pela diferença de métricas dessas fontes especificamente (ascender/cap-height diferentes), ou se é outra coisa.

```bash
mutool show /tmp/p749-texto-vanilla.pdf | grep -i "BaseFont\|FontFile"
mutool show /tmp/p749-texto-cristalino.pdf | grep -i "BaseFont\|FontFile"
```

### Critério de fecho da verificação

- [ ] Posição do centro do círculo calculada na mesma referência nos dois lados, comparada directamente.
- [ ] Diff de pixels confirma (ou refuta) a magnitude do desalinhamento calculado.
- [ ] Fontes reais usadas confirmadas nos dois lados, não assumidas.
- [ ] Causa da diferença de texto confirmada com números (métricas da fonte real), não só atribuída.

---

## Decisão

Se o círculo estiver de facto desalinhado por ~17pt: é uma regressão ou correcção incompleta de P748 — corrigir.

Se a conversão mostrar que está correcto (e a comparação original de P748 estava só mal calculada, não o código): corrigir o registo do relatório, confirmar com números certos.

Se a diferença de texto for confirmada como devida a fontes diferentes: aceitável como divergência conhecida (fonte por defeito diferente é uma escolha, não um bug de posicionamento), mas registada com a confirmação real, não a suposição.

---

## Critério de fecho do passo

- [ ] Verificação completa, círculo comparado na mesma referência.
- [ ] Diff de pixels usado para confirmar a magnitude real.
- [ ] Causa da diferença de texto confirmada com números.
- [ ] Se houver bug real: corrigido e testado.
- [ ] `cargo test --workspace` sem regressão, se houver mudança de código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p749.md`.
