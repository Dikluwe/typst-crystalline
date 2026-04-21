# Passo 80 — Layout de Grid com Fracções (DEBT-34)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/value.rs` — Confirmar se `Value::Fraction` já existe
  ou se é novo.
- `01_core/src/entities/layout_types.rs` — Onde `TrackSizing` será adicionado.
- `01_core/src/entities/content.rs` — Onde `Content::Grid` será adicionado.
- `01_core/src/rules/layout/mod.rs` — Onde o algoritmo de resolução de
  colunas vive. Confirmar se `measure_content` já existe e com que assinatura.
- `01_core/DEBT.md` — Confirmar que DEBT-30 está encerrado e DEBT-33 em aberto.

Pré-condição: `cargo test` — ~721 L1 + ~147 L3, zero violations. Passo 79
concluído. Clipping paths, polígonos e caminhos livres funcionam.

---

## Contexto

O motor de layout conhece rectângulos, imagens, texto, figuras numeradas e
transformações afins. Falta o bloco estrutural mais comum em documentos reais:
o grid de colunas. Este passo implementa `#grid()` com três tipos de
dimensionamento por coluna:

- **Fixed** (`50pt`): largura absoluta, inquestionável.
- **Auto**: ajusta-se ao conteúdo mais largo da coluna, limitado por
  `safe_available` para não destruir o espaço das fracções.
- **Fraction** (`1fr`, `2fr`): divide o espaço restante após Fixed e Auto.

A política de prioridade é Fixed → Auto → Fraction. Um Auto guloso pode
esmagar fr — isto é documentado como DEBT-34d, não escondido.

Quatro débitos são registados para funcionalidades fora do escopo:

- **DEBT-34b** — parâmetro `rows` ignorado (todas as linhas são Auto).
- **DEBT-34c** — alinhamento vertical de células (baseline, center, bottom).
- **DEBT-34d** — Auto não encolhe para min-content antes de matar fr.
- **DEBT-34e** — `colspan`/`rowspan` (células que ocupam múltiplas colunas/linhas).

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar se Value::Fraction já existe
grep -n "Fraction\|fraction" 01_core/src/entities/value.rs | head -5

# 2. Confirmar a assinatura de measure_content (se existir)
grep -n "fn measure_content\|Metrics\|struct Metrics" \
  01_core/src/rules/layout/mod.rs | head -10

# 3. Confirmar se Value::Array existe e como é iterado
grep -n "Array\|Vec<Value>" 01_core/src/entities/value.rs | head -5

# 4. Confirmar como page_width e margin estão acessíveis no layouter
grep -n "page_width\|available_width\|margin" \
  01_core/src/rules/layout/mod.rs | head -10
```

Reportar o output completo antes de continuar. O diagnóstico 2 é crítico:
se `measure_content` não existir ou não aceitar restrição de largura,
`measure_content_constrained` tem de ser implementado antes do algoritmo
de grid — pode ser um pré-requisito dentro desta tarefa ou adicionado como
parte da Tarefa 5.

---

## Tarefa 0 — Actualizar DEBT.md

Registar os quatro débitos novos antes de qualquer código:

```markdown
### DEBT-34b — Parâmetro rows em Content::Grid ignorado — EM ABERTO (Passo 80)
O campo rows de Content::Grid é armazenado no AST mas ignorado pelo layouter.
Todas as linhas operam como Auto. Resolução: passo futuro de layout de grid
com rows explícitos.

### DEBT-34c — Alinhamento vertical de células no Grid — EM ABERTO (Passo 80)
As células assumem top-alignment. Alinhamento vertical (baseline, center, bottom)
requer calcular a altura máxima da linha antes de posicionar os itens.
Resolução: passo futuro.

### DEBT-34d — Auto não encolhe antes de matar fr — EM ABERTO (Passo 80)
Um Auto guloso (célula com texto longo) pode consumir todo o safe_available,
deixando 0pt para as colunas fr. Resolução futura: implementar min-content e
max-content para Auto, com negociação entre Auto e fr.

