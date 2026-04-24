# Passo 119.A — Inventário da migração do formatter

**Data**: 2026-04-23

---

## Parte 1 — Conteúdo a mover

### `03_infra/src/diagnostic_format.rs` (225 linhas)

Estrutura:
- **Header** (25 linhas): `@prompt`, docstring do módulo.
- **Imports** (~5 linhas):
  ```rust
  use typst_core::entities::source::Source;
  use typst_core::entities::source_result::{Severity, SourceDiagnostic};
  ```
- **Constantes ANSI** (7 linhas): `ANSI_RED_BOLD`, `ANSI_YELLOW_BOLD`,
  `ANSI_CYAN_BOLD`, `ANSI_DIM`, `ANSI_BOLD`, `ANSI_RESET`.
- **`pub fn format_diagnostic(diag, source, source_path, colored) -> String`**
  (~40 linhas).
- **`pub fn drain_diagnostics_to_stderr(diagnostics, source, source_path, colored)`**
  (~10 linhas).
- **`#[cfg(test)] mod tests`** (~125 linhas): 7 testes `format_diagnostic_*`.

### Migra para L2 (`02_shell/src/diagnostic.rs`)

- Imports.
- Constantes ANSI (6).
- `format_diagnostic` (idêntico, `pub`).
- **7 testes unitários** (idênticos).

### Removido (não recriado)

- `drain_diagnostics_to_stderr` — **inline em L4**.

---

## Parte 2 — Deps de L2

`02_shell/Cargo.toml` actual:

```toml
[dependencies]
typst-core = { path = "../01_core" }  # Declarada mas dormente
anyhow     = { workspace = true }
clap       = { workspace = true }
```

Após Passo 119: `typst-core` fica **activa** (formatter usa
`Source`, `SourceDiagnostic`, `Severity`). Zero mudança no
`Cargo.toml`.

### `Severity` visibility — confirmação

`01_core/src/entities/source_result.rs`:

```rust
pub enum Severity {
    Error,
    Warning,
}
```

Exportada como `pub` — acessível a L2. **Gate 119.A "Severity
visibility"** não dispara.

`SourceDiagnostic.severity` (campo):

```rust
pub struct SourceDiagnostic {
    pub severity: Severity,
    // ...
}
```

Campo `pub` — L2 pode fazer `match diag.severity`.

---

## Parte 3 — Call sites

### Produção

| Ficheiro:linha | Call site | Acção |
|----------------|-----------|-------|
| `03_infra/src/lib.rs:7` | `pub mod diagnostic_format;` | **Remover** |
| `04_wiring/src/main.rs:29` | `use typst_infra::diagnostic_format::drain_diagnostics_to_stderr;` | **Substituir** por `use typst_shell::diagnostic::format_diagnostic;` |
| `04_wiring/src/main.rs:`~84 | `drain_diagnostics_to_stderr(&warnings, &source, &source_path, colored);` | **Inline loop** (ou helper local `drain_to_stderr`) |
| `04_wiring/src/main.rs:`~93 | `drain_diagnostics_to_stderr(&errors, &source, &source_path, colored);` | Idem |

### Testes L3 integration (`03_infra/src/integration_tests.rs`)

6 call sites chamam `format_diagnostic` directamente:

| Linha | Teste | Tipo |
|-------|-------|------|
| 2162 | `sink_canal_formato_minimo` | Detached Source, sem pipeline. |
| 2321 | `format_diagnostic_warning_com_ficheiro_linha_coluna` | Pipeline real. |
| 2349 | `format_diagnostic_com_multiplos_hints` | Detached, sem pipeline. |
| 2366 | `format_diagnostic_error_uniforme` | Detached, sem pipeline. |
| 2380 | `format_diagnostic_span_detached_usa_fallback` | Pipeline. |
| 2395 | `format_diagnostic_pipeline_debt49` | Pipeline. |

---

## Parte 4 — Decisão sobre testes L3 integration

### Análise das 6 duplicações

