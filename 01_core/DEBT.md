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
