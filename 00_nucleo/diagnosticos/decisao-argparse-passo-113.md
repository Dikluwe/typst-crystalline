# Passo 113.A — Decisões diferidas (argparsing + helpers)

**Data**: 2026-04-23
**Input**: análise Passo 112.

---

## Parte 1 — Argparsing

### Grep `clap|argh|structopt` no workspace

`Cargo.toml` raiz: **zero matches**. Nenhuma lib de argparsing
declarativo está em `[workspace.dependencies]`.

### Decisão: **manual** (`std::env::args`)

Razões:
1. **Zero deps novas**. Adicionar `clap` ao workspace exige
   decisão cross-cutting (versão, features) que vai além do
   escopo deste passo.
2. **Positional `input output` chega para MVP**. 2 args é
   trivialmente parseável à mão.
3. **Passo futuro pode migrar para clap** quando flags forem
   adicionadas (Candidato 3 do Passo 112). Custo de migração é
   localizado em 04_wiring.

### Implicações

- `main.rs` tem ~10 linhas de parsing manual (match sobre
  `&args[..]`).
- Help mensagem é manual: `Usage: typst <input.typ> <output.pdf>`.
- `--version` e `--help` **não existem** neste passo.
- Mensagens de erro de argparsing são manuais.

### Versão alternativa (clap) — não escolhida

Se `clap` fosse adicionado:

```toml
# Cargo.toml raiz
[workspace.dependencies]
clap = { version = "4", features = ["derive"] }

# 04_wiring/Cargo.toml
clap = { workspace = true }
```

```rust
// main.rs
#[derive(Parser)]
struct Args { input: PathBuf, output: PathBuf }
```

Custo: +1 dep, ~15 linhas de derive. Benefícios: `--help`
gratuito, mensagens de erro automáticas. **Adiado** para passo
que adicione flags.

---

## Parte 2 — Helpers test-only

### Grep em `03_infra/src/integration_tests.rs`

Quatro funções úteis à CLI, todas `fn` (não `pub fn`) dentro de
`#[cfg(test)] mod integration`:

| Função | Linhas | Dependências |
|--------|-------:|--------------|
| `do_eval_with_sink(&SystemWorld, &Source) -> (SourceResult<Module>, Vec<SourceDiagnostic>)` | 20 | `Routines`, `Traced`, `Sink`, `Route`, `eval`, `comemo::Track` |
| `format_diagnostic(&SourceDiagnostic, &Source, &str) -> String` | 24 | `Severity`, `SourceDiagnostic`, `Source::span_to_line_col` (Passo 111) |
| `drain_diagnostics_to_stderr(&[SourceDiagnostic], &Source, &str)` | 9 | `format_diagnostic` |
| `compile_to_pdf(&str) -> Vec<u8>` | 13 | `world_from_str` (test helper), `do_eval`, `introspect`, `layout`, `export_pdf` |

### Análise de visibilidade

Tipos usados:
- `Routines`, `Traced`, `Sink`, `Route` — todos `pub` em
  `typst_core::entities::world_types`.
- `SourceDiagnostic`, `Severity` — todos `pub` em
  `typst_core::entities::source_result`.
- `SourceResult<T>`, `Module`, `Source` — todos `pub`.
- `eval`, `introspect`, `layout`, `export_pdf` — todos `pub`.
- `Source::span_to_line_col` — `pub` (Passo 111).
- `comemo::Track` — `pub` (crate externa).

**Cadeia de visibilidade: zero problemas.** Todos os tipos já são
`pub`. Gate 113.A.2 não dispara.

### Decisão: **promover** (opção B do spec)

Razões:
1. **Reuso > duplicação**: `do_eval_with_sink`, `format_diagnostic`,
   `drain_diagnostics_to_stderr` serão re-usados tanto pela CLI
   como pelos testes existentes. Duplicar criaria divergência
   potencial.
2. **Zero mudança de visibilidade em tipos** — só as 3 funções
   helper mudam de `fn` privado para `pub fn` público.
3. **Organização melhora**: `03_infra` ganha módulos públicos
   com API clara — `pipeline`, `diagnostic_format`.

### Localização escolhida

- **`03_infra/src/pipeline.rs`** (novo) — `do_eval_with_sink`
  (renomeado para `eval_to_module_with_sink` para clareza) +
  função alto-nível `compile_to_pdf_bytes(&SystemWorld, &Source)
  -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>)`
  que orquestra eval → introspect → layout → export_pdf.
- **`03_infra/src/diagnostic_format.rs`** (novo) — `format_diagnostic`
  + `drain_diagnostics_to_stderr`.

`compile_to_pdf` de test-only **não é movido directamente** —
depende de `world_from_str` (test helper específico). Substituído
por `compile_to_pdf_bytes(world, source)` em `pipeline.rs` que é
mais geral.

### Consequência em `03_infra/Cargo.toml`

`comemo` hoje é `[dev-dependencies]`. Para `do_eval_with_sink`
chamar `eval()` + `.track()` em produção, `comemo` move para
`[dependencies]`:

```toml
[dependencies]
typst-core = { path = "../01_core" }
thiserror  = { workspace = true }
comemo     = { workspace = true }  # NOVO — promovido de dev
ttf-parser = { workspace = true }
# ...
```

Custo: zero (workspace dep já declarada).

### Testes existentes

Os helpers privados em `integration_tests.rs` são reescritos como
thin wrappers sobre as versões `pub`. Ou os testes passam a
chamar directamente o `pub` path. Decisão de 113.C.

---

## Conclusões 113.A

| Decisão | Escolha | Razão |
|---------|---------|-------|
| Argparsing | **Manual** (`std::env::args`) | clap não está no workspace; positional 2 args é suficiente. |
| Helpers | **Promover** para `pub` | Zero visibility chain; reuso > duplicação; organização melhora. |
| Localização | `03_infra/src/pipeline.rs` + `03_infra/src/diagnostic_format.rs` | Módulos coesos por domínio (ADR-0037). |
| Comemo | `[dependencies]` em 03_infra | Requisito para promover `do_eval_with_sink`. Custo zero. |

Gate 113.A.2 **não disparado** — todos os tipos são `pub`.
**Pronto para 113.B e 113.C.**
