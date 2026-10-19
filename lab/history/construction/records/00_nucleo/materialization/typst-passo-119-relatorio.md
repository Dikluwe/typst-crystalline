# Passo 119 — Relatório (formatter migrado para L2)

**Data**: 2026-04-23
**Precondição**: Passo 118 encerrado (auditoria); 811 L1 + 6 L2
+ 201 L3 + 5 L4 + 6 ignorados; 1023 total; zero violations.
**Natureza**: correcção arquitectural que **completa** o Passo 117.
**ADR criada**: ADR-0050 "Formatter em L2" — **PROMOVIDA A EM VIGOR**
em 119.E. **ADR-0049** anotada com nota de completude.

---

## Sumário

O Passo 118 auditoria confirmou que `format_diagnostic` era
candidato óbvio L2 e `drain_diagnostics_to_stderr` era candidato
dependente. Este passo migra ambos e elimina duplicação de testes.

Resultado: `03_infra/src/diagnostic_format.rs` **desaparece**.
L2 ganha `02_shell/src/diagnostic.rs` com 9 testes. L4 ganha
helper local `drain_to_stderr` (5 linhas).

Comportamento externo **inalterado**: `typst input.typ out.pdf
--color=auto|always|never` funciona exactamente como Passo 117.

**811 L1 + 15 L2 (+9) + 186 L3 (−15) + 5 L4** + 6 ignorados =
**1017 total** (−6 face a 1023: duplicados removidos sem perda
de cobertura). Zero violations. **50 ADRs activas** (+0050).

---

## 119.A — Inventário

Inventário em
`00_nucleo/diagnosticos/inventario-migracao-formatter-passo-119.md`.

**Decisões**:

| Decisão | Escolha |
|---------|---------|
| Localização L2 | `02_shell/src/diagnostic.rs` dedicado |
| `drain_diagnostics_to_stderr` | **Remover**; inline em L4 como helper local |
| Testes L3 integration duplicados | **Deletar 6** (opção c do spec) |
| `Severity` visibility | Já `pub` — gate não dispara |
| Deps L2 | Zero mudança (`typst-core` activa-se) |

Gate 119.A não disparou.

---

## 119.B — ADR-0050

Criada em `00_nucleo/adr/typst-adr-0050-formatter-em-l2.md`.
**EM VIGOR em 119.E**.

Pontos-chave:
- Completa ADR-0049 (que moveu parte da CLI para L2 mas deixou o
  formatter em L3).
- L3 perde `diagnostic_format.rs` **completamente**.
- L4 ganha helper local `drain_to_stderr` (5 linhas, função
  privada).
- Paleta ANSI (6 const) migra junto ao formatter.
- Testes duplicados eliminados — cobertura real preservada.

---

## 119.C — Implementação

### Ficheiros criados

1. `02_shell/src/diagnostic.rs` — 205 linhas (formatter + 9
   testes).
2. `00_nucleo/prompts/shell/diagnostic.md` — prompt L0.

### Ficheiros removidos

1. `03_infra/src/diagnostic_format.rs` — removido.
2. `00_nucleo/prompts/infra/diagnostic_format.md` — removido.

### Ficheiros alterados

| Ficheiro | Mudança |
|----------|---------|
| `02_shell/src/lib.rs` | +`pub mod diagnostic;` |
| `03_infra/src/lib.rs` | remove `pub mod diagnostic_format;` |
| `03_infra/src/integration_tests.rs` | remove `use crate::diagnostic_format::format_diagnostic;` + 6 testes duplicados |
| `04_wiring/src/main.rs` | substitui import de L3 `drain_*` por import de L2 `format_diagnostic`; adiciona helper local `drain_to_stderr`; 2 `drain_diagnostics_to_stderr` → 2 `drain_to_stderr` |
| `00_nucleo/prompts/shell.md` | menciona `diagnostic` submódulo |
| `00_nucleo/prompts/wiring.md` | descreve helper local + ADR-0050 |
| `00_nucleo/adr/typst-adr-0049-cli-em-l2.md` | +Nota Passo 119 |

### `main.rs` — diff conceitual

