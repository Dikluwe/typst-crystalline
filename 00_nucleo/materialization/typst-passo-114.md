# Passo 114 — CLI tests automatizados (`04_wiring/tests/`)

**Série**: 114 (passo pequeno; só testes, sem mudanças de código
de produção).
**Precondição**: Passo 113 encerrado; CLI real em `04_wiring/src/main.rs`;
811 L1 + 195 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0046 (CLI mínima).
**ADR nova**: **não**. Este passo só adiciona testes; não requer
decisão arquitectural nova.

---

## Objectivo

Materializar os 5 casos validados manualmente no Passo 113.D
como testes automatizados que correm em `cargo test --workspace`.

Casos:

1. **Sucesso com warning**: input válido com `#set text(font: ...)`.
   Exit 0, PDF escrito, stderr contém warning.
2. **Erro de eval**: input com erro semântico (variável
   desconhecida). Exit 1, stderr contém `error:`.
3. **Erro de I/O**: input inexistente. Exit 2, stderr contém
   mensagem de erro.
4. **Sem argumentos**: invocação `typst` sem args. Exit 2,
   stderr contém `Usage:`.
5. **Compilação limpa**: input válido sem warnings. Exit 0,
   PDF escrito, stderr vazio ou só contém conteúdo esperado.

Este passo **não**:
- Adiciona `assert_cmd` ou outras deps.
- Cobre casos que o 113.D não validou.
- Testa verificações de "disciplina" (stdout vazio, PDF magic
  header, ordem warnings/errors) — essas foram consideradas mas
  ficaram fora do escopo mínimo.
- Mexe em `main.rs` ou em L3.

---

## Decisões já tomadas

1. **Localização**: `04_wiring/tests/` (convenção Rust para
   integration tests de crate binária).
2. **Invocação**: `std::process::Command` com path do binário
   via `env!("CARGO_BIN_EXE_typst")`.
3. **Escopo**: 5 casos (os validados manualmente em 113.D), sem
   extras.
4. **Ficheiros temporários**: `std::env::temp_dir()` + cleanup
   manual. Cada teste cria input próprio; limpa ao terminar.

---

## Escopo

**Dentro**:
- `04_wiring/tests/cli.rs` — ficheiro novo com os 5 testes.
- Eventual helper local para criar input, correr binário,
  verificar output.

**Fora**:
- `04_wiring/Cargo.toml` — não muda (nenhuma dep nova).
- `04_wiring/src/main.rs` — não muda.
- L1, L3 — não tocam.
- `assert_cmd`, `tempfile`, outras deps ergonómicas.
- Testes de flags, subcomandos, watch (CLI ainda não tem).

---

## Sub-passos

### 114.A — Inventário rápido

**Parte 1 — Estrutura Cargo de `04_wiring`**:

1. `view` em `04_wiring/Cargo.toml`. Confirmar:
   - `[[bin]] name = "typst"` (ou equivalente).
   - Se já existe `[[test]]` ou pasta `tests/`.
   - `[dev-dependencies]` — quais estão presentes.
2. `view` em `04_wiring/` (directoria). Confirmar se `tests/`
   já existe.

**Parte 2 — Convenção do projecto**:

1. Grep por `#[test]` em `04_wiring/`. Registar se há testes
   pré-existentes no crate (improvável mas possível).
2. Grep por `std::process::Command` em qualquer lado do
   workspace. Ver se há padrão estabelecido para tests do
   binário.

**Parte 3 — `env!("CARGO_BIN_EXE_typst")` disponível?**

O Cargo expõe esta variável em tempo de compilação **automaticamente**
para `integration tests` em `tests/`, desde que o crate tenha
`[[bin]] name = "typst"`. Confirmar em 114.A.1 que o nome do
binário é `typst`; se for diferente (`typst-wiring`, etc.), a
variável chama-se diferente (`CARGO_BIN_EXE_typst-wiring`).

**Escrever** em `00_nucleo/diagnosticos/inventario-cli-tests-passo-114.md`:

```
04_wiring/Cargo.toml:
  [[bin]] name: <literal>
  env var: CARGO_BIN_EXE_<nome>
  tests/ existe? sim/não
  dev-dependencies: [lista]
```

**Gate 114.A**: se `tests/` já existe com testes, este passo
**adiciona** a `tests/cli.rs` sem substituir. Se existe algo
ortogonal (ex: `tests/smoke.rs`), coexiste.

### 114.B — Implementação

`04_wiring/tests/cli.rs`:

```rust
//! Integration tests for the typst CLI binary.
//!
//! Uses std::process::Command to invoke the compiled binary
//! (path via env!("CARGO_BIN_EXE_typst")).

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_typst");

/// Cria um ficheiro temporário com o conteúdo dado.
/// Devolve o path; o chamador é responsável por remover.
fn temp_typ(name: &str, content: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!("typst-passo-114-{}-{}.typ", name, std::process::id()));
    fs::write(&path, content).expect("escrever input temporário");
    path
}

fn temp_pdf(name: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!("typst-passo-114-{}-{}.pdf", name, std::process::id()));
    path
}

/// Limpa ficheiros temporários, ignorando erros (podem não existir).
fn cleanup(paths: &[&PathBuf]) {
    for p in paths {
        let _ = fs::remove_file(p);
    }
}

#[test]
fn cli_sucesso_com_warning() {
    let input = temp_typ("warn", "#set text(font: \"Arial\")\n\nOlá");
    let output = temp_pdf("warn");
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg(&output)
        .output()
        .expect("executar binário");
    
    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);
    
    assert_eq!(result.status.code(), Some(0),
        "exit code; stderr:\n{}\nstdout:\n{}", stderr, stdout);
    assert!(output.exists(), "PDF deve existir");
    assert!(stderr.contains("warning:"),
        "stderr deve conter 'warning:'; got:\n{}", stderr);
    assert!(stderr.contains("font"),
        "stderr deve mencionar 'font'; got:\n{}", stderr);
    
    cleanup(&[&input, &output]);
}

#[test]
fn cli_erro_de_eval() {
    // `#variavel_desconhecida` — erro de eval
    let input = temp_typ("err", "#variavel_desconhecida");
    let output = temp_pdf("err");
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg(&output)
        .output()
        .expect("executar binário");
    
    let stderr = String::from_utf8_lossy(&result.stderr);
    
    assert_eq!(result.status.code(), Some(1),
        "exit code; stderr:\n{}", stderr);
    assert!(stderr.contains("error:"),
        "stderr deve conter 'error:'; got:\n{}", stderr);
    
    cleanup(&[&input, &output]);
}

#[test]
fn cli_erro_de_io_input_inexistente() {
    let input = env::temp_dir().join("typst-passo-114-inexistente-xyz.typ");
    let output = temp_pdf("io");
    
    // Garantir que input **não** existe
    let _ = fs::remove_file(&input);
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg(&output)
        .output()
        .expect("executar binário");
    
    let stderr = String::from_utf8_lossy(&result.stderr);
    
    assert_eq!(result.status.code(), Some(2),
        "exit code; stderr:\n{}", stderr);
    assert!(!stderr.is_empty(),
        "stderr deve ter mensagem de erro");
    
    cleanup(&[&output]);
}

#[test]
fn cli_sem_argumentos() {
    let result = Command::new(BIN)
        .output()
        .expect("executar binário");
    
    let stderr = String::from_utf8_lossy(&result.stderr);
    
    assert_eq!(result.status.code(), Some(2),
        "exit code; stderr:\n{}", stderr);
    assert!(stderr.contains("Usage"),
        "stderr deve conter 'Usage'; got:\n{}", stderr);
}

