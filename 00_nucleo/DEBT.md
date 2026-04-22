## DEBT-2 — Closures eager vs lazy capture — PARCIALMENTE RESOLVIDO

### Resolvido no Passo 31

- `ClosureRepr::captured` mudou de `IndexMap<String, Value>` (clone eager O(N)) para `Arc<Scope>`
- Captura no momento da definição: snapshot O(N) uma única vez, depois partilhado em O(1)
- `apply_closure` usa `Scopes::with_parent(Arc::clone(&captured))` — lookup sem clone dos valores
- `eval_let` trata `LetBindingKind::Closure`: sintaxe `#let fib(n) = ...` agora funciona
- `Expr::Closure` arm lê `closure_expr.name()` — nome propagado correctamente para recursão

### Divergência residual

- Semântica de captura: ainda eager (snapshot). `#let x=1; #let f()=x; #let x=2; f()` retorna `1`
  (snapshot), não `2` (lazy). O original via `comemo` retornaria `2`.
  **Confirmado no Passo 31**: o snapshot é uma cópia independente do scope, não uma referência
  partilhada. Divergência semântica documentada com o original. Não bloqueante.
- A integração com `comemo` para tracking semântico real aguarda `TrackedWorld` real.
- Registado como sub-DEBT se cenários avançados de shadowing forem encontrados nos testes de paridade.

### Pendente

- Integração com `comemo` para tracking semântico real
- Testes de paridade com o original para cenários avançados de shadowing

---

## DEBT-6 — eval_for_test coverage blind spot — RESOLVIDO

### Registado e resolvido no Passo 32

**Problema**: `eval_for_test` usa `MockWorld` — um mundo artificial que não passa pelo mecanismo
de tracking real (`TrackedWorld`). Os testes de L1 nunca exercitavam o caminho de código de produção.

**Resolvido no Passo 32**:

- Testes de integração em `03_infra/src/integration_tests.rs`
- Pipeline completo exercitado: `SystemWorld` → `eval` → `layout` → `export_pdf`
- `eval()` pública confirmada como genérica sobre `TrackedWorld`
- `eval_for_test` mantida para testes unitários rápidos de L1

**Cobertura adicionada**:

- `pipeline_texto_simples`: eval + layout via SystemWorld
- `pipeline_export_pdf_helvetica`: export com fallback Helvetica
- `pipeline_export_pdf_com_fonte_real`: export com fonte do sistema (ou fallback)
- `pipeline_com_set_text_bold`: StyleChain via pipeline real
- `pipeline_com_closures`: closures via pipeline real
- `pipeline_eval_retorna_err_em_sintaxe_invalida`: robustez a input inválido

---

## DEBT-7 — Merge bold em layout — RESOLVIDO

**Registado no Passo 32. Resolvido no Passo 33.**

**Resolução**:

- Save/restore de `ctx.styles` em `Expr::CodeBlock` (`#{ }`)
- Save/restore de `ctx.styles` em `Expr::ContentBlock` (`[ ]`)
- Save/restore de `ctx.styles` em `apply_closure` (body de closures)
- Merge `bold || node_style.bold` e `italic || node_style.italic` removidos de `layout.rs`
- `#set text(bold: false)` dentro de um bloco agora reverte correctamente ao sair do bloco
- `node_style` capturado em eval já inclui o estilo correcto de Strong/Emph/Heading

**Ficheiros alterados**: `rules/eval.rs`, `rules/layout.rs`

---

# Dívida de instrumentação — ADR-0006

Os seguintes pontos de timing foram removidos para manter L1 puro.
Religação prevista no Passo 10 (isolamento de comemo/infra).

| Função       | Nome do scope original |
|--------------|------------------------|
| parse()      | "parse"                |
| parse_code() | "parse-code"           |
| parse_math() | "parse-math"           |

## Como religar no Passo 10

Localizar todos os `// ADR-0006: timing removed — ver 01_core/DEBT.md`
em `01_core/src/rules/parse.rs` e substituir `timing_scope!("...")` por
o mecanismo de telemetria escolhido (trait injectável ou outro).

Ver: `00_nucleo/adr/typst-adr-0006-typst-timing.md`

