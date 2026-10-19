# Passo 117 — Relatório (CLI migrada para L2)

**Data**: 2026-04-23
**Precondição**: Passo 116 encerrado; 811 L1 + 207 L3 + 5 L4 + 6
ignorados; zero violations.
**Natureza**: correcção arquitectural pura. Zero mudança de UX.
**ADR criada**: ADR-0049 "CLI vive em L2" — **PROMOVIDA A EM VIGOR**
em 117.E.
**ADRs anotadas**: 0046, 0047, 0048 — cada uma ganha nota de
correcção de camada (decisões funcionais preservadas).

---

## Sumário

Código que habitava L3 (`ColorWhen`, `resolve_colored_with`, testes)
e L4 (`Args`, `resolve_colored`) foi **migrado para L2**
(`02_shell/src/cli.rs`). L2 passou de stub (5 linhas de header) a
módulo real com ~150 linhas. L3 voltou a ser I/O puro (sem `clap`).
L4 ficou composição thin (~75 linhas).

Comportamento externo **inalterado**: `typst input.typ output.pdf
--color=auto|always|never` funciona exactamente como no Passo 116.
Tests 114 passam sem modificação.

**811 L1 + 201 L3 (−6) + 6 L2 (+6) + 5 L4** + 6 ignorados. Total
**1023 tests** (inalterado — redistribuição pura). Zero violations.
**49 ADRs activas** (+0049).

---

## 117.A — Inventário

Inventário em
`00_nucleo/diagnosticos/inventario-l2-refactor-passo-117.md`.

| Decisão | Escolha |
|---------|---------|
| Localização | `02_shell/src/cli.rs` dedicado (>50 linhas) |
| `lib.rs` | `pub mod cli;` + header |
| `clap` em L2 | Sim (novo) |
| `clap` em L3 | Remover |
| `clap` em L4 | Remover |
| Testes total | 1023 (inalterado) |

Gate 117.A não disparou — L2 era stub limpo.

---

## 117.B — ADR-0049

Criada em `00_nucleo/adr/typst-adr-0049-cli-em-l2.md`.
**EM VIGOR em 117.E**.

Pontos-chave:

- **Corrige ADRs 0046/0047/0048** especificamente na **camada**.
  Decisões funcionais (clap, cores, paleta, flags) **mantêm-se**.
- `02_shell/src/cli.rs` ganha `Args`, `ColorWhen`, `RunIntent`,
  `parse()`, `resolve_colored_with`.
- L3 perde `clap` como dep.
- L4 fica ~75 linhas de composição; sem `clap` directo.
- ADRs 0046/0047/0048 ganham "Nota Passo 117" documentando
  correcção de camada sem revogação.

---

## 117.C — Implementação

### Ficheiros criados

1. `02_shell/src/cli.rs` — ~150 linhas com `ColorWhen`, `Args`
   (privado), `RunIntent`, `parse`, `resolve_colored` (privado),
   `resolve_colored_with` (público), 6 testes unitários.
2. `00_nucleo/prompts/shell/cli.md` — prompt L0 do novo módulo.

### Ficheiros alterados

| Ficheiro | Mudança |
|----------|---------|
| `02_shell/Cargo.toml` | +`clap = { workspace = true }`. |
| `02_shell/src/lib.rs` | `pub mod cli;` + header actualizado. |
| `03_infra/Cargo.toml` | **Remove `clap`**. |
| `03_infra/src/diagnostic_format.rs` | Remove `ColorWhen`, `resolve_colored_with`, 6 testes `resolve_colored_*`. Mantém paleta ANSI, `format_diagnostic(colored: bool)`, `drain_diagnostics_to_stderr`, 7 testes `format_diagnostic_*`. |
| `04_wiring/Cargo.toml` | **Remove `clap`**. |
| `04_wiring/src/main.rs` | Simplifica para 75 linhas: chama `cli::parse()`, orquestra pipeline, propaga `colored`. Zero imports `clap`. |
| `00_nucleo/prompts/shell.md` | Reescrito — aponta para `cli` submodule. |
| `00_nucleo/prompts/infra/diagnostic_format.md` | Documenta remoção de `ColorWhen` e remoção de dep `clap`. |
| `00_nucleo/prompts/wiring.md` | Reescrito — L4 como composição thin. |
| `00_nucleo/adr/typst-adr-0046-cli-minima.md` | +Nota Passo 117. |
| `00_nucleo/adr/typst-adr-0047-cli-clap.md` | +Nota Passo 117. |
| `00_nucleo/adr/typst-adr-0048-diagnosticos-com-cor.md` | +Nota Passo 117. |

