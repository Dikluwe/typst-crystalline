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

## DEBT-10 — Contadores em duas passagens — PENDENTE

**Registado no Passo 57.**

**Situação actual**: `CounterState` resolve contadores numa única passagem. Sufficiente
para numeração sequencial de headings (`1`, `1.1`, `2`). Não suporta referências para
a frente (ex: `@intro` resolver para `"Secção 1"` antes do heading aparecer no documento).

**Divergência do original**: O Typst original usa `comemo` e duas passagens de layout
para resolver contadores com referências para a frente. `CounterDisplay` actualmente
emite o valor do contador no momento do layout, não o valor final após todas as passagens.

**Como resolver**:
1. Primeira passagem: recolher todos os `Content::Heading` e calcular o mapa completo
   `Label → número de secção`.
2. Segunda passagem: `CounterDisplay` e `Ref` consultam o mapa para resolver valores.
3. Integrar com `TrackedWorld` e `comemo` para tracking semântico real.

**Ficheiros envolvidos**:
- `01_core/src/entities/counter_state.rs` — adicionar mapa de labels
- `01_core/src/rules/layout.rs` — duas passagens em `layout()` / `layout_with_state()`

**Não bloqueante**: numeração sequencial de headings funciona. Referências para a frente
são o único cenário não coberto.

---

## DEBT-9 — Cobertura de paridade — tracking contínuo

**Estado**: Baseline estabelecido no Passo 35. Sem divergências no corpus actual.

**Descrição**: O parity_runner testa 50 inputs (40 markup/code/math gerais + 10 math
específicos do Passo 34). Todos passam. À medida que o motor de equações e novas
funcionalidades forem implementadas, adicionar casos de paridade correspondentes.

**Quando expandir**: A cada passo que adicione novo SyntaxKind ou altere semântica do parser.

**Referência**: `lab/parity/tests/parse_parity.rs`, baseline em
`00_nucleo/materialization/parity-baseline-passo-35.md`