## DEBT-3 — Safety rails hardcoded — RESOLVIDO (estrutura)

### Resolvido no Passo 28

- **`while` limit**: 10.000 → 1.000.000, via `EvalContext::tick_loop()`
- **`MAX_CALL_DEPTH`**: 200 → 250, via `EvalContext::check_call_depth()`
- **Limites documentados em `EvalContext`**: não mais magia inline
- **Métodos de verificação**: `check_call_depth()` e `tick_loop()` implementados
- **Limite global de loop**: contador acumulado ao longo de toda eval, não por loop

### Resolvido no Passo 29

- **Detecção de ciclos de importação**: `EvalContext::enter_import()` + `ImportGuard`
- **`import_stack: Vec<FileId>`**: rastreamento de ficheiros em avaliação
- **`ModuleImport` e `ModuleInclude`**: retornam Err limpo (não panic)

### Pendente (não é DEBT — é feature futura)

- **Implementação de `import` completo**: Passo 33+
- **Integração com `comemo`**: tracking semântico real (aguarda TrackedWorld real)

### Ficheiros alterados

- `01_core/src/rules/eval.rs`: `EvalContext` (max_call_depth 250, import_stack), `ImportGuard`, `ModuleImport`/`ModuleInclude` handling
- `01_core/src/rules/eval.rs`: testes novos de import_stack e ModuleImport/Include

## DEBT-1 — StyleChain — PARCIALMENTE RESOLVIDO

### Resolvido no Passo 30

- **`StyleChain`** implementada em `entities/style_chain.rs` (L1)
- **`StyleDelta { bold, italic, size }`** como delta de herança
- **`#set text(bold:, italic:, size:)`** avaliado em `eval_expr` (Expr::SetRule)
- **`EvalContext::styles: StyleChain`** — cadeia activa durante eval
- **`TextStyle::from(&StyleChain)`** — bridge para layout/export actuais
- **`Content::Text(EcoString, TextStyle)`** — estilo capturado em eval
- **Strong/Emph/Heading** em eval: push/pop de estilos correcto

### Divergência intencional

- `#set` é global ao eval (não tem scoping por bloco) — DEBT menor
- Apenas `text` como target suportado — outros targets ignorados silenciosamente
- StyleChain não integrada com `#show` rules (Passo futuro)
- Layout usa merge de node_style + self.style para compatibilidade com testes directos

### Pendente

- Scoping de `#set` por bloco `{ }`
- Propriedades adicionais (fill, font-family, weight numérico, etc.)
- `#show` rules
- Paridade total com o sistema de styles do original
- Remover os wrappers Content::Strong/Emph do layout quando eval os tiver totalmente substituído

**Ficheiros alterados**: `entities/style_chain.rs` (novo), `entities/mod.rs`,
`entities/content.rs`, `rules/eval.rs`, `rules/layout.rs`

---

## DEBT-8 — Motor de equações — PARCIALMENTE RESOLVIDO

**Parcialmente resolvido no Passo 37**. Registado no Passo 34.

**Resolvido no Passo 36**:

- `MathLayouter` criado em `rules/math/layout.rs` (L1 puro)
- `MathIdent` e `MathText` → `FrameItem::Text` (sem placeholder)
- `MathFrac`, `MathAttach`, `MathRoot` → texto plano sem `[...]`
- Placeholder `[equação]` removido do layouter principal
- `Content::Equation` delega ao `MathLayouter` para inline e block

**Resolvido no Passo 39**:

- `rules/math/symbols.rs` com `ident_to_unicode`, `shorthand_to_unicode`,
  `is_math_function`, `is_single_letter_var`
- `Expr::MathIdent` em `eval_math_expr`: símbolo conhecido → `Content::MathText(unicode)`
- `MathShorthand::get()` no AST já retorna o char Unicode — sem alteração necessária
- Variáveis de uma letra → `TextStyle { italic: true }` em `MathLayouter::layout_node`
- Funções (sin, cos, lim, etc.) → `TextStyle { italic: false }`

**Resolvido no Passo 38**:

- `FrameItem::Line { start, end, thickness }` adicionado a `layout_types.rs`
- `layout_frac` emite `FrameItem::Line` para a linha horizontal entre num/den
- `MathBox::place()` e `hconcat()` propagam offsets para `FrameItem::Line`
- `export.rs` trata `FrameItem::Line` com operadores PDF `q w m l S Q`
- `frac(a, b)` em `eval_math_expr` via `Expr::FuncCall` → `Content::MathFrac`
- `MathDelimited` em `eval_math_expr` inclui `open` e `close` como `MathText`

**Resolvido no Passo 37**:

- `MathBox { width, ascent, descent, items }` como unidade de layout
- `MathFrac`: numerador acima da baseline, denominador abaixo,
  tamanho 70% do texto base. Sem linha de fracção (Passo 38+)
- `MathAttach`: sup elevado a 50% do ascender, sub baixado a 30%
  da descida, tamanho 65% do texto base
- `hconcat`: concatenação horizontal de `MathBox`es
- `MathLayouter` refactorizado para stateless (`&self`)
- `layout_equation` retorna `Vec<FrameItem>` (posições relativas)
- Integração em `layout.rs` usa `y: offset_y + pos.y` para
  posicionamento vertical correcto

**Resolvido no Passo 40**:

- `sqrt(x)` como função nativa em `eval_math_expr` → `Content::MathRoot { index: None }`
- `root(n, x)` como função nativa → `Content::MathRoot { index: Some(n) }`
- Validação de aridade: `sqrt()` e `sqrt(x,y)` retornam `Err`; `root(3)` retorna `Err`
- `MathLayouter::layout_root`: símbolo `√`, overline (`FrameItem::Line`), radicando posicionado à direita
- `offset_item` helper adicionado em `math/layout.rs`
- `sqrt` e `root` adicionados a `is_math_function` (renderizados sem itálico)

**Ainda pendente**:

- Kern matemático entre símbolos
- Fontes OpenType MATH (tabelas MATH, variantes de tamanho)
- `MathPrimes`, `MathAlignPoint`
- Baseline correcta em relação ao x-height da fonte

---

## DEBT-10 — Contadores em duas passagens — RESOLVIDO

**Registado no Passo 57. Encerrado no Passo 62.**

**Resolução (Passagens 1 e 2)**:

- `01_core/src/rules/introspect.rs` criado: pré-passagem analítica que percorre
  `Content` sem alocações visuais, popula `resolved_labels: HashMap<Label, String>`.
- `layout()` recebe `CounterState` externo — o orquestrador chama `introspect` primeiro.
- Forward refs (`@conclusao` antes do heading `= Conclusão <conclusao>`) resolvem.
- Backward refs continuam a funcionar.
- `Content::Figure` implementado no Passo 62: introspecção rastreia figuras na
  Passagem 1, Layouter desenha blocos numerados na Passagem 2.

**Ficheiros criados/alterados**:

- `01_core/src/rules/introspect.rs` (novo)
- `01_core/src/entities/counter_state.rs` — `resolved_labels`, `headings_for_toc`, `auto_label_counter`
- `01_core/src/rules/layout/mod.rs` — `layout()` com estado externo
- `01_core/src/rules/layout/figure.rs` — braço `Content::Figure` (Passo 62)

---

---

## DEBT-11 — Decomposição de `layout.rs` — RESOLVIDO

**Registado no Passo 60. Resolvido no Passo 61.**

**Resolução**:

- `layout.rs` (1408 linhas) convertido em `layout/mod.rs` (orquestrador).
- `rules/layout/counters.rs`: braços `SetHeadingNumbering`, `CounterUpdate`, `CounterDisplay`.
- `rules/layout/references.rs`: braços `Ref` e `Labelled`.
- `rules/layout/outline.rs`: braço `Content::Outline` (TOC).
- L0 criados: `layout_counters.md`, `layout_references.md`, `layout_outline.md`.
- `layout()` passa a receber `CounterState` externo — o orquestrador chama `introspect` primeiro.

---

## DEBT-9 — Cobertura de paridade — tracking contínuo

**Estado**: Baseline estabelecido no Passo 35. Sem divergências no corpus actual.