```diff
- use typst_infra::diagnostic_format::drain_diagnostics_to_stderr;
+ use typst_core::entities::source::Source;
+ use typst_core::entities::source_result::SourceDiagnostic;
+ use typst_shell::diagnostic::format_diagnostic;

  // ... em main:
-     drain_diagnostics_to_stderr(&warnings, &source, &source_path, colored);
+     drain_to_stderr(&warnings, &source, &source_path, colored);

-     drain_diagnostics_to_stderr(&errors, &source, &source_path, colored);
+     drain_to_stderr(&errors, &source, &source_path, colored);

+ // Helper local (não cria tipo — V12 OK):
+ fn drain_to_stderr(
+     diagnostics: &[SourceDiagnostic],
+     source: &Source,
+     source_path: &str,
+     colored: bool,
+ ) {
+     for diag in diagnostics {
+         eprint!("{}", format_diagnostic(diag, source, source_path, colored));
+     }
+ }
```

L4 cresce ~14 linhas (helper + 2 imports); perde 1 import. Líquido
~13 linhas. Ainda thin (~100 linhas totais, confortavelmente
abaixo de limite mental de ~120).

---

## 119.D — Testes + Validação manual

### Distribuição de testes

| Crate | Antes (Passo 118) | Depois (Passo 119) | Δ |
|-------|------:|-------:|---|
| `typst-core` (L1) | 811 | 811 | 0 |
| `typst-shell` (L2) | 6 | **15** | +9 |
| `typst-infra` (L3) | 201 | **186** | −15 |
| `typst-wiring` (L4 integration) | 5 | 5 | 0 |
| **Total workspace** | **1023** | **1017** | **−6** |

Perda de 6 testes corresponde a **6 duplicados removidos**:
- `sink_canal_formato_minimo` — duplicado literal de
  `formato_warning_detached_sem_cores`.
- `format_diagnostic_com_multiplos_hints` — duplicado de
  `formato_com_hints_sem_cores`.
- `format_diagnostic_error_uniforme` — duplicado de
  `formato_error_uniforme_sem_cores`.
- `format_diagnostic_warning_com_ficheiro_linha_coluna`,
  `format_diagnostic_span_detached_usa_fallback`,
  `format_diagnostic_pipeline_debt49` — testes de format em
  pipeline; cobertura **real** preservada por `debt49_*` e
  `sink_canal_*` que assevam `.message`/`.hints` directamente.

Os 9 testes do antigo `diagnostic_format.rs` (7 unit + 2 extra)
migraram intactos para `02_shell/src/diagnostic.rs`.

### Validação manual

```bash
$ ./target/release/typst /tmp/test.typ /tmp/out.pdf 2>&1 | cat
/tmp/test.typ:5:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text

$ ./target/release/typst /tmp/test.typ /tmp/out.pdf --color=always 2>&1 | cat
[2m/tmp/test.typ:5:11[0m: [1;33mwarning[0m: [1mtext: ...[0m
  [1;36mhint[0m: ver ADR-0040 ...

$ ./target/release/typst --help
Typst compiler (crystalline)
Usage: typst [OPTIONS] <INPUT> <OUTPUT>
# ... (idêntico ao Passo 117)

$ ./target/release/typst
error: the following required arguments were not provided:
# ... exit 2
```

Comportamento idêntico ao Passo 117. **Zero mudança observável
para utilizador externo**.

### Tests 114 (L4 integration)

Tests em `04_wiring/tests/cli.rs` passaram **sem modificação** —
assertions são sobre stderr output, não sobre imports internos.

---

## 119.E — Encerramento

### Verificação

```
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 1.62s

$ cargo test --workspace | grep "test result"
test result: ok. 811 passed ...  (L1 inalterado)
test result: ok. 186 passed ...  (L3 −15: diag_format removido + 6 dupes)
test result: ok. 15 passed  ...  (L2 +9: diagnostic.rs com 9 testes)
test result: ok. 5 passed   ...  (L4 integration inalterado)

$ crystalline-lint .
✓ No violations found
```

### ADR

