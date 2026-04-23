# Passo 113 — CLI real em `04_wiring` (Candidato 2 — Mínimo com warnings)

**Série**: 113 (passo de construção; primeiro passo substantivo em
L4).
**Precondição**: Passo 112 encerrado (análise); 811 L1 + 189 L3 + 6
ignorados; zero violations.
**ADRs aplicáveis**: ADR-0033 (paridade funcional), ADR-0043
(canal Sink), ADR-0045 (formato de diagnósticos).
**ADR nova**: ADR-00NN "CLI mínima em L4 — compile com
diagnostics" — `PROPOSTO` em 113.B, `EM VIGOR` em 113.E.

---

## Objectivo

Substituir o stub `04_wiring/src/main.rs` (11 linhas,
`println!` apenas) por CLI real que:

1. Aceita invocação `typst input.typ output.pdf`.
2. Lê `input.typ` via `SystemWorld` (L3).
3. Corre pipeline: `eval` → `layout` → `export_pdf`.
4. Escreve bytes PDF em `output.pdf`.
5. Dreneia warnings (Sink) e errors (SourceResult::Err) para
   stderr via formatter do Passo 111.
6. Retorna exit code 0 em sucesso, 1 em erro.

Este passo **não**:
- Adiciona subcomandos (`watch`, `query`, `fonts`, `init`, etc.).
- Implementa cores ANSI, JSON, SARIF.
- Resolve imports cross-file com fontes alternativas
  (`SystemWorld` default é suficiente).
- Adiciona flags (`--root`, `--font-path`, etc.).
- Materializa infra nova em L1/L3 — só usa o que já existe.

---

## Decisões já tomadas

1. **Escopo**: Candidato 2 do Passo 112 (Mínimo com warnings).
2. **File loading**: `SystemWorld` real em L3 (já existe,
   production-ready).
3. **Pipeline**: reutilizar o que existe (`eval`, `layout`,
   `export_pdf`).
4. **Formatter**: reutilizar Passo 111
   (`format_diagnostic` + `drain_diagnostics_to_stderr`).

## Decisões diferidas para este passo (113.A)

5. **Argparsing**: manual (`std::env::args`) ou `clap`. Depende
   de:
   - Se `clap` está em `[workspace.dependencies]` do workspace.
     Se sim, custo é trivial.
   - Se positional `input output` chega para MVP.
   Recomendação preliminar: **manual** se clap não está já no
   workspace; **clap** se está.
6. **Helpers test-only**: promover `format_diagnostic`,
   `drain_diagnostics_to_stderr`, `do_eval_with_sink` de
   `#[cfg(test)]` para `pub`, ou re-implementar em `main.rs`.
   Depende de:
   - Número de linhas real dos helpers (análise 112 estimou ~30
     de duplicação se re-implementar).
   - Se algum helper depende de tipos que não são `pub` fora do
     módulo de testes.
   Recomendação preliminar: **promover** (reuso > duplicação).

---

## Escopo

**Dentro**:
- `04_wiring/src/main.rs` — CLI real (~120-150 linhas estimadas).
- `04_wiring/Cargo.toml` — adicionar deps conforme 113.A.
- `03_infra/src/` — promover helpers a `pub` se 113.A decidir.
  Ficheiros novos ou reorganização de `lib.rs` conforme
  necessário.
- `Cargo.toml` workspace — adicionar `clap` a
  `[workspace.dependencies]` se 113.A decidir.
- Testes: **nenhum teste novo obrigatório**. Os 189 L3
  existentes já cobrem o pipeline. Um teste de integração que
  invoca o binário (se viável no harness) é nice-to-have mas
  fora do escopo obrigatório.

**Fora**:
- Subcomandos adicionais.
- `--root`, `--font-path`, `--input`, flags.
- Watch mode.
- Cores ANSI.
- Outros exports (PNG, SVG, HTML).
- Mudar API pública de `eval()`, `layout()`, `export_pdf()`.

---

## Sub-passos

### 113.A — Inventário final (decisões diferidas)

**Parte 1 — Argparsing**:

1. `grep` em `Cargo.toml` raiz por `clap` em `[workspace.dependencies]`.
2. Se presente, registar versão e features. Se ausente, registar.
3. Decidir:
   - **Se clap está**: usar clap. Ganho: `--help`, `--version`,
     mensagens de erro gratuitas. Custo: 1 bloco derive, ~15
     linhas.
   - **Se clap não está**: manual. Ganho: zero deps novas,
     zero mudança no workspace. Custo: ~20 linhas de parsing
     positional + mensagens de erro manuais.