**Descrição**: O parity_runner testa 50 inputs (40 markup/code/math gerais + 10 math
específicos do Passo 34). Todos passam. À medida que o motor de equações e novas
funcionalidades forem implementadas, adicionar casos de paridade correspondentes.

**Quando expandir**: A cada passo que adicione novo SyntaxKind ou altere semântica do parser.

**Referência**: `lab/parity/tests/parse_parity.rs`, baseline em
`00_nucleo/materialization/parity-baseline-passo-35.md`

---

## DEBT-12 — Números de página na TOC — RESOLVIDO (Passo 63)

**Registado no Passo 61. Resolvido no Passo 63.**

**Resolução**: orquestração em 3 passagens no `compile_to_pdf` de L3:

- Passagem 1 (introspecção): `introspect()` popula `resolved_labels` e `headings_for_toc`.
- Passagem 2 (draft): `layout()` regista `label_pages` via `layout_labelled` e expõe-o em
  `PagedDocument::extracted_label_pages`. A TOC ainda não tem números de página.
- Passagem 3 (final): `layout()` com `initial_state.label_pages` preenchido; a TOC lê
  os números reais de `label_pages` para cada linha.

**Limitação residual**: registada como DEBT-17 (fixpoint da TOC).

---

## DEBT-14 — SetRule para `#set figure(numbering: ...)` — ENCERRADO ✓ (Passo 75)

**Registado no Passo 62.**

A numeração de figuras está activa por defeito (`CounterState::new()` insere
`"figure" = true`). O utilizador não consegue desactivá-la com
`#set figure(numbering: none)` até que o braço de SetRule para "figure" seja
adicionado ao `eval.rs`, produzindo um nó equivalente a `SetHeadingNumbering`.
Quando implementado, figuras sem numeração cujas labels forem referenciadas
mostrarão o fallback `@label` (comportamento intencional — ver braço `Labelled`
em `introspect.rs`).

---

## DEBT-15 — Campo `kind` hardcoded em `Content::Figure` — ENCERRADO ✓ (Passo 75)

**Registado no Passo 62.**

A chave `"figure"` está hardcoded em `step_flat("figure")` tanto na introspecção
como no layout. No Typst original, `#figure` aceita um argumento `kind`
(ex: `image`, `table`, `code`), e cada kind tem contador próprio —
"Tabela 1" e "Figura 1" são independentes. Com a implementação actual, tabelas
e gráficos partilham o mesmo contador.

**Resolução futura**: adicionar campo `kind: String` (default `"figure"`) a
`Content::Figure` e usar `step_flat(&kind)` em vez da string fixa.

---

## DEBT-13 — Efeitos colaterais duplicados na TOC — RESOLVIDO (mitigado, Passo 63)

**Registado no Passo 61. Mitigado no Passo 63.**

**Mitigação**: flag `CounterState::is_readonly` activa durante a renderização de cada
linha da TOC em `outline.rs`. Enquanto `is_readonly = true`, os métodos
`step_flat`, `step_hierarchical` e `update_flat` são no-ops — `CounterUpdate` embebido
nos clones de heading não avança os contadores.

**Limitação residual**: `CounterDisplay` ainda lê estado incorrecto na TOC (lê valores
do momento em que a TOC é renderizada, não do momento do heading real). Registado
como DEBT-18 (perda de contexto temporal em AST clonado na TOC).

---

## DEBT-16 — Acoplamento do Avaliador à Stdlib — RESOLVIDO (Passo 64)

**Registado no Passo 62. Resolvido no Passo 64.**

**Resolução**:

- `NativeFunc.call` mudou de `fn(&[Value])` para `fn(&Args)` — aceita positional e named args.
- `apply_func` passa `&args` directamente: `(native.call)(&args)`.
- Função auxiliar `expect_no_named()` adicionada em `stdlib.rs` — funções que não aceitam
  named args retornam `Err` semântico (não silencioso).
- `native_figure` migrada do interceptador hardcoded em `eval.rs` para `stdlib.rs`.
  `eval.rs` não contém nenhuma referência ao nome "figure".
