# Passo 75 — Caminhos Relativos e Figuras Numeradas (DEBT-25, DEBT-14, DEBT-15)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — Onde `EvalContext` está definido.
  Confirmar os campos actuais e como o avaliador recebe o `FileId` ou path
  do ficheiro principal.
- `01_core/src/rules/stdlib.rs` — Onde `native_image` e `native_figure`
  estão definidas. Confirmar a assinatura actual de `ctx.world.file()`.
- `01_core/src/entities/content.rs` — Variante actual de `Content::Figure`.
  Confirmar se `kind` e `numbering` já existem.
- `01_core/src/rules/layout/figure.rs` — Como o layouter processa figuras.
- `01_core/src/entities/counter_state.rs` — Como `CounterState` regista e
  lê contadores. Confirmar o padrão estabelecido nos Passos 57–60.
- `01_core/DEBT.md` — Confirmar que DEBT-25, DEBT-14, DEBT-15 estão
  registados e DEBT-26/27/28/29 estão encerrados.

Pré-condição: `cargo test` — ~706 L1 + ~126 L3, zero violations.
DEBT-25 (caminhos relativos), DEBT-14 (numeração de figuras), DEBT-15
(`kind` em `Figure`) registados. Todos os DEBTs de imagem do Passo 74
encerrados.

---

## Contexto

O Passo 74 completou o suporte a imagens no PDF. Ficaram três débitos de
usabilidade directamente visíveis nos documentos:

- **DEBT-25 — Caminhos relativos:** `#image("foto.png")` num ficheiro em
  `capitulo1/intro.typ` resolve `foto.png` relativamente à raiz do projecto
  em vez de relativamente ao ficheiro fonte. Documentos com sub-pastas
  partem ao carregar imagens. A resolução passa o `FileId` do ficheiro
  em avaliação para `World::file`, que faz o `join` antes de ler o disco.

- **DEBT-15 — `kind` em `Content::Figure`:** figuras de tipos diferentes
  (imagens, tabelas, código) devem ter contadores independentes. O campo
  `kind: String` no AST é o discriminador. Sem ele, `#figure(table(...),
  kind: "table")` ignora o parâmetro e mistura a contagem com as imagens.

- **DEBT-14 — Numeração de figuras:** `#set figure(numbering: "1")` activa
  a numeração automática. O layouter precisa consultar `CounterState` para
  ler o valor actual do contador do `kind` correspondente e injectá-lo
  como prefixo da legenda antes de a dispor na página.

Os três débitos estão relacionados: DEBT-25 é independente, mas DEBT-14
depende de DEBT-15 (sem `kind` não há contadores separados por tipo).
A ordem de implementação é DEBT-25 → DEBT-15 → DEBT-14.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Confirmar a assinatura actual de World::file e EvalContext
grep -n "fn file\|current_file\|FileId" \
  01_core/src/rules/eval.rs \
  01_core/src/entities/func.rs \
  03_infra/src/world.rs 2>/dev/null | head -15

# 2. Confirmar como o FileId ou path do ficheiro actual chega ao avaliador
grep -n "source\|main\|FileId\|current" \
  03_infra/src/integration_tests.rs | head -10

# 3. Confirmar os campos actuais de Content::Figure
grep -A 6 "Figure {" 01_core/src/entities/content.rs | head -10

# 4. Confirmar como CounterState regista contadores de heading
# (padrão a replicar para figuras)
grep -n "heading\|counter\|increment\|step" \
  01_core/src/entities/counter_state.rs | head -15

# 5. Confirmar como o layouter acede ao CounterState durante o layout
grep -n "counter_state\|CounterState" \
  01_core/src/rules/layout/mod.rs | head -10

# 6. Confirmar como heading(numbering) é processado no avaliador
# (padrão a replicar para figure(numbering))
grep -n "numbering\|SetHeadingNumbering" \
  01_core/src/rules/eval.rs \
  01_core/src/rules/stdlib.rs 2>/dev/null | head -10
