# Passo 73 — FrameItem::Image e Exportação de PDF (DEBT-24c)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/layout/frame.rs` — Onde `FrameItem` está definido.
- `01_core/src/rules/layout/mod.rs` — Braço `Content::Image` do Passo 72.
- O ficheiro onde `export_pdf` vive — confirmar com o diagnóstico 2 se é
  L1 ou L3. Esta distinção determina onde a Tarefa 3 será implementada.
- `Cargo.toml` do exportador — confirmar qual biblioteca PDF está a ser usada.

Pré-condição: `cargo test` — ~706 L1 + ~126 L3, zero violations.
DEBT-24c registado. `FrameItem::Image` ainda não existe.

---

## Contexto

O Passo 72 ensinou o layouter a reservar o espaço geométrico correcto para
uma imagem. O cursor avança `height_pt` mas nenhum `FrameItem` é emitido —
a imagem não aparece no PDF.

Este passo adiciona `FrameItem::Image` à display list e actualiza o exportador
PDF para emitir o XObject correspondente. A prova de conceito usa JPEG (bytes
crus com `/DCTDecode`) porque PNG requer descodificação completa para ser
embutido num PDF — esse suporte fica para o Passo 74 com `crate image` em L3.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Localizar o enum FrameItem
grep -rn "pub enum FrameItem" 01_core/src/ 03_infra/src/ | head -5

# 2. Localizar export_pdf — determina se a Tarefa 3 fica em L1 ou L3
grep -rn "pub fn export_pdf" 01_core/src/ 03_infra/src/ | head -5

# 3. Confirmar qual biblioteca PDF está a ser usada
grep -n "pdf-writer\|lopdf\|printpdf" \
  01_core/Cargo.toml 03_infra/Cargo.toml 2>/dev/null | head -10

# 4. Confirmar como o layouter armazena FrameItems no frame activo
grep -n "push_frame_item\|push(\|frame\." 01_core/src/rules/layout/mod.rs | head -15

# 5. Confirmar como as coordenadas Y são tratadas no exportador actual
# (PDF usa Y crescente de baixo para cima; layout usa Y crescente de cima para baixo)
grep -n "cursor_y\|pdf_y\|page_height" 01_core/src/rules/layout/mod.rs | head -10
```

Reportar o output completo antes de continuar. O diagnóstico 2 é crítico:
se `export_pdf` viver em L1, a Tarefa 3 implementa-se em L1 sem dependências
externas de imagesize. Se viver em L3, a Tarefa 3 vai para L3 e pode usar
`crate image` no Passo 74.

---

## Tarefa 0 — Actualizar DEBT.md

Antes de qualquer código, registar em `01_core/DEBT.md`:

```markdown
### DEBT-29 — Detecção de ColorSpace para JPEGs crus (Passo 73)
O exportador assume DeviceRGB para todos os JPEGs. JPEGs Grayscale (1 canal)
ou CMYK (4 canais) com ColorSpace errado produzem lixo visual ou rejeição pelo
leitor PDF. Resolução: Passo 74 lê o marcador SOF0/SOF2 do cabeçalho JPEG para
determinar o número de canais e escolher DeviceRGB, DeviceGray ou DeviceCMYK.

### DEBT-28 — Dupla leitura de cabeçalho de imagem no layouter (Passo 73)
calculate_dimensions e sizer.size() lêem o cabeçalho da imagem separadamente.
O custo é mínimo (imagesize lê apenas o cabeçalho, não os píxeis), mas é
redundante. Resolução futura: calculate_dimensions retorna ImageDimensions
com width_pt, height_pt, intrinsic_width, intrinsic_height — eliminando a
segunda chamada a sizer.size() na Tarefa 2.

### DEBT-27 — Suporte a transparência PNG no exportador PDF (Passo 73)
PDF não suporta ficheiros PNG crus. PNG requer descodificação de píxeis
(canal RGBA separado para /SMask de transparência) antes de ser embutido.
Prova de conceito do Passo 73 usa apenas JPEG (/DCTDecode).
Resolução: Passo 74 adiciona crate image a L3 para descodificar PNG
em RGBA plano e passar ao gerador de PDF.
```

---

## Tarefa 1 — `FrameItem::Image` (L1)

Em `01_core/src/rules/layout/frame.rs`, adicionar a variante ao enum:

```rust
use std::sync::Arc;

