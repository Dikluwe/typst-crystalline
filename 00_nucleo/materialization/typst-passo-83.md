# Passo 83 — Grid: Linhas Explícitas (`rows`) e Alinhamento Vertical de Células

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/content.rs` — Onde `Content::Grid` será estendido com
  o campo `rows`.
- `01_core/src/entities/layout_types.rs` — Onde `TrackSizing` já existe para
  o eixo X. Confirmar que pode ser reaproveitada no eixo Y sem alterações.
- `01_core/src/rules/stdlib.rs` — Onde `native_grid` extrai `columns` e onde
  será replicada a mesma lógica para `rows`.
- `01_core/src/rules/layout/mod.rs` (ou ficheiro anexo do Grid) — Onde o
  motor de layout itera sobre os items e calcula X/Y das células. Ponto de
  inserção das três passagens de altura (Fixed/Auto/Fraction).
- `01_core/DEBT.md` — DEBT-34b e DEBT-34c serão encerrados.

Pré-condição: `cargo test` — 898 testes (732 L1 + 166 L3, 1 ignorado
pré-existente), zero violations. Passo 82 concluído. `resolve_alignment`
existe e aceita `available_w`/`available_h` como parâmetros — isto é
condição necessária para a Tarefa 4.

---

## Contexto

O Grid do Passo 80 aceita apenas `columns`. As linhas são geradas
implicitamente com altura `Auto` — cada linha ocupa a altura do item mais
alto que contém. Não existe forma de declarar uma linha com altura fixa,
fraccional, ou de alinhar verticalmente um item dentro da sua célula.

Este passo adiciona dois mecanismos complementares:

**Parâmetro `rows`** — vector de `TrackSizing` aplicado ao eixo vertical
com o mesmo algoritmo das colunas: três passagens (Fixed → Auto → Fraction)
e indexação cíclica (`N % rows.len()`). Permite declarar
`grid(columns: 2, rows: (50pt, auto, 1fr))` e obter linhas com alturas
determinadas.

**Alinhamento vertical de célula** — com `cell_height` conhecido, um
`Content::Align` dentro de uma célula passa a receber a altura da célula
como `available_h` em `resolve_alignment`. `VAlign::Bottom` ancora ao
limite inferior da célula, `VAlign::Horizon` centra verticalmente.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Localizar a extracção de columns em native_grid.
# A lógica para rows será idêntica — replicar o padrão confirmado.
grep -B 2 -A 20 "fn native_grid" 01_core/src/rules/stdlib.rs | head -40

# 2. Localizar o bloco de resolução do Grid no Layouter.
# Identificar onde os items são iterados, onde cell_x é calculado,
# e onde cursor_y avança entre linhas.
grep -n "Content::Grid\|ShapeKind::Grid\|resolve_grid\|layout_grid" \
  01_core/src/rules/layout/mod.rs 01_core/src/rules/layout/*.rs

# 3. Confirmar que TrackSizing é genérica o suficiente para o eixo Y.
# As variantes Fixed(f64), Auto, Fraction(f64) não devem ter nomes
# ou documentação que as amarrem a "coluna" ou "largura".
grep -B 1 -A 10 "enum TrackSizing" 01_core/src/entities/layout_types.rs

# 4. Confirmar como layout_sub_frame_with_width mede altura intrínseca.
# A passagem Auto precisa de medir cada item sem o comprometer ao frame final.
grep -A 5 "fn layout_sub_frame_with_width" 01_core/src/rules/layout/mod.rs
```

Reportar o output completo antes de continuar. O diagnóstico 2 é um gate
obrigatório para a Tarefa 3 — não avançar sem identificar o ponto exacto
onde as linhas são iteradas.

**Se a iteração de items do Grid ocupa um ficheiro dedicado** (ex:
`layout/grid.rs`): as Tarefas 3 e 4 concentram-se nesse ficheiro.

**Se a iteração está inline em `layout_content`**: considerar extrair para
um método privado `layout_grid` antes de adicionar a lógica de `rows` —
a complexidade do bloco triplica com as três passagens.