```

Reportar o output completo antes de continuar. Os diagnósticos 1 e 2 são
críticos para a Tarefa 1: se `EvalContext` já tem um campo para o ficheiro
actual (introduzido num passo anterior), a Tarefa 1 apenas actualiza
`World::file`. Se não tem, o campo tem de ser adicionado primeiro.

---

## Tarefa 0 — Actualizar DEBT.md

Antes de qualquer código, marcar os três débitos como `EM CURSO`:

```markdown
### DEBT-25 — Resolução de caminhos relativos — EM CURSO (Passo 75)
### DEBT-15 — Campo kind em Content::Figure — EM CURSO (Passo 75)
### DEBT-14 — #set figure(numbering) — EM CURSO (Passo 75)
```

Os três serão marcados como `ENCERRADO ✓` no final da Tarefa 5.

---

## Tarefa 1 — `current_file` em `EvalContext` e `World::file` (DEBT-25)

### 1a — Adicionar `current_file` a `EvalContext` (L1)

Em `01_core/src/rules/eval.rs`, adicionar o campo ao contexto de avaliação.
O tipo concreto depende do diagnóstico 1 — pode ser `FileId`, `String`, ou
um índice numérico:

```rust
pub struct EvalContext<'a> {
    // ... campos existentes ...

    /// Identificador do ficheiro Typst actualmente em avaliação.
    ///
    /// Usado por World::file para resolver caminhos relativos: um #image("foto.png")
    /// num ficheiro em "capitulo1/intro.typ" deve encontrar "capitulo1/foto.png",
    /// não "foto.png" na raiz. O avaliador passa este campo para world.file().
    ///
    /// Se o avaliador processar #include recursivamente, este campo deve ser
    /// actualizado antes de descer no ficheiro incluído e restaurado ao regressar
    /// — guardar o valor anterior numa variável local antes de actualizar.
    pub current_file: FileId,
}
```

Actualizar todos os locais que constroem `EvalContext` para passar o ficheiro
actual. Na entrada do pipeline (onde `eval` é chamado com um `Source`), o
`FileId` do ficheiro principal está disponível via `world.main()` ou equivalente
— confirmar com o diagnóstico 2.

### 1b — Actualizar a trait `World::file` (L1 e L3)

Em L1 (onde a trait `World` está definida), alterar a assinatura:

```rust
// Era:
fn file(&self, path: &str) -> Result<Arc<Vec<u8>>, String>;

// Passa a ser:
fn file(&self, current_file: FileId, path: &str) -> Result<Arc<Vec<u8>>, String>;
```

Em L3 (`03_infra/src/world.rs`), actualizar a implementação de `SystemWorld`:

```rust
fn file(&self, current_file: FileId, path: &str) -> Result<Arc<Vec<u8>>, String> {
    // Determinar o directório do ficheiro actual.
    // current_file é o FileId/path do ficheiro em avaliação — pode ser um
    // índice que o SystemWorld mapeia para um path absoluto, ou já um path.
    let base_dir = self.directory_of(current_file);

    // Resolver o path fornecido relativamente ao directório do ficheiro actual.
    // join() em PathBuf: se path for absoluto, substitui base_dir inteiramente;
    // se for relativo, concatena. Isso é o comportamento correcto para Typst.
    let absolute_path = base_dir.join(path);

    // Ler e devolver os bytes, com cache se já implementado.
    std::fs::read(&absolute_path)
        .map(|bytes| Arc::new(bytes))
        .map_err(|e| format!("Ficheiro não encontrado: {} — {}", absolute_path.display(), e))
}
```

`directory_of(FileId)` é um método auxiliar que devolve o `Path` do directório
do ficheiro identificado por `FileId`. A implementação concreta depende de como
`SystemWorld` mapeia `FileId` para paths no disco — confirmar com o diagnóstico 2.

### 1c — Actualizar `native_image` em stdlib (L1)

Em `01_core/src/rules/stdlib.rs`, na função `native_image`, passar
`ctx.current_file` para `world.file`:

```rust
// Era:
let data = ctx.world.file(path)
    .map_err(|e| format!("Erro ao carregar imagem: {}", e))?;

// Passa a ser:
let data = ctx.world.file(ctx.current_file, path)
    .map_err(|e| format!("Erro ao carregar imagem: {}", e))?;