- Cascata de 17 funções actualizadas (8 stdlib + 9 calc).
- Testes de `stdlib.rs` actualizados para usar `Args::positional(...)` em vez de `&[Value]`.

---

## DEBT-17 — Fixpoint da TOC — RESOLVIDO (Passo 65)

**Registado no Passo 63. Resolvido no Passo 65.**

**Resolução**: o algoritmo de fixpoint foi movido para dentro de `layout()` em L1.
O ciclo corre até que `extracted_label_pages` não mude entre iterações (máximo 5).
O orquestrador L3 (`compile_to_pdf`) voltou a ser linear:
`introspect()` → `layout()` → `export_pdf()`.

**Separação leitura/escrita**: `CounterState` tem dois campos distintos:

- `label_pages`: escrito por `references.rs` em cada iteração (começa vazio via `Layouter::new()`).
- `known_page_numbers`: lido por `outline.rs`, injectado pelo fixpoint com o mapa da iteração anterior.

`CounterState::has_outline`: sinalizado por `introspect()` na presença de `Content::Outline`.
Sem `has_outline`, `layout()` usa short-circuit de passagem única (sem fixpoint).

---

## DEBT-18 — Perda de Contexto Temporal em AST Clonado na TOC — RESOLVIDO (Passo 66)

**Registado no Passo 63. Resolvido no Passo 66.**

**Resolução**: materialização de AST na Passagem 1 (Introspecção).
A função `materialize_time(content, state)` em `introspect.rs` percorre o AST do
body de cada título e substitui cada `CounterDisplay` pelo seu valor em texto
estático (`Content::Text`) usando o `CounterState` actual naquele momento exacto.

`headings_for_toc.push` passou de `*body.clone()` para `materialize_time(body, state)`.

`CounterState::display_value(kind)` centraliza a lógica de leitura (hierárquico
vs plano) usada por `materialize_time` e por `layout/counters.rs`.

---

## DEBT-23 — Travessia múltipla em apply_show_rules (Passo 69)

`apply_show_rules` percorre a árvore uma vez por `ShowRule` activa: O(R×N)
com R regras e N nós. Resolução: `map_content` chamado uma única vez, com
todas as regras testadas por nó dentro da closure. Requer mudança de
assinatura de `map_content` para aceitar uma lista de regras em vez de uma
closure genérica.

---

## DEBT-19 — Avaliação superficial de NodeKind — **ENCERRADO (Passo 69)**

`map_content` bottom-up implementado em `content.rs`. `apply_show_rules`
reescrito para usar `map_content` em seletores de nó — a árvore inteira
é percorrida. `NodeKind` expandido com Strong, Emph, Raw, Equation, ListItem.
`intercept_content` adicionado a `Expr::Strong` e `Expr::Emph` além de
`Expr::Heading` e `Expr::FuncCall`.

Substituição Texto→Content via Func/Content continua a gerar Err explícito.

---

## DEBT-20 — Guard anti-recursão booleano global — **ENCERRADO (Passo 70)** ✓

Substituído por `active_guards: Vec<RuleId>` no `EvalContext`. Pilha de
regras activas permite composição de regras sem stack overflow — a regra
cujo ID está na pilha é saltada, outras regras continuam a actuar.

---

## DEBT-21 — Resolução de NodeKind por string — **MITIGADO (Passo 70)**

`Func::name()` continua a ser usado. Aliasing não é detectado. Resolução
completa por ponteiro adiada (requer Rust >= 1.85 para `fn_addr_eq` estável).
Mensagens de erro melhoradas para closures anónimas (None → Err explícito).

---

## DEBT-22 — Clone de show_rules por nó (Passo 68)

`ctx.show_rules.clone()` em `intercept_content` é O(N) por cada nó de
conteúdo gerado. Em documentos grandes com muitas regras, o custo acumula.
Resolução: usar `Rc<[ShowRule]>` ou indexação para partilhar a lista sem
copiar, separando o estado de mutação da leitura.

---

## DEBT-23 — Travessia múltipla O(R×N) — **ENCERRADO (Passo 70)** ✓

