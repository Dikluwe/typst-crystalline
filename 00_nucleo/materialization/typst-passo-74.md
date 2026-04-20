# Passo 74 — PNG com /FlateDecode, /SMask e ColorSpace JPEG (DEBT-27, DEBT-29, DEBT-28, DEBT-26)

## Estado actual antes de começar

Ler antes de começar:
- `03_infra/src/export.rs` — Onde `export_pdf`, `detect_format`, e o mapa de
  deduplicação por `Arc::as_ptr` vivem (Passo 73).
- `03_infra/Cargo.toml` — Onde `image` e `flate2` vão ser injectados.
- `01_core/src/rules/layout/frame.rs` — Confirmar os cinco campos de
  `FrameItem::Image` introduzidos no Passo 73.
- `01_core/DEBT.md` — Confirmar que DEBT-27, DEBT-28, DEBT-29 estão registados
  e DEBT-24c está encerrado.

Pré-condição: `cargo test` — ~706 L1 + ~126 L3, zero violations.
DEBT-24c encerrado. DEBT-27 (PNG), DEBT-28 (dupla leitura), DEBT-29
(ColorSpace JPEG) registados. `FrameItem::Image` com cinco campos existe.

---

## Contexto

O Passo 73 estabeleceu a prova de conceito de imagens no PDF: JPEG embutido
com `/DCTDecode` e deduplicação por `Arc::as_ptr`. PNG era rejeitado
silenciosamente com um `eprintln!` a apontar DEBT-27.

Este passo fecha os três débitos de imagem que ficaram em aberto:

- **DEBT-27 — PNG:** O PDF não aceita ficheiros PNG crus. Os píxeis têm de ser
  descodificados, o canal RGB comprimido com Zlib (`/FlateDecode`), e o canal
  Alpha (se existir) escrito como um XObject separado de `DeviceGray` ligado
  ao principal pela entrada `/SMask`.
- **DEBT-29 — ColorSpace JPEG:** O exportador assume `DeviceRGB` para todos os
  JPEGs. JPEGs com 1 canal (Grayscale) ou 4 canais (CMYK) produzem lixo visual
  ou são recusados pelos leitores PDF. A resolução lê o marcador `SOF0`/`SOF2`
  do cabeçalho JPEG para escolher `DeviceGray`, `DeviceRGB`, ou `DeviceCMYK`.
- **DEBT-28 — Dupla leitura:** `calculate_dimensions` e `sizer.size()` lêem
  o cabeçalho da imagem separadamente. Resolver passando `intrinsic_width` e
  `intrinsic_height` de volta em `ImageDimensions` e eliminando a segunda
  chamada.
- **DEBT-26 — `PtrEqArc`:** `Arc<Vec<u8>>` comparado com `==` desreferencia e
  compara byte a byte (O(N)). Implementar `PtrEqArc<T>` com `Arc::ptr_eq`
  para comparação O(1) em `Content::Image`.

Todos os quatro débitos fecham neste passo. DEBT-25, DEBT-14, DEBT-15 ficam
para o Passo 75.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar que NADA de image/flate2 escapou para L1
grep -rn "image\|flate2" 01_core/Cargo.toml || echo "L1 Limpo"

# 2. Confirmar o padrão de serialização manual e as variáveis do exportador
# (contador de objetos, buffer de saída, tabela xref — necessários na Tarefa 6)
grep -n "obj_counter\|xref_positions\|out\.extend\|endobj\|detect_format\|image_resources\|Arc::as_ptr" \
  03_infra/src/export.rs | head -20

# 3. Confirmar os campos actuais de FrameItem::Image
grep -A 8 "Image {" 01_core/src/rules/layout/frame.rs | head -15

# 4. Confirmar a assinatura actual de ImageDimensions (DEBT-28)
grep -n "struct ImageDimensions\|width_pt\|height_pt\|intrinsic" \
  01_core/src/rules/layout/image.rs | head -10