### `main.rs` (L4) — diff concetual

```diff
- use clap::Parser;
- #[derive(Parser, Debug)] struct Args { ... ColorWhen }
- fn resolve_colored(...) { ... }
  use typst_shell::cli::{self, RunIntent};

  fn main() -> ExitCode {
-     let args = Args::parse();
-     let input = args.input;
-     let output = args.output;
-     let colored = resolve_colored(&args.color);
+     let RunIntent { input, output, colored } = cli::parse();
      // ... resto do pipeline inalterado
  }
```

L4 perdeu 25 linhas (Args + ColorWhen + resolve_colored) e ganhou
1 linha (import `RunIntent`). Líquido: ~25 linhas mais magras.

### Deps workspace — inalterado

`clap` continua em `[workspace.dependencies]`. Só muda onde é
**consumido**:
- Antes: L3 + L4.
- Depois: **apenas L2**.

### Distribuição de testes

| Crate | Antes | Depois | Δ |
|-------|------:|-------:|---|
| `typst-core` (L1) | 811 | 811 | 0 |
| `typst-shell` (L2) | 0 | **6** | +6 |
| `typst-infra` (L3) | 207 | **201** | −6 |
| `typst-wiring` tests/ (L4) | 5 | 5 | 0 |
| **Total workspace** | 1023 | **1023** | 0 |

Redistribuição pura. Os 6 testes `resolve_colored_*` moveram-se
de L3 para L2 (próximos da definição) com corpo idêntico.

---

## 117.D — Validação

### Tests automatizados

```
$ cargo test --workspace | grep "test result"
test result: ok. 811 passed ...  (L1 inalterado)
test result: ok. 201 passed ...  (L3 −6: resolve_colored_* movidos)
test result: ok. 6 passed   ...  (L2 +6: resolve_colored_* aqui)
test result: ok. 5 passed   ...  (L4 integration — inalterado)
```

Tests 114 passam **sem modificação** — comportamento externo
inalterado.

### Validação manual

Todos os cenários idênticos ao Passo 116:

```bash
$ ./target/release/typst /tmp/test.typ /tmp/out.pdf
/tmp/test.typ:5:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
# exit 0

$ ./target/release/typst /tmp/test.typ /tmp/out.pdf --color=always 2>&1 | cat
[2m/tmp/test.typ:5:11[0m: [1;33mwarning[0m: [1mtext: ...[0m
  [1;36mhint[0m: ver ADR-0040 ...
# ANSI escapes presentes

$ ./target/release/typst --help
Typst compiler (crystalline)
Usage: typst [OPTIONS] <INPUT> <OUTPUT>
# ... flags idênticas ao Passo 116

$ ./target/release/typst
error: the following required arguments were not provided:
  <INPUT>
  <OUTPUT>
# exit 2
```

---

## 117.E — Encerramento

### Verificação

```
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 1.57s

$ cargo test --workspace  (ver tabela acima)
Total: 1023 (inalterado)

$ crystalline-lint .
✓ No violations found
```

### ADR

**ADR-0049** `EM VIGOR`. ADRs **0046, 0047, 0048** anotadas com
"Nota Passo 117 — camada corrigida".

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 (inalterado) |
| L2 tests | 0 | **6** (+6) |
| L3 tests | 207 | **201** (−6) |
| L4 tests | 5 | 5 (inalterado) |
| **Total** | **1023** | **1023** (redistribuição) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 48 | **49** (+0049) |
| DEBTs abertos | 11 | 11 (inalterado) |
| `L2` linhas de código | ~5 (header) | **~150** (+cli.rs) |
| `L3` deps externas | 4 | 3 (−clap) |
| `L4` deps externas | 5 | 4 (−clap) |
| `L4` linhas `main.rs` | 99 | **~75** (thin) |

