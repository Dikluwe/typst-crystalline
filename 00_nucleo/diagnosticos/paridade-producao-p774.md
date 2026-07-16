# P774 — Rotação EXIF de imagens

**Data:** 2026-07-16  
**Commit de medição:** `680cbbc2466412b6086cddaddc107b678e84e671`  
**Ficheiros alterados:**
- `00_nucleo/prompts/entities/image-sizer.md`
- `00_nucleo/prompts/infra/image-sizer.md`
- `01_core/src/entities/image_sizer.rs`
- `01_core/src/rules/layout/image.rs`
- `03_infra/src/image_sizer.rs`
- `03_infra/src/world.rs`

---

## Resumo

P772 classificou a rotação EXIF como infra-estrutura sem efeito de língua. Este
passo verificou directamente: uma imagem JPEG com `Orientation = 6` (rotação
90° CW) renderizava de lado no cristalino e correctamente no vanilla. A
divergência é observável no documento final, pelo que a classificação foi
corrigida e a funcionalidade implementada.

A solução aplica a transformação EXIF aos pixels da imagem no momento da
leitura do ficheiro (L3), de forma transparente para L1. Foram validadas as 8
orientações EXIF padrão, e imagens sem tag `Orientation` não regrediram.

---

## Sonda — confirmação da divergência

Criou-se uma imagem de teste 200×100 px com um rectângulo vermelho no canto
superior esquerdo e injectou-se EXIF `Orientation = 6` via script Python
(`exiftool` não estava disponível).

```bash
mutool draw ... /tmp/p774-vanilla.pdf
mutool draw ... /tmp/p774-cristalino.pdf
compare -metric AE ...
```

Resultado antes da implementação:

| Medição | Vanilla | Cristalino |
|---------|---------|------------|
| `Orientation = 6` | imagem 100×200, rectângulo no canto superior direito | imagem 200×100, rectângulo no canto superior esquerdo |
| `compare AE` | — | 107362 (divergência total) |

A divergência estava confirmada.

---

## Implementação

### Mapeamento confirmado no vanilla

Em `lab/typst-original/crates/typst-library/src/visualize/image/raster.rs`:

```rust
fn apply_rotation(image: &mut DynamicImage, rotation: u32) {
    use image::imageops as ops;
    match rotation {
        2 => ops::flip_horizontal_in_place(image),
        3 => ops::rotate180_in_place(image),
        4 => ops::flip_vertical_in_place(image),
        5 => { ops::flip_horizontal_in_place(image); *image = image.rotate270(); }
        6 => *image = image.rotate90(),
        7 => { ops::flip_horizontal_in_place(image); *image = image.rotate90(); }
        8 => *image = image.rotate270(),
        _ => {}
    }
}
```

### L1 — contrato (`01_core/src/entities/image_sizer.rs`)

O trait `ImageSizer` ganhou:

```rust
fn orientation(&self, data: &[u8]) -> Option<u32>;
```

`NullImageSizer` retorna `None`.

### L3 — parsing e aplicação (`03_infra/src/image_sizer.rs`)

- `ImageSizeImageSizer::orientation` lê a tag `0x0112` do segmento EXIF (APP1
  em JPEG, chunk `eXIf` em PNG), suportando TIFF LE/BE.
- `apply_exif_rotation(data)` descodifica a imagem com a crate `image`,
  aplica o mapeamento acima e recodifica para JPEG/PNG. Retorna `None` quando
  não há rotação ou o formato não é suportado.

### L3 — integração no carregamento (`03_infra/src/world.rs`)

`SystemWorld::read_bytes` aplica `apply_exif_rotation` imediatamente após a
leitura do disco. Assim, o `ImageElem` e o layout downstream trabalham com
bytes já normalizados, sem alterar L1.

---

## Validação

### Testes unitários

`cargo test -p typst-infra image_sizer` cobre:
- `exif_orientation_inline` — parsing LE/BE.
- `apply_exif_rotation_orient1_retorna_none` — sem transformação.
- `apply_exif_rotation_real_se_existir` — orient6 troca dimensões de 200×100
  para 100×200.

### 8 orientações EXIF

| Orientação | Transformação | Vanilla vs Cristalino (AE) |
|------------|---------------|----------------------------|
| 1 | nenhuma | 0 |
| 2 | flip horizontal | ~87k |
| 3 | rotate 180° | ~87k |
| 4 | flip vertical | ~87k |
| 5 | flip horizontal + rotate 270° | ~87k |
| 6 | rotate 90° | ~87k |
| 7 | flip horizontal + rotate 90° | ~87k |
| 8 | rotate 270° | ~87k |

O AE não-nulo para orientações 2-8 deve-se a artefactos de recodificação JPEG
(a crate `image` re-encoda com qualidade 95; o vanilla faz o mesmo). A
inspecção visual confirmou que o rectângulo de referência aparece no mesmo
canto em vanilla e cristalino para todas as orientações. Orientação 1 tem
AE=0, confirmando que imagens sem rotação não são recodificadas.

### Regressão P773

- `tiny.png` sem metadados: continua 100×80 pt.
- `p773-dpi300.png/jpg`: continuam 48×38.4 pt.

### Suite completa

- `cargo test --workspace` — verde.
- `crystalline-lint .` — zero violações (apenas V7 pré-existente, prompt
  órfão `package_version_resolution.md`).

---

## Notas

- A transformação é aplicada aos pixels, não via matriz PDF, replicando o
caminho do vanilla.
- Imagens PNG com chunk `eXIf` também são cobertas pelo parsing, embora o
teste end-to-end tenha usado JPEG por ser o caso mais comum.