### DEBT-34e — colspan e rowspan — EM ABERTO (Passo 80)
Células que ocupam múltiplas colunas ou linhas requerem um algoritmo de
placement diferente. Resolução: passo futuro.
```

---

## Tarefa 1 — `Value::Fraction` (L1)

Em `01_core/src/entities/value.rs`, adicionar a variante se ainda não existir
(confirmar com diagnóstico 1):

```rust
// Na enum Value:
/// Fracção para dimensionamento relativo (ex: 1fr, 2.5fr).
Fraction(f64),
```

Garantir que o parser/lexer já produz `Value::Fraction(1.0)` a partir do
literal `1fr`. Se o parser não suporta ainda `fr`, adicionar o token
ao lexer neste passo — é pré-requisito para `#grid(columns: (1fr, 2fr))`.

---

## Tarefa 2 — `TrackSizing` e `Content::Grid` (L1)

### 2a — `TrackSizing` em `layout_types.rs`

```rust
/// Dimensionamento de uma coluna ou linha de grid.
#[derive(Debug, Clone, PartialEq)]
pub enum TrackSizing {
    /// Largura absoluta em pontos.
    Fixed(f64),
    /// Ajusta-se ao conteúdo mais largo da coluna, limitado por safe_available.
    Auto,
    /// Fracção do espaço restante após Fixed e Auto.
    /// Pode receber 0pt se Fixed + Auto esgotarem o espaço disponível (DEBT-34d).
    Fraction(f64),
}
```

### 2b — `Content::Grid` em `content.rs`

```rust
// Na enum Content:
/// Grid de colunas com células posicionadas por ordem de leitura (left-to-right,
/// top-to-bottom). Cada célula ocupa exactamente uma coluna.
///
/// `rows` é armazenado no AST mas ignorado pelo layouter neste passo (DEBT-34b).
Grid {
    columns: Vec<TrackSizing>,
    rows:    Vec<TrackSizing>, // DEBT-34b: ignorado — todas as linhas são Auto
    cells:   Vec<Content>,
},
```

Actualizar todos os `match` sobre `Content` — adicionar
`Content::Grid { .. } => {}` nos braços que não tratam grids (introspecção,
show rules, map_content, etc.). O compilador lista os locais.

---

## Tarefa 3 — `native_grid` na stdlib (L1)

Em `01_core/src/rules/stdlib.rs`:

```rust
/// Converte um Value em TrackSizing.
fn parse_track_sizing(val: &Value) -> Option<TrackSizing> {
    match val {
        Value::Float(f)    => Some(TrackSizing::Fixed(*f)),
        Value::Length(f)   => Some(TrackSizing::Fixed(*f)),
        Value::Fraction(fr) => Some(TrackSizing::Fraction(*fr)),
        Value::String(s) if s == "auto" => Some(TrackSizing::Auto),
        _ => None,
    }
}

/// Extrai um Vec<TrackSizing> de um argumento que pode ser um único valor
/// ou um array de valores.
fn extract_tracks(val: Option<&Value>) -> Vec<TrackSizing> {
    match val {
        Some(Value::Array(arr)) => arr.iter()
            .filter_map(parse_track_sizing)
            .collect(),
        Some(v) => parse_track_sizing(v).into_iter().collect(),
        None    => vec![],
    }
}

pub fn native_grid(_ctx: &mut EvalContext, args: &Args) -> Result<Value, String> {
    let columns = extract_tracks(args.named::<Value>("columns").as_ref());
    let rows    = extract_tracks(args.named::<Value>("rows").as_ref());

    let cells: Vec<Content> = args.positional_items().iter()
        .filter_map(|v| v.cast_content())
        .collect();

    Ok(Value::Content(Content::Grid { columns, rows, cells }))
}
```

Registar:

```rust
ctx.register("grid", native_grid);
```

---

## Tarefa 4 — `measure_content_constrained` (L1)

Se `measure_content` já existe mas sem restrição de largura, adicionar a
variante com constraint. Esta função é pré-requisito para a Fase 1 do
algoritmo de grid:

```rust
impl Layouter {
    /// Mede conteúdo impondo uma largura máxima para quebra de linha.
    ///
    /// Para texto: simula quebras de linha quando a acumulação de glyphs
    /// excede max_width.
    /// Para blocos: propaga o constraint recursivamente.
    /// Para elementos com dimensão intrínseca (rect, imagem): retorna o
    /// tamanho intrínseco — o layout da célula forçará clip/overflow se
    /// necessário.
    fn measure_content_constrained(&self, content: &Content, max_width: f64) -> Metrics {
        match content {
            Content::Text(text) => {
                let mut max_line_w   = 0.0_f64;
                let mut current_w    = 0.0_f64;
                let mut line_count   = 1usize;

                for word in text.split_whitespace() {
                    let word_w = self.measure_word(word);

                    if current_w + word_w > max_width && current_w > 0.0 {
                        // Quebra de linha — actualizar máximo e resetar acumulador.
                        max_line_w = max_line_w.max(current_w);
                        line_count += 1;
                        current_w  = word_w;
                    } else {
                        current_w += word_w;
                    }
                }
                max_line_w = max_line_w.max(current_w);

                Metrics {
                    width:  Pt(max_line_w),
                    height: Pt(self.line_height.0 * line_count as f64),
                }
            },

            // Blocos: propagar constraint aos filhos, somar alturas.
            Content::Sequence(children) => {
                let mut total_h = 0.0_f64;
                let mut max_w   = 0.0_f64;
                for child in children {
                    let m = self.measure_content_constrained(child, max_width);
                    total_h += m.height.0;
                    max_w    = max_w.max(m.width.0);
                }
                Metrics { width: Pt(max_w), height: Pt(total_h) }
            },

            // Elementos com dimensão intrínseca — não quebram.
            other => {
                let m = self.measure_content(other);
                // Clampar à max_width para garantir que o valor não excede
                // o espaço disponível reportado ao algoritmo de grid.
                Metrics {
                    width:  Pt(m.width.0.min(max_width)),
                    height: m.height,
                }
            },
        }
    }
}
```

---

## Tarefa 5 — Algoritmo de resolução de grid no layouter (L1)

Em `01_core/src/rules/layout/mod.rs`:

```rust
Content::Grid { columns, rows: _, cells } => {
    // rows ignorado — DEBT-34b.

    let available_width = self.page_width.0 - (self.margin.0 * 2.0);
    let start_x = self.cursor_x.0;

    // Normalizar: grid sem colunas definidas → uma coluna Auto.
    let cols = if columns.is_empty() {
        vec![TrackSizing::Auto]
    } else {
        columns.clone()
    };
    let num_cols = cols.len();

    // --- Pré-agrupamento O(N): associar células às colunas ---
    // Cada célula pertence à coluna (idx % num_cols).
    // Usado apenas para medir as colunas Auto — não afecta a ordem de layout.
    let mut cols_cells: Vec<Vec<&Content>> = vec![vec![]; num_cols];
    for (idx, cell) in cells.iter().enumerate() {
        cols_cells[idx % num_cols].push(cell);
    }

    // --- FASE 1: Resolver Fixed e Auto ---
    //
    // Política de prioridade: Fixed → Auto → Fraction.
    // safe_available = espaço ainda não consumido por Fixed ao chegar
    // a uma coluna Auto. Impede que Auto consuma tudo e deixe 0pt para fr.
    // DEBT-34d: Auto ainda pode esmagar fr se safe_available for esgotado.
    let mut resolved_widths  = vec![0.0_f64; num_cols];
    let mut total_fixed_width = 0.0_f64;
    let mut total_fr          = 0.0_f64;

    for (i, sizing) in cols.iter().enumerate() {
        match sizing {
            TrackSizing::Fixed(w) => {
                resolved_widths[i] = *w;
                total_fixed_width  += *w;
            },
            TrackSizing::Auto => {
                // Limite superior para Auto: espaço disponível menos o que
                // as colunas Fixed anteriores já consumiram.
                let safe_available = f64::max(0.0, available_width - total_fixed_width);

                let mut max_w = 0.0_f64;
                for cell in &cols_cells[i] {
                    let metrics = self.measure_content_constrained(cell, safe_available);
                    max_w = max_w.max(metrics.width.0);
                }
                resolved_widths[i] = max_w;
                total_fixed_width  += max_w;
            },
            TrackSizing::Fraction(_) => {
                // Acumular o total de fracções para a Fase 2.
                total_fr += match sizing {
                    TrackSizing::Fraction(fr) => fr,
                    _ => unreachable!(),
                };
            },
        }
    }

    // --- FASE 2: Distribuir Fracções ---
    //
    // remaining_width é o espaço depois de Fixed + Auto.
    // f64::max(0.0, ...) evita valores negativos se Fixed + Auto > available.
    let remaining_width = f64::max(0.0, available_width - total_fixed_width);

    if total_fr > 0.0 {
        let width_per_fr = remaining_width / total_fr;
        for (i, sizing) in cols.iter().enumerate() {
            if let TrackSizing::Fraction(fr) = sizing {
                resolved_widths[i] = fr * width_per_fr;
            }
        }
    }
    // Se total_fr == 0.0 e remaining_width > 0.0, o espaço sobrante não é
    // distribuído — comportamento consistente com CSS Grid sem fr tracks.

    // --- FASE 3: Layout das células ---
    self.flush_line();

    let mut current_col_idx   = 0usize;
    let mut current_row_max_h = 0.0_f64;

    for cell in cells {
        // Início de nova linha: resetar X e altura máxima da linha.
        if current_col_idx == 0 {
            current_row_max_h = 0.0;
            self.cursor_x = Pt(start_x);
        }

        let cell_width = resolved_widths[current_col_idx];

        // Guardar posição antes de layoutar a célula.
        let saved_x = self.cursor_x;
        let saved_y = self.cursor_y;

        // Layoutar a célula num frame isolado com largura forçada.
        // layout_sub_frame_with_width propaga cell_width como available_width.
        let cell_frame = self.layout_sub_frame_with_width(cell, cell_width);

        current_row_max_h = current_row_max_h.max(cell_frame.height.0);

        // Transferir os itens do frame da célula para o frame global,
        // ajustando as posições para o canto superior esquerdo da célula.
        for (item_pos, item) in cell_frame.items {
            let abs_pos = Point {
                x: Pt(saved_x.0 + item_pos.x.0),
                y: Pt(saved_y.0 + item_pos.y.0),
            };
            self.push_frame_item(abs_pos, item);
        }

        // Avançar cursor X para a próxima célula.
        self.cursor_x += Pt(cell_width);
        current_col_idx += 1;

        // Fim de linha: avançar Y e verificar quebra de página.
        if current_col_idx >= num_cols {
            current_col_idx = 0;
            self.cursor_y  += Pt(current_row_max_h);

            if self.cursor_y.0 > self.page_height.0 - self.margin.0 {
                self.new_page();
                self.cursor_y = self.margin;
            }
        }
    }

    // Células restantes numa linha incompleta: avançar Y pelo máximo da linha.
    if current_col_idx > 0 {
        self.cursor_y += Pt(current_row_max_h);
    }
},
```

---

## Tarefa 6 — Testes

### Teste L1 — distribuição de fracções com Auto cooperativo

```rust
#[test]
fn grid_fr_distribution_quando_auto_e_pequeno() {
    // Página 400pt, margens 25pt cada lado → available = 350pt.
    // columns: (50pt, auto, 1fr, 2fr)
    // Célula Auto: texto curto → mede ~40pt.
    // Remaining: 350 - 50 - 40 = 260pt; total_fr = 3.
    // Col 2 (1fr): 86.67pt; Col 3 (2fr): 173.33pt.
    // Soma: 50 + 40 + 86.67 + 173.33 = 350pt.
    //
    // Verificar: resolved_widths[2] ≈ 86.67 e resolved_widths[3] ≈ 173.33.
    // Adaptar à forma como o layouter expõe resolved_widths nos testes.
}
```

### Teste L1 — Auto guloso esmaga fr (comportamento documentado)

```rust
#[test]
fn grid_fr_recebe_zero_quando_auto_e_guloso() {
    // Regressão: Auto com parágrafo longo consome safe_available inteiro.
    // columns: (50pt, auto, 1fr)
    // Célula Auto: texto sem espaços → measure retorna safe_available (300pt).
    // Remaining: 350 - 50 - 300 = 0pt.
    // Col 2 (1fr): 0pt — ACEITÁVEL para Passo 80.
    //
    // Verificar: não entra em pânico. resolved_widths[2] == 0.0.
    // A célula fr é renderizada com largura zero (invisível mas válida).
}
```

### Teste L1 — altura da linha avança pelo máximo da coluna mais alta