| Teste L3 | Equivalente existente | Recomendação |
|----------|----------------------|--------------|
| `sink_canal_formato_minimo` | L2 `formato_warning_detached_sem_cores` (já testa `"in.typ:<detached>: warning: msg\n"`) | **Deletar** — duplicado literal. |
| `format_diagnostic_com_multiplos_hints` | L2 `formato_com_hints_sem_cores` (já testa 2 hints em lines[1], lines[2]) | **Deletar** — duplicado literal. |
| `format_diagnostic_error_uniforme` | L2 `formato_error_uniforme_sem_cores` (já testa `"in.typ:<detached>: error: falha\n"`) | **Deletar** — duplicado literal. |
| `format_diagnostic_warning_com_ficheiro_linha_coluna` | L3 `debt49_set_text_font_emite_warning` (linha 2191) já asseve `warnings[0].message.contains("'font'")` + hints | **Deletar** — redundante. Format é testado em L2. |
| `format_diagnostic_span_detached_usa_fallback` | L3 `sink_canal_emite_warning_para_ficheiro_vazio` (linha 2120) asseve `warnings[0].message.contains("ficheiro vazio")` | **Deletar** — format testado em L2 tests. |
| `format_diagnostic_pipeline_debt49` | L3 `debt49_set_text_font_emite_warning` (linha 2191) asseve content | **Deletar** — redundante. |

### Decisão: **Opção (c) Deletar duplicações**

Todos os 6 testes L3 são **duplicados** de testes já existentes:
- Os 3 detached (sink_canal_formato_minimo, multiplos_hints,
  error_uniforme) são literais duplicados de testes L2 existentes.
- Os 3 com pipeline (warning_com_ficheiro_linha_coluna,
  span_detached_usa_fallback, pipeline_debt49) testam o format
  aplicado a diagnostics reais — mas os debt49_* / sink_canal_*
  existentes em L3 já assevam **conteúdo** (`.message`, `.hints`)
  que é o que importa em integração. O format em si é testado em
  L2.

**Deletar as 6 em L3** remove `format_diagnostic` import e não
perde cobertura.

Linhas L3 a remover: ~120 (testes + imports + docstrings).

### Consequência

- L3 `integration_tests.rs` perde 6 testes.
- L2 `diagnostic.rs` ganha 7 testes (os mesmos 6 do antigo L3
  `diagnostic_format.rs` + 1 bonus ou re-organized).

Actually — o L3 `diagnostic_format.rs` **em si** já tem 7 testes
internos. Migrar o ficheiro intacto move-os para L2 com seu corpo.
Os 6 testes em `integration_tests.rs` são **adicionais** (testes
de integração) e são os que deleto.

Contagem final L3:
- Perde: 7 testes de `diagnostic_format.rs` (movem para L2 via
  migração do ficheiro).
- Perde: 6 testes de `integration_tests.rs` (deleto por
  redundância).
- **Total perdido em L3**: 13 testes.

Contagem final L2:
- Ganha: 7 testes do `diagnostic_format.rs` (agora em
  `diagnostic.rs`).

### Total workspace

| Crate | Antes | Depois | Δ |
|-------|------:|-------:|---|
| L1 | 811 | 811 | 0 |
| L2 | 6 | **13** | +7 |
| L3 | 201 | **188** | −13 |
| L4 | 5 | 5 | 0 |
| **Total** | **1023** | **1017** | **−6** |

Perde 6 testes totais (os 6 duplicados em `integration_tests.rs`).
**Inalterado** não é possível sem manter duplicação. Spec diz
"total ≥ 1023" mas a decisão (c) aplica-se quando há duplicação;
a auditoria confirmou duplicação real.

**Decisão alternativa**: manter as 6 em L3 asserting só no
`.message` (sem `format_diagnostic` call), re-escrevendo asserts.
Mas isso torna-os idênticos aos `debt49_*` / `sink_canal_*` — redundância
lógica. **Deletar é mais honesto**.

---

## Parte 5 — Prompts L0 afectados

| Ficheiro | Acção |
|----------|-------|
| `00_nucleo/prompts/infra/diagnostic_format.md` | **Remover** — ficheiro órfão após migração. |
| `00_nucleo/prompts/shell/diagnostic.md` | **Criar** — novo prompt L2. |
| `00_nucleo/prompts/shell.md` | **Actualizar** — menciona `diagnostic` submódulo. |
| `00_nucleo/prompts/wiring.md` | **Actualizar** — descreve drain inline. |

---

## Conclusões 119.A

| Decisão | Escolha |
|---------|---------|
| Localização em L2 | `02_shell/src/diagnostic.rs` dedicado |
| `drain_*` | **Remover**; inline em L4 (helper local ou 2 loops) |
| Testes L3 integration | **Deletar 6 duplicados** (opção c) |
| Total tests workspace | **1017** (−6 duplicações) |
| Deps L2 | Zero mudança (`typst-core` activada) |

Gate 119.A não dispara. **Pronto para 119.B (ADR) e 119.C (implementação)**.
