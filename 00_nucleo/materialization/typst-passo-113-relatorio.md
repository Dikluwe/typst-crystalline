# Passo 113 — Relatório de encerramento (CLI mínima em 04_wiring)

**Data**: 2026-04-23
**Precondição**: Passo 112 encerrado (análise); 811 L1 + 189 L3 + 6
ignorados; zero violations.
**ADR criada**: ADR-0046 "CLI mínima em L4 — compile com
diagnostics" — **PROMOVIDA A EM VIGOR** em 113.E.

---

## Sumário

`04_wiring/src/main.rs` passou de stub (11 linhas, `println!` só)
a **CLI real** (99 linhas) que compila `input.typ → output.pdf`
via `SystemWorld` + pipeline L3 + formatter ADR-0045.

Teste manual end-to-end passa: compila, emite warning DEBT-49 no
formato gcc/clang, escreve PDF 1195 bytes, exit 0. Errors dão
exit 1, I/O errors dão exit 2.

**Crescimento**: 811 L1 (inalterado) + **195 L3** (+6 testes novos
em `pipeline.rs` e `diagnostic_format.rs`). Zero violations.

---

## 113.A — Decisões diferidas

Inventário em
`00_nucleo/diagnosticos/decisao-argparse-passo-113.md`.

| Decisão | Escolha | Razão |
|---------|---------|-------|
| Argparsing | **Manual** (`std::env::args`) | `clap` ausente do workspace; positional 2 args chega para MVP. |
| Helpers | **Promover** para `pub` | Zero cadeia de visibilidade; reuso > duplicação; organização coesa. |
| Localização | `pipeline.rs` + `diagnostic_format.rs` | ADR-0037 (coesão por domínio). |
| Comemo | `[dependencies]` em 03_infra | Requisito para promover `do_eval_with_sink`. Custo zero. |

Gate 113.A.2 **não disparou** — todos os tipos usados são `pub`.

---

## 113.B — ADR-0046

Criada em `00_nucleo/adr/typst-adr-0046-cli-minima.md`.
**EM VIGOR em 113.E**.

Pontos-chave:

- **Argparsing manual** — clap fica para passo que adicione flags.
- **`SystemWorld` real** — L3 já tem, sem materialização nova.
- **Pipeline L3** (`pipeline.rs` + `diagnostic_format.rs`)
  promovido de test-only para API pública.
- **Exit codes**: 0 sucesso, 1 erro compilação, 2 erro I/O.
- **Tudo diagnóstico para stderr** — stdout nunca usado.

---

## 113.C — Implementação

### Ficheiros criados

1. `03_infra/src/pipeline.rs` — `eval_to_module_with_sink` +
   `compile_to_pdf_bytes` + 3 testes.
2. `03_infra/src/diagnostic_format.rs` — `format_diagnostic` +
   `drain_diagnostics_to_stderr` + 3 testes.
3. `00_nucleo/prompts/infra/pipeline.md` — prompt L0.
4. `00_nucleo/prompts/infra/diagnostic_format.md` — prompt L0.

### Ficheiros alterados

| Ficheiro | Mudança |
|----------|---------|
| `04_wiring/src/main.rs` | Stub (11 linhas) → **CLI real (99 linhas)**. |
| `03_infra/Cargo.toml` | `comemo` movido de `[dev-dependencies]` → `[dependencies]`. |
| `03_infra/src/lib.rs` | Adiciona `pub mod diagnostic_format; pub mod pipeline;`. |
| `03_infra/src/integration_tests.rs` | Remove helpers duplicados; usa API pública (`use crate::pipeline::...`; `use crate::diagnostic_format::...`). |
| `00_nucleo/prompts/wiring.md` | Reescrito descrevendo a CLI real. |

### API nova pública em 03_infra

```rust
// src/pipeline.rs
pub fn eval_to_module_with_sink(
    world: &dyn World,
    source: &Source,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>);

pub fn compile_to_pdf_bytes(
    world: &dyn World,
    source: &Source,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

```rust
// src/diagnostic_format.rs
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    source: &Source,
    source_path: &str,
) -> String;

pub fn drain_diagnostics_to_stderr(
    diagnostics: &[SourceDiagnostic],
    source: &Source,
    source_path: &str,
);
```

### `04_wiring/src/main.rs` — esqueleto (99 linhas)

```rust
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let (input, output) = match parse_args(&args) { ... };
    let root = input.parent()...;
    let main_path = input.file_name()...;
    let world = SystemWorld::new(&root, &main_path)?;
    let source = world.source(world.main())?;
    let (result, warnings) = compile_to_pdf_bytes(&world, &source);
    drain_diagnostics_to_stderr(&warnings, &source, &source_path);
    match result {
        Ok(pdf) => fs::write(&output, pdf).map(|_| ExitCode::SUCCESS)...
        Err(errors) => {
            drain_diagnostics_to_stderr(&errors, &source, &source_path);
            ExitCode::from(1)
        }
    }
}

fn parse_args(args: &[String]) -> Option<(PathBuf, PathBuf)> {
    match args {
        [_bin, input, output] => Some((PathBuf::from(input), PathBuf::from(output))),
        _ => None,
    }
}
```

### Contagem de linhas

- `04_wiring/src/main.rs`: 99 (de 11 do stub; +88).
- `03_infra/src/pipeline.rs`: 152 (com testes).
- `03_infra/src/diagnostic_format.rs`: 93 (com testes).
- `03_infra/src/integration_tests.rs`: -95 (helpers duplicados
  removidos, imports simplificados).

Líquido: ~240 linhas novas de código de produção + testes.

---

## 113.D — Validação manual

Input (`/tmp/test113.typ`):

```typst
= Olá