```

O compilador lista todos os outros locais que chamam `world.file` e que
precisam de ser actualizados com o mesmo padrão.

---

## Tarefa 2 — `kind` e `numbering` em `Content::Figure` (DEBT-15)

Em `01_core/src/entities/content.rs`, expandir a variante `Figure`:

```rust
Figure {
    body:      Box<Content>,
    caption:   Option<Box<Content>>,
    /// Tipo da figura — discriminador para contadores independentes.
    ///
    /// Valores típicos: "image", "table", "raw" (blocos de código).
    /// O padrão Typst infere o kind a partir do tipo do body (se body for
    /// Content::Image, kind padrão é "image"). Por agora, o padrão fixo
    /// é "image" — inferência automática fica para passo futuro.
    ///
    /// Contadores são indexados por kind: figuras "image" e "table"
    /// têm contagens independentes.
    kind:      String,
    /// Padrão de numeração, se activo.
    ///
    /// None — numeração desligada (comportamento actual).
    /// Some("1") — numeração arábica: "Figura 1", "Figura 2", ...
    /// Some("I") — numeração romana maiúscula (suporte futuro).
    ///
    /// Activado por #set figure(numbering: "1") no documento.
    numbering: Option<String>,
},
```

Actualizar todos os `match` sobre `Content::Figure` — o compilador lista os
locais. Os locais de construção em `stdlib.rs` precisam de passar os dois
campos novos com os valores padrão:

```rust
// Em native_figure, ao construir Content::Figure:
Content::Figure {
    body:      Box::new(body_content),
    caption:   caption_opt,
    kind:      args.named::<String>("kind").unwrap_or_else(|| "image".to_string()),
    numbering: args.named::<String>("numbering"),
}
```

**`#set figure(numbering: ...)` e `Content::SetFigureNumbering`:**
se o sistema já usa um nó explícito no AST para `SetHeadingNumbering`
(Passo 57), a abordagem consistente é criar `Content::SetFigureNumbering`
em vez de depender de estado temporário em `EvalContext`. O nó no AST é
seguro em relação a escopo: o seu efeito é limitado à posição onde aparece
no documento e não vaza entre ramos do AST. Confirmar com o diagnóstico 6
se `SetHeadingNumbering` é um nó do AST ou estado do `EvalContext` — e
replicar exactamente o mesmo padrão para figuras.

Se o padrão for um nó AST, adicionar em `content.rs`:

```rust
/// Activa a numeração automática de figuras a partir deste ponto do documento.
/// Equivalente a #set figure(numbering: "1").
/// Padrão idêntico a SetHeadingNumbering (Passo 57).
SetFigureNumbering {
    pattern: String, // ex: "1"
},
```

O avaliador emite `Content::SetFigureNumbering` quando encontra
`set figure(numbering: ...)`. A introspecção e o layouter consultam
este nó para saber se as figuras seguintes têm numeração activa.

---

## Tarefa 3 — Introspecção de figuras em `CounterState` (DEBT-14, parte 1)

O `CounterState` é construído numa passagem analítica antes do layout
(Passos 56–60). O problema de apenas incrementar e depois ler o total é que
o layouter sempre leria o valor final (N), e todas as figuras receberiam
o mesmo prefixo "Figure N:".

A solução é gravar o número calculado de cada figura **no momento da
introspecção**, quando o contador ainda tem o valor correcto para essa posição.
O `CounterState` mantém um mapa `figure_numbers: HashMap<String, Vec<usize>>`
onde a chave é o `kind` e o `Vec` contém os números na ordem de aparecimento
no documento.
O layouter consulta este mapa pelo índice de progresso, não pelo total.

Em `01_core/src/entities/counter_state.rs`, adicionar o mapa:

```rust
pub struct CounterState {
    // ... campos existentes ...

    /// Números pré-calculados para figuras numeradas, indexados pela ordem
    /// de aparecimento no documento (0-based). Chave: índice da figura.
    /// Valor: número a exibir (1-based).
    ///
    /// Separado por kind: "figure_image" e "figure_table" têm mapas distintos.
    /// Implementação: HashMap<String, Vec<usize>> onde a chave é o kind e
    /// o Vec contém os números na ordem de aparecimento.
    pub figure_numbers: HashMap<String, Vec<usize>>,
}
```

Em `01_core/src/rules/introspect.rs`, na função `walk`, processar
`Content::Figure` incrementando o contador local e gravando o número:

```rust
Content::Figure { kind, numbering, .. } => {
    if numbering.is_some() {
        // Incrementar o contador local para este kind.
        let counter_key = format!("figure_{}", kind);
        let next_val = state.local_figure_counters
            .entry(counter_key.clone())
            .or_insert(0);
        *next_val += 1;
        let figure_number = *next_val;

        // Gravar o número calculado na lista ordenada para este kind.
        // O layouter usará figure_numbers[kind][i] para a i-ésima figura
        // deste kind, na ordem de travessia do documento.
        state.figure_numbers
            .entry(kind.clone())
            .or_default()
            .push(figure_number);
    }
}
```