---

## Tarefa 0 — Actualizar DEBT.md

Mover para a secção de encerrados:

```markdown
### DEBT-34b — Parâmetro `rows` em `Content::Grid` ignorado — ENCERRADO (Passo 83) ✓
`Content::Grid` passa a aceitar `rows: Vec<TrackSizing>`. Algoritmo de três
passagens (Fixed → Auto → Fraction) espelha a resolução de colunas, com
indexação cíclica `N % rows.len()` e recálculo de `fr` após quebra de página.

### DEBT-34c — Alinhamento vertical de células no Grid — ENCERRADO (Passo 83) ✓
`cell_height` é agora passado como `available_h` a `resolve_alignment` para
items dentro de células. `VAlign::Bottom` e `VAlign::Horizon` ancoram ao
limite inferior e ao centro da célula, respectivamente.
```

Adicionar à secção de abertos:

```markdown
### DEBT-38 — Cache de sub-frames no Grid Auto — EM ABERTO (Passo 83)
A resolução de altura de linhas Auto chama `layout_sub_frame_with_width`
para medir a altura intrínseca de cada item, descartando os FrameItems
produzidos. Quando a célula é emitida no documento, a mesma função é
chamada de novo para o mesmo item com a mesma largura, duplicando o
trabalho de layout em todas as células Auto.
Resolução: cache de `(Content*, width) → (height, Vec<FrameItem>)` válido
dentro da resolução de um Grid. Reutilizar o resultado da medição na
emissão.
```

---

## Tarefa 1 — Campo `rows` em `Content::Grid` (L1)

Em `01_core/src/entities/content.rs`:

```rust
// Na enum Content, variante Grid:

/// Grelha bidimensional com colunas e linhas explícitas.
///
/// Ambos os eixos aceitam Vec<TrackSizing>. Itens são distribuídos por
/// ordem, preenchendo linha a linha. Comprimento cíclico: se o número de
/// items excede columns.len() × rows.len(), o Grid gera mais linhas
/// repetindo o vector rows com indexação modular.
Grid {
    columns: Vec<TrackSizing>,
    rows:    Vec<TrackSizing>,
    items:   Vec<Content>,
},
```

Actualizar todos os `match` exaustivos sobre `Content::Grid` no compilador
— introspecção, show rules, `map_content`, avaliador, layouter. O pattern
matching exige o novo campo explicitamente.

**Construção a partir da stdlib:** quando `rows` não é fornecido (ou é
vazio), usar `vec![TrackSizing::Auto]` como valor por omissão. O layout
repete este valor para todas as linhas geradas — comportamento idêntico ao
Passo 80 quando o Grid tinha apenas `columns`.

---

## Tarefa 2 — Extracção de `rows` em `native_grid` (L1)

Em `01_core/src/rules/stdlib.rs`, dentro de `native_grid`:

```rust
// Após a extracção de columns (já existente — não alterar esse bloco).
// Adaptar a extracção ao padrão confirmado pelo diagnóstico 1.

let rows = args.named::<Value>("rows")
    .and_then(|v| cast_track_sizings(&v))
    .unwrap_or_else(|| vec![TrackSizing::Auto]);

// cast_track_sizings é o helper já usado para columns.
// Se for inline em vez de helper: extrair para função partilhada agora,
// antes de duplicar a lógica de conversão Int→Fixed, Length→Fixed,
// Fraction→Fraction, "auto"→Auto.
```

Se `rows` for um inteiro (ex: `grid(rows: 3)`), converter para
`vec![TrackSizing::Auto; 3]` — consistente com o comportamento de `columns`
quando recebe um inteiro.

Se `rows` for `()` (tuplo vazio) ou omitido, aplicar `vec![TrackSizing::Auto]`.

**Nota sobre ordem:** `rows` é extraído depois de `columns` e antes de
`items`. Isto não altera a semântica — os argumentos são nomeados — mas
facilita a leitura do código.

---

## Tarefa 3 — Motor de layout: cálculo de altura das linhas (L1)

