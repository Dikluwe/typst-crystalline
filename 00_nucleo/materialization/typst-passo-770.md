---
# P770 — Investigação: `image(height:...)` explícito diverge no vanilla + correcção da escala `PX_TO_PT`

> **Passo:** 770
> **Data:** 2026-07-16
> **Foco:** P769 fechou o caso `A #image("tiny.png", width: 2cm, height: 1.5cm) B` como corrigido porque "B" alinhou (ΔY=0,000), mas a própria medição mostra que o vanilla renderizou a imagem com 45,354pt de altura apesar de `height: 1.5cm` (42,52pt) ter sido pedido explicitamente — o cristalino, pelo contrário, obedeceu ao valor pedido. O espaço entre o topo da imagem e a baseline de "B" é 19,021pt no vanilla mas 20,438pt no cristalino — diferente. É matematicamente possível que esses dois erros (altura da imagem + espaço pós-imagem) se cancelem por coincidência nesse caso específico, produzindo ΔY=0 em B sem o ancoramento estar de facto correcto. P769 não investigou a causa da divergência de altura, só a atribuiu a "escala ligeiramente diferente" sem explicar. Este passo investiga essa causa e resolve, separadamente, o problema já identificado de escala em imagens sem dimensões explícitas (100×80 vanilla vs 75×60 cristalino, `PX_TO_PT`/DPI).
> **Tipo:** Investigação + Implementação (duas causas distintas, tratadas separadamente).
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — não aceitar "B alinhou" como prova de ancoramento correcto sem explicar por que a altura da imagem também diverge; duas divergências que se cancelam não é a mesma coisa que nenhuma divergência.
> **Dependências:** P769 (medições originais, achado de escala em backlog).

---

## Parte A — Por que o vanilla não respeita `height: 1.5cm` explícito?

### Sonda

```bash
cat > /tmp/p770-image-altura.typ <<'EOF'
#image("tiny.png", width: 2cm, height: 1.5cm)
EOF
lab/typst-original/target/release/typst compile /tmp/p770-image-altura.typ /tmp/p770-altura-vanilla.pdf
mutool trace /tmp/p770-altura-vanilla.pdf | grep -A5 "Image\|Do\b"
```

Confirmar, por leitura directa do PDF (dimensões do XObject e da matriz de transformação aplicada), qual é a altura real desenhada e se `height: 1.5cm` foi de facto ignorado, ou se há um efeito de `fit`/aspect-ratio a interferir:

```bash
grep -n "fn layout_image\|fit\|aspect" lab/typst-original/crates/typst-library/src/visualize/image.rs 2>/dev/null | head -30
```

Hipóteses a confirmar (não assumir):
1. `image()` tem um parâmetro `fit` (`cover`/`contain`/`stretch`) com um valor por defeito que preserva aspect-ratio mesmo quando `width`/`height` são ambos dados — se a imagem original não tiver a proporção 2cm:1.5cm, o vanilla pode estar a ajustar um dos dois para preservar proporção.
2. A imagem de teste `tiny.png` pode ter metadados de DPI que interagem com `width`/`height` de forma que `1.5cm` não seja literal.
3. Erro de leitura da medição em P769 (confirmar reproduzindo do zero, não confiando no número já registado).

### Confirmar se o "B alinhado" de P769 é coincidência

Com a causa da Parte A identificada, recalcular manualmente: se a altura real da imagem no vanilla é 45,354pt (não 42,52pt) por causa de `fit`, o cristalino deveria replicar esse mesmo `fit`, não usar a altura literal pedida. Se for esse o caso, a correcção certa não é "nada, B já bate" — é fazer o cristalino respeitar o mesmo `fit` que o vanilla usa, e só então confirmar se B continua a bater (ou se passa a bater por razão correcta, não por coincidência).

```bash
cat > /tmp/p770-image-altura-proporcional.typ <<'EOF'
#image("tiny.png", width: 2cm, height: 2cm)
EOF
```

Testar com proporções que forcem o `fit` (se existir) a divergir claramente do literal, tornando o efeito mais fácil de confirmar isoladamente.