`local_figure_counters: HashMap<String, usize>` é um campo auxiliar interno
da introspecção, análogo ao contador local que já existe para headings.
Não é exposto ao layouter — apenas `figure_numbers` é.

**Nota sobre o fixpoint:** o `CounterState` é construído antes do layout e
não é mutado durante o layout (`is_readonly = true` — Passo 63). A lista
`figure_numbers` é estável entre iterações do fixpoint porque é calculada
pela introspecção analítica, não pelo layout.

---

## Tarefa 4 — Prefixo de numeração no layouter (DEBT-14, parte 2)

Em `01_core/src/rules/layout/figure.rs`, o layouter precisa de manter um
contador local de progresso por kind — não para calcular o número, mas para
saber qual a posição na lista `figure_numbers` que a introspecção pré-calculou.
Ler `figure_numbers[kind][i]` para a i-ésima figura deste kind garante que
cada figura recebe o seu próprio número, não o total final.

O layouter mantém um campo auxiliar:

```rust
// No struct do Layouter, junto aos outros contadores de progresso:
figure_progress: HashMap<String, usize>, // kind → número de figuras já dispostas
```

No braço `Content::Figure`:

```rust
Content::Figure { body, caption, kind, numbering } => {
    // 1. Dispor o corpo da figura (imagem, tabela, etc.)
    self.layout_content(body);

    // 2. Construir o prefixo de numeração, se activo.
    let caption_prefix: Option<String> = if let Some(num_pattern) = numbering {
        // Obter o índice desta figura dentro das figuras do mesmo kind.
        // figure_progress conta quantas já foram dispostas — começa em 0.
        let progress = self.figure_progress.entry(kind.clone()).or_insert(0);
        let idx = *progress;
        *progress += 1; // avançar para a próxima figura deste kind

        // Ler o número pré-calculado pela introspecção.
        // figure_numbers[kind][idx] foi gravado na Tarefa 3 na ordem de
        // travessia do documento — coincide com a ordem de layout.
        let figure_number = self.counter_state.figure_numbers
            .get(kind)
            .and_then(|v| v.get(idx))
            .copied()
            .unwrap_or(idx + 1); // fallback defensivo

        let num_str = match num_pattern.as_str() {
            "1" => figure_number.to_string(),
            _   => figure_number.to_string(), // outros padrões ficam para passo futuro
        };

        Some(format!("Figure {}: ", num_str))
    } else {
        None
    };

    // 3. Dispor a legenda com o prefixo, se existir.
    if let Some(caption_content) = caption {
        if let Some(prefix) = caption_prefix {
            self.layout_text_inline(&prefix);
        }
        self.layout_content(caption_content);
    }
},
```

**Invariante:** o layouter nunca modifica `counter_state.figure_numbers` —
apenas lê. `figure_progress` é estado local do layouter, reiniciado por
invocação de `layout()`, e não precisa de fazer parte do `CounterState`.
A divisão é: introspecção calcula e grava os números; layouter avança um
índice local para os consumir na ordem correcta.

### 4b — Referências a figuras em `references.rs`

Em `01_core/src/rules/layout/references.rs`, o braço que processa
`Content::Ref` precisa de ser actualizado para figuras numeradas.
Actualmente, `Content::Ref` resolve o label e exibe o número de página
ou o texto do nó referenciado. Se o label pertencer a uma figura numerada,
o texto da referência deve ser o número da figura (ex: "Figure 1"),
não o número de página.

O `CounterState` já mantém `resolved_labels` e `label_pages` (Passos 56–60).
Para figuras, é necessário associar cada label de figura ao seu número
pré-calculado. A introspecção deve gravar este mapeamento quando encontra
uma figura com label:

Em `introspect.rs`, estender o braço `Content::Figure` para registar
o label quando presente:

```rust
Content::Figure { kind, numbering, body, .. } => {
    if numbering.is_some() {
        let counter_key = format!("figure_{}", kind);
        let next_val = state.local_figure_counters
            .entry(counter_key.clone())
            .or_insert(0);
        *next_val += 1;
        let figure_number = *next_val;

        state.figure_numbers
            .entry(kind.clone())
            .or_default()
            .push(figure_number);

        // Se o body contiver um label (Content::Labelled { label, .. }),
        // associar o label ao número calculado para que Content::Ref
        // o possa consultar.
        // Extrair o label do body — adaptar conforme a estrutura real.
        if let Some(label) = extract_label(body) {
            state.figure_label_numbers.insert(label, figure_number);
        }
    }
}
```

`figure_label_numbers: HashMap<Label, usize>` é um campo novo em
`CounterState`. `extract_label` é uma função auxiliar local que percorre
um `Content` à procura de `Content::Labelled` e retorna o `Label` se
existir.

Em `references.rs`, no braço `Content::Ref { target }`:

```rust
Content::Ref { target } => {
    // 1. Verificar se este label pertence a uma figura numerada.
    if let Some(&fig_num) = self.counter_state.figure_label_numbers.get(target) {
        // Exibir o número da figura como texto clicável (ex: "Figure 1").
        // O kind não é necessário aqui porque figure_label_numbers já
        // resolve directamente para o número.
        self.layout_text_inline(&format!("Figure {}", fig_num));
        // Não processar como referência de página — retornar cedo.
        return; // ou `continue`, conforme a estrutura de iteração
    }

    // 2. Fallback: resolver como referência de página (comportamento anterior).
    // ... lógica existente de resolved_labels e label_pages ...
}
```

**Nota sobre `extract_label`:** se a arquitectura existente já regista labels
durante a introspecção de outros nós (ex: `Content::Labelled` é processado
pelo `walk` antes de chegar ao `Figure`), pode ser possível consultar
`state.resolved_labels` em vez de chamar `extract_label` no braço da figura.
Confirmar com o diagnóstico antes de codificar — o objectivo é que
`figure_label_numbers` fique preenchido antes de o layout começar.

---

## Tarefa 5 — Testes

### Teste L3 — caminho relativo resolve correctamente (DEBT-25)

```rust
#[test]
fn image_resolve_caminho_relativo() {
    let root = criar_dir_temporario();

    // Estrutura de pastas:
    //   root/
    //     capitulo1/
    //       intro.typ   ← chama #image("foto.jpg")
    //       foto.jpg
    //     main.typ      ← chama #include "capitulo1/intro.typ"
    std::fs::create_dir(root.join("capitulo1")).unwrap();
    std::fs::write(
        root.join("capitulo1/foto.jpg"),
        &[0xFF, 0xD8, 0xFF, 0xE0], // magic bytes JPEG
    ).unwrap();
    std::fs::write(
        root.join("capitulo1/intro.typ"),
        "#image(\"foto.jpg\")",
    ).unwrap();
    std::fs::write(
        root.join("main.typ"),
        "#include \"capitulo1/intro.typ\"",
    ).unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    // Não deve entrar em pânico com "Ficheiro não encontrado".
    // Se a resolução for incorrecta, world.file() procura foto.jpg na raiz.
    let result = eval(&world, &src);
    assert!(result.is_ok(), "Avaliador falhou: {:?}", result.err());
}
```

### Teste L1 — `Content::Figure` aceita `kind` e `numbering` (DEBT-15)

```rust
#[test]
fn figure_tem_kind_e_numbering() {
    // Confirmar que a variante compilada tem os dois campos.
    let fig = Content::Figure {
        body:      Box::new(Content::Text("corpo".to_string())),
        caption:   Some(Box::new(Content::Text("legenda".to_string()))),
        kind:      "image".to_string(),
        numbering: Some("1".to_string()),
    };

    if let Content::Figure { kind, numbering, .. } = fig {
        assert_eq!(kind, "image");
        assert_eq!(numbering, Some("1".to_string()));
    } else {
        panic!("Variante inesperada");
    }
}
```

### Teste L1 — contadores de figuras por `kind` são independentes (DEBT-14)

