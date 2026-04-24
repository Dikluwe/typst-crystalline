# Passo 118.C — Auditoria de L4

**Data**: 2026-04-23
**Objectivo**: confirmar que L4 continua thin pós-Passo 117 e que
nenhuma lógica escapou para lá.

---

## Contagem

`04_wiring/src/main.rs`: **85 linhas** (incluindo header + vazias).

Linhas úteis (excluindo ~25 de header + comentários + vazias):
**~55 linhas de código efectivo**.

Limite mental estabelecido pelo Passo 117: ~100 linhas. Folga
confortável.

---

## Imports

```rust
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use typst_core::contracts::world::World;
use typst_infra::diagnostic_format::drain_diagnostics_to_stderr;
use typst_infra::pipeline::compile_to_pdf_bytes;
use typst_infra::world::SystemWorld;
use typst_shell::cli::{self, RunIntent};
```

### Análise

| Import | Legitimidade em L4 |
|--------|--------------------|
| `std::path::{Path, PathBuf}` | **OK** — path manipulation básica em composição. |
| `std::process::ExitCode` | **OK** — valor de retorno de `main()`. |
| `typst_core::contracts::world::World` | **OK** — trait usado para chamar `world.main()`, `world.source(id)`. |
| `typst_infra::diagnostic_format::drain_diagnostics_to_stderr` | **Fronteiriço** — se `format_diagnostic` migrar para L2, `drain_*` pode desaparecer (inline no main). |
| `typst_infra::pipeline::compile_to_pdf_bytes` | **OK** — composição L3 invocada por L4. |
| `typst_infra::world::SystemWorld` | **OK** — construção do World concreto. |
| `typst_shell::cli::{self, RunIntent}` | **OK** — L4 chama `cli::parse()`, consome `RunIntent`. |

**Zero `use clap::`**. Zero `use std::env::` (env vars só lidas em L2).
Zero `use std::io::` para escrita (o `eprintln!` macro-expande sem
`use`).

---

## Classificação linha-a-linha do `main()`

Grupos lógicos:

1. **Parse args → chama L2**: `cli::parse()` — **correcto** (L4
   consome L2).
2. **Derivar root + main_path**: `input.parent()`,
   `input.file_name()` — path manipulation de composição,
   **correcto**.
3. **Construir SystemWorld**: `SystemWorld::new(&root, &main_path)` —
   **correcto** (composição L3).
4. **Carregar source**: `world.source(world.main())` — **correcto**.
5. **Pipeline de compilação**: `compile_to_pdf_bytes(&world,
   &source)` — **correcto** (composição L3).
6. **Drenar warnings**: `drain_diagnostics_to_stderr(...)` —
   **correcto mas candidato** (se `format_diagnostic` migrar para
   L2, este drain desaparece de L3 e L4 passa a fazer
   `for ... eprint!(format_diagnostic_em_l2(...))`).
7. **Match no result + `fs::write`**: **correcto** (orquestração,
   I/O trivial).
8. **Drenar errors**: idem 6.

### Mensagens de erro inline

```rust
eprintln!("error: input path must have a file name: {}", input.display());
eprintln!("error: {}", e);               // SystemWorldError display
eprintln!("error: failed to load source: {:?}", e);
eprintln!("error: failed to write {}: {}", output.display(), e);
```

**Análise**: estas 4 mensagens são de **I/O errors** que acontecem
**antes** do pipeline (falha ao construir World) ou **depois**
(falha ao escrever PDF). Não passam pelo formatter
`format_diagnostic` porque não são `SourceDiagnostic` — são erros
do próprio L4 sobre args/I/O.

**Candidato**: poderiam ser migradas para L2 (`cli::emit_io_error(...)`)
para uniformidade? Possível mas cerimónia alta. Estas 4 mensagens
têm cada uma path + detalhe; formatar em L2 exigiria passar
contexto. Aceitável manter em L4 — são mensagens de **composição
falhou**, não diagnostics de compilação.

---

## Verificações finais

### Criação de tipos

`grep 'struct|enum|trait'` em `main.rs`: **zero matches**. L4 não
cria tipos (V12 respeitado).

### I/O

- `std::fs::write(&output, &pdf_bytes)` — escrita de PDF.
  **Aceitável** em L4 (composição final do pipeline).
- `eprintln!` — 4 mensagens de erro de args/I/O (não de eval).
  **Aceitável** em L4 (errors de composição).

### ExitCode

- `ExitCode::SUCCESS` — sucesso.
- `ExitCode::from(1)` — erro de eval.
- `ExitCode::from(2)` — erro de I/O/args.

Decisão de exit code é orquestração. **Correcto** em L4.

---

## Conclusão

**L4 pós-Passo 117 está thin e limpo.**

- **85 linhas total** (~55 código útil). Folga para 100.
- **Zero criação de tipos** — V12 respeitado.
- **Zero `clap` directo** — argparsing via L2.
- **Zero `std::env::`** — env vars via L2.
- **I/O de composição apenas** — `fs::write` para PDF, `eprintln!`
  para errors de composição.

**Nenhum candidato urgente de migração em L4.** O único
"candidato dependente" é remover `drain_diagnostics_to_stderr`
de L3 e inline em L4 quando/se `format_diagnostic` migrar para L2.