#[test]
fn cli_sucesso_sem_warnings() {
    let input = temp_typ("clean", "= Título\n\nTexto simples.");
    let output = temp_pdf("clean");
    
    let result = Command::new(BIN)
        .arg(&input)
        .arg(&output)
        .output()
        .expect("executar binário");
    
    let stderr = String::from_utf8_lossy(&result.stderr);
    
    assert_eq!(result.status.code(), Some(0),
        "exit code; stderr:\n{}", stderr);
    assert!(output.exists(), "PDF deve existir");
    assert!(!stderr.contains("warning:"),
        "stderr não deve conter warnings; got:\n{}", stderr);
    assert!(!stderr.contains("error:"),
        "stderr não deve conter errors; got:\n{}", stderr);
    
    cleanup(&[&input, &output]);
}
```

**Ajustes a fazer conforme 114.A**:

- Se o nome do binário não é `typst`, mudar `BIN` para
  `env!("CARGO_BIN_EXE_<nome_real>")`.
- Se o warning "ficheiro vazio" do micro-piloto 106 dispara em
  `cli_sucesso_sem_warnings` (input `= Título\n\nTexto simples.`),
  não dispara — esse é para ficheiros vazios. Se disparar num
  input não-vazio, há bug e é útil detectar.
- Se a mensagem exacta da variável desconhecida em
  `cli_erro_de_eval` usa palavras diferentes (ex: "unknown
  variable" vs "variável desconhecida"), ajustar assert.

### 114.C — Verificação local

Antes de encerrar, correr:

```bash
cargo build --release
cargo test -p typst-wiring
```

Os 5 testes devem passar. Se algum falha, ver:

- Mensagem concreta do assert para identificar o que não casou.
- Se é erro de lógica do teste (expectativa errada), corrigir o
  teste.
- Se é bug do binário, **parar** e reportar — não mascarar com
  asserts mais fracos.

### 114.D — Encerramento

1. `cargo test --workspace`: 811 L1 + 195 L3 + **5 novos em
   `typst-wiring`** + 6 ignorados.
2. `crystalline-lint .`: zero violations.
3. Relatório `typst-passo-114-relatorio.md`:
   - Confirmar localização (`04_wiring/tests/cli.rs`).
   - Confirmar que `env!("CARGO_BIN_EXE_<nome>")` resolve.
   - Listar os 5 testes com breve descrição.
   - Tempo de execução total (comando `cargo test -p typst-wiring
     -- --nocapture` para ver outputs se útil).
   - Limitações: ficheiros temporários com `process::id()` no
     nome — se dois testes do mesmo nome correm em paralelo
     (único possível se alguém rodar `cargo test` duas vezes em
     paralelo), colidem. Aceitar.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 114.A escrito (breve).
2. `04_wiring/tests/cli.rs` criado com 5 testes.
3. Zero mudanças em `src/`, Cargo.toml, L1, L3.
4. `cargo test --workspace` passa com 5 testes novos em
   `typst-wiring`.
5. `crystalline-lint` zero violations.
6. Relatório 114.D escrito.

---

## O que pode sair errado

- **`env!("CARGO_BIN_EXE_typst")` não resolve**. Acontece se:
  - Nome do binário não é `typst` (verificar em 114.A.1 e
    ajustar).
  - Teste está em `src/` em vez de `tests/` (Cargo não injecta
    a var nos `src/`).
  - Crate não tem `[[bin]]` (improvável neste caso).
- **Binário não está compilado**. `cargo test` compila
  automaticamente deps que o crate precisa, incluindo o
  binário. Se falhar, há problema de configuração do Cargo.
- **Mensagens de erro traduzidas/mudam**. Asserts sobre
  conteúdo de stderr (`stderr.contains("Usage")`) partem do
  princípio que o texto está em inglês ou na língua que o 113
  usou ("Usage" em inglês, "error:" / "warning:" em inglês).
  Se o 113 usou português ("Uso:", "erro:", "aviso:"), ajustar
  asserts. Grep em `main.rs` para confirmar strings literais.
- **Testes em paralelo com mesmo nome**. `process::id()` no
  nome do ficheiro temporário evita colisões entre **processos**
  mas não entre **testes do mesmo processo** com mesmo `name`
  parameter. Cada teste usa `name` diferente — garantido.
- **Cleanup falha silenciosamente**. `cleanup` ignora erros por
  design. Se um teste falha a meio, o ficheiro fica. Aceitar —
  `/tmp` limpa-se sozinho eventualmente.
- **`variavel_desconhecida` é sintaxe válida em Typst?**
  `#variavel_desconhecida` tenta resolver a variável; se não
  existe no scope, erro. Confirmar em 114.C que o erro é de
  eval e não de parsing. Se for de parsing, usar outra
  construção (ex: `#{let x = undefined_fn()}`).
- **Warning "ficheiro vazio" dispara inesperadamente**. O
  micro-piloto do Passo 106 dispara para `text().is_empty()`.
  Se algum input de teste for vazio por acaso, warning
  inesperado. Inputs aqui têm conteúdo — não dispara.

---

## Notas operacionais

- Este passo valida empiricamente o que 113.D validou
  manualmente. Regressões futuras (se alguém mudar exit
  codes, stderr format, etc.) são apanhadas.
- Sem `assert_cmd`, os testes são mais verbosos mas zero deps.
  Se a suite crescer para 20+ testes, vale a pena reconsiderar.
  Para 5, manual é OK.
- Os testes correm o binário real — são **mais lentos** que
  testes unitários. Esperar ~1-5 segundos total. Aceitar.
- Se algum teste falhar flakily (passa às vezes, falha outras),
  pode ser race condition. Diagnosticar antes de aceitar.
- O Passo 113 documentou que "tudo diagnóstico para stderr,
  nada para stdout excepto bytes PDF, e mesmo PDF vai para
  ficheiro". Esta disciplina não é testada aqui (escolha
  "mínimo" em vez de "mínimo + disciplina"). Passo futuro pode
  adicionar.