4. Documentar em `00_nucleo/diagnosticos/decisao-argparse-passo-113.md`.

**Parte 2 — Helpers test-only**:

1. Grep por `format_diagnostic`, `drain_diagnostics_to_stderr`,
   `do_eval_with_sink` em `03_infra/src/`.
2. Para cada, confirmar:
   - Está em `#[cfg(test)]` hoje.
   - Linhas exactas de código.
   - Dependências (tipos, imports que precisam de ser
     acessíveis).
3. Decidir:
   - **Promover a `pub`**: criar módulo público (ex:
     `03_infra/src/pipeline.rs` + `03_infra/src/diagnostic_format.rs`).
     Mover código. Adicionar re-exports em `lib.rs`. Custo:
     organização.
   - **Duplicar em main.rs**: copiar lógica para `04_wiring/src/main.rs`.
     Custo: ~30 linhas duplicadas até ser consolidado em passo
     futuro.
4. Documentar decisão com razão.

**Gate 113.A**: se o inventário revelar que algum helper depende
de tipo `pub(crate)` fora do módulo (ex: um `SourceDiagnostic`
com visibilidade restrita), promover exige mudar visibilidade de
múltiplos tipos — cadeia que pode inchar. Se cadeia > 3 tipos,
duplicar em main.rs é a via menos invasiva. Registar e
prosseguir.

### 113.B — ADR nova

Criar `00_nucleo/adr/typst-adr-00NN-cli-minima.md` com
`PROPOSTO`.

Conteúdo:

- **Contexto**: `04_wiring/main.rs` é stub desde o Passo 0.
  Passos 106, 107, 111 construíram infraestrutura de diagnósticos
  consumível; faltava CLI para a expor ao utilizador externo.
- **Decisão**:
  - CLI em `04_wiring/src/main.rs` com invocação
    `typst input.typ output.pdf`.
  - Argparsing: [manual | clap] conforme 113.A.
  - Helpers: [promovidos | duplicados] conforme 113.A.
  - Pipeline: `SystemWorld::new` → `eval` → `layout` →
    `export_pdf` → `write` output.
  - Diagnostics: `format_diagnostic` + `drain_diagnostics_to_stderr`
    para warnings e errors.
  - Exit codes: 0 sucesso, 1 erro de eval, 2 erro de I/O
    (leitura ou escrita).
- **Alternativas rejeitadas**:
  - **Subcomandos desde já**: fora do escopo. Passo dedicado
    para cada.
  - **Flags**: `--root`, `--font-path`, etc. — fora do escopo.
  - **Watch**: requer `notify` + threading. Passo separado.
- **Limitações documentadas**:
  - Sem cores, JSON, SARIF.
  - Fonte default (Helvetica Type1 via `export_pdf` ou fonte
    descoberta por `SystemWorld` se existe).
  - Cross-file diagnostics ainda usam fallback `<detached>`
    (ADR-0045).

Promover a `EM VIGOR` em 113.E.

### 113.C — Implementação

Ordem obrigatória:

**113.C.1 — Promoção de helpers (se decidido em 113.A)**:

Se a decisão foi **promover**, criar módulos públicos em
`03_infra/src/`. Mover funções. Adicionar a `lib.rs`:

```rust
pub mod pipeline;
pub mod diagnostic_format;
```

Verificar com `cargo build -p typst-infra` que compila.

Se a decisão foi **duplicar**, saltar este sub-passo.

**113.C.2 — `04_wiring/Cargo.toml`**:

Adicionar deps conforme 113.A:

```toml
[dependencies]
typst-core  = { path = "../01_core" }
typst-shell = { path = "../02_shell" }
typst-infra = { path = "../03_infra" }
anyhow      = { workspace = true }
# Se clap decidido em 113.A:
clap        = { workspace = true, features = ["derive"] }
```

Se clap foi adicionado ao workspace, editar `Cargo.toml` raiz:

```toml
[workspace.dependencies]
clap = { version = "4.x.x", features = ["derive"] }
```

Versão: verificar a usada por vanilla em
`lab/typst-original/Cargo.toml` para consistência.

**113.C.3 — `04_wiring/src/main.rs`**:

Substituir stub por CLI real. Esboço (ajustar conforme
113.A/113.C.1):

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash <hash>
//! @layer L4
//! @updated 2026-04-23