pub enum FrameItem {
    Text(/* ... */),
    Shape(/* ... */),
    /// Imagem a desenhar na página.
    ///
    /// `data`: bytes raw da imagem (PNG, JPEG, etc.) — Arc para zero-copy.
    /// `width`, `height`: dimensões físicas no documento, em pontos (tamanho de layout).
    /// `intrinsic_width`, `intrinsic_height`: dimensões reais em píxeis, lidas do
    /// cabeçalho da imagem. Obrigatórias para o dicionário do XObject no PDF —
    /// o PDF exige /Width e /Height intrínsecos, não o tamanho de layout.
    /// As coordenadas de posição vivem no frame pai (Point associado ao item).
    Image {
        data:             Arc<Vec<u8>>,
        width:            Pt,
        height:           Pt,
        intrinsic_width:  u32,
        intrinsic_height: u32,
    },
}
```

Actualizar qualquer `match` sobre `FrameItem` no mesmo ficheiro (bounding box,
debug print, etc.) — o compilador lista os locais em falta.

---

## Tarefa 2 — Emitir `FrameItem::Image` no layouter (L1)

Em `01_core/src/rules/layout/mod.rs`, substituir o braço `Content::Image`
do Passo 72 (que só avançava o cursor):

```rust
Content::Image { path: _, data, width, height } => {
    let dims = image::calculate_dimensions(
        data,
        width.as_deref(),   // Option<Box<Value>> ou Option<Value> — ver Passo 72
        height.as_deref(),
        self.sizer,
    );

    // Verificar se a imagem cabe na página actual antes de emiti-la.
    // Se cursor_y + height ultrapassar page_height - margem, acionar
    // quebra de página e actualizar pos.y para o topo da nova página.
    // Se flush_line ou advance_block já tratam quebras de página
    // automaticamente, esta verificação pode ser desnecessária —
    // confirmar com o comportamento actual do layouter para blocos de
    // texto longos (que já devem provocar quebra de página).
    if self.cursor_y.0 + dims.height_pt > self.page_height.0 - self.margin.0 {
        self.new_page(); // adaptar ao método real de quebra de página
    }
    // Garantir que a imagem começa após o texto da linha actual.
    self.flush_line();

    // Posição actual do cursor — onde o canto superior esquerdo da imagem fica.
    // Se houve quebra de página acima, cursor_y já aponta para o topo da nova página.
    let pos = Point { x: self.cursor_x, y: self.cursor_y };

    // DEBT-28: segunda leitura do cabeçalho — calculate_dimensions já leu o mesmo
    // cabeçalho internamente. Custo mínimo (apenas cabeçalho, não píxeis), mas
    // redundante. Resolução: calculate_dimensions retorna intrinsic_* directamente.
    //
    // Dimensões intrínsecas em píxeis — necessárias para o dicionário do XObject
    // no PDF (/Width, /Height intrínsecos ≠ tamanho de layout na página).
    // O fallback (100, 100) só ocorre se sizer.size() retornar None (formato
    // desconhecido) — documentado nos comentários de calculate_dimensions.
    let (intrinsic_w, intrinsic_h) = self.sizer.size(data).unwrap_or((100, 100));

    // Emitir o item visual na display list.
    // Adaptar a chamada ao método real do layouter (push_frame_item, push, etc.)
    // conforme o diagnóstico 4.
    //
    // NOTA sobre pos.y: deve ser o TOPO da bounding box da imagem, não o baseline
    // de texto. O exportador usa pos.y para calcular pdf_y = page_height - pos.y - height.
    // Se o layouter usar baseline como referência, a imagem ficará desalinhada.
    self.push_frame_item(pos, FrameItem::Image {
        data:             Arc::clone(data),
        width:            Pt(dims.width_pt),
        height:           Pt(dims.height_pt),
        intrinsic_width:  intrinsic_w,
        intrinsic_height: intrinsic_h,
    });

    // Avançar o cursor vertical pelo espaço ocupado.
    self.cursor_y += Pt(dims.height_pt);
},
```

---

## Tarefa 3 — Exportação para PDF

**Esta tarefa vai para a camada onde `export_pdf` vive** (confirmar com o
diagnóstico 2). Se `export_pdf` viver em L3 (onde `pdf-writer` está), a Tarefa 3
fica em L3 — L1 não deve conhecer `Filter::DctDecode`, `ObjId`, ou qualquer
conceito do formato PDF. Esses são detalhes de infraestrutura, não de domínio.

### 3a — Detecção de formato por magic numbers

Função auxiliar para distinguir JPEG de PNG sem `crate image`:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum ImageFormat {
    Jpeg,
    Png,
    Unknown,
}

fn detect_format(data: &[u8]) -> ImageFormat {
    if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        ImageFormat::Jpeg
    } else if data.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
        ImageFormat::Png
    } else {
        ImageFormat::Unknown
    }
}
```

