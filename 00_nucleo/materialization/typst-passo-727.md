---
# P727 — Corrigir o render de `curve` (página em branco)

> **Passo:** 727
> **Data:** 2026-07-10
> **Foco:** P726 confirmou, com diff de pixels, que `#curve(...)` compila (`exit 0`) mas renderiza página em branco — zero pixels não-brancos, contra 1043 no vanilla para o mesmo documento directo. `rect`/`circle` renderizam correctamente (979 vs 972 pixels), confirmando que o bug é específico de `curve`/paths, não de formas vectoriais em geral. Este é o único bloqueio restante para paridade visual completa de `cetz`.
> **Tipo:** Sonda + Implementação. Prioridade máxima — último passo conhecido da cadeia.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P723 (onde o bug foi encontrado pela primeira vez), P726 (confirmação com diff de pixels, isolamento como específico de paths).

---

## Sonda

### Reproduzir isoladamente, com o mínimo possível

```bash
cat > /tmp/p727-curve-minimo.typ <<'EOF'
#curve(
  curve.move((0pt, 0pt)),
  curve.line((50pt, 50pt)),
)
EOF
lab/typst-original/target/release/typst compile /tmp/p727-curve-minimo.typ /tmp/p727-vanilla.pdf
mutool draw -o /tmp/p727-vanilla.png -r 150 /tmp/p727-vanilla.pdf
./target/release/typst /tmp/p727-curve-minimo.typ /tmp/p727-cristalino.pdf
mutool draw -o /tmp/p727-cristalino.png -r 150 /tmp/p727-cristalino.pdf
```

Confirmar se mesmo o caso mínimo (só `move` + `line`, sem `cubic`/`close`) já falha — isola se o problema está na construção do `CurveElem`, na conversão para `FrameItem::Shape`, ou no render do PDF em si.

### Localizar exactamente onde a informação se perde

```bash
grep -n "CurveElem\|FrameItem::Shape\|Path(" 01_core/src/rules/layout/curve.rs 01_core/src/entities/content.rs | head -30
```

Seguir o caminho completo: `curve.move/.line/.cubic/.close` → segmentos internos → `CurveElem` → `FrameItem::Shape { kind: Path(...) }` → export PDF. Confirmar em qual destes passos os pontos desaparecem (array vazio, coordenadas erradas, ou o `FrameItem` nunca chega ao export).

### Confirmar se o PDF tem o objecto de path, mas invisível (cor/opacidade), ou se não tem o objecto de todo

```bash
python3 -c "
import re
data = open('/tmp/p727-cristalino.pdf', 'rb').read()
# procurar operadores de path no content stream (m, l, c, etc.)
print(b'/Filter' in data, len(data))
"
qpdf --qdf --object-streams=disable /tmp/p727-cristalino.pdf /tmp/p727-cristalino-qdf.pdf 2>/dev/null || echo "qpdf indisponível, inspeccionar manualmente"
```

Confirmar se o PDF gerado tem algum conteúdo de desenho (operadores `m`/`l`/`c` no content stream) que simplesmente não é visível (cor branca, opacidade zero, fora da página), ou se não tem nada — isto muda completamente onde procurar a causa.

### Critério de fecho da sonda

- [ ] Caso mínimo confirmado (falha mesmo com só `move`+`line`, ou só com formas mais complexas).
- [ ] Ponto exacto onde a informação se perde, localizado com `file:line`.
- [ ] Confirmado se o PDF tem conteúdo invisível ou nenhum conteúdo.

---

## Implementação

Corrigir o ponto exacto identificado pela sonda.

### Critério de fecho da implementação

- [ ] `curve` renderiza visualmente, testado com o caso mínimo e com o documento completo de `cetz`.
- [ ] `rect`/`circle` (já correctos) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p727-curve-minimo.typ /tmp/p727-depois.pdf
mutool draw -o /tmp/p727-depois.png -r 150 /tmp/p727-depois.pdf
```

Diff de pixels contra `/tmp/p727-vanilla.png`.

```bash
cargo test --workspace
crystalline-lint .
```

### Reprodução final de `cetz` — diff de pixels

```bash
cat > /tmp/p727-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p727-cetz.typ /tmp/p727-cetz.pdf
mutool draw -o /tmp/p727-cetz.png -r 150 /tmp/p727-cetz.pdf
lab/typst-original/target/release/typst compile /tmp/p727-cetz.typ /tmp/p727-cetz-vanilla.pdf
mutool draw -o /tmp/p727-cetz-vanilla.png -r 150 /tmp/p727-cetz-vanilla.pdf
```

Diff de pixels entre os dois PNG — se for zero (ou muito próximo, dentro de uma margem de anti-aliasing), a cadeia P678-727 fecha de vez, com prova visual, não só `exit 0`.

---

## Critério de fecho do passo

- [ ] Sonda completa, causa exacta localizada.
- [ ] Corrigido, testado com diff de pixels no caso mínimo.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — diff de pixels final registado, com número exacto, não "parece igual".
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p727.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
- [ ] Se este for o fecho real da cadeia P678-727: declarar isso explicitamente no relatório, com o número total de passos e o resumo do que foi corrigido ao longo do caminho.