```rust
#[test]
fn grid_altura_da_linha_e_o_maximo_das_celulas() {
    // columns: (100pt, 100pt)
    // Células: rect(height: 20pt), rect(height: 40pt), rect(height: 10pt)
    //
    // Linha 0: max(20, 40) = 40pt → cursor_y avança 40pt.
    // Linha 1: 10pt (célula única, linha incompleta) → cursor_y avança 10pt.
    // Y final: start_y + 40 + 10 = start_y + 50pt.
    let root = criar_dir_temporario();
    std::fs::write(
        root.join("main.typ"),
        "#grid(columns: (100pt, 100pt),\n\
          rect(width: 100pt, height: 20pt),\n\
          rect(width: 100pt, height: 40pt),\n\
          rect(width: 100pt, height: 10pt))",
    ).unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    // Verificar que o documento tem exactamente uma página (sem quebra de página).
    assert_eq!(doc.pages.len(), 1, "Grid simples deve caber numa página");

    // Verificar o número total de FrameItems (3 rectângulos).
    let total_items = doc.pages[0].items.len();
    assert_eq!(total_items, 3, "Deve haver 3 FrameItems no frame");
}
```

### Teste L1 — `safe_available` impede Auto de exceder a página

```rust
#[test]
fn grid_auto_respects_safe_available() {
    // Uma única coluna Auto com conteúdo muito largo não deve exceder
    // available_width. O resultado de measure_content_constrained deve
    // ser ≤ safe_available.
    //
    // Verificar: o FrameItem do texto não tem posição X além da margem direita.
    // (Implementar conforme o helper de extracção de posições disponível.)
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
- [ ] `Value::Fraction(f64)` adicionado (ou confirmado existente). Parser
  produz `Value::Fraction` a partir de `1fr`.
- [ ] `TrackSizing` definido em `layout_types.rs` com `Fixed`, `Auto`,
  `Fraction`.
- [ ] `Content::Grid` adicionado ao AST. Todos os `match` actualizados.
- [ ] `native_grid` e helpers `parse_track_sizing`/`extract_tracks` implementados
  e registados.
- [ ] `measure_content_constrained` implementado; quebra texto por palavras
  quando `current_w + word_w > max_width`.
- [ ] Fase 1: Fixed acumula em `total_fixed_width` antes de Auto ser medido.
  `safe_available = f64::max(0.0, available_width - total_fixed_width)`.
- [ ] Fase 2: `remaining_width = f64::max(0.0, available_width - total_fixed_width)`.
  Sem pânico quando `remaining_width == 0.0`.
- [ ] Fase 3: posições das células são absolutas (saved_x + item_pos.x).
  `current_row_max_h` avança apenas no fim da linha.
- [ ] Linha incompleta no fim do grid avança `cursor_y` por `current_row_max_h`.
- [ ] DEBT-34b, 34c, 34d, 34e registados em `01_core/DEBT.md`.
- [ ] Teste `grid_altura_da_linha_e_o_maximo_das_celulas` passa.
- [ ] Teste `grid_fr_recebe_zero_quando_auto_e_guloso` não entra em pânico.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `Value::Fraction` já existia ou foi adicionado agora.
- Se `measure_content` já existia com restrição de largura, ou se
  `measure_content_constrained` foi implementado de raiz.
- Como `layout_sub_frame_with_width` foi implementado — se reutiliza
  `layout_sub_frame` com um campo temporário de `available_width`, ou se
  é uma função nova.

**Da implementação:**
- Se o pré-agrupamento `cols_cells` causou algum problema de lifetime com
  referências a `cells` — e se foi necessário usar índices em vez de `&Content`.
- Número total de testes após o passo e zero violations confirmados.

**Go/No-Go para o Passo 81:**
- **GO — grid renderiza no PDF:** três colunas de larguras diferentes
  aparecem correctamente posicionadas num leitor PDF real.
- **NO-GO — posições absolutas erradas:** se as células aparecem todas no
  mesmo X, verificar que `saved_x` é capturado antes de `cursor_x` ser
  avançado e que a transferência usa `saved_x.0 + item_pos.x.0`.
- **NO-GO — fr recebe largura negativa:** se `resolved_widths` tem valores
  negativos, verificar que `remaining_width` usa `f64::max(0.0, ...)` e
  que `total_fixed_width` acumula correctamente Auto antes de chegar à
  Fase 2.
