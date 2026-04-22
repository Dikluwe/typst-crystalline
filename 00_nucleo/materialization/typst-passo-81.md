# Passo 81 — Configuração Dinâmica de Página (`#set page`) e Snapshots Imutáveis

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/layout_types.rs` — Onde `PageConfig` e `Page` serão
  definidos/actualizados.
- `01_core/src/entities/content.rs` — Onde `Content::SetPage` será adicionado.
- `01_core/src/rules/layout/mod.rs` — Onde `Layouter` tem actualmente
  constantes hardcoded de dimensão de página (`MARGIN`, `SIZE_A4`, ou equivalente).
- `03_infra/src/export.rs` — Onde o exportador usa uma `page_height` global
  que tem de passar a usar `page.height` por iteração.
- `00_nucleo/DEBT.md` — Confirmar estado dos DEBTs activos.

Pré-condição: `cargo test` — ~721 L1 + ~147 L3, zero violations.
Grid (`Content::Grid`) implementado no Passo 80.

---

## Contexto

Até este passo, o motor assume implicitamente A4 com margens fixas. As
dimensões estão hardcoded no layouter e no exportador. Isto impede documentos
com páginas de tamanhos diferentes — por exemplo, um relatório A4 com um
slide em formato wide intercalado.

Este passo resolve o problema com três mudanças coordenadas:

- **`PageConfig`** encapsula width, height e margin como estado mutável do
  layouter. `available_width()` e `available_height()` centralizam a
  matemática derivada — nunca espalhar esta subtração pelo código.

- **Snapshot imutável em `Page`**: quando uma página é fechada, captura
  `page_config.width` e `page_config.height` no momento do fecho. O
  exportador usa `page.height` por iteração, nunca uma variável global.

- **`current_page_is_empty()`** usa `current_items.is_empty()` como fonte
  de verdade estrutural. Comparar `cursor_y == margin` é frágil — o cursor
  pode estar na margem depois de transformações ou espaçamentos negativos.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Localizar todas as constantes hardcoded de dimensão de página em L1
grep -rn "841\|595\|A4\|MARGIN\|page_height\|page_width" \
  01_core/src/ | grep -v "test\|//.*DEBT" | head -20

# 2. Confirmar a estrutura actual de Page e como o exportador a itera
grep -n "struct Page\|pub pages\|page\.height\|page\.width" \
  01_core/src/entities/layout_types.rs \
  03_infra/src/export.rs 2>/dev/null | head -15

# 3. Confirmar a assinatura de new_page() e como o cursor é resetado
grep -A 8 "fn new_page" 01_core/src/rules/layout/mod.rs | head -15

# 4. Confirmar onde measure_content_constrained usa largura disponível
grep -n "measure_content_constrained\|safe_available\|available_width" \
  01_core/src/rules/layout/mod.rs | head -10
```

Reportar o output completo antes de continuar. O diagnóstico 1 identifica
todos os locais que precisam de ser substituídos por `self.available_width()`
ou `self.page_config.*`. O diagnóstico 4 confirma se `measure_content_constrained`
recebe a largura disponível como parâmetro ou calcula internamente — se for
internamente, tem de ser actualizado para consultar `page_config` após
`SetPage` (ver Tarefa 5).

---

## Tarefa 0 — Actualizar DEBT.md

```markdown
### DEBT-35b — Invalidação de cache de available_width após SetPage — EM ABERTO (Passo 81)
Se alguma função guardar available_width em cache como campo do Layouter,
esse cache tem de ser invalidado no processamento de Content::SetPage.
Actualmente available_width() é calculado em tempo real sem cache —
este DEBT documenta o risco caso um cache venha a ser adicionado.
```

---

## Tarefa 1 — `PageConfig` e actualização de `Page` (L1)

### 1a — `PageConfig` em `layout_types.rs`

```rust
/// Configuração da página activa no layouter.
///
/// Mutável durante o layout — Content::SetPage altera estes valores.
/// As páginas já fechadas têm os seus próprios snapshots de width/height.
#[derive(Debug, Clone, PartialEq)]
pub struct PageConfig {
    pub width:  f64, // em pontos
    pub height: f64, // em pontos
    pub margin: f64, // margem uniforme em pontos
}

impl Default for PageConfig {
    fn default() -> Self {
        Self {
            width:  595.28, // A4 portrait
            height: 841.89, // A4 portrait
            margin:  70.87, // ≈ 2.5 cm
        }
    }
}
```

### 1b — Actualizar `Page` para guardar o snapshot

`Page` já existe — adicionar `width` e `height` como campos se ainda não
existirem (confirmar com diagnóstico 2):

