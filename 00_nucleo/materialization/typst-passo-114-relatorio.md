# Passo 114 — Relatório (CLI tests automatizados)

**Data**: 2026-04-23
**Precondição**: Passo 113 encerrado (CLI real); 811 L1 + 195 L3 +
6 ignorados; zero violations.
**Natureza**: passo pequeno — **só testes**, zero mudanças em
código de produção.

---

## Sumário

Os 5 casos validados manualmente em 113.D foram materializados
em `04_wiring/tests/cli.rs` como integration tests que invocam o
binário via `std::process::Command`.

Todos os 5 passam: **811 L1 + 195 L3 + 5 `typst-wiring`**.

Zero deps novas — apenas `std::process::Command` + `std::fs`
(ADR-0046 estabelece CLI sem deps ergonómicas neste passo).

---

## 114.A — Inventário

Inventário em
`00_nucleo/diagnosticos/inventario-cli-tests-passo-114.md`.

- `[[bin]] name = "typst"` → `env!("CARGO_BIN_EXE_typst")`.
- `04_wiring/tests/` não existia — criada.
- Zero testes pré-existentes em `04_wiring/`.
- Strings em `main.rs` confirmadas em inglês: `"Usage"`,
  `"error:"`, `"warning:"` (via ADR-0045 formatter).

Gate 114.A não disparou.

---

## 114.B — Implementação

### Ficheiros criados

1. `04_wiring/tests/cli.rs` (179 linhas com header + helpers + 5
   testes).

### Helpers locais (sem deps externas)

```rust
const BIN: &str = env!("CARGO_BIN_EXE_typst");

fn temp_typ(name: &str, content: &str) -> PathBuf { ... }
fn temp_pdf(name: &str) -> PathBuf { ... }
fn cleanup(paths: &[&PathBuf]) { ... }
```

Nome dos ficheiros: `typst-passo-114-<name>-<pid>.{typ,pdf}` em
`std::env::temp_dir()` — `pid` evita colisões entre invocações
paralelas de `cargo test`; `name` evita colisões entre testes do
mesmo processo.

### Os 5 testes

| Teste | Cenário | Exit esperado | Assert |
|-------|---------|:-:|--------|
| `cli_sucesso_com_warning` | `#set text(font: "Arial")` + texto | 0 | PDF existe, stderr contém `warning:` e `font` |
| `cli_erro_de_eval` | `#variavel_desconhecida` | 1 | stderr contém `error:` |
| `cli_erro_de_io_input_inexistente` | Path não existe | 2 | stderr não vazio |
| `cli_sem_argumentos` | `typst` sem args | 2 | stderr contém `Usage` |
| `cli_sucesso_sem_warnings` | `= Título\nTexto simples.` | 0 | PDF existe, stderr sem `warning:` nem `error:` |

---

## 114.C — Verificação

```
$ cargo test -p typst-wiring
running 0 tests   (src/main.rs: binário sem #[test])
test result: ok. 0 passed

running 5 tests   (tests/cli.rs)
test cli_sem_argumentos ... ok
test cli_erro_de_io_input_inexistente ... ok
test cli_erro_de_eval ... ok
test cli_sucesso_sem_warnings ... ok
test cli_sucesso_com_warning ... ok
test result: ok. 5 passed; 0 failed

$ cargo test --workspace | grep "test result"
test result: ok. 811 passed ...  (L1)
test result: ok. 195 passed ...  (L3)
test result: ok. 5 passed   ...  (typst-wiring tests/cli.rs)
... outros test targets vazios ...

$ crystalline-lint .
✓ No violations found
```

Tempo total típico de `tests/cli.rs`: < 1 segundo (5 invocações
sequenciais do binário, cada uma compila um ficheiro pequeno).

---

## 114.D — Encerramento

### Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 (inalterado) |
| L3 tests | 195 | 195 (inalterado) |
| L4 tests (`typst-wiring`) | 0 | **5** (+5) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 46 | 46 (inalterado) |
| DEBTs abertos | 11 | 11 (inalterado) |

### Ficheiros produzidos

- `04_wiring/tests/cli.rs` — 5 testes + helpers.
- `00_nucleo/diagnosticos/inventario-cli-tests-passo-114.md`
- `00_nucleo/materialization/typst-passo-114-relatorio.md`

Zero mudanças em `04_wiring/src/`, `04_wiring/Cargo.toml`,
`03_infra/`, `01_core/`, ADRs, DEBT.md, prompts (excepto o novo
teste ponta a `wiring.md`).

---

## Limitações aceites

1. **Ficheiros temporários usam `process::id()`**: evita colisões
   entre processos paralelos, mas se dois testes do mesmo processo
   usassem o mesmo `name`, colidiriam. Cada teste usa `name`
   distinto (`warn`, `err`, `io`, `clean`). Garantido.
2. **Cleanup silencioso**: se um teste falha antes de criar o
   ficheiro, `cleanup` não encontra nada e ignora o erro.
   `/tmp` limpa-se sozinho.
3. **Sem `assert_cmd`**: sintaxe mais verbosa (~15 linhas por
   teste) mas zero deps. Se a suite crescer para 20+ testes,
   reconsiderar.
4. **Sem teste de "disciplina stdout/PDF header/ordem warnings/errors"**:
   escolha "mínimo" em vez de "mínimo + disciplina". Passo
   futuro pode adicionar.
5. **Testes correm o binário real** — mais lentos que unitários
   (<1s total). Aceitável.

---

## Lições

1. **`env!("CARGO_BIN_EXE_typst")` resolve directamente**: Cargo
   injecta a var em integration tests (`tests/`) quando o crate
   tem `[[bin]]`. Sem config adicional.
2. **Strings em inglês simplificam asserts**: "Usage", "error:",
   "warning:" são literais em inglês em `main.rs`. Asserts
   directos em Rust (literais inglesas) sem i18n.
3. **`#variavel_desconhecida` é erro de eval, não de parse**:
   confirmado empiricamente — parser aceita o identificador;
   eval falha na resolução de scope.
4. **Inputs temporários + cleanup manual funciona**: sem
   `tempfile` ou `assert_fs`, `env::temp_dir()` + `process::id()`
   + match names dão isolamento suficiente.
5. **Lint cobre `tests/*.rs` também**: adicionar header
   `@prompt` apontando para `wiring.md` satisfaz lint V1.
   Alternativa seria ignorar em config; header explícito é mais
   simples.

---

## Estado pós-Passo 114

### Regressão captura automática

```
Se alguém mudar:
- exit codes → teste falha.
- formato de stderr → teste falha.
- "Usage" para "Uso" → teste falha.
- warning para warn → teste falha.
```

Suite cobre os 3 caminhos principais (sucesso, erro de eval,
erro de I/O) + o caso de argumentos inválidos + um smoke test
de compilação limpa.

### Trabalho futuro identificado

1. **Testes de disciplina** — stdout sempre vazio; PDF magic
   header presente; warnings antes de errors na ordem de
   stderr. Passo dedicado pequeno.
2. **Testes de conteúdo do PDF** — parsing do PDF gerado para
   verificar que texto/headings estão lá. Passo dedicado maior
   (requer lib de leitura PDF ou grep binário).
3. **Testes para flags** — quando a CLI ganhar `--root`,
   `--font-path`, etc. (passos futuros de 04_wiring).
4. **`assert_cmd` adopção** — se a suite crescer para 20+ testes.