Em `01_core/src/rules/layout/mod.rs` (ou no ficheiro dedicado ao Grid,
conforme diagnóstico 2):

### Guarda contra `rows` vazio

Antes de qualquer cálculo que envolva `rows.len()` no motor de layout,
aplicar uma guarda de segurança:

```rust
// A stdlib garante que rows nunca chega vazio — native_grid aplica o default
// vec![TrackSizing::Auto] quando o argumento é omitido (Tarefa 2).
// Esta guarda protege contra construção manual do AST em testes que
// ignoram a stdlib — rows.len() == 0 causa panic por divisão por zero
// em `N % rows.len()`.
let rows: &[TrackSizing] = if rows.is_empty() {
    &[TrackSizing::Auto]
} else {
    rows.as_slice()
};
```

Aplicar a mesma guarda a `columns` se ainda não existir — o vector de
colunas vazio tem o mesmo problema.

### Matriz de items

```rust
// Dividir o vector unidimensional de items em blocos (linhas) do tamanho
// do número de colunas. O último bloco pode estar incompleto — preencher
// com placeholder vazio ou tratar o caso na iteração.
let num_cols = columns.len();
let num_rows_produced = (items.len() + num_cols - 1) / num_cols;
let rows_of_items: Vec<&[Content]> = items
    .chunks(num_cols)
    .collect();
```

### Resolução de altura por linha — três passagens