```rust
pub struct Page {
    /// Largura da página no momento em que foi fechada.
    /// Capturada como snapshot de PageConfig — pode diferir de outras páginas.
    pub width:  f64,
    /// Altura da página no momento em que foi fechada.
    pub height: f64,
    pub items:  Vec<(Point, FrameItem)>,
}
```

---

## Tarefa 2 — `Content::SetPage` (L1)

Em `01_core/src/entities/content.rs`:

```rust
// Na enum Content:
/// Altera a configuração da página a partir deste ponto do documento.
///
/// Se existir conteúdo na página actual, força uma quebra de página antes
/// de aplicar a nova configuração. Se a página actual estiver vazia, aplica
/// directamente sem quebra.
SetPage {
    width:  Option<f64>,
    height: Option<f64>,
    margin: Option<f64>,
},
```

Actualizar todos os `match` sobre `Content` — adicionar
`Content::SetPage { .. } => {}` nos braços que não tratam mudança de página
(introspecção, show rules, map_content, etc.).

---

## Tarefa 3 — `native_page` na stdlib (L1)

Em `01_core/src/rules/stdlib.rs`:

```rust
pub fn native_page(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    let width  = args.named::<Value>("width") .and_then(|v| extract_pt(&v));
    let height = args.named::<Value>("height").and_then(|v| extract_pt(&v));
    let margin = args.named::<Value>("margin").and_then(|v| extract_pt(&v));

    Ok(Value::Content(Content::SetPage { width, height, margin }))
}
```

Registar:

```rust
ctx.register("page", native_page);
```

`extract_pt` é o helper existente que converte `Value::Length` ou
`Value::Float` para `f64` em pontos — o mesmo usado em `native_rect` e
`native_grid`.

---

## Tarefa 4 — Layouter stateful sem hardcodes (L1)

### 4a — Substituir constantes por `PageConfig`

Em `01_core/src/rules/layout/mod.rs`, substituir os campos hardcoded pelo
`page_config`:

```rust
pub struct Layouter<'a> {
    // ... campos existentes ...

    /// Configuração da página activa. Mutável via Content::SetPage.
    pub page_config: PageConfig,

    /// Items acumulados na página actual (ainda não fechada).
    pub current_items: Vec<(Point, FrameItem)>,

    /// Páginas já fechadas com snapshots imutáveis de width/height.
    pub pages: Vec<Page>,

    // REMOVER: page_width, page_height, MARGIN hardcoded.
    // Substituir todos os usos por page_config.width, page_config.height,
    // page_config.margin, available_width(), available_height().
}
```

### 4b — Métodos de estado derivado

```rust
impl<'a> Layouter<'a> {
    /// Largura disponível para conteúdo (exclui margens dos dois lados).
    /// Nunca espalhar esta subtração pelo código — usar sempre este método.
    fn available_width(&self) -> f64 {
        f64::max(0.0, self.page_config.width - 2.0 * self.page_config.margin)
    }

    /// Altura disponível para conteúdo (exclui margens topo e base).
    fn available_height(&self) -> f64 {
        f64::max(0.0, self.page_config.height - 2.0 * self.page_config.margin)
    }

    /// Fonte de verdade estrutural: a página actual não tem nenhum item visual.
    ///
    /// Não usar cursor_y == margin como proxy — o cursor pode estar na margem
    /// após transformações ou espaçamentos negativos. A Display List é a
    /// fonte de verdade.
    fn current_page_is_empty(&self) -> bool {
        self.current_items.is_empty()
    }

    /// Fecha a página actual com snapshot imutável e abre uma nova.
    ///
    /// Chamar flush_line() ANTES de new_page() se existir line_buffer pendente.
    fn new_page(&mut self) {
        let page = Page {
            width:  self.page_config.width,   // snapshot no momento do fecho
            height: self.page_config.height,  // snapshot no momento do fecho
            items:  std::mem::take(&mut self.current_items),
        };
        self.pages.push(page);

        // Resetar cursor para o topo da nova página.
        self.cursor_x = Pt(self.page_config.margin);
        self.cursor_y = Pt(self.page_config.margin);
    }
}
```

### 4c — Actualizar todos os usos de constantes hardcoded

Com base no diagnóstico 1, substituir cada ocorrência:

```rust
// ANTES → DEPOIS
self.page_height.0          → self.page_config.height
self.page_width.0           → self.page_config.width
self.margin.0               → self.page_config.margin
available_width (calculado) → self.available_width()
```

O compilador lista os locais após remover os campos antigos. Substituir
sistematicamente — não deixar nenhum cálculo de margem inline.

---