# 5. Confirmar onde Content::Image usa PartialEq (DEBT-26)
grep -n "PartialEq\|Arc<Vec" 01_core/src/entities/content.rs | head -10
```

Reportar o output completo antes de continuar. O diagnóstico 3 confirma que
`intrinsic_width`/`intrinsic_height` já existem em `FrameItem::Image` — se
não existirem, DEBT-24c não está encerrado e o Passo 73 tem de ser revisto
antes de avançar.

---

## Tarefa 0 — Actualizar DEBT.md

Antes de qualquer código, marcar os débitos que este passo vai fechar como
`EM CURSO` em `01_core/DEBT.md`, para que o estado seja visível durante
a implementação:

```markdown
### DEBT-27 — Suporte a PNG no exportador PDF — EM CURSO (Passo 74)
### DEBT-29 — Detecção de ColorSpace para JPEGs crus — EM CURSO (Passo 74)
### DEBT-28 — Dupla leitura de cabeçalho de imagem — EM CURSO (Passo 74)
### DEBT-26 — PartialEq exaustivo em Content::Image — EM CURSO (Passo 74)
```

Os quatro serão marcados como `ENCERRADO ✓` no final da Tarefa 5.

---

## Tarefa 1 — Dependências em L3 (L3)

Em `03_infra/Cargo.toml`, adicionar:

```toml
[dependencies]
image  = { version = "0.24", default-features = false, features = ["png", "jpeg"] }
flate2 = "1.0"
```

`default-features = false` exclui suporte a TGA, BMP, GIF, e outros formatos
não usados, reduzindo o tempo de compilação de L3. A `crate image` não entra
em L1 — confirmar com o diagnóstico 1 antes e depois desta edição.

Verificar que a crate compila sem erros antes de avançar:

```bash
cargo build -p typst-infra 2>&1 | head -20
```

---

## Tarefa 2 — `PtrEqArc<T>` em L1 (DEBT-26)

Em `01_core/src/entities/content.rs` (ou num módulo auxiliar
`01_core/src/entities/ptr_eq_arc.rs`), implementar:

```rust
use std::sync::Arc;

/// Arc<T> com PartialEq e Hash por ponteiro em vez de por valor.
///
/// Arc<Vec<u8>> com PartialEq derivado desreferencia e compara byte a byte
/// — O(N) onde N é o tamanho dos dados. Para imagens grandes (JPEGs de 5 MB)
/// isso é inaceitável em estruturas comparadas frequentemente.
///
/// PtrEqArc compara apenas o endereço do bloco de controlo do Arc — O(1).
/// Seguro enquanto os dados forem imutáveis (Vec<u8> não é mutado após criação).
#[derive(Debug, Clone)]
pub struct PtrEqArc<T>(pub Arc<T>);

impl<T> PartialEq for PtrEqArc<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for PtrEqArc<T> {}

impl<T> std::hash::Hash for PtrEqArc<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (Arc::as_ptr(&self.0) as usize).hash(state);
    }
}

