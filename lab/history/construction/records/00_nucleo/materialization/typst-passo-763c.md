---
# P763c — Investigação directa: AE=10725 em `cetz` (line + circle)

> **Passo:** 763c
> **Data:** 2026-07-15
> **Foco:** P763b mediu AE=10725 para um documento `cetz` com `line`+`circle` — 44× o ruído de base medido no mesmo relatório (AE=241 para um `#rect()` simples) — e concluiu "diferença de natureza mecânica de renderização" sem mapa de diferenças, sem comparação de coordenadas, e sem confirmar se o documento de teste de P762 (que reportou AE=0 para `cetz`, segundo o handoff) era o mesmo documento ou outro. O handoff regista explicitamente que essa mesma explicação ("mecânica"/"anti-aliasing") já escondeu bugs reais duas vezes nesta linha de trabalho (P745-748: margem de página; P759-762: modelo de avanço entre linhas). Este passo não aceita a conclusão de P763b sem a medição directa que a regra do handoff exige.
> **Tipo:** Sonda / Investigação. Sem implementação especulativa — só corrige se a investigação confirmar uma causa concreta.
> **Tamanho:** M — pode escalar se a causa for estrutural (como aconteceu em P745-762).
> **ADR-0108 EM VIGOR** — nunca aceitar "parece razão suficiente" sem confirmar directamente.
> **Dependências:** P763b (medição original, AE=10725).

---

## Passo 0 — Reconciliar com P762

Antes de investigar a causa, confirmar se P763b está a comparar o mesmo documento que P762 usou para reportar AE=0.

```bash
grep -rn "AE=0\|paridade de pixels exacta\|cetz" 00_nucleo/diagnosticos/paridade-producao-p762.md 00_nucleo/handoff-novo-chat-p762.md 2>/dev/null
```

Se o documento de P762 for diferente (mais completo, ou com conteúdo distinto de `line`+`circle`), registar explicitamente que P763b introduziu um caso novo, não uma regressão do caso já validado. Se for o mesmo documento, isto é uma regressão e a prioridade sobe.

---

## Passo 1 — Mapa de diferenças (não só a métrica AE)

```bash
compare -metric AE /tmp/p763b-vanilla.png /tmp/p763b-cristalino.png -compose src /tmp/p763c-diffmap.png
compare -metric AE /tmp/p763b-vanilla.png /tmp/p763b-cristalino.png -highlight-color red -lowlight-color none /tmp/p763c-diffmap-highlight.png
```

Inspeccionar visualmente `/tmp/p763c-diffmap-highlight.png`: a diferença está concentrada num deslocamento uniforme dos dois elementos (`line`+`circle` movidos em bloco), num deslocamento de um só elemento, numa diferença de espessura de traço, ou espalhada de forma difusa (consistente com antialiasing real)?

---

## Passo 2 — Comparação de coordenadas exactas

```bash
cat > /tmp/p763c-coords.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
```

Extrair as coordenadas reais desenhadas no PDF (não a imagem rasterizada) para os dois PDFs — vanilla e cristalino — usando o mesmo método já estabelecido em P745-762 para medir posição exacta (extracção de operadores de desenho do PDF, `mutool trace` ou equivalente).

```bash
mutool trace /tmp/p763b-cetz-vanilla.pdf > /tmp/p763c-trace-vanilla.txt
mutool trace /tmp/p763b-cetz.pdf > /tmp/p763c-trace-cristalino.txt
diff /tmp/p763c-trace-vanilla.txt /tmp/p763c-trace-cristalino.txt
```

Registar a diferença de coordenadas em pontos (pt), não só "parece deslocado".

---

## Passo 3 — Isolar a variável

Testar `line` e `circle` separadamente, e sem `cetz` (usando as primitivas nativas do cristalino, `line()`/`circle()` do stdlib, não do pacote), para isolar se o deslocamento vem:

(a) do próprio `cetz` (transformação de coordenadas do canvas, cálculo de bounding box),
(b) das primitivas nativas de desenho do cristalino (`line`/`circle` do stdlib), ou
(c) de posicionamento do canvas em si (top-edge/bottom-edge, a mesma família de bug corrigida em P761-762).

```bash
cat > /tmp/p763c-native-line.typ <<'EOF'
#line(start: (0pt, 0pt), end: (56pt, 28pt))
#circle(radius: 10pt)
EOF
./target/release/typst compile /tmp/p763c-native-line.typ /tmp/p763c-native-cristalino.pdf
lab/typst-original/target/release/typst compile /tmp/p763c-native-line.typ /tmp/p763c-native-vanilla.pdf
compare -metric AE /tmp/p763c-native-vanilla.pdf /tmp/p763c-native-cristalino.pdf /tmp/p763c-native-diff.png
```

Se as primitivas nativas já baterem (AE≈baseline), a causa é específica do `cetz` (item a). Se não baterem, a causa é mais funda (item b ou c) e a prioridade sobe.

---

## Critério de fecho do passo

- [ ] Confirmado se o documento de P762 (AE=0) é o mesmo ou diferente do de P763b.
- [ ] Mapa de diferenças gerado e inspeccionado — descrição do padrão (deslocamento uniforme / traço / difuso).
- [ ] Coordenadas exactas comparadas (pt), não só a métrica AE.
- [ ] Variável isolada — `cetz` vs primitivas nativas vs posicionamento do canvas.
- [ ] Causa concreta identificada e registada, **ou** confirmação directa (com evidência, não suposição) de que é antialiasing/rasterização sem impacto no documento observável.
- [ ] Se causa real encontrada: passo de correcção aberto (P763d), não corrigido especulativamente aqui.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p763c.md`, com o mapa de diferenças anexado/referenciado.

---

## Próximo passo

Se causa real: P763d — correcção específica, com o mesmo padrão de validação (AE antes/depois) usado em P745-762.
Se confirmado antialiasing/rasterização sem impacto observável: fechar a linha de trabalho de download de pacotes (P763-P763c) como concluída, com a divergência registada e justificada por evidência directa, não por analogia.
