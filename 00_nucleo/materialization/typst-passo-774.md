---
# P774 — Rotação EXIF de imagens

> **Passo:** 774
> **Data:** 2026-07-16
> **Foco:** P772 classificou `apply_rotation`/`exif_rotation` como infra-estrutura Rust sem efeito de língua; P773 já tinha sinalizado que isso merecia verificação separada, dado que uma imagem com tag `Orientation` do EXIF pode renderizar de lado ou invertida se o cristalino ignorar essa informação — efeito claramente observável no documento final, o mesmo critério que tirou o DPI da categoria "sem efeito" em P773. Este passo confirma com um caso de teste real (imagem com EXIF `Orientation` não-padrão) e corrige se confirmado.
> **Tipo:** Sonda + Implementação condicional.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar por medição directa antes de implementar.
> **ADR-0107** — orientação de imagem é comportamento observável do documento, não detalhe de implementação.
> **Dependências:** P773 (parsing EXIF já implementado para DPI — reutilizar a mesma infra-estrutura de leitura de segmento APP1/chunk `eXIf`, não duplicar).

---

## Sonda — confirmar a divergência com uma imagem rodada por EXIF

### Gerar uma imagem de teste com `Orientation` não-padrão

```bash
convert -size 200x100 xc:lightblue -fill red -draw "rectangle 0,0 40,40" /tmp/p774-base.png
convert /tmp/p774-base.png -orient TopRight /tmp/p774-orient6.jpg
exiftool -Orientation="Rotate 90 CW" -n /tmp/p774-orient6.jpg 2>/dev/null || \
  exiftool -Orientation=6 /tmp/p774-orient6.jpg
```

Confirmar a tag gravada:

```bash
exiftool -Orientation /tmp/p774-orient6.jpg
```

Se `exiftool` não estiver disponível, usar `convert -auto-orient` ao contrário, ou escrever o byte da tag `0x0112` directamente no segmento EXIF via script Python (`piexif` ou manipulação manual dos bytes, replicando o formato já lido por P773).

### Medir a orientação renderizada

```bash
cat > /tmp/p774-image-orient.typ <<'EOF'
#image("/tmp/p774-orient6.jpg")
EOF
lab/typst-original/target/release/typst compile /tmp/p774-image-orient.typ /tmp/p774-vanilla.pdf
./target/release/typst compile /tmp/p774-image-orient.typ /tmp/p774-cristalino.pdf
mutool draw -o /tmp/p774-vanilla.png -r 150 /tmp/p774-vanilla.pdf
mutool draw -o /tmp/p774-cristalino.png -r 150 /tmp/p774-cristalino.pdf
```

Inspeccionar visualmente os dois PNGs: o rectângulo vermelho de referência (desenhado no canto para servir de âncora de orientação) aparece no mesmo canto nos dois, ou rodado?

```bash
compare -metric AE /tmp/p774-vanilla.png /tmp/p774-cristalino.png /tmp/p774-diff.png
```

Se AE alto e a inspecção confirmar rotação/espelhamento diferente: divergência confirmada, prosseguir para implementação. Se AE baixo/zero: o cristalino já trata isto correctamente por algum outro caminho (ex: crate `image` já aplica auto-orientação na descodificação) — registar isso como achado, sem implementar nada.

---

## Implementação (condicional à confirmação)

### Confirmar o mapeamento de valores EXIF do vanilla

```bash
grep -n "fn exif_rotation\|Orientation\|match.*orientation" lab/typst-original/crates/typst-library/src/visualize/image/raster.rs 2>/dev/null
```

As 8 orientações EXIF padrão (1-8) mapeiam para combinações de rotação (0°/90°/180°/270°) e espelhamento (flip horizontal/vertical). Confirmar exactamente essa tabela no vanilla, não assumir da especificação EXIF genérica — usar o mapeamento real do código.

### Aplicar no cristalino

Reutilizar o parsing de segmento EXIF já implementado em P773 (`03_infra/src/image_sizer.rs`, leitura de APP1/`eXIf`) para também extrair a tag `Orientation` (`0x0112`). Aplicar a transformação correspondente (rotação/flip) aos dados de pixel antes de embeber no PDF, ou como transformação na matriz `cm` do PDF, conforme o vanilla fizer — confirmar qual dos dois caminhos o vanilla usa antes de escolher.

```bash
grep -n "fn apply_rotation" lab/typst-original/crates/typst-library/src/visualize/image/raster.rs 2>/dev/null
```

---

## Validação

```bash
mutool draw -o /tmp/p774-final-vanilla.png -r 300 /tmp/p774-vanilla.pdf
mutool draw -o /tmp/p774-final-cristalino.png -r 300 /tmp/p774-cristalino.pdf
compare -metric AE /tmp/p774-final-vanilla.png /tmp/p774-final-cristalino.png -highlight-color red -lowlight-color none /tmp/p774-final-diff.png
```

Testar as 8 orientações EXIF padrão, não só a usada na sonda — o mapeamento tem casos com e sem espelhamento, que são categorias de bug diferentes (só rotação vs rotação+flip).

Confirmar que imagens sem tag `Orientation` (o caso comum, incluindo os testes de P773) não regridem.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Divergência confirmada com imagem de teste real e inspecção visual, não só suposição.
- [ ] Se confirmado: mapeamento das 8 orientações EXIF confirmado contra o código do vanilla.
- [ ] Rotação/flip aplicados correctamente para pelo menos as orientações com uso prático mais comum (3, 6, 8 — rotações simples sem flip) e idealmente as 8.
- [ ] Imagens sem `Orientation` continuam sem regressão (P773 preservado).
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de `image` actualizado com a lógica de rotação, antes do código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p774.md`.

---

## Próximo passo

Com DPI (P773) e rotação (este passo, se confirmado) resolvidos, os 13 itens de `image::raster` levantados por P772 ficam totalmente reclassificados: 2 bugs reais corrigidos (DPI, rotação), 11 confirmados como infra-estrutura sem efeito observável. Retomar a varredura da stdlib em P772a (lote 3).