```rust
// Fase 1 — Fixed e Auto numa única travessia.
// Fraction precisa do total de espaço consumido, logo é calculada depois.
let mut row_heights: Vec<f64> = Vec::with_capacity(num_rows_produced);
let mut total_fixed_and_auto: f64 = 0.0;
let mut fraction_indices: Vec<(usize, f64)> = Vec::new(); // (row_idx, fr_value)

for (row_idx, row_items) in rows_of_items.iter().enumerate() {
    // Indexação cíclica idêntica às colunas.
    let track = &rows[row_idx % rows.len()];

    let height = match track {
        TrackSizing::Fixed(pt) => {
            let h = *pt;
            total_fixed_and_auto += h;
            h
        },
        TrackSizing::Auto => {
            // Altura intrínseca da linha = máximo das alturas intrínsecas
            // dos items contidos. Medir cada um via layout_sub_frame_with_width
            // com a cell_width da coluna correspondente.
            //
            // DEBT-38 (conhecida, aberta neste passo, ver Tarefa 0):
            // os sub_items medidos aqui são descartados. Quando chegar o
            // momento de emitir a célula no documento, layout_sub_frame_with_width
            // é chamada novamente para o mesmo item com a mesma largura,
            // produzindo os mesmos FrameItems uma segunda vez. Isto duplica o
            // custo de layout para toda célula Auto. Não resolver aqui — o
            // cache de sub-frames é trabalho do DEBT-38, não deste passo.
            let mut max_h: f64 = 0.0;
            for (col_idx, item) in row_items.iter().enumerate() {
                let cell_w = column_widths[col_idx];
                let (sub_h, _sub_items) = self.layout_sub_frame_with_width(
                    item,
                    cell_w,
                );
                if sub_h > max_h {
                    max_h = sub_h;
                }
            }
            total_fixed_and_auto += max_h;
            max_h
        },
        TrackSizing::Fraction(fr) => {
            // Reservar o slot — preencher depois de Fixed e Auto estarem resolvidas.
            fraction_indices.push((row_idx, *fr));
            0.0 // placeholder
        },
    };

    row_heights.push(height);
}

// Fase 1.5 — Decisão de paginação ANTES da fase 2.
//
// Crítica ao pseudo-código naïve: se emitirmos o Grid linha a linha e
// `new_page()` disparar no meio, as linhas ainda não emitidas mantêm
// as alturas `fr` calculadas com o `available_below` da página anterior.
// Isso produz linhas visualmente encolhidas na página 2.
//
// Solução: a decisão de paginação é tomada AQUI, com base em
// `cursor_y + total_fixed_and_auto`. A fase 2 corre UMA ÚNICA VEZ,
// depois de `cursor_y` estar estabilizado na página onde o Grid vai
// efectivamente assentar. Isto evita re-entrar na fase 2 durante a
// emissão e elimina o bug de alturas `fr` "fósseis" da página anterior.
//
// Nota: esta decisão cobre o caso geral "Grid não cabe onde começou".
// O caso específico "linha Fixed individual maior que o que resta na
// página, apesar do resto do Grid caber" é tratado na lógica de
// emissão mais abaixo (ver secção "Atenção — quebra de página com
// linhas Fixed"). As duas decisões não se sobrepõem: esta decide a
// página onde o Grid começa; a outra trata Fixed gigantes após o
// Grid ter começado.
if total_fixed_and_auto > f64::max(0.0, self.page_bottom_limit() - self.cursor_y.0) {
    // O somatório Fixed+Auto não cabe no espaço restante da página
    // actual. Quebrar para a página seguinte antes de calcular fr.
    //
    // Caso de excepção: se o somatório também não cabe numa página
    // vazia (página inteira menor que Fixed+Auto), não há new_page()
    // que resolva. Aplicar a decisão conservadora documentada abaixo
    // ("Atenção — quebra de página com linhas Fixed") — truncar
    // visualmente ou retornar erro. Não tentar new_page() em loop.
    let page_usable_height = self.page_config.height - 2.0 * self.page_config.margin;
    if total_fixed_and_auto <= page_usable_height {
        self.new_page();
    }
    // Se total_fixed_and_auto > page_usable_height, seguir com cursor_y
    // na página actual e aceitar o overflow — a emissão decide
    // truncagem linha a linha.
}

// Fase 2 — resolver Fraction.
// Agora `cursor_y` reflecte a página onde o Grid vai efectivamente
// assentar (após o new_page() eventual da fase 1.5). Calcular `fr`
// com o `available_below` correcto.
if !fraction_indices.is_empty() {
    let grid_top_y = self.cursor_y.0;
    let available_below = f64::max(0.0, self.page_bottom_limit() - grid_top_y);

    if total_fixed_and_auto > available_below {
        // Caso patológico residual: mesmo após fase 1.5, Fixed+Auto
        // excedem o disponível. Só chega aqui se Fixed+Auto >
        // page_usable_height (Grid maior que uma página inteira).
        // Atribuir 0pt aos fr — não há espaço para eles.
        for (row_idx, _fr) in &fraction_indices {
            row_heights[*row_idx] = 0.0;
        }
    } else {
        let remaining = available_below - total_fixed_and_auto;
        let total_fr: f64 = fraction_indices.iter().map(|(_, fr)| fr).sum();

        if total_fr > 0.0 {
            for (row_idx, fr) in &fraction_indices {
                row_heights[*row_idx] = remaining * (fr / total_fr);
            }
        }
        // Se total_fr == 0.0 os slots fr ficam com 0pt — caso patológico,
        // não deve ocorrer dado que Fraction(fr) é construído com fr > 0.
    }
}
```

### Atenção — quebra de página durante a emissão linha a linha

Esta secção trata decisões de paginação DURANTE a emissão, após a fase 1.5
ter decidido a página onde o Grid começa. São casos que a fase 1.5 não
cobre porque surgem a meio da emissão.

**Linha `Fixed(pt)` individual maior que o espaço restante, após o Grid
ter começado a emitir noutras linhas acima:** a fase 1.5 só garante que
o somatório Fixed+Auto cabe a partir do topo do Grid. Se uma linha Fixed
específica faz com que `cursor_y + row_height > page_bottom_limit()`
durante a emissão, opções:

1. **Conservador (recomendado para este passo):** chamar `new_page()` antes
   de emitir a linha. A linha começa no topo da página seguinte.
2. **Agressivo (fora de escopo):** partir a linha entre páginas. Deixar
   para passo futuro se surgir a necessidade — abrir DEBT se aplicável.

**Linha `Fixed(pt)` individual maior que uma página inteira** (maior que
`page_config.height - 2 × margin`): não cabe mesmo após `new_page()`.
Opções:

1. Truncar visualmente (o conteúdo é cortado no limite da página).
2. Retornar erro de layout.

Para este passo, escolher a opção conservadora em ambos os casos e
documentar a decisão.

---

## Tarefa 4 — Alinhamento vertical dentro da célula (L1)

Ao fazer o layout final de cada item dentro da sua célula, agora são
conhecidos `cell_width` (Passo 80) e `cell_height` (Tarefa 3).

No braço `Content::Align` do layouter (já existente, Passo 82), o cálculo
de `available_h` depende do contexto. Dentro de uma célula de Grid:

```rust
// No loop de emissão de células do Grid (Tarefa 3), antes de layoutar
// cada item, definir o contexto:

let prev_is_height_unconstrained = self.is_height_unconstrained;
let prev_cell_available_h = self.cell_available_h; // novo campo

// Dentro de uma célula com altura determinada (Fixed, Auto ou Fraction
// com valor positivo), a altura é conhecida — Align pode ancorar Bottom.
self.is_height_unconstrained = false;
self.cell_available_h = Some(cell_height);

// Layout do item (que pode ser Content::Align).
let (sub_h, sub_items) = self.layout_sub_frame_with_width(item, cell_w);

// Restaurar o contexto anterior.
self.is_height_unconstrained = prev_is_height_unconstrained;
self.cell_available_h = prev_cell_available_h;
```

### Adaptação do braço `Content::Align`

No braço `Content::Align` (Passo 82), consultar `cell_available_h`:

```rust
let remaining_h = if self.is_height_unconstrained {
    sub_frame.height.0
} else if let Some(cell_h) = self.cell_available_h {
    // Dentro de uma célula de grid: available_h é a altura da célula.
    cell_h
} else {
    // No fluxo normal da página.
    f64::max(0.0, self.page_bottom_limit() - self.cursor_y.0)
};
```

### Mecânica de compensação de Y

**Confirmado pelo diagnóstico do Passo 82:** `layout_sub_frame_with_width`
retorna items ancorados em `(cell_x, ascender_local)`, não em `(0, 0)`. O
código de transferência de items deve compensar `ascender_local` via
`sub_origin_y` — o padrão já está estabelecido no Passo 82.

Quando `VAlign::Bottom` ancora ao limite inferior da célula:

```rust
// target_y vindo de resolve_alignment já considera (cell_height - content_h).
// A compensação de ascender_local aplica-se da mesma forma que no Passo 82.

for (item_pos, item) in sub_frame.items {
    self.push_frame_item(
        Point {
            x: Pt(target_x + item_pos.x.0 - sub_origin_x),
            y: Pt(target_y + item_pos.y.0 - sub_origin_y),
        },
        item,
    );
}
```

Se o iterador já retorna X compensado (como no Passo 82), o Y segue o
mesmo padrão — subtrair `sub_origin_y` para que `VAlign::Bottom` ancore no
limite inferior da `cell_height` em vez de acumular o ascender.

---

## Tarefa 5 — Testes

### Nota antes de escrever os testes — `VAlign` dentro de linha `Auto`

Armadilha a evitar ao desenhar variantes dos testes abaixo:

Se a linha é `Auto` e a célula contém apenas um item com
`align("bottom", ...)`, visualmente parece que o alinhamento não funciona.
Isto não é um bug — é consequência da regra: `Auto` faz a linha encolher
para abraçar a altura exacta do item. Se o item tem 20pt, a célula tem
20pt. Não existe "espaço vazio" onde o item possa descer.

Para validar `VAlign::Bottom` ou `VAlign::Horizon` correctamente, usar
uma das duas abordagens:

1. **Linha `Fixed` com altura superior à do item** (abordagem dos testes
   abaixo). `rows: (100pt)` com um rect de 20pt cria 80pt de espaço
   vazio onde o alinhamento é observável.
2. **Linha `Auto` com um item grande ao lado que estica a altura da
   linha**. Num grid de 2 colunas, se a primeira célula tem um rect de
   100pt e a segunda tem `align("bottom", rect(height: 20pt))`, a linha
   inteira fica com 100pt e o alinhamento na segunda célula é visível.

