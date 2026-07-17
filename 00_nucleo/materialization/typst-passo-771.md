---
# P771 — Implementação: `clip_path` para imagens com `fit`

> **Passo:** 771
> **Data:** 2026-07-16
> **Foco:** P770 corrigiu a escala píxel→ponto (72 DPI) e aplicou `ImageFit::Cover` às dimensões da transformação da imagem, mas deixou scope-out o recorte (`clip_path`) que o vanilla emite quando a imagem transformada excede o rectângulo alvo (`width`×`height` pedidos). Sem isso, `A #image(..., width: 2cm, height: 1.5cm) B` diverge 2,834pt no alinhamento de "B", porque o cristalino desenha a imagem inteira pós-`fit` (maior que o target) e avança o cursor a partir dessas dimensões maiores, em vez de avançar a partir do rectângulo de clip (do tamanho pedido).
> **Tipo:** Implementação directa. Causa já confirmada por P770 com leitura do PDF do vanilla.
> **Tamanho:** M — toca `FrameItem::Image`, o exportador PDF, e o cálculo de avanço de cursor em `image.rs`.
> **ADR-0108 EM VIGOR** — validar por coordenadas, não só por AE.
> **Dependências:** P770 (causa confirmada, `PX_TO_PT`/`fit` já corrigidos).

---

## Sonda — confirmar a estrutura exacta do `clip_path` no vanilla

```bash
mutool trace /tmp/p770-vanilla-com-dimensoes.pdf 2>/dev/null | grep -B2 -A10 "clip\|W n"
```

Confirmar:
1. O operador PDF exacto usado para recorte (`W n` antes do `Do` da imagem, com um `re` definindo o rectângulo).
2. Se o rectângulo de clip é sempre `width × height` pedidos (o target), independentemente do `fit` escolhido, ou se varia por modo (`Cover`/`Contain`/`Stretch`).

```bash
grep -n "ImageFit::\|clip\|fn layout_image" lab/typst-original/crates/typst-layout/src/image.rs 2>/dev/null | head -30
```

Confirmar como o vanilla trata os três modos de `fit` (`Cover`, `Contain`, `Stretch`) em relação ao clip — `Stretch` provavelmente não precisa de clip (a imagem já cabe exactamente no target); `Contain` pode não exceder o target (logo também sem clip real); só `Cover` excede e precisa de clip nos dois eixos ou só num, conforme a proporção.

---

## Implementação

### 1. `FrameItem::Image` — adicionar informação de clip

```bash
grep -n "enum FrameItem\|Image {" 01_core/src/entities/*.rs
```

Adicionar ao variant `Image` (ou estrutura equivalente) um campo para o rectângulo de clip (`clip_rect: Option<Rect>` ou equivalente), preenchido só quando `fit` exigir recorte (conforme confirmado pela sonda).

### 2. `01_core/src/engine/layout/image.rs` — avanço de cursor a partir do target, não da transformação

O avanço do cursor (e o cálculo de `image_base`/altura usado no ancoramento de P769) deve usar as dimensões do **rectângulo de clip/target** (`width`×`height` pedidos), não as dimensões da imagem pós-`fit` (que podem ser maiores). Confirmar que isto não desfaz a correcção de P769 — o ancoramento vertical continua igual, só a altura usada no cálculo muda de "altura da transformação" para "altura do target".

### 3. Exportador PDF — emitir o clip

```bash
grep -n "fn export_image\|FrameItem::Image" 03_infra/src/export.rs 2>/dev/null
```

Emitir `W n` com o rectângulo de clip antes do `Do` da imagem, replicando a estrutura confirmada pela sonda, só quando `clip_rect` estiver presente.

---

## Validação

### Caso principal de P770 (a divergência de 2,834pt)

```bash
cat > /tmp/p771-image-dims.typ <<'EOF'
A #image("tiny.png", width: 2cm, height: 1.5cm) B
EOF
lab/typst-original/target/release/typst compile /tmp/p771-image-dims.typ /tmp/p771-vanilla.pdf
./target/release/typst compile /tmp/p771-image-dims.typ /tmp/p771-cristalino.pdf
mutool trace /tmp/p771-vanilla.pdf > /tmp/p771-trace-vanilla.txt
mutool trace /tmp/p771-cristalino.pdf > /tmp/p771-trace-cristalino.txt
```

Reconstruir a tabela de P770 (A, image base, image topo, B) e confirmar ΔY ≈ 0 em todos os pontos, incluindo B (que estava em 2,834pt de divergência).

```bash
mutool draw -o /tmp/p771-vanilla.png -r 300 /tmp/p771-vanilla.pdf
mutool draw -o /tmp/p771-cristalino.png -r 300 /tmp/p771-cristalino.pdf
compare -metric AE /tmp/p771-vanilla.png /tmp/p771-cristalino.png -highlight-color red -lowlight-color none /tmp/p771-diffmap.png
```

Inspeccionar visualmente: confirmar que a imagem aparece recortada ao target no cristalino, do mesmo jeito que no vanilla — não só que o número de B bate.

### Regressão nos outros dois casos de P770

```bash
# Sem dimensões (deve continuar igual)
cat > /tmp/p771-sem-dim.typ <<'EOF'
A #image("tiny.png") B
EOF
# Com dimensões que forcem Contain/Stretch, se existirem parâmetros para isso
```

Confirmar que os casos que já batiam (sem dimensões, `width: 2cm, height: 2cm` de P770) continuam a bater depois desta mudança.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Estrutura exacta do `clip_path` do vanilla confirmada por leitura do PDF (operador, posição no content stream).
- [ ] Comportamento por modo de `fit` (`Cover`/`Contain`/`Stretch`) confirmado — clip só onde necessário.
- [ ] `FrameItem::Image` com informação de clip.
- [ ] Avanço de cursor usa dimensões do target, não da transformação pós-`fit`.
- [ ] Exportador PDF emite o clip correspondente.
- [ ] Caso `A #image(width:2cm,height:1.5cm) B`: ΔY ≈ 0 em todos os pontos, incluindo B.
- [ ] Inspecção visual confirma recorte real, não só o número do cursor.
- [ ] Casos sem regressão (sem dimensões, outras combinações de P770) revalidados.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de `image` actualizado com a especificação de clip, antes do código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p771.md`.

---

## Próximo passo

Se tudo fechar: a cadeia completa de investigação de layout (P763-P771, iniciada pela pergunta sobre download de pacotes) fica sem itens conhecidos em aberto. Vale um resumo final da cadeia toda, dado o volume de achados.