### 3b — Mapa de deduplicação por ponteiro do Arc

A mesma imagem pode aparecer 10 vezes no documento. Os bytes não devem ser
escritos 10 vezes no PDF. O ponteiro do `Arc` é estável enquanto existir pelo
menos uma referência — serve como chave de deduplicação:

```rust
// Estado por invocação de export_pdf:
let mut image_resources: HashMap<usize, PdfImageRef> = HashMap::new();
let mut image_counter: usize = 1;

// PdfImageRef guarda o ID do XObject e o nome usado nas páginas:
struct PdfImageRef {
    obj_id: ObjId,     // ID interno do pdf-writer — ajustar ao tipo real
    name:   String,    // ex: "Im1", "Im2"
}
```

### 3c — Escrita do XObject e referência na página

**Prova de conceito: apenas JPEG neste passo.** PNG regista DEBT-27 e fica
para o Passo 74.

Adaptar ao API da biblioteca PDF usada (diagnóstico 3). Exemplo com
`pdf-writer` como referência conceptual:

```rust
FrameItem::Image { data, width, height, intrinsic_width, intrinsic_height } => {
    // Seguro porque 'doc' (PagedDocument) mantém todos os Arcs vivos durante
    // export_pdf, impedindo que o alocador reutilize os mesmos endereços.
    // Fora deste contexto (Arcs que podem ser destruídos durante a iteração),
    // esta técnica seria insegura — endereços de memória podem ser reutilizados.
    let arc_ptr = Arc::as_ptr(data) as usize;

    // Abortar precocemente para PNG e Unknown — sem alocar IDs no PDF.
    // Se alocarmos pdf.alloc() mas não escrevermos os dados do XObject,
    // content.x_object(...) referencia um objeto inexistente e o PDF fica
    // corrompido (Adobe Acrobat e Chrome recusam abrir o ficheiro).
    let format = detect_format(data);
    if format != ImageFormat::Jpeg {
        // DEBT-27: PNG requer descodificação completa (Passo 74).
        // Omitir silenciosamente — o espaço está reservado no layout mas
        // a imagem não aparece. O PDF continua válido.
        eprintln!("DEBT-27: apenas JPEG suportado neste passo — imagem omitida");
        // `continue` ou equivalente conforme a estrutura de iteração do exportador.
        continue;
    }

    let img_ref = image_resources.entry(arc_ptr).or_insert_with(|| {
        let name   = format!("Im{}", image_counter);
        image_counter += 1;

        // Só chegamos aqui para JPEG — o XObject é sempre escrito com dados válidos.
        // pdf.alloc() só é chamado quando temos certeza de que vamos escrever o objeto.
        let obj_id = pdf.alloc();  // ajustar ao API real

        // O dicionário do XObject EXIGE as dimensões intrínsecas em píxeis.
        // intrinsic_width e intrinsic_height vêm do FrameItem (Passo 72/73).
        // Um JPEG de 800×600 com /Width 100 /Height 100 produz PDF corrompido.
        let mut image = pdf.image_xobject(obj_id, data);
        image.width(*intrinsic_width as i32);
        image.height(*intrinsic_height as i32);
        // DEBT-29: assumimos DeviceRGB (3 canais). Um JPEG Grayscale ou CMYK
        // com ColorSpace errado produz lixo visual ou rejeição pelo leitor PDF.
        // Resolução: Passo 74 extrai o número de canais do cabeçalho JPEG
        // (SOF0/SOF2) para escolher DeviceRGB, DeviceGray ou DeviceCMYK.
        image.color_space().device_rgb();
        image.bits_per_component(8);
        image.filter(Filter::DctDecode); // bytes JPEG crus — sem recodificação
        // pdf-writer calcula /Length automaticamente no finish() do stream.
        // Se a biblioteca usada não o fizer, passar data.len() explicitamente.

        PdfImageRef { obj_id, name }
    });

    // Inversão do eixo Y: PDF tem Y crescente de baixo para cima;
    // layout tem Y crescente de cima para baixo.
    // pos.y é o TOPO da imagem → canto inferior esquerdo no sistema PDF
    // é page_height - pos.y - height (ajustar conforme diagnóstico 5).
    let pdf_y = page_height - pos.y.0 - height.0;

    // Matriz [w 0 0 h x y] escala o XObject (1×1 no espaço de unidade)
    // para as dimensões físicas da imagem na página.
    content.save_state();
    content.transform([
        width.0 as f32,
        0.0,
        0.0,
        height.0 as f32,
        pos.x.0 as f32,
        pdf_y as f32,
    ]);
    content.x_object(Name(img_ref.name.as_bytes()));
    content.restore_state();
},
```

