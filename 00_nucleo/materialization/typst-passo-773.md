---
# P773 — Leitura de DPI real de metadados de imagem (EXIF/JFIF/PNG)

> **Passo:** 773
> **Data:** 2026-07-16
> **Foco:** P772 classificou `determine_dpi`/`exif_dpi`/`jpeg_dpi`/`png_dpi` como "infra-estrutura Rust sem símbolo de língua", mas essa classificação ignora que P770 fixou `PX_TO_PT = 1.0` (72 DPI) como constante única para todas as imagens sem dimensões explícitas. O vanilla só usa 72 DPI como fallback quando a imagem não tem metadados de DPI — quando tem (EXIF, JFIF APP0, PNG `pHYs`), usa o valor real. O próprio caso de teste de P770 (`tiny.png`) só validou o cenário sem metadados, por não ter DPI embutido — não prova que a correcção generaliza. Qualquer imagem real com DPI não-padrão vai renderizar com dimensões erradas no cristalino. Este passo confirma isso com um caso de teste real e corrige se confirmado.
> **Tipo:** Sonda + Implementação condicional.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar por medição directa, não assumir que P770 generaliza.
> **ADR-0107** — DPI afecta o tamanho renderizado observável de `image()`; não é detalhe de implementação sem efeito, ao contrário dos outros 12 itens de P772.
> **Dependências:** P772 (achado, classificação a corrigir parcialmente), P770 (`PX_TO_PT` fixo, ponto a estender).

---

## Sonda — confirmar a divergência com uma imagem de DPI não-padrão

### Gerar uma imagem de teste com DPI embutido

```bash
python3 -c "
from PIL import Image
img = Image.new('RGB', (200, 160), color='red')
img.save('/tmp/p773-dpi300.png', dpi=(300, 300))
img.save('/tmp/p773-dpi300.jpg', dpi=(300, 300))
"
```

Se `PIL`/`Pillow` não estiver disponível, usar `exiftool`/`convert` (ImageMagick) para gravar DPI num PNG/JPEG existente:

```bash
convert -size 200x160 xc:red -density 300 /tmp/p773-dpi300.png
convert -size 200x160 xc:red -density 300 /tmp/p773-dpi300.jpg
```

### Medir a dimensão renderizada sem `width`/`height` explícitos

```bash
cat > /tmp/p773-image-dpi.typ <<'EOF'
#image("/tmp/p773-dpi300.png")
EOF
lab/typst-original/target/release/typst compile /tmp/p773-image-dpi.typ /tmp/p773-dpi-vanilla.pdf
./target/release/typst compile /tmp/p773-image-dpi.typ /tmp/p773-dpi-cristalino.pdf
mutool trace /tmp/p773-dpi-vanilla.pdf | grep -A3 "cm /"
mutool trace /tmp/p773-dpi-cristalino.pdf | grep -A3 "cm /"
```

Confirmar as dimensões da transformação em pt nos dois. Esperado, se a hipótese estiver certa: vanilla usa 200px/300dpi×72 = 48pt (aprox.), cristalino usa 200px×1.0 = 200pt (por `PX_TO_PT=1.0` fixo) — divergência grande, proporcional à razão entre 300 DPI e 72 DPI (≈4,17×).

Repetir para JPEG (JFIF APP0) e, se possível, para um PNG com chunk `pHYs` mas sem JFIF, para confirmar que ambos os caminhos de leitura de metadados faltam, não só um.

---

## Implementação (condicional à confirmação)

### Localizar onde o cristalino descodifica e onde deveria ler DPI

```bash
grep -n "fn.*decode\|magic\|PNG\|JPEG" 03_infra/src/export/images.rs
```

Replicar a lógica do vanilla (`typst_library::visualize::image::raster`, achada por P772):

1. `png_dpi`: ler o chunk `pHYs` do PNG (unidades por metro → converter para DPI).
2. `jpeg_dpi`: ler o segmento JFIF APP0 (densidade + unidade).
3. `exif_dpi`: ler tags EXIF de resolução, se presentes (pode coexistir com JFIF; confirmar prioridade no vanilla — qual metadado ganha se ambos existirem).
4. Fallback: 72 DPI quando nenhum metadado presente (comportamento actual, preservado).

Adicionar campo de DPI descodificado à estrutura interna de imagem (equivalente a `RasterImage` do vanilla), e usar esse valor em vez da constante fixa `PX_TO_PT` sempre que disponível.

### Não esquecer rotação EXIF (mesmo módulo, mas separado)

`apply_rotation`/`exif_rotation` são um achado relacionado mas distinto — confirmar neste passo se também têm efeito observável (uma imagem rodada 90° por EXIF renderiza na orientação errada se o cristalino ignorar a tag). Se confirmado, é outro item a mover de "infra-estrutura sem efeito" para "bug real" — medir separadamente, não misturar a correcção de DPI com a de rotação no mesmo commit se ambas forem necessárias.

---

## Validação

```bash
mutool draw -o /tmp/p773-dpi-vanilla.png -r 300 /tmp/p773-dpi-vanilla.pdf
mutool draw -o /tmp/p773-dpi-cristalino.png -r 300 /tmp/p773-dpi-cristalino.pdf
compare -metric AE /tmp/p773-dpi-vanilla.png /tmp/p773-dpi-cristalino.png /tmp/p773-dpi-diff.png
```

Confirmar dimensões idênticas para PNG e JPEG com DPI embutido, e que o caso de P770 (sem metadados, fallback 72 DPI) continua a bater — não regredir o que já estava corrigido.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Divergência de DPI confirmada com imagem de teste real (PNG e JPEG com DPI não-padrão).
- [ ] Se confirmado: leitura de `pHYs` (PNG), JFIF APP0 (JPEG) e, se aplicável, EXIF implementada.
- [ ] Prioridade entre metadados conflitantes (EXIF vs JFIF) confirmada contra o vanilla, não assumida.
- [ ] Fallback de 72 DPI preservado para imagens sem metadados (caso de P770 sem regressão).
- [ ] Rotação EXIF avaliada separadamente — corrigida se tiver efeito observável confirmado, ou registada como achado à parte.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de `image` actualizado com a lógica de DPI, antes do código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p773.md`, com as dimensões antes/depois para PNG e JPEG com DPI embutido.

---

## Próximo passo

Se rotação EXIF também divergir: passo dedicado, mesma disciplina.
Retomar a varredura de `lacuna-inventario` (P772a, lote 3) com os 9 itens restantes de `image::raster` reclassificados: DPI (e rotação, se aplicável) como bug real corrigido aqui; os outros permanecem infra-estrutura sem efeito observável, conforme P772 já tinha concluído para eles.