### Teste L1 — Grid com linhas fixas

```rust
#[test]
fn grid_rows_fixed_coordenadas_y_correctas() {
    // grid(columns: 1, rows: (50pt, 100pt)) com 3 items rectangulares.
    // Linha 0: 50pt (índice 0 do vector rows).
    // Linha 1: 100pt (índice 1).
    // Linha 2: 50pt (índice 2 % 2 = 0 — ciclo).
    //
    // Se o grid começa em y0 = margin = 20pt:
    // - Item 0 (linha 0) em y = 20pt.
    // - Item 1 (linha 1) em y = 20 + 50 = 70pt.
    // - Item 2 (linha 2) em y = 70 + 100 = 170pt.
    // (O deslocamento entre item N e item N+1 é a altura da linha N,
    // não a altura da linha N+1.)
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 400pt, height: 400pt, margin: 20pt)\n\
        #grid(columns: 1, rows: (50pt, 100pt),\n\
          rect(width: 100pt, height: 10pt),\n\
          rect(width: 100pt, height: 10pt),\n\
          rect(width: 100pt, height: 10pt),\n\
        )\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    let items = &doc.pages[0].items;
    assert_eq!(items.len(), 3, "Deve haver 3 FrameItems (um por célula)");

    let y0 = frame_item_pos(&items[0]).y.0;
    let y1 = frame_item_pos(&items[1]).y.0;
    let y2 = frame_item_pos(&items[2]).y.0;

    assert!(
        (y1 - (y0 + 50.0)).abs() < 0.5,
        "Item 1 deve estar em y = y0 + 50 (altura da linha 0), obteve y={:.1} (y0={:.1})",
        y1, y0
    );
    assert!(
        (y2 - (y1 + 100.0)).abs() < 0.5,
        "Item 2 deve estar em y = y1 + 100 (altura da linha 1), obteve y={:.1} (y1={:.1})",
        y2, y1
    );
}
```

### Teste L1 — Alinhamento vertical em linha fixa

```rust
#[test]
fn grid_valign_bottom_ancora_ao_limite_inferior_da_celula() {
    // grid(columns: 1, rows: (100pt)) com #align("bottom", rect(height: 20pt)).
    // Altura da célula: 100pt. Altura do conteúdo: 20pt.
    // VAlign::Bottom ancora em y = cell_top + (cell_height - content_h)
    //                           = cell_top + 80.
    // Se cell_top = margin = 20pt, o rect deve estar em y = 100pt.
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 400pt, height: 400pt, margin: 20pt)\n\
        #grid(columns: 1, rows: (100pt),\n\
          align(\"bottom\", rect(width: 80pt, height: 20pt)),\n\
        )\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    let items = &doc.pages[0].items;
    assert!(!items.is_empty(), "Deve haver pelo menos um item");

    let rect_y = frame_item_pos(&items[0]).y.0;
    assert!(
        (rect_y - 100.0).abs() < 0.5,
        "Rect com valign bottom deve estar em y=100pt (cell_top 20 + 80 offset), obteve y={:.1}",
        rect_y
    );
}
```

### Teste L1 — Interacção Auto vs Fraction (opcional)

```rust
#[test]
fn grid_rows_auto_e_fraction_coexistem() {
    // grid(columns: 1, rows: (auto, 1fr)) — a linha auto mede o conteúdo,
    // a linha 1fr consome todo o espaço vertical restante da página.
    //
    // Página 400pt, margin 20pt → available_height = 360pt.
    // Linha 0 (auto): rect de 40pt → altura da linha = 40pt.
    // Linha 1 (1fr): 360 - 40 = 320pt.
    //
    // Item 0 em y = 20pt. Item 1 em y = 60pt com altura de célula 320pt.
    // Se o item 1 é um rect de 10pt com valign default (top), y = 60pt.
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 400pt, height: 400pt, margin: 20pt)\n\
        #grid(columns: 1, rows: (auto, 1fr),\n\
          rect(width: 100pt, height: 40pt),\n\
          rect(width: 100pt, height: 10pt),\n\
        )\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    let items = &doc.pages[0].items;
    assert_eq!(items.len(), 2);

    let y0 = frame_item_pos(&items[0]).y.0;
    let y1 = frame_item_pos(&items[1]).y.0;

    assert!(
        (y1 - (y0 + 40.0)).abs() < 0.5,
        "Item 1 deve estar em y = y0 + 40 (altura da linha auto), obteve y={:.1}",
        y1
    );
}
```