## Tarefa 5 — Processamento de `Content::SetPage` (L1)

```rust
Content::SetPage { width, height, margin } => {
    let mut new_config = self.page_config.clone();
    let mut changed    = false;

    if let Some(w) = width  { new_config.width  = *w; changed = true; }
    if let Some(h) = height { new_config.height = *h; changed = true; }
    if let Some(m) = margin { new_config.margin = *m; changed = true; }

    if changed {
        // Regra Typst: quebra obrigatória se a página actual tem conteúdo.
        // Se estiver vazia, aplicar directamente sem quebra.
        if !self.current_page_is_empty() {
            self.flush_line(); // flush antes de fechar — sem conteúdo pendente
            self.new_page();
        }

        // Aplicar o novo regime. A partir daqui, available_width() e
        // available_height() retornam os valores da nova configuração.
        self.page_config = new_config;
        self.cursor_x    = Pt(self.page_config.margin);
        self.cursor_y    = Pt(self.page_config.margin);
        // DEBT-35b: se available_width() vier a ter cache, invalidar aqui.
    }
},
```

**Nota sobre `measure_content_constrained`:** esta função recebe `max_width`
como parâmetro — não calcula internamente. Portanto, ela herda automaticamente
o novo `available_width()` porque os chamadores (algoritmo de grid, quebra de
linha) invocam `self.available_width()` para obter o valor a passar. Não é
necessário nenhum ajuste adicional na função em si.

---

## Tarefa 6 — Exportador PDF usa snapshot por página (L3)

Em `03_infra/src/export.rs`, substituir qualquer `page_height` global pelo
`page.height` do snapshot:

```rust
for page in &doc.pages {
    // MediaBox usa as dimensões do snapshot desta página específica.
    // Duas páginas consecutivas podem ter MediaBox diferentes.
    let media_box = format!("[0 0 {:.2} {:.2}]", page.width, page.height);

    // ... escrever dicionário da página com media_box ...

    // Inversão Y: usar page.height DESTA página, não uma constante global.
    for (pos, item) in &page.items {
        // pdf_y = page.height - pos.y.0 - item_height
        // Adaptar conforme cada FrameItem — o padrão é o mesmo de antes,
        // mas page.height vem do snapshot em vez de uma variável externa.
        match item {
            FrameItem::Text { .. } => {
                let pdf_y = page.height - pos.y.0;
                // ... emitir operadores de texto com pdf_y ...
            },
            FrameItem::Shape { height: item_h, .. } => {
                let pdf_y = page.height - pos.y.0 - item_h;
                // ... emitir operadores de shape com pdf_y ...
            },
            FrameItem::Group { matrix, .. } => {
                // A transformação herda o sistema de coordenadas da página actual.
                // page.height é a referência para inversão Y deste grupo.
                let pdf_y = page.height - pos.y.0;
                // ... emitir cm com pdf_y ...
            },
            // ... outros FrameItems ...
        }
    }
}
```

Verificar com o diagnóstico 1 que não resta nenhuma referência a uma
`page_height` global no exportador após esta substituição.

---

## Tarefa 7 — Testes

### Teste L3 — `#set page` com conteúdo anterior força quebra

```rust
#[test]
fn set_page_forca_quebra_com_conteudo() {
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        Primeira linha\n\
        #set page(width: 200pt, height: 200pt, margin: 10pt)\n\
        Segunda página\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    assert_eq!(doc.pages.len(), 2,
        "SetPage com conteúdo deve criar 2 páginas");
    assert!(doc.pages[0].height > 800.0,
        "Primeira página deve ser A4 (height > 800pt)");
    assert_eq!(doc.pages[1].height, 200.0,
        "Segunda página deve ter height = 200pt do SetPage");
}
```

### Teste L3 — `#set page` no topo não cria página extra

```rust
#[test]
fn set_page_no_topo_nao_quebra() {
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 300pt, height: 400pt)\n\
        Conteúdo único\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    assert_eq!(doc.pages.len(), 1,
        "SetPage sem conteúdo anterior não deve criar página extra");
    assert_eq!(doc.pages[0].width,  300.0);
    assert_eq!(doc.pages[0].height, 400.0);
}
```

### Teste L3 — múltiplas mudanças de página preservam snapshots

```rust
#[test]
fn multiplas_mudancas_de_pagina_preservam_snapshots() {
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        P1\n\
        #set page(width: 200pt, height: 200pt)\n\
        P2\n\
        #set page(width: 100pt, height: 300pt)\n\
        P3\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    assert_eq!(doc.pages.len(), 3);
    assert!(doc.pages[0].width > 500.0, "Página 1 deve ser A4");
    assert_eq!(doc.pages[1].width,  200.0);
    assert_eq!(doc.pages[1].height, 200.0);
    assert_eq!(doc.pages[2].width,  100.0);
    assert_eq!(doc.pages[2].height, 300.0);
}
```