Texto normal com *negrito* e _itálico_.

#set text(font: "Arial")
```

Invocação:

```bash
$ ./target/release/typst /tmp/test113.typ /tmp/test113.pdf
/tmp/test113.typ:5:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
$ echo $?
0
$ ls -l /tmp/test113.pdf
-rw-rw-r-- 1 dikluwe dikluwe 1195 abr 23 20:37 /tmp/test113.pdf
```

PDF válido: `%PDF-1.7` header + `startxref ... %%EOF` footer.

### Testes de erro

| Cenário | Comando | Exit | Output |
|---------|---------|-----:|--------|
| Sucesso com warning | `typst ok.typ ok.pdf` | 0 | warning + PDF |
| Erro de eval | `typst err.typ err.pdf` (input `#unknown_var`) | 1 | `/tmp/err.typ:1:2: error: unknown variable: ...` |
| I/O não encontrado | `typst inexistente.typ out.pdf` | 2 | `error: main file not found: ...` |
| Sem argumentos | `typst` | 2 | `Usage: typst <input.typ> <output.pdf>` |

Todos os exit codes correctos, todos os outputs no formato
esperado, nenhum output em stdout.

---

## 113.E — Encerramento

### Verificação

```
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 10.12s

$ cargo test --workspace | grep "test result"
test result: ok. 811 passed; 0 failed; 0 ignored ...  (L1 inalterado)
test result: ok. 195 passed; 0 failed; 6 ignored ...  (L3 +6 testes novos)

$ crystalline-lint .
✓ No violations found
```

### ADR promovida

**ADR-0046** `EM VIGOR`.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 (inalterado) |
| L3 tests | 189 | **195** (+6 pipeline/diagnostic_format) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 45 | **46** (+0046) |
| DEBTs abertos | 11 | 11 (inalterado) |
| `main.rs` linhas | 11 (stub) | **99** (CLI real) |

---

## Limitações aceites

1. **Sem flags** (`--root`, `--font-path`, `--output`, `--format`,
   `--help`, `--version`). Positional apenas.
2. **Sem subcomandos** (watch, query, init, eval, fonts, update,
   completions, info).
3. **Fonte default Helvetica Type1** (via `export_pdf`).
   Unicode rico parcialmente quebrado.
4. **Sem cores ANSI** nos diagnósticos.
5. **Cross-file spans** → fallback `<detached>` (herdado ADR-0045).
6. **Windows paths** com `\` no formato de diagnóstico — editores
   em Windows podem ter dificuldade.
7. **Sem `-` (stdin/stdout)** — input e output são sempre
   ficheiros.

---

## Lições

1. **Gate 113.A.2 era falso alarme**: a preocupação com cadeia de
   visibilidade não se materializou — todos os tipos consumidos
   pelos helpers test-only já eram `pub` em L1. Promover foi
   reorganização mecânica.

2. **`comemo` em `[dev-dependencies]` era deliberação antiga**
   (Passo 32): isolamento de comemo a test-only. Movê-lo para
   `[dependencies]` foi custo zero (workspace dep) e desbloqueou
   API pública em L3.

3. **Argparsing manual é estranhamente OK**: para 2 args
   positional, match sobre `&args[..]` é mais directo que derive
   macros. 10 linhas incluindo mensagem de uso. Clap adicionar-se-á
   quando trouxer valor (flags).

4. **Stdout vs stderr disciplina**: tudo diagnóstico em stderr
   desde o início, mesmo `Usage:`. Se o utilizador redirecciona
   stdout para ficheiro (p.e., `2>/dev/null`), ainda vê ajuda.

5. **Path handling com `input.parent()` + `file_name()`**:
   `SystemWorld::new` espera `root` e `main` separados.
   `canonicalize` passou a ser aplicado dentro do SystemWorld
   (Passo anterior). Ter `file_name()` como "main path"
   relativo funciona.

6. **Teste manual mais útil que o spec antecipou**: o spec
   antecipava output detalhado do teste manual — e de facto
   confirmou que warnings DEBT-49 (span numbered) resolvem com
   linha/coluna correctas em produção, não só em testes.

---

## Estado pós-Passo 113

### CLI funcional

```
$ typst input.typ output.pdf
input.typ:5:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
```

Compila ficheiros `.typ` reais do filesystem. Utilizador externo
pode usar o binário (`cargo install --path 04_wiring` quando
aparecer a decisão).

### Trabalho futuro identificado

1. **Flags**: `--root`, `--font-path`, `--output`, `--format`
   com clap. Passo dedicado.
2. **Watch mode**: Candidato 4 do Passo 112. Passo dedicado com
   `notify`.
3. **Outros subcomandos**: `query`, `init`, `eval`, `fonts`.
   Cada um é passo próprio.
4. **Cores ANSI**: `--color=auto|always|never`, respeitar `NO_COLOR`.
5. **PNG/SVG/HTML export**: passos dedicados.
6. **`-` para stdin/stdout**: passo pequeno quando aparecer
   procura.
7. **CLI tests** (testar o binário via `std::process::Command`):
   sub-passo de verificação, ficou fora de 113.