impl<T> std::ops::Deref for PtrEqArc<T> {
    type Target = Arc<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

Substituir o campo `data: Arc<Vec<u8>>` em `Content::Image` por
`data: PtrEqArc<Vec<u8>>`:

```rust
// Em entities/content.rs — variante Image de Content
Image {
    path:   String,
    data:   PtrEqArc<Vec<u8>>,   // era: Arc<Vec<u8>>
    width:  Option<Value>,
    height: Option<Value>,
},
```

Actualizar todos os locais que constroem ou desestruturavam `Content::Image`
— o compilador lista os locais em falta. O campo `FrameItem::Image { data: Arc<Vec<u8>> }`
em L1 não é alterado: `FrameItem` é a display list do layouter, não uma
entidade de domínio comparada frequentemente. A deduplicação por `Arc::as_ptr`
no exportador continua a funcionar sobre o `Arc` interno de `PtrEqArc` via
`Deref`.

---

## Tarefa 3 — `ImageDimensions` com dimensões intrínsecas (DEBT-28)

Em `01_core/src/rules/layout/image.rs`, estender `ImageDimensions`:

```rust
pub struct ImageDimensions {
    pub width_pt:         f64,
    pub height_pt:        f64,
    /// Dimensões reais em píxeis, lidas do cabeçalho da imagem.
    /// Retornadas por calculate_dimensions para evitar uma segunda
    /// chamada a sizer.size() no layouter (DEBT-28).
    /// None se o sizer retornar None (formato desconhecido — fallback usado).
    pub intrinsic_width:  Option<u32>,
    pub intrinsic_height: Option<u32>,
}
```

Em `calculate_dimensions`, ao chamar `sizer.size(data)`, guardar o resultado
e passá-lo nos dois campos novos:

```rust
pub fn calculate_dimensions(
    data:   &[u8],
    width:  Option<&Value>,
    height: Option<&Value>,
    sizer:  &dyn ImageSizer,
) -> ImageDimensions {
    let intrinsic = sizer.size(data); // única leitura do cabeçalho

    let (intrinsic_w, intrinsic_h) = intrinsic.unwrap_or((100, 100));
    let native_w = intrinsic_w as f64;
    let native_h = intrinsic_h as f64;

    // ... lógica de aspect ratio e overrides do utilizador (inalterada) ...

    ImageDimensions {
        width_pt:         /* resultado calculado */,
        height_pt:        /* resultado calculado */,
        intrinsic_width:  intrinsic.map(|(w, _)| w),
        intrinsic_height: intrinsic.map(|(_, h)| h),
    }
}
```

No layouter (`01_core/src/rules/layout/mod.rs`), substituir a segunda chamada
a `sizer.size()`:

```rust
Content::Image { path: _, data, width, height } => {
    let dims = image::calculate_dimensions(
        data,
        width.as_deref(),
        height.as_deref(),
        self.sizer,
    );

    // DEBT-28 encerrado: intrinsic_width/height vêm de calculate_dimensions.
    // A segunda chamada a self.sizer.size(data) foi eliminada.
    let intrinsic_w = dims.intrinsic_width.unwrap_or(100);
    let intrinsic_h = dims.intrinsic_height.unwrap_or(100);

    // ... resto do braço inalterado (quebra de página, flush_line, push_frame_item) ...

    self.push_frame_item(pos, FrameItem::Image {
        data:             Arc::clone(&data.0), // .0 para aceder ao Arc interno de PtrEqArc
        width:            Pt(dims.width_pt),
        height:           Pt(dims.height_pt),
        intrinsic_width:  intrinsic_w,
        intrinsic_height: intrinsic_h,
    });

    self.cursor_y += Pt(dims.height_pt);
},
```

---

## Tarefa 4 — Processador de imagem para PDF (L3)

Em `03_infra/src/export.rs` (ou num submódulo `03_infra/src/pdf/image.rs`),
criar a estrutura de resultado e a função de processamento:

```rust
use std::io::Write;
use flate2::write::ZlibEncoder;
use flate2::Compression;

/// Dados de imagem prontos para emissão como XObject(s) num PDF.
pub struct PdfImagePayload {
    pub width:                u32,
    pub height:               u32,
    /// "/DeviceRGB", "/DeviceGray" — determinado pelos dados da imagem.
    pub color_space:          &'static str,
    /// Canal de cor comprimido com Zlib (/FlateDecode).
    pub rgb_data_compressed:  Vec<u8>,
    /// Canal alpha comprimido com Zlib, se a imagem tiver transparência não trivial.
    /// None se a imagem for totalmente opaca ou não tiver canal alpha.
    pub alpha_data_compressed: Option<Vec<u8>>,
}

/// Comprime um buffer de bytes com Zlib. Função auxiliar partilhada pelos
/// caminhos RGB e Alpha para evitar duplicação do bloco ZlibEncoder.
fn compress_zlib(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
    enc.write_all(data).map_err(|e| e.to_string())?;
    enc.finish().map_err(|e| e.to_string())
}

/// Descodifica um PNG e prepara os dados para emissão como XObject(s) num PDF.
///
/// Dois caminhos de execução consoante o modo de cor da imagem:
///
/// **Sem alpha** (`!img.color().has_alpha()`): converte para RGB8 com `to_rgb8()`
/// e passa os bytes planos directamente para `compress_zlib`. Sem alocação de
/// buffer de alpha, sem iteração pixel a pixel. Retorna `alpha_data_compressed: None`.
///
/// **Com alpha**: converte para RGBA8, itera os píxeis separando os canais RGB
/// e A em dois buffers, comprime ambos. Se o canal A for totalmente opaco
/// (todos os bytes == 255), descarta-o antes de comprimir — um /SMask com alpha
/// uniforme 255 não tem efeito visual e aumenta o tamanho do PDF desnecessariamente.
pub fn process_png_for_pdf(raw_data: &[u8]) -> Result<PdfImagePayload, String> {
    let img = image::load_from_memory(raw_data)
        .map_err(|e| format!("Falha ao descodificar imagem: {}", e))?;

    let width  = img.width();
    let height = img.height();

    // Caminho sem alpha — to_rgb8().as_raw() é um &[u8] plano RGBRGB...
    // sem necessidade de iteração pixel a pixel ou alocação extra.
    if !img.color().has_alpha() {
        return Ok(PdfImagePayload {
            width,
            height,
            color_space:           "/DeviceRGB",
            rgb_data_compressed:   compress_zlib(img.to_rgb8().as_raw())?,
            alpha_data_compressed: None,
        });
    }

    // Caminho com alpha — separar canais RGB e A pixel a pixel.
    let rgba = img.to_rgba8();
    let mut rgb_buffer   = Vec::with_capacity((width * height * 3) as usize);
    let mut alpha_buffer = Vec::with_capacity((width * height) as usize);

    for pixel in rgba.pixels() {
        rgb_buffer.push(pixel[0]); // R
        rgb_buffer.push(pixel[1]); // G
        rgb_buffer.push(pixel[2]); // B
        alpha_buffer.push(pixel[3]);
    }

    // Descartar o canal alpha se for totalmente opaco — o /SMask seria
    // redundante e aumentaria o tamanho do PDF sem benefício visual.
    let alpha_compressed = if alpha_buffer.iter().all(|&a| a == 255) {
        None
    } else {
        Some(compress_zlib(&alpha_buffer)?)
    };

    Ok(PdfImagePayload {
        width,
        height,
        color_space:           "/DeviceRGB",
        rgb_data_compressed:   compress_zlib(&rgb_buffer)?,
        alpha_data_compressed: alpha_compressed,
    })
}
```

---

## Tarefa 5 — ColorSpace JPEG por SOF0/SOF2 (DEBT-29)

Em `03_infra/src/export.rs`, adicionar a função de detecção de ColorSpace
junto a `detect_format` (sem dependências externas — lê apenas os bytes crus):

```rust
/// Lê o marcador SOF0 (0xC0) ou SOF2 (0xC2) do cabeçalho JPEG para determinar
/// o número de canais e retorna o ColorSpace correcto para o dicionário do XObject.
///
/// Um JPEG com ColorSpace errado produz lixo visual (Grayscale renderizado como
/// RGB monocromático) ou é recusado por alguns leitores PDF (CMYK).
/// O fallback "/DeviceRGB" cobre a maioria dos JPEGs de câmara.
fn jpeg_color_space(data: &[u8]) -> &'static str {
    // Estrutura do cabeçalho JPEG (JFIF/Exif):
    // FF D8          — SOI (Start Of Image)
    // FF xx LL LL   — marcador, comprimento big-endian (inclui os 2 bytes de LL)
    // SOF0/SOF2 contém: LL LL P HH HH WW WW CC
    //   P  = precisão em bits (normalmente 8)
    //   HH = altura, WW = largura (2 bytes cada)
    //   CC = número de canais — é este que nos interessa

    let mut i = 2usize; // saltar SOI (FF D8)
    while i + 3 < data.len() {
        if data[i] != 0xFF {
            break; // bytes corrompidos ou fim dos marcadores
        }
        let marker = data[i + 1];
        // Comprimento do segmento (inclui os 2 bytes de comprimento, exclui FF+marcador)
        if i + 3 >= data.len() { break; }
        let len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;

        if marker == 0xC0 || marker == 0xC2 {
            // byte 9 do segmento SOF (i+2 é o início do comprimento,
            // i+2+7 = i+9 é o número de canais)
            if i + 9 < data.len() {
                return match data[i + 9] {
                    1 => "/DeviceGray",
                    3 => "/DeviceRGB",
                    4 => "/DeviceCMYK",
                    _ => "/DeviceRGB",
                };
            }
            break;
        }

        // Parar se atingir o marcador SOS (Start of Scan, 0xDA).
        // Após o SOS começam os dados comprimidos da imagem, que não têm
        // marcadores com comprimento válido. Continuar a ler produziria
        // saltos de memória arbitrários ou um loop que nunca termina.
        if marker == 0xDA {
            break;
        }

        // Avançar para o próximo marcador
        if len < 2 { break; } // comprimento inválido — evitar loop infinito
        i += 2 + len;
    }
    "/DeviceRGB" // fallback para cabeçalhos malformados
}
```

No braço JPEG do exportador, substituir a linha que escrevia `/ColorSpace /DeviceRGB`
hardcoded. O exportador é manual — não existe `image.color_space()`. O ColorSpace
é uma entrada no dicionário do XObject, que é uma string formatada:

```rust
// DEBT-29 encerrado: ColorSpace determinado pelo cabeçalho JPEG.
// O exportador é manual — o dicionário do XObject é construído com format!().
// Substituir a linha anterior que tinha "/ColorSpace /DeviceRGB" fixo por:
let color_space = jpeg_color_space(data);

// Ao construir o dicionário do XObject JPEG, usar color_space em vez da
// string hardcoded. Exemplo com o padrão de serialização manual do Passo 73:
//
//   format!(
//       "{obj_id} 0 obj\n\
//        << /Type /XObject /Subtype /Image\n\
//           /Width {w} /Height {h}\n\
//           /ColorSpace {cs}\n\
//           /BitsPerComponent 8\n\
//           /Filter /DCTDecode\n\
//           /Length {len}\n\
//        >>\nstream\n",
//       obj_id = img_obj_id,
//       w  = intrinsic_width,
//       h  = intrinsic_height,
//       cs = color_space,       // ← variável, não string literal
//       len = data.len(),
//   )
//
// Adaptar os nomes de variável ao código real do exportador.
```

---

## Tarefa 6 — Emissão de PNG no exportador (L3)

No braço `ImageFormat::Png` do loop de exportação, substituir o `eprintln!`
de DEBT-27 pelo processamento completo.

**O exportador é manual** — não existe `pdf.alloc()`, `pdf.image_xobject()`, nem
`smask_xobj.filter()`. Os objetos PDF são escritos como strings formatadas para
um `Vec<u8>`, com números de objeto geridos por um contador local. Adaptar ao
padrão de serialização estabelecido no Passo 73.

Os quatro pontos críticos para o exportador manual:

**1. A verificação de deduplicação é a primeira instrução do loop**, antes de
qualquer detecção de formato ou processamento de bytes. Um cache hit significa
que o XObject já existe no PDF — só é necessário emitir o comando de desenho
na página e saltar todo o resto com `continue`.

**2. `/Length` tem de ser o tamanho exacto dos bytes comprimidos.** Os dados Zlib
já estão em `payload.rgb_data_compressed` e `payload.alpha_data_compressed` —
o seu `.len()` é o valor correcto. Não é necessário calcular nada extra.

**3. O XObject do /SMask tem de ser escrito antes do XObject principal**, porque
o dicionário do principal referencia o número de objeto do /SMask (`N 0 R`). No
exportador manual, a ordem de escrita no `Vec<u8>` é a ordem dos objetos no
ficheiro — escrever primeiro o /SMask garante que a referência cruzada é válida.

**4. O número de objeto do /SMask tem de ser alocado antes de ser referenciado.**
Alocar ambos os IDs antes de escrever qualquer dicionário resolve a dependência
circular entre o dicionário do principal (que referencia o ID do /SMask) e a
ordem de escrita.

A estrutura do loop completo (JPEG e PNG) com deduplicação no início:

```rust
for (pos, item) in &page.items {
    if let FrameItem::Image { data, width, height, intrinsic_width, intrinsic_height } = item {

        // Obter o ponteiro único do Arc — estável enquanto PagedDocument existir.
        let arc_ptr = Arc::as_ptr(data) as usize;

        // 1. Verificação de cache (cache hit) — PRIMEIRA instrução, antes de
        //    qualquer detecção de formato ou chamada a process_png_for_pdf.
        //    Se a imagem já foi escrita nesta invocação de export_pdf, reutilizar
        //    o XObject existente emitindo apenas o comando de desenho na página.
        if let Some(img_ref) = image_resources.get(&arc_ptr) {
            // Matriz [w 0 0 h x pdf_y] + comando Do para desenhar o XObject.
            // Adaptar a fórmula de pdf_y ao padrão do Passo 73.
            let pdf_y = page_height - pos.y.0 - height.0;
            page_stream.push_str(&format!(
                "q {w} 0 0 {h} {x} {y} cm /{name} Do Q\n",
                w    = width.0,
                h    = height.0,
                x    = pos.x.0,
                y    = pdf_y,
                name = img_ref.name,
            ));
            continue; // Salta toda a escrita de XObjects abaixo.
        }

        // 2. Cache miss — detectar formato e processar.
        let format = detect_format(data);

        match format {
            ImageFormat::Jpeg => {
                // ... lógica JPEG com jpeg_color_space e /DCTDecode (Tarefa 5) ...
            },

            ImageFormat::Png => {
                let payload = match process_png_for_pdf(data) {
                    Ok(p)  => p,
                    Err(e) => {
                        eprintln!("PNG inválido — imagem omitida: {}", e);
                        continue;
                    }
                };

                // Alocar ambos os IDs antes de escrever qualquer dicionário,
                // porque o dicionário do XObject principal referencia o ID do /SMask.
                let smask_obj_id: Option<usize> = if payload.alpha_data_compressed.is_some() {
                    obj_counter += 1;
                    Some(obj_counter)
                } else {
                    None
                };
                obj_counter += 1;
                let img_obj_id = obj_counter;

                let img_name = format!("Im{}", image_counter);
                image_counter += 1;

                // 3. Escrever o XObject do /SMask (canal alpha) antes do principal.
                if let (Some(smask_id), Some(alpha_data)) =
                    (smask_obj_id, &payload.alpha_data_compressed)
                {
                    xref_positions.insert(smask_id, out.len());
                    out.extend_from_slice(
                        format!(
                            "{smask_id} 0 obj\n\
                             << /Type /XObject /Subtype /Image\n\
                                /Width {w} /Height {h}\n\
                                /ColorSpace /DeviceGray\n\
                                /BitsPerComponent 8\n\
                                /Filter /FlateDecode\n\
                                /Length {len}\n\
                             >>\nstream\n",
                            smask_id = smask_id,
                            w   = payload.width,
                            h   = payload.height,
                            len = alpha_data.len(),
                        )
                        .as_bytes(),
                    );
                    out.extend_from_slice(alpha_data);
                    out.extend_from_slice(b"\nendstream\nendobj\n");
                }

                // 4. Escrever o XObject principal (canal RGB).
                xref_positions.insert(img_obj_id, out.len());

                let smask_entry = match smask_obj_id {
                    Some(id) => format!("/SMask {id} 0 R\n   "),
                    None     => String::new(),
                };

                out.extend_from_slice(
                    format!(
                        "{img_obj_id} 0 obj\n\
                         << /Type /XObject /Subtype /Image\n\
                            /Width {w} /Height {h}\n\
                            /ColorSpace /DeviceRGB\n\
                            /BitsPerComponent 8\n\
                            {smask_entry}/Filter /FlateDecode\n\
                            /Length {len}\n\
                         >>\nstream\n",
                        img_obj_id  = img_obj_id,
                        w           = payload.width,
                        h           = payload.height,
                        smask_entry = smask_entry,
                        len         = payload.rgb_data_compressed.len(),
                    )
                    .as_bytes(),
                );
                out.extend_from_slice(&payload.rgb_data_compressed);
                out.extend_from_slice(b"\nendstream\nendobj\n");

                // 5. Guardar no cache. Referências futuras à mesma imagem
                //    serão resolvidas pelo cache hit no topo do loop.
                image_resources.insert(arc_ptr, PdfImageRef { obj_id: img_obj_id, name: img_name });
            },

            ImageFormat::Unknown => {
                eprintln!("Formato de imagem desconhecido — imagem omitida");
                continue;
            },
        }

        // Emitir o comando de desenho na página para este cache miss.
        let img_ref = image_resources.get(&arc_ptr).unwrap(); // acabou de ser inserido
        let pdf_y   = page_height - pos.y.0 - height.0;
        page_stream.push_str(&format!(
            "q {w} 0 0 {h} {x} {y} cm /{name} Do Q\n",
            w    = width.0,
            h    = height.0,
            x    = pos.x.0,
            y    = pdf_y,
            name = img_ref.name,
        ));
    }
}
```

**Sobre variáveis do exportador manual:** os nomes `obj_counter`, `xref_positions`,
`out`, e `page_stream` são os nomes prováveis com base no padrão do Passo 73,
mas podem diferir. O diagnóstico 2 confirma os nomes reais antes de codificar.

---

## Tarefa 7 — Testes

### Teste L1 — `ImageDimensions` retorna dimensões intrínsecas (DEBT-28)

```rust
#[test]
fn calculate_dimensions_retorna_intrinsic() {
    use crate::entities::image_sizer::NullImageSizer;

    // NullImageSizer retorna None — os campos intrinsic devem ser None.
    let dims = calculate_dimensions(
        &[0xFF, 0xD8, 0xFF, 0x00],
        None,
        None,
        &NullImageSizer,
    );

    assert_eq!(dims.intrinsic_width,  None);
    assert_eq!(dims.intrinsic_height, None);

    // Com um sizer que retorna dimensões reais, os campos devem ser preenchidos.
    struct FixedSizer;
    impl ImageSizer for FixedSizer {
        fn size(&self, _data: &[u8]) -> Option<(u32, u32)> { Some((800, 600)) }
    }

    let dims2 = calculate_dimensions(&[], None, None, &FixedSizer);
    assert_eq!(dims2.intrinsic_width,  Some(800));
    assert_eq!(dims2.intrinsic_height, Some(600));
}
```

### Teste L1 — `PtrEqArc` compara por ponteiro (DEBT-26)

```rust
#[test]
fn ptr_eq_arc_compara_por_ponteiro() {
    use crate::entities::ptr_eq_arc::PtrEqArc;
    use std::sync::Arc;

    let arc1 = Arc::new(vec![1u8, 2, 3]);
    let arc2 = Arc::clone(&arc1);
    let arc3 = Arc::new(vec![1u8, 2, 3]); // mesmo conteúdo, ponteiro diferente

    let p1 = PtrEqArc(arc1);
    let p2 = PtrEqArc(arc2);
    let p3 = PtrEqArc(arc3);

    assert_eq!(p1, p2, "Clones do mesmo Arc são iguais por ponteiro");
    assert_ne!(p1, p3, "Arcs diferentes com mesmo conteúdo são desiguais");
}
```

### Teste L3 — ColorSpace JPEG lido do cabeçalho (DEBT-29)

```rust
#[test]
fn jpeg_color_space_grayscale() {
    // Cabeçalho JPEG mínimo com SOF0 e 1 canal (Grayscale).
    // Estrutura: FF D8 (SOI) | FF C0 (SOF0) | 00 0B (len=11) |
    //   08 (precision) | 00 01 (height=1) | 00 01 (width=1) | 01 (components=1)
    let jpeg = vec![
        0xFF, 0xD8,             // SOI
        0xFF, 0xC0,             // SOF0
        0x00, 0x0B,             // length = 11
        0x08,                   // precision = 8 bits
        0x00, 0x01,             // height = 1
        0x00, 0x01,             // width = 1
        0x01,                   // components = 1 → DeviceGray
    ];

    assert_eq!(jpeg_color_space(&jpeg), "/DeviceGray");
}

#[test]
fn jpeg_color_space_fallback_rgb() {
    // Cabeçalho JPEG sem marcador SOF0/SOF2 — fallback deve ser DeviceRGB.
    let jpeg = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x04];
    assert_eq!(jpeg_color_space(&jpeg), "/DeviceRGB");
}
```

### Teste L3 — Pipeline completo com PNG com transparência (DEBT-27)

```rust
#[test]
fn pipeline_png_transparente_gera_smask() {
    use image::{RgbaImage, Rgba};

    // Gerar um PNG 2×2 com dois píxeis semi-transparentes e dois opacos.
    let root = criar_dir_temporario();
    let mut img = RgbaImage::new(2, 2);
    img.put_pixel(0, 0, Rgba([255, 0,   0,   128])); // vermelho, semi-transparente
    img.put_pixel(1, 0, Rgba([0,   255, 0,   255])); // verde, opaco
    img.put_pixel(0, 1, Rgba([0,   0,   255, 0  ])); // azul, transparente
    img.put_pixel(1, 1, Rgba([255, 255, 0,   255])); // amarelo, opaco
    img.save(root.join("alpha.png")).unwrap();

    std::fs::write(root.join("main.typ"), "#image(\"alpha.png\")").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    let pdf    = export_pdf(&doc);

    let pdf_str = String::from_utf8_lossy(&pdf);

    assert!(!pdf.is_empty(),
        "export_pdf deve produzir bytes");
    assert!(pdf_str.contains("/Filter /FlateDecode"),
        "PNG deve usar /FlateDecode (Zlib)");
    assert!(pdf_str.contains("/SMask"),
        "PNG com transparência deve emitir /SMask");
    assert!(pdf_str.contains("/ColorSpace /DeviceGray"),
        "XObject do canal alpha deve usar /DeviceGray");
    assert!(pdf_str.contains("/ColorSpace /DeviceRGB"),
        "XObject principal do PNG deve usar /DeviceRGB");
}

#[test]
fn pipeline_png_opaco_sem_smask() {
    use image::{RgbaImage, Rgba};

    // PNG totalmente opaco — nenhum /SMask deve ser emitido.
    let root = criar_dir_temporario();
    let mut img = RgbaImage::new(1, 1);
    img.put_pixel(0, 0, Rgba([100, 150, 200, 255]));
    img.save(root.join("opaco.png")).unwrap();

    std::fs::write(root.join("main.typ"), "#image(\"opaco.png\")").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    let pdf    = export_pdf(&doc);

    let pdf_str = String::from_utf8_lossy(&pdf);
    assert!(!pdf_str.contains("/SMask"),
        "PNG totalmente opaco não deve emitir /SMask");
}
```

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] `image` e `flate2` estão apenas em `03_infra/Cargo.toml`. `01_core/Cargo.toml` não foi tocado.
- [ ] `PtrEqArc<T>` implementado em L1 com `PartialEq` e `Hash` por ponteiro (O(1)).
- [ ] `Content::Image { data: PtrEqArc<Vec<u8>> }` — campo actualizado em todos os locais de construção e desestruturação.
- [ ] `ImageDimensions` tem `intrinsic_width: Option<u32>` e `intrinsic_height: Option<u32>`. A segunda chamada a `sizer.size()` no layouter foi eliminada.
- [ ] `jpeg_color_space` lê `SOF0`/`SOF2` e retorna `DeviceGray`, `DeviceRGB`, ou `DeviceCMYK` conforme o número de canais.
- [ ] O exportador usa `jpeg_color_space` para todos os JPEGs — a string `DeviceRGB` hardcoded foi removida.
- [ ] `process_png_for_pdf` descodifica PNG em RGBA, separa canal alpha, comprime ambos com Zlib.
- [ ] PNG totalmente opaco (todos os píxeis alpha == 255) não emite XObject `/SMask`.
- [ ] PNG com transparência emite dois XObjects: `DeviceGray` para o alpha e `DeviceRGB` para a cor, ligados por `/SMask N 0 R`.
- [ ] PNG corrompido ou formato desconhecido omite a imagem silenciosamente sem corromper o PDF.
- [ ] Deduplicação por `Arc::as_ptr` continua a funcionar para PNG — um PNG repetido 10 vezes gera um único XObject.
- [ ] Comentário no exportador explica por que `Arc::as_ptr` é seguro durante `export_pdf`.
- [ ] DEBT-26, DEBT-27, DEBT-28, DEBT-29 marcados como **ENCERRADO ✓** em `01_core/DEBT.md`.
- [ ] Teste `ptr_eq_arc_compara_por_ponteiro` passa.
- [ ] Teste `calculate_dimensions_retorna_intrinsic` passa.
- [ ] Teste `jpeg_color_space_grayscale` passa.
- [ ] Teste `pipeline_png_transparente_gera_smask` passa.
- [ ] Teste `pipeline_png_opaco_sem_smask` passa.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Da implementação:**
- Se `process_png_for_pdf` precisou de tratamento especial para PNGs com modo
  de cor `Grayscale+Alpha` (LA8) — a iteração `to_rgba8().pixels()` é uniforme,
  mas confirmar que o `color_space` do XObject principal é `/DeviceRGB` mesmo
  nesse caso (após conversão).
- Se `jpeg_color_space` encontrou cabeçalhos JPEG onde o marcador SOF0 não está
  imediatamente a seguir a JFIF/Exif — alguns encoders colocam segmentos DQT
  antes de SOF0, o que o loop while trata correctamente.
- Número total de testes após o passo e zero violations confirmados.

**Go/No-Go para o Passo 75:**
- **GO — DEBT-25 (caminhos relativos):** `#image("subdir/foto.png")` em
  ficheiros Typst que não são o ficheiro raiz resolve incorrectamente.
  Passo 75 expõe o `FileId` do ficheiro em avaliação em `EvalContext` para
  que `World::file` resolva relativamente ao ficheiro fonte.
- **GO — DEBT-14 (`#set figure(numbering)`) e DEBT-15 (`kind` em `Figure`):**
  numeração automática de figuras e filtragem por tipo.
- **NO-GO — `/SMask` recusado pelo leitor PDF:** se Adobe Acrobat ou Chrome
  recusarem o PDF gerado, verificar que o XObject do alpha tem `/Subtype /Image`
  e que `/SMask` referencia o ID correcto antes de avançar para o Passo 75.