### Teste L3 — grid respeita `available_width` dinâmico

```rust
#[test]
fn grid_respeita_page_config_dinamico() {
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 400pt, height: 400pt, margin: 20pt)\n\
        #grid(columns: (1fr, 1fr), [A], [B])\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    // available_width = 400 - 2*20 = 360pt; cada 1fr = 180pt.
    // O segundo item (célula B) deve começar em margin + 180 = 200pt.
    assert_eq!(doc.pages.len(), 1);
    let second_item_x = doc.pages[0].items
        .iter()
        .filter(|(p, _)| p.x.0 > 150.0) // filtrar o segundo item
        .map(|(p, _)| p.x.0)
        .next();
    assert!(
        second_item_x.map(|x| (x - 200.0).abs() < 2.0).unwrap_or(false),
        "Segundo item do grid deve estar em x ≈ 200pt"
    );
}
```

### Teste L3 — PDF tem MediaBox correcto por página

```rust
#[test]
fn pdf_mediabox_diferente_por_pagina() {
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        A\n\
        #set page(width: 200pt, height: 600pt)\n\
        B\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);
    let pdf    = export_pdf(&doc);

    let pdf_str = String::from_utf8_lossy(&pdf);
    assert!(pdf_str.contains("[0 0 595.28 841.89]"),
        "Primeira página deve ter MediaBox A4");
    assert!(pdf_str.contains("[0 0 200.00 600.00]"),
        "Segunda página deve ter MediaBox 200×600pt");
}
```

---

## Verificação final

```bash
# Confirmar que não resta nenhuma constante hardcoded de dimensão de página
grep -rn "841\|595\|MARGIN\b\|page_height\b" \
  01_core/src/ 03_infra/src/ | grep -v "test\|//\|\.md"

cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] `PageConfig` definido com `Default` (A4, ~2.5cm).
- [ ] `Page` tem campos `width` e `height` capturados como snapshot.
- [ ] `Content::SetPage` adicionado. Todos os `match` actualizados.
- [ ] `native_page` implementada e registada.
- [ ] `Layouter` tem `page_config: PageConfig`. Campos hardcoded removidos.
- [ ] `available_width()` e `available_height()` existem como métodos.
  Nenhum cálculo de margem inline fora deles.
- [ ] `current_page_is_empty()` usa `current_items.is_empty()`.
- [ ] `new_page()` captura snapshot de `page_config.width`/`height` via
  `std::mem::take`.
- [ ] `Content::SetPage` aplica quebra apenas quando `!current_page_is_empty()`.
- [ ] `flush_line()` é chamado antes de `new_page()` em `SetPage`.
- [ ] Exportador usa `page.height` por iteração de página — sem `page_height`
  global.
- [ ] MediaBox do PDF usa as dimensões do snapshot de cada página.
- [ ] `measure_content_constrained` recebe `max_width` como parâmetro e
  não calcula internamente — herda automaticamente o novo `available_width()`.
- [ ] DEBT-35b registado.
- [ ] `grep` de verificação retorna vazio para constantes hardcoded.
- [ ] Todos os cinco testes passam.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Quantos locais tinham `page_height` ou `MARGIN` hardcoded — indica a
  dimensão da refactorização.
- Se `Page` já tinha `width`/`height` ou se foram adicionados agora.

**Da implementação:**
- Se `measure_content_constrained` precisou de algum ajuste após este passo
  (ou se a nota da Tarefa 5 confirmou que não era necessário).
- Número total de testes após o passo e zero violations confirmados.

**Go/No-Go para o Passo 82:**
- **GO — páginas com dimensões distintas no PDF:** documento com A4 seguida
  de página 200×200pt produz dois MediaBox diferentes num leitor PDF.
- **NO-GO — snapshot errado:** se `doc.pages[1].height` retorna 841.89 em
  vez de 200.0, `new_page()` não está a capturar o snapshot antes de
  `page_config` ser alterado. Verificar a ordem das operações em
  `Content::SetPage`: snapshot acontece em `new_page()`, depois
  `page_config = new_config`.
- **NO-GO — página extra no início:** se `set_page_no_topo_nao_quebra`
  falha com `pages.len() == 2`, `current_page_is_empty()` está a retornar
  `false` mesmo sem itens. Verificar que usa `current_items.is_empty()` e
  não `cursor_y != margin`.