**ADR-0050** `EM VIGOR`.
**ADR-0049** anotada: "migração completada por ADR-0050 no Passo 119".

---

## Números finais

| Métrica | Antes (Passo 118) | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 |
| L2 tests | 6 | **15** (+9) |
| L3 tests | 201 | **186** (−15) |
| L4 tests | 5 | 5 |
| **Total** | **1023** | **1017** (−6 dupes) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 49 | **50** (+0050) |
| DEBTs abertos | 11 | 11 |
| L3 módulos | `diagnostic_format`, `export`, `fonts`, `font_metrics`, `image_sizer`, `layout`, `pipeline`, `world` | **7** (sem diagnostic_format) |
| L2 módulos | `cli` | `cli`, **`diagnostic`** (+1) |
| L4 `main.rs` linhas | 85 | ~100 (+helper) |

---

## Lições

1. **Auditoria 118 foi precisa**: previu exactamente os 2
   candidatos (format + drain) + decisão "deletar dupes em L3".
   Passos de auditoria dedicados pagam-se — evitam migração
   cega.

2. **"Correcção arquitectural" é padrão honesto**: ADR-0049
   completada por ADR-0050 mostra que decisões de camada podem
   ser revisitadas sem revogação. O "Nota Passo N" nas ADRs
   preserva contexto histórico sem inflar lista de ADRs
   revogadas.

3. **Duplicação de tests é custo real**: os 6 testes L3
   duplicados cobriam o mesmo que L2 unit tests e L3
   field-assertion tests. Manter todos era 6× custo de
   manutenção sem ganho de cobertura. Deletar é limpeza.

4. **Helper local em L4 vs função em L2**: o `drain_to_stderr`
   de L4 (5 linhas) evita inversão de dep (L3 → L2) que
   aconteceria se `drain_*` ficasse em L3 após formatter migrar.
   **Não cria tipo** — V12 OK. L4 pode ter funções privadas
   de composição.

5. **L3 perdeu um ficheiro completo**: `diagnostic_format.rs`
   simplesmente desapareceu. Isto sinaliza que a auditoria 118
   estava correcta — o ficheiro não pertencia ali. L3 agora
   tem 7 módulos, todos de I/O puro (export, fonts,
   font_metrics, image_sizer, layout, pipeline, world).

6. **Paleta ANSI como unidade coesa**: moveu junto com o
   formatter. Coesão > duplicação potencial futura. Se L3
   algum dia precisar de ANSI, duplica 6 linhas. Aceitável.

7. **Severity já era `pub`**: gate 119.A sobre visibility não
   disparou. Sinal de que L1 já estava preparado — só faltava
   a camada certa para consumir.

---

## Estado pós-Passo 119

### Topologia final

```
L1 (01_core)   — entities, contracts, rules.
L2 (02_shell)  — CLI (cli.rs) + formatter (diagnostic.rs).
L3 (03_infra)  — I/O puro (export, fonts, layout, pipeline, world,
                  font_metrics, image_sizer).
L4 (04_wiring) — composição thin (main + helper drain_to_stderr).
```

### Dependências por camada

- **L1**: puro (comemo, thiserror, rustc_hash, unicode_*, time,
  indexmap, ecow).
- **L2**: clap, anyhow, typst-core (usado).
- **L3**: thiserror, comemo, ttf-parser, rustybuzz, time,
  imagesize, image, flate2, typst-core. **Sem clap, sem
  diagnostic_format.**
- **L4**: typst-core, typst-shell, typst-infra, anyhow. **Sem
  clap directo.**

### Trabalho futuro (não bloqueado)

1. **Formatters alternativos**: `diagnostic_json.rs`,
   `diagnostic_sarif.rs` em L2 — novos módulos.
2. **Flags funcionais** (`--root`, `--font-path`, `-o`, `-f`)
   — entram em `cli.rs` / `Args` de L2.
3. **Subcomandos** (compile, watch, query, init) — em L2.
4. **Candidatos 3 e 4 do Passo 118** (`eval_to_module_with_sink`,
   `compile_to_pdf_bytes`) permanecem em L3 como "pipeline
   braçal" — aceite.