---

## Parte B — Corrigir a escala `PX_TO_PT` (imagens sem dimensões explícitas)

### Sonda

```bash
grep -n "PX_TO_PT\|0\.75\|DPI\|96\.0\|72\.0" 01_core/src/engine/layout/image.rs 03_infra/src/*.rs 2>/dev/null
```

Confirmar a constante actual usada pelo cristalino (P769 registou `PX_TO_PT = 0.75`, correspondente a 96 DPI → 72pt/polegada). Confirmar o valor real usado pelo vanilla:

```bash
grep -n "px_to_pt\|Ratio::new\|96\.0\|72\.0\|Image::.*size\|natural_size" lab/typst-original/crates/typst-library/src/visualize/image.rs lab/typst-original/crates/typst-library/src/foundations/*.rs 2>/dev/null | head -20
```

O vanilla renderizou 100×80pt para uma imagem sem dimensões explícitas; o cristalino renderizou 75×60pt — razão exacta de 100/75 = 80/60 = 1,333... = 4/3. Confirmar se o vanilla usa `pixels × 1.0` (assumindo 72 DPI nativo) em vez de `pixels × 0.75` (assumindo 96 DPI), ou alguma outra convenção — não assumir qual dos dois está "certo" sem ler o código.

### Implementação

Corrigir a constante/fórmula de conversão píxel→pt em `01_core/src/engine/layout/image.rs` (ou onde estiver centralizada), conforme o valor confirmado do vanilla.

### Validação

```bash
cat > /tmp/p770-image-sem-dim.typ <<'EOF'
#image("tiny.png")
EOF
lab/typst-original/target/release/typst compile /tmp/p770-image-sem-dim.typ /tmp/p770-semdim-vanilla.pdf
./target/release/typst compile /tmp/p770-image-sem-dim.typ /tmp/p770-semdim-cristalino.pdf
mutool trace /tmp/p770-semdim-vanilla.pdf > /tmp/p770-semdim-trace-vanilla.txt
mutool trace /tmp/p770-semdim-cristalino.pdf > /tmp/p770-semdim-trace-cristalino.txt
```

Confirmar dimensões idênticas (100×80pt nos dois), e repetir o caso `A #image(...) B` sem dimensões para confirmar que "B" continua alinhado depois da correcção de escala.

```bash
mutool draw -o /tmp/p770-semdim-vanilla.png -r 300 /tmp/p770-semdim-vanilla.pdf
mutool draw -o /tmp/p770-semdim-cristalino.png -r 300 /tmp/p770-semdim-cristalino.pdf
compare -metric AE /tmp/p770-semdim-vanilla.png /tmp/p770-semdim-cristalino.png /tmp/p770-semdim-diff.png
```

---

## Critério de fecho do passo

- [ ] Causa da divergência de altura com `height:` explícito identificada por leitura de código (fit/aspect-ratio, metadado da imagem, ou erro de medição).
- [ ] Confirmado se o "B alinhado" de P769 (caso 2.1) era coincidência de dois erros a cancelar-se, ou resultado correcto por razão válida.
- [ ] Se coincidência: corrigido para que o cristalino replique o mesmo comportamento de `fit` do vanilla, e recalculada a medição de B pela razão certa.
- [ ] Constante de conversão píxel→pt confirmada contra o código do vanilla (não assumida).
- [ ] `PX_TO_PT` corrigido; dimensões de imagem sem `width`/`height` explícitos batem com o vanilla (100×80pt no caso de teste).
- [ ] Caso `A #image(...) B` sem dimensões revalidado depois da correcção de escala.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de `image` actualizado com a fórmula de escala correcta, antes do código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p770.md`, com a causa da Parte A explicada por evidência, não suposição.

---

## Próximo passo

Se ambas as partes fecharem: revisitar rapidamente outros casos de `image()` com dimensões parciais (só `width`, só `height`, nenhuma) para confirmar que a correcção de escala generaliza, não só para o caso sem nenhuma dimensão.