`apply_show_rules` chama `map_content` uma única vez para todas as regras
`NodeKind`. Dentro da closure bottom-up, todas as regras activas são testadas
por nó antes de prosseguir — custo reduzido de O(R×N) para O(N).

---

## DEBT-24b — Dimensões reais de imagem — **ENCERRADO (Passo 72)** ✓

Placeholder 100×100 substituído por leitura real de dimensões via trait
`ImageSizer` injectado no layouter. L3 implementa com `imagesize`;
L1 mantém-se puro (zero dependências externas adicionais).

---

## DEBT-24c — FrameItem::Image e export PDF — **ENCERRADO (Passo 73)** ✓

`FrameItem::Image` adicionado ao layouter e ao exportador PDF. JPEG embutido
com `/DCTDecode`. Deduplicação por `Arc::as_ptr`. Espaço reservado no layout
e imagem emitida no PDF para ficheiros JPEG.

---

## DEBT-27 — Suporte a transparência PNG no exportador PDF — **ENCERRADO (Passo 74)** ✓

PDF não suporta ficheiros PNG crus. PNG requer descodificação de píxeis
(canal RGBA separado para /SMask de transparência) antes de ser embutido.
Prova de conceito do Passo 73 usa apenas JPEG (/DCTDecode).
Resolução: Passo 74 adiciona `crate image` a L3 para descodificar PNG
em RGBA plano e passar ao gerador de PDF.

---

## DEBT-28 — Dupla leitura de cabeçalho de imagem no layouter — **ENCERRADO (Passo 74)** ✓

`calculate_dimensions` e `sizer.size()` lêem o cabeçalho da imagem separadamente.
O custo é mínimo (imagesize lê apenas o cabeçalho, não os píxeis), mas é
redundante. Resolução futura: `calculate_dimensions` retorna `ImageDimensions`
com `intrinsic_width`, `intrinsic_height` — eliminando a segunda chamada.

---

## DEBT-29 — Detecção de ColorSpace para JPEGs crus — **ENCERRADO (Passo 74)** ✓

O exportador assume `DeviceRGB` para todos os JPEGs. JPEGs Grayscale (1 canal)
ou CMYK (4 canais) com ColorSpace errado produzem lixo visual ou rejeição pelo
leitor PDF. Resolução: Passo 74 lê o marcador SOF0/SOF2 do cabeçalho JPEG para
determinar o número de canais e escolher `DeviceRGB`, `DeviceGray` ou `DeviceCMYK`.

---

## DEBT-25 — Resolução de caminhos relativos em stdlib (Passo 71) — **ENCERRADO (Passo 75)** ✓

`native_image` passa o caminho ao `World::read_bytes` tal como fornecido pelo
utilizador (relativo à raiz do projecto). Não há resolução relativa ao ficheiro
fonte activo. Resolução: `EvalContext` deve expor o path do ficheiro corrente
para que `native_image` construa um caminho absoluto.

---

## DEBT-32 — Alinhamento da bounding box para linhas com deltas negativos — **ENCERRADO (Passo 77)** ✓

O exportador desenhava a linha a partir de `pos.x` sem considerar o sinal de `dx`/`dy`.
Se `dx < 0`, a linha saía para a esquerda da bounding box. A correcção mapeia
início/fim dentro da bounding box com base no sinal dos deltas.

---

## DEBT-30 — Suporte a clipping paths (clip: true) — ENCERRADO ✓ (Passo 79)

Contentores com `clip: true` requerem o operador `W` (clipping path) e a sequência
`W n` antes de desenhar o conteúdo interno. O PDF mantém um clipping path activo
por estado gráfico (`q`/`Q`), portanto `clip: true` exige um push/pop de estado
adicional. Campo `clip_mask: Option<ShapeKind>` adicionado a `FrameItem::Group`.
O exportador emite `W n` no espaço local do Group, após a matriz `cm`.

## DEBT-33 — Bounding Box de curvas Bézier (Passo 79) — EM ABERTO

A bounding box de `ShapeKind::Path` é calculada verificando o min/max dos pontos
de controlo. Para `CubicTo`, a curva real pode ultrapassar a caixa delimitadora
dos pontos de controlo, causando vazamento visual subtil. Resolução futura:
cálculo analítico dos extremos da curva paramétrica B(t) para obter a AABB exacta.

