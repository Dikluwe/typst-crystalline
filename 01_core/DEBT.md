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