```rust
#[test]
fn figuras_kind_diferente_contadores_independentes() {
    // Dois tipos de figura não devem partilhar o contador.
    let doc = Content::Sequence(vec![
        Content::Figure {
            body:      Box::new(Content::Text("img1".to_string())),
            caption:   None,
            kind:      "image".to_string(),
            numbering: Some("1".to_string()),
        },
        Content::Figure {
            body:      Box::new(Content::Text("tab1".to_string())),
            caption:   None,
            kind:      "table".to_string(),
            numbering: Some("1".to_string()),
        },
        Content::Figure {
            body:      Box::new(Content::Text("img2".to_string())),
            caption:   None,
            kind:      "image".to_string(),
            numbering: Some("1".to_string()),
        },
    ]);

    let state = introspect(&doc);

    // figure_numbers["image"] deve ser [1, 2] — duas figuras numeradas independentemente.
    // figure_numbers["table"] deve ser [1] — contador separado, não afectado pelas imagens.
    let image_nums = state.figure_numbers.get("image").cloned().unwrap_or_default();
    let table_nums = state.figure_numbers.get("table").cloned().unwrap_or_default();

    assert_eq!(image_nums, vec![1, 2],
        "Duas figuras de kind 'image' devem produzir [1, 2]");
    assert_eq!(table_nums, vec![1],
        "Uma figura de kind 'table' deve produzir [1] independentemente");
}
```

### Teste L3 — pipeline completo com figura numerada (DEBT-14)

```rust
#[test]
fn pipeline_figura_numerada_prefixo_no_pdf() {
    let root = criar_dir_temporario();
    std::fs::write(root.join("foto.jpg"), &[0xFF, 0xD8, 0xFF, 0xE0]).unwrap();
    std::fs::write(
        root.join("main.typ"),
        "#set figure(numbering: \"1\")\n#figure(image(\"foto.jpg\"), caption: [A foto])",
    ).unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());

    // figure_numbers["image"] deve ser [1] — uma figura, número 1.
    let image_nums = state.figure_numbers.get("image").cloned().unwrap_or_default();
    assert_eq!(image_nums, vec![1],
        "Uma figura de imagem deve produzir figure_numbers[\"image\"] = [1]");

    let doc = layout(module.content(), state);
    let pdf = export_pdf(&doc);
    assert!(!pdf.is_empty(), "PDF não pode estar vazio");
}
```

### Teste L3 — `current_file` restaurado após `#include` (DEBT-25)

Este teste verifica que o avaliador restaura `current_file` ao regressar
de um `#include`. Se o contexto não for restaurado, a segunda imagem em
`main.typ` procurará `capa.jpg` no directório do ficheiro incluído
(`capitulo1/`) em vez da raiz, falhando com "Ficheiro não encontrado".

```rust
#[test]
fn current_file_restaurado_apos_include() {
    let root = criar_dir_temporario();

    // Estrutura de pastas:
    //   root/
    //     capa.jpg          ← imagem na raiz, referenciada APÓS o #include
    //     capitulo1/
    //       intro.typ       ← ficheiro incluído, tem a sua própria imagem
    //       foto.jpg        ← imagem em capitulo1, referenciada dentro do include
    //     main.typ          ← usa imagem antes e depois do #include
    std::fs::create_dir(root.join("capitulo1")).unwrap();
    std::fs::write(root.join("capa.jpg"),             &[0xFF, 0xD8, 0xFF, 0xE0]).unwrap();
    std::fs::write(root.join("capitulo1/foto.jpg"),   &[0xFF, 0xD8, 0xFF, 0xE0]).unwrap();
    std::fs::write(
        root.join("capitulo1/intro.typ"),
        "#image(\"foto.jpg\")", // resolve em capitulo1/ — correcto
    ).unwrap();
    std::fs::write(
        root.join("main.typ"),
        // Imagem antes do include → resolve na raiz
        // Include de capitulo1/intro.typ → current_file muda temporariamente
        // Imagem depois do include → deve resolver na raiz novamente
        "#image(\"capa.jpg\")\n\
         #include \"capitulo1/intro.typ\"\n\
         #image(\"capa.jpg\")",
    ).unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let result = eval(&world, &src);

    // Se current_file não for restaurado, a segunda #image("capa.jpg")
    // procura em capitulo1/ e falha. O sucesso aqui prova a restauração.
    assert!(result.is_ok(),
        "Avaliador falhou — provavelmente current_file não foi restaurado após #include: {:?}",
        result.err());
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
- [ ] `EvalContext` tem o campo `current_file: FileId` (ou tipo equivalente).
- [ ] `World::file` aceita `current_file` como primeiro argumento. Todos os
  locais de chamada actualizados — o compilador confirma.
- [ ] `SystemWorld::file` resolve o path relativamente ao directório do
  ficheiro actual via `PathBuf::join`.
- [ ] `native_image` passa `ctx.current_file` para `world.file`.
- [ ] O avaliador guarda e restaura `current_file` ao entrar e sair de
  `#include` — confirmado pelo teste `current_file_restaurado_apos_include`.