**Nota sobre o eixo Y (diagnóstico 5):** a fórmula `page_height - pos.y - height`
pode precisar de ajuste dependendo de como o layouter armazena as coordenadas.
Verificar com um teste simples: uma imagem no topo da página deve aparecer no
topo do PDF, não no fundo.

---

## Tarefa 4 — Testes

### Teste L1 — `FrameItem::Image` emitido pelo layouter

```rust
#[test]
fn layout_image_gera_frameitem() {
    use crate::entities::image_sizer::NullImageSizer;

    // JPEG mínimo com magic numbers correctos — imagesize não é necessário
    // porque NullImageSizer retorna None e o fallback 100×100 é usado.
    let jpeg_magic = vec![0xFF, 0xD8, 0xFF, 0x00];

    let content = Content::Image {
        path:   "teste.jpg".to_string(),
        data:   std::sync::Arc::new(jpeg_magic),
        width:  None,
        height: None,
    };

    let state = introspect(&content);
    let doc   = layout_with_sizer(&content, state, &NullImageSizer);

    assert!(!doc.pages.is_empty(), "Documento deve ter pelo menos uma página");

    let page = &doc.pages[0];
    let has_image = page.items.iter().any(|(_, item)| {
        matches!(item, FrameItem::Image { .. })
    });
    assert!(has_image, "O layouter deve emitir FrameItem::Image");
}
```

**Nota:** se `layout` não aceitar `ImageSizer` como parâmetro directamente,
usar `layout_with_sizer` ou o padrão equivalente estabelecido no Passo 72.

### Teste L1 — deduplicação por ponteiro

```rust
#[test]
fn frameitem_image_deduplica_por_ponteiro() {
    // Dois FrameItems com o mesmo Arc devem ter o mesmo ponteiro.
    // A lógica de deduplicação no exportador usa Arc::as_ptr.
    let data = std::sync::Arc::new(vec![0xFF, 0xD8, 0xFF, 0x00u8]);
    let item1 = FrameItem::Image {
        data:   Arc::clone(&data),
        width:  Pt(100.0),
        height: Pt(100.0),
    };
    let item2 = FrameItem::Image {
        data:   Arc::clone(&data),
        width:  Pt(50.0),
        height: Pt(50.0),
    };

    if let (FrameItem::Image { data: d1, .. }, FrameItem::Image { data: d2, .. }) =
        (&item1, &item2)
    {
        assert!(
            Arc::as_ptr(d1) == Arc::as_ptr(d2),
            "Clones do mesmo Arc devem ter o mesmo ponteiro"
        );
    }
}
```