Este teste é marcado opcional no enunciado original. Se a implementação da
passagem Fraction levantar problemas de paginação não previstos, comentar
o teste e registar a causa antes de avançar para o Passo 84.

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint --fix-hashes .
crystalline-lint .
```

Critérios de conclusão:
- [ ] Campo `rows: Vec<TrackSizing>` adicionado a `Content::Grid`. Todos os
  `match` actualizados.
- [ ] `native_grid` extrai `rows` com a mesma lógica de conversão de
  `columns`. `rows` omitido ou vazio → `vec![TrackSizing::Auto]`.
- [ ] `rows` inteiro (ex: `rows: 3`) converte para `vec![Auto; 3]`.
- [ ] Guarda contra `rows.is_empty()` aplicada no início do motor de
  layout do Grid. Previne `panic` por divisão por zero em `N % rows.len()`
  quando o AST é construído manualmente em testes sem passar pela stdlib.
  Mesma guarda aplicada a `columns` se ainda não existir.
- [ ] Motor de layout calcula `row_heights` em três passagens:
  Fixed → Auto → Fraction. Auto usa `layout_sub_frame_with_width` para
  medir altura intrínseca.
- [ ] Indexação cíclica `N % rows.len()` aplicada — array de `rows`
  repete-se para linhas além do seu comprimento.
- [ ] Decisão de `new_page()` antes da fase 2 de Fraction: se
  `cursor_y + total_fixed_and_auto > page_bottom_limit()` e o somatório
  cabe numa página vazia, quebrar para a página seguinte ANTES de
  calcular `fr`. A fase 2 corre uma única vez, depois de `cursor_y`
  estar estabilizado.
- [ ] Fraction trata o caso residual `total_fixed_and_auto > available_below`
  que só pode ocorrer quando o Grid é maior que uma página inteira: em
  vez de distribuir espaço negativo, atribui `0pt` a todas as linhas
  `fr`. Nunca produzir alturas negativas por divisão de espaço inexistente.
- [ ] Caso patológico "Grid maior que página inteira"
  (`total_fixed_and_auto > page_usable_height`) não entra em loop de
  `new_page()` — aceita o overflow e deixa a lógica de emissão decidir
  truncagem linha a linha.
- [ ] Linha `Fixed` individual maior que o espaço restante na página
  (após o Grid já ter começado a emitir) desencadeia `new_page()` antes
  de emitir essa linha específica (opção conservadora).
- [ ] Campo `cell_available_h: Option<f64>` adicionado ao Layouter.
  Definido antes de layoutar cada célula, restaurado ao terminar.
- [ ] `Content::Align` dentro de célula usa `cell_available_h` como
  `available_h` em `resolve_alignment` — não `page_bottom_limit - cursor_y`.
- [ ] `VAlign::Bottom` e `VAlign::Horizon` dentro de célula ancoram ao
  limite inferior e ao centro da célula, respectivamente.
- [ ] Compensação de `sub_origin_y` aplicada no loop de transferência de
  items — `VAlign::Bottom` não acumula o ascender_local.
- [ ] DEBT-34b e DEBT-34c movidos para a secção de encerrados em
  `01_core/DEBT.md`.
- [ ] DEBT-38 (cache de sub-frames no Grid Auto) aberta em
  `01_core/DEBT.md`. Resolução adiada — não implementar cache neste passo.
- [ ] Teste `grid_rows_fixed_coordenadas_y_correctas` passa.
- [ ] Teste `grid_valign_bottom_ancora_ao_limite_inferior_da_celula` passa.
- [ ] Teste `grid_rows_auto_e_fraction_coexistem` passa (ou está comentado
  com justificação).
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Se `cast_track_sizings` (ou equivalente) existe como helper partilhado ou
  se a lógica de conversão está inline em `native_grid` — afecta a
  reutilização na extracção de `rows`.
- Se a iteração de items do Grid está num ficheiro dedicado
  (`layout/grid.rs`) ou inline em `layout_content` — afecta onde as três
  passagens de altura são implementadas.
- Se `TrackSizing` teve de ser renomeada ou generalizada por ter
  documentação amarrada a "coluna" — afecta a clareza semântica no
  eixo Y.

**Da implementação:**
- Se o comportamento de paginação precisou de ser alterado para lidar com
  células `Fixed` maiores que a página. Qual a opção escolhida (truncar,
  erro, ou deixar o conteúdo escorrer).
- Se a passagem Fraction precisou de recalcular após quebra de página ou
  se o Grid cabe sempre na página onde começa nos casos testados.
- Se a compensação de `sub_origin_y` dentro da célula produziu o Y correcto
  na primeira tentativa ou se houve acumulação de ascender_local (sintoma:
  rect com `valign bottom` aparece uns pontos acima do limite esperado).
- Número total de testes após o passo e zero violations confirmados.

**As coordenadas Y exactas validadas nos três testes:**
- Teste 1 (`rows` fixas): `y0`, `y1`, `y2` com os deslocamentos 50pt e 100pt.
- Teste 2 (`valign bottom`): `rect_y` em 100pt.
- Teste 3 (`auto + 1fr`): `y0` e `y1` com o deslocamento de 40pt.

**Go/No-Go para o Passo 84:**
- **GO — linhas e alinhamento vertical correctos:** os três testes passam
  com as coordenadas Y esperadas, Grid com `rows: (50pt, 100pt, 1fr)`
  renderiza visualmente como pretendido num leitor PDF real.
- **NO-GO — `valign bottom` ignora `cell_height`:** se o teste 2 coloca o
  rect em y=20 (topo da célula) em vez de y=100 (fundo), o braço
  `Content::Align` ainda está a ler `page_bottom_limit - cursor_y` em vez
  de `cell_available_h`. Confirmar que `cell_available_h` é `Some(100.0)`
  no momento do layout do Align.
- **NO-GO — `panic` por `rows` vazio em teste manual:** se um teste que
  constrói `Content::Grid { rows: vec![], .. }` manualmente (sem passar
  pela stdlib) faz `panic` com "attempt to calculate the remainder with
  a divisor of zero", a guarda no início do motor de layout não foi
  aplicada ou aplica-se só parcialmente. Confirmar que a guarda está
  antes da primeira ocorrência de `rows.len()`.
- **NO-GO — Fraction produz altura negativa:** se um `1fr` após `Fixed`
  que excede a página produz altura negativa ou positiva em vez de 0pt,
  o ramo `total_fixed_and_auto > available_below` da fase 2 não está a
  entrar. Verificar com um teste onde Fixed soma 500pt numa página de
  400pt — a linha `fr` deve ficar com 0pt, não com `(400 - 500) × fr`.
- **NO-GO — `fr` fóssil na segunda página:** se um Grid com
  `rows: (200pt, 1fr, 1fr)` numa página de 380pt útil mostra as linhas
  `fr` na segunda página com alturas calculadas para a primeira (ex:
  90pt cada em vez de 80pt cada após o avanço do cursor), a fase 1.5
  não está a correr antes da fase 2 — o `new_page()` está a disparar
  durante a emissão e as alturas `fr` ficam congeladas com o
  `available_below` antigo. Confirmar que a decisão de paginação
  precede o cálculo de `fr`.
- **NO-GO — linha Fixed gigante trava o layout:** se `rows: (500pt)` numa
  página de 400pt trava ou produz loop infinito, a decisão conservadora
  (truncar ou erro) não foi aplicada.