use std::path::PathBuf;
use std::process::ExitCode;

use typst_core::entities::source::Source;
use typst_infra::system_world::SystemWorld;
use typst_infra::pipeline::{compile_to_pdf};  // ou inline se duplicado
use typst_infra::diagnostic_format::{drain_diagnostics_to_stderr};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    
    let (input, output) = match parse_args(&args) {
        Some(pair) => pair,
        None => {
            eprintln!("Usage: typst <input.typ> <output.pdf>");
            return ExitCode::from(2);
        }
    };
    
    let root = input.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
    let world = match SystemWorld::new(root, input.clone()) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::from(2);
        }
    };
    
    let source = world.source(world.main())
        .expect("main source should load");
    let source_path = input.display().to_string();
    
    // Pipeline com Sink
    let (result, warnings) = do_eval_and_drain(&world, &source);
    
    // Imprimir warnings primeiro
    drain_diagnostics_to_stderr(&warnings, &source, &source_path);
    
    // Resultado
    match result {
        Ok(pdf_bytes) => {
            if let Err(e) = std::fs::write(&output, pdf_bytes) {
                eprintln!("error: failed to write output: {}", e);
                return ExitCode::from(2);
            }
            ExitCode::SUCCESS
        }
        Err(errors) => {
            drain_diagnostics_to_stderr(&errors, &source, &source_path);
            ExitCode::from(1)
        }
    }
}

fn parse_args(args: &[String]) -> Option<(PathBuf, PathBuf)> {
    match args {
        [_, input, output] => Some((PathBuf::from(input), PathBuf::from(output))),
        _ => None,
    }
}
```

Se **clap** escolhido em 113.A, substituir `parse_args` por
derive Args:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "typst")]
#[command(about = "Typst compiler (crystalline)")]
struct Args {
    /// Input .typ file
    input: PathBuf,
    /// Output PDF file
    output: PathBuf,
}

fn main() -> ExitCode {
    let args = Args::parse();
    // ... resto igual
}
```

**113.C.4 — Pipeline compile_to_pdf**:

Se helpers promovidos (113.C.1), `compile_to_pdf` já existe em
`03_infra/src/pipeline.rs`. Se duplicados, função equivalente
inline em `main.rs`:

```rust
fn do_eval_and_drain(
    world: &SystemWorld,
    source: &Source,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    // 1. Construir Sink
    let mut sink = Sink::new();
    // 2. Construir Route
    let mut route = Route::new();
    // 3. eval
    // 4. layout
    // 5. export_pdf
    // 6. drain sink
    todo!("copiar de integration_tests.rs ou promover")
}
```

Ajustar conforme a API real de `eval()`, `layout()`,
`export_pdf()`. A análise 112 confirmou que todos existem.

**113.C.5 — Header e hash**:

`main.rs` precisa de header cristalino actualizado:

```rust
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash <novo hash>
```

O prompt `wiring.md` hoje é minimal ("Em migração."). Passo 113
actualiza-o com descrição da CLI real:

```markdown
# Wiring — typst-wiring

CLI mínima para o compilador cristalino.

- Uso: `typst input.typ output.pdf`
- Pipeline: SystemWorld → eval → layout → export_pdf
- Diagnostics: warnings + errors via drain_diagnostics_to_stderr
  (ADR-0045).
- Exit codes: 0 sucesso, 1 erro de eval, 2 erro de I/O.

Argparsing: [manual | clap] (decidido em Passo 113).
```

Correr `crystalline-lint --fix-hashes .` para actualizar o
hash do header.

### 113.D — Validação manual

**Teste manual end-to-end** antes de verificações automáticas:

1. Criar `/tmp/test.typ`:
   ```
   = Olá
   
   Texto normal com *negrito* e _itálico_.
   
   #set text(font: "Arial")
   ```
2. Correr:
   ```bash
   cargo run --bin typst -- /tmp/test.typ /tmp/test.pdf
   ```
3. Verificar:
   - Ficheiro `/tmp/test.pdf` foi criado.
   - Stderr contém:
     ```
     /tmp/test.typ:5:11: warning: text: propriedade 'font' ainda não suportada
       hint: ver ADR-0040 para propriedades cobertas por set text
     ```
   - Exit code 0.
4. Teste de erro:
   - Input inválido: `#let x = }}}`.
   - Verificar stderr tem `error:` e exit code 1.