### Teste L3 — pipeline completo com JPEG

```rust
#[test]
fn pipeline_jpeg_gera_pdf_sem_panic() {
    let root = criar_dir_temporario();
    // JPEG mínimo válido (apenas magic numbers — imagesize lê as dimensões)
    std::fs::write(root.join("foto.jpg"), &[0xFF, 0xD8, 0xFF, 0xE0]).unwrap();
    std::fs::write(root.join("main.typ"), "#image(\"foto.jpg\")").unwrap();

    let world = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    let pdf    = export_pdf(&doc);

    // O PDF deve ter bytes — não entrar em panic mesmo que a imagem
    // não apareça visualmente (DEBT-27 para PNG, fallback para JPEG).
    assert!(!pdf.is_empty(), "export_pdf deve produzir bytes");
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
- [ ] `FrameItem::Image` tem cinco campos: `data`, `width`, `height`, `intrinsic_width: u32`, `intrinsic_height: u32`. Todos os `match` actualizados.
- [ ] Todos os `match` sobre `FrameItem` actualizados — compilador confirma.
- [ ] Layouter emite `FrameItem::Image` com as dimensões de `calculate_dimensions`.
- [ ] Quebra de página verificada antes de emitir a imagem — imagem nunca
  desenhada fora dos limites do papel.
- [ ] `detect_format` implementado com magic numbers para JPEG e PNG.
- [ ] Mapa de deduplicação por `Arc::as_ptr` implementado no exportador.
- [ ] JPEG embutido com `/DCTDecode` e dicionário `/Width`/`/Height` intrínsecos (em píxeis, não pt).
- [ ] Comentário no exportador explica por que `Arc::as_ptr` é seguro durante `export_pdf`.
- [ ] DEBT-29 registado (`ColorSpace` assumido DeviceRGB — JPEG Grayscale/CMYK não detectados).
- [ ] Tarefa 3 implementada na camada onde `export_pdf` vive — `Filter::DctDecode`
  e `ObjId` nunca aparecem em L1.
- [ ] PNG e Unknown abortam antes de `pdf.alloc()` — sem referências fantasmas no PDF.
- [ ] `content.x_object` nunca é chamado para formatos não suportados.
- [ ] Inversão do eixo Y documentada e correcta (imagem no topo aparece no topo).
- [ ] `pos.y` no layouter é o TOPO da bounding box — não o baseline de texto.
  Comentário no código confirma este invariante.
- [ ] DEBT-24c marcado como **encerrado** em `01_core/DEBT.md`.
- [ ] DEBT-27 registado em `DEBT.md` antes de qualquer código.
- [ ] Teste `layout_image_gera_frameitem` passa.
- [ ] Teste `pipeline_jpeg_gera_pdf_sem_panic` passa.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Camada onde `export_pdf` vive (L1 ou L3) — determinou onde a Tarefa 3
  foi implementada.
- Biblioteca PDF em uso — e se o seu API para XObjects difere do exemplo.
- Como o layouter armazena FrameItems (método `push_frame_item`, campo
  `items`, ou outro).

**Da implementação:**
- Se a inversão do eixo Y precisou de ajuste em relação à fórmula
  `page_height - pos.y - height`.
- Se JPEG funcionou como prova de conceito — imagem aparece no PDF.
- Número total de testes e zero violations confirmados.

**Go/No-Go para o Passo 74:**
- **GO — DEBT-27 (PNG com crate image em L3):** Passo 74 adiciona
  `crate image` a L3, descodifica PNG em RGBA, e emite com `/FlateDecode`
  e `/SMask` para transparência.
- **GO — DEBT-26 (PtrEqArc):** se DEBT-26 causou lentidão nos testes
  com imagens grandes, Passo 74 implementa `PtrEqArc<T>`.
- **NO-GO — eixo Y incorrecto:** se a imagem aparece no fundo em vez
  do topo; Passo 74 corrige a fórmula antes de avançar para PNG.