- [ ] `Content::Figure` tem os campos `kind: String` e `numbering: Option<String>`.
  Todos os `match` e locais de construção actualizados.
- [ ] `native_figure` extrai `kind` e `numbering` dos argumentos nomeados.
  `kind` tem padrão `"image"` quando omitido.
- [ ] `#set figure(numbering: ...)` usa o mesmo padrão de `SetHeadingNumbering`
  (nó AST `Content::SetFigureNumbering` ou estado em `EvalContext`, conforme
  o diagnóstico 6 confirmar). A abordagem é idêntica à usada para headings.
- [ ] `CounterState` tem os campos `figure_numbers: HashMap<String, Vec<usize>>`,
  `local_figure_counters: HashMap<String, usize>` (auxiliar interno), e
  `figure_label_numbers: HashMap<Label, usize>`.
- [ ] A introspecção, no braço `Content::Figure`, incrementa o contador local,
  grava o número em `figure_numbers[kind]`, e associa o label da figura ao
  número em `figure_label_numbers` (quando existe label).
- [ ] O layouter tem o campo local `figure_progress: HashMap<String, usize>`,
  reiniciado por invocação de `layout()`.
- [ ] O layouter lê `counter_state.figure_numbers[kind][figure_progress[kind]]`
  e incrementa apenas `figure_progress` — nunca `counter_state`.
- [ ] A primeira figura de kind `"image"` recebe "Figure 1: " e a segunda
  recebe "Figure 2: " — confirmado pelo teste de pipeline.
- [ ] `references.rs` — o braço `Content::Ref` consulta
  `counter_state.figure_label_numbers` antes de tentar resolver como
  referência de página. Labels de figuras numeradas produzem "Figure N".
- [ ] Figuras de `kind` diferentes têm listas `figure_numbers` independentes
  — confirmado pelo teste `figuras_kind_diferente_contadores_independentes`.
- [ ] DEBT-25, DEBT-14, DEBT-15 marcados como **ENCERRADO ✓** em `01_core/DEBT.md`.
- [ ] Teste `image_resolve_caminho_relativo` passa.
- [ ] Teste `current_file_restaurado_apos_include` passa.
- [ ] Teste `figure_tem_kind_e_numbering` passa.
- [ ] Teste `figuras_kind_diferente_contadores_independentes` passa.
- [ ] Teste `pipeline_figura_numerada_prefixo_no_pdf` passa.
- [ ] Zero violações no linter e no clippy.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Tipo concreto usado para `FileId` — e se `EvalContext` já tinha um campo
  de identificação de ficheiro antes desta tarefa.
- Como `SystemWorld` mapeia `FileId` para paths absolutos no disco
  (`directory_of` ou equivalente).
- Se `SetHeadingNumbering` é um nó AST ou estado em `EvalContext` — determina
  qual o padrão replicado para `SetFigureNumbering`.

**Da implementação:**
- Se `figure_numbers` e `local_figure_counters` foram adicionados a
  `CounterState` ou se o padrão existente de headings já tinha uma estrutura
  equivalente que foi reutilizada.
- Se `figure_progress` no layouter precisou de ser reiniciado explicitamente
  ou se a estrutura de invocação de `layout()` já garantia um estado limpo.
- Número total de testes após o passo e zero violations confirmados.

**Go/No-Go para o Passo 76:**
- **GO — gráficos e export avançado:** se figuras numeradas aparecem
  correctamente no PDF com o prefixo "Figure N:" e imagens em sub-pastas
  resolvem sem erro.
- **NO-GO — prefixo duplicado:** se a figura aparece com "Figure 1: Figure 1:"
  no PDF, o contador está a ser lido duas vezes (na introspecção e no layout).
  Verificar que o incremento acontece apenas na introspecção.
- **NO-GO — caminho relativo não resolvido:** se o teste
  `image_resolve_caminho_relativo` falha com "Ficheiro não encontrado" na raiz
  em vez de em `capitulo1/`, o `directory_of(current_file)` não está a devolver
  o directório correcto.