---

## DEBT-31 — Transformações afins (rotate, scale) em nós — **ENCERRADO (Passo 78)** ✓

Rotação e escala requerem a matriz `cm` com valores não-identidade:
`[cos -sin sin cos tx ty]` para rotação. A bounding box de um nó rodado não é
um rectângulo alinhado aos eixos — o layouter precisaria de calcular o
bounding box transformado. `Ellipse` usa rectângulo placeholder no exportador.
Resolução: passo futuro de transformações.

---

## DEBT-26 — PartialEq O(N) sobre Arc<Vec<u8>> em Content::Image — **ENCERRADO (Passo 74)** ✓

A implementação manual de `PartialEq` para `Content::Image` compara `data`
por valor (`da == db`), o que é O(N) nos bytes da imagem. Em contextos de
teste ou deduplicação frequente, usar `Arc::ptr_eq` primeiro e cair para
comparação por valor apenas se os ponteiros diferirem.

---

## DEBT-34b — Parâmetro rows em Content::Grid ignorado — **ENCERRADO (Passo 83)** ✓

`Content::Grid` passa a respeitar `rows: Vec<TrackSizing>`. O motor de layout
implementa três passagens (Fixed → Auto → Fraction) espelhando a resolução de
colunas, com indexação cíclica `N % rows.len()` e decisão de paginação antes da
fase Fraction (evita alturas `fr` "fósseis" da página anterior).

## DEBT-34c — Alinhamento vertical de células no Grid — **ENCERRADO (Passo 83)** ✓

`cell_height` é agora passado como `available_h` a `resolve_alignment` para
items dentro de células (campo `cell_available_h: Option<f64>` no Layouter).
`VAlign::Bottom` ancora ao limite inferior da célula e `VAlign::Horizon` centra
verticalmente.

## DEBT-34d — Auto não encolhe antes de matar fr — EM ABERTO (Passo 80)

Um Auto guloso (célula com texto longo) pode consumir todo o `safe_available`,
deixando 0pt para as colunas fr. Resolução futura: implementar min-content e
max-content para Auto, com negociação entre Auto e fr.

## DEBT-34e — colspan e rowspan — EM ABERTO (Passo 80)

Células que ocupam múltiplas colunas ou linhas requerem um algoritmo de
placement diferente. Resolução: passo futuro.

## DEBT-35b — Invalidação de cache de available_width após SetPage — EM ABERTO (Passo 81)

Se alguma função guardar available_width em cache como campo do Layouter,
esse cache tem de ser invalidado no processamento de Content::SetPage.
Actualmente available_width() é calculado em tempo real sem cache —
este DEBT documenta o risco caso um cache venha a ser adicionado.

## DEBT-36 — Operadores simbólicos de alinhamento (center + bottom) — EM ABERTO (Passo 82)

`align` e `place` aceitam strings (`"center"`, `"top-right"`) porque o parser
ainda não suporta operadores de composição simbólica como `center + bottom`.
Resolução: quando o parser suportar `Value::Align` com composição, substituir
`Align2D::from_string` pelo parse directo da variante.

## DEBT-37 — Place relativo ao contentor pai — EM ABERTO (Passo 82)

`Content::Place` ancora às margens absolutas da página. O Typst suporta
`place` relativo ao bloco pai (ex: dentro de um grid, `place` ancora na célula).
Resolução: passar a área de âncora como parâmetro ao processar `Place`.

## DEBT-38 — Cache de sub-frames no Grid Auto — EM ABERTO (Passo 83)

A resolução de altura de linhas Auto chama `layout_sub_frame_with_width` para
medir a altura intrínseca de cada item, descartando os FrameItems produzidos.
Quando a célula é emitida no documento, a mesma função é chamada de novo para o
mesmo item com a mesma largura, duplicando o trabalho de layout em todas as
células Auto.
Resolução: cache de `(Content*, width) → (height, Vec<FrameItem>)` válido
dentro da resolução de um Grid. Reutilizar o resultado da medição na emissão.