---

## Lições

1. **Correcção arquitectural é passo legítimo**: 4 passos consecutivos
   (113, 115, 116, 117) colocaram CLI em camadas erradas. O erro só
   foi detectado quando V12 disparou em Passo 116; a correcção mesmo
   só aconteceu em Passo 117. O atraso deve-se a **resistência
   menor** — mover `ColorWhen` para L3 em 116 (onde já havia outras
   deps) era mais fácil que para L2 (onde não havia nada). Lição:
   tratar stubs vazios como sinal de omissão, não conveniência.

2. **ADR que corrige sem revogar**: ADR-0049 corrige especificamente
   a **camada** das ADRs 0046/0047/0048, mantendo as decisões
   funcionais. Padrão novo para o projecto — evita inflar
   "revogações" quando só a localização está errada. Documentar em
   "Nota Passo 117" nas ADRs corrigidas preserva contexto histórico.

3. **Fronteira L3/L2 agora clara**: L3 é I/O (filesystem, fonts,
   export, formatter sem deps de CLI). L2 é "apresentação ao
   utilizador" (argparsing, cores, eventualmente formatters
   high-level). L4 é composição de L2+L3. Esta divisão vem desde o
   Passo 0 mas só agora é enforçada em código.

4. **`RunIntent` explicita o contrato L2→L4**: em vez de L4 importar
   `clap::Parser` + `Args`, L2 devolve struct pura
   (`RunIntent { input, output, colored }`). L4 consome dados, não
   tipos de argparsing. Facilita passos futuros — `RunIntent` ganha
   campos sem L4 precisar de conhecer clap.

5. **V12 é detector útil, mas só quando atendido**: a violation no
   Passo 116 sinalizou o problema, mas foi tratada como "menor mal"
   (mover para L3 em vez de L2). Um passo dedicado a corrigir
   completamente é mais limpo do que contornos.

6. **Redistribuição ≠ regressão**: 1023 tests totais iguais.
   Deslocar 6 testes de L3 para L2 preserva cobertura e aproxima
   os testes da definição. `cargo test --workspace` valida tudo
   de uma vez — não importa onde os testes vivam.

7. **L4 ficou thin**: 75 linhas vs 99 do Passo 115. Adicionar
   flags futuras vai a L2; L4 não cresce. Se alguma flag precisar
   de "algo mais" que `RunIntent` não captura, é sinal de que
   falta um campo em `RunIntent`, não que L4 precisa de lógica.

---

## Estado pós-Passo 117

### Topologia final

```
L1 (01_core)       — entities, contracts, rules. Pure domain.
L2 (02_shell)      — CLI (clap, argparsing, ColorWhen, RunIntent).
L3 (03_infra)      — I/O (SystemWorld, export_pdf, format_diagnostic).
L4 (04_wiring)     — composição pura (main chama L2 → L3).
```

Deps externas por camada:

- L1: comemo, thiserror, time, indexmap, ecow, rustc_hash, unicode_*.
- **L2**: clap (novo), typst-core, anyhow.
- L3: typst-core, thiserror, comemo, ttf-parser, rustybuzz, time,
  image, flate2.
- L4: typst-core, typst-shell, typst-infra, anyhow.

### Trabalho futuro identificado

1. **Flags funcionais** (`--root`, `--font-path`, `-o/--output`,
   `-f/--format`) — **entram em L2**. Padrão estabelecido.
2. **Subcomandos** (`compile`, `watch`, etc.) — entram em L2.
3. **PNG/SVG/HTML exports** — funções em L3; selecção em L2
   (via `-f`).
4. **JSON/SARIF diagnostics** — formatter adicional em L3; flag em L2.
5. **Auditoria de L4**: se alguma vez L4 crescer além de ~100
   linhas, sinal de que lógica escapou — mover para L2/L3.