5. Teste I/O:
   - Input inexistente: `/tmp/nao-existe.typ`.
   - Verificar erro I/O e exit code 2.

### 113.E — Encerramento

1. `cargo build --release` em workspace root — deve compilar.
2. `cargo test --workspace` — testes existentes passam,
   contagem ≥ 811 L1 + 189 L3.
3. `crystalline-lint .` — zero violations.
4. Validação manual (113.D) passa.
5. ADR promovida a `EM VIGOR`.
6. Relatório `typst-passo-113-relatorio.md`:
   - Argparsing escolhido + razão.
   - Helpers promovidos ou duplicados + razão.
   - Linhas exactas de `main.rs` final.
   - Output do teste manual (input típico + stderr + PDF size).
   - Limitações aceites (sem flags, sem cores, etc.).

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 113.A escrito.
2. ADR-00NN criada e promovida.
3. `04_wiring/src/main.rs` implementa CLI mínima funcional.
4. Pipeline end-to-end funciona: `.typ → .pdf` + diagnostics em
   stderr.
5. Exit codes correctos (0, 1, 2).
6. Validação manual 113.D passa.
7. `cargo build --release` passa.
8. `cargo test --workspace` passa com contagem ≥ linha de base.
9. `crystalline-lint` zero violations.
10. Relatório 113.E escrito.

---

## O que pode sair errado

- **Gate 113.A.Parte 2**: cadeia de visibilidade. Se promover
  `format_diagnostic` exigir tornar `SourceDiagnostic` campos
  `pub` que hoje são `pub(crate)`, a cadeia propaga. Se > 3
  tipos, duplicar em main.rs em vez de promover.
- **`SystemWorld::new` tem assinatura inesperada**. A análise
  112 registou-a mas não confirmou API exacta. Pode aceitar
  `PathBuf` ou `&Path`; pode devolver `Result` ou `Self`.
  Ajustar conforme real.
- **`export_pdf` precisa de fonte**. Se a função sem argumentos
  de fonte só aceita Type1 Helvetica, Unicode falha. Duas vias:
  1. Tentar `discover_fonts` e usar `export_pdf_with_font` se
     houver fonte encontrada.
  2. Usar `export_pdf` simples e aceitar que PDF tem texto
     parcialmente quebrado para Unicode.
  Decidir em 113.C.4. Preferência: via 1, com fallback para
  via 2 se falhar.
- **Compilador emite warnings que o utilizador não espera**.
  O micro-piloto "ficheiro vazio" do Passo 106 pode disparar.
  Se for o caso, decidir:
  - Manter (warnings útil).
  - Remover o micro-piloto (trabalho em L1 — fora do escopo
    de 113).
  Preferência: manter; é warning correcto.
- **Path handling em Windows**. `input.display()` dá path
  com `\` em Windows. O formato `path:linha:coluna` funciona
  mas editores podem não abrir. Fora do escopo — aceitar e
  registar como limitação futura.
- **`cargo build --release` produz binário grande**. L1 + L3
  completos compilados. Tamanho esperado: 5-20 MB. Aceitar.
- **Stderr e stdout misturam em terminal**. Se algum `println!`
  ou log vai para stdout em vez de stderr, polui o output.
  Convenção: **tudo diagnóstico para stderr**, **nada para
  stdout excepto bytes do PDF** — e mesmo o PDF vai para
  ficheiro, não stdout. Revisar em 113.C.3.

---

## Notas operacionais

- Este é o primeiro passo substantivo em L4. Estabelece padrão
  para CLI futura. Decisões aqui propagam.
- Se 113.A decidir **manual**, um passo futuro migrará para
  clap quando adicionar flags. Trabalho duplicado é conhecido;
  trade-off aceite.
- Se 113.A decidir **clap**, prompts de help vêm automaticamente.
  Nice-to-have: `typst --help` mostra algo útil.
- `SystemWorld` em L3 já existe — não precisa de materializar
  mais. Se a API real exigir ajustes pequenos (ex: `.unwrap()`
  num `Result` que hoje é `Self`), ajustar sem expandir.
- O binário resultante chama-se `typst` (definido em
  `[[bin]] name = "typst"` no `Cargo.toml` desde o Passo 0).
  Mantém-se.
- Se o teste manual de 113.D falhar num passo específico,
  reportar o sub-passo exacto que falhou (ex: "2.C.4 exportar
  PDF: função devolve Err"). Não tentar contornar sem reportar.
