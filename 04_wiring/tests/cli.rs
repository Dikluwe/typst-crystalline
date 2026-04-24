//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash c664c9b2
//! @layer L4
//! @updated 2026-04-23
//!
//! Integration tests para o binário `typst` (Passo 114).
//!
//! Usa `std::process::Command` para invocar o binário compilado —
//! path via `env!("CARGO_BIN_EXE_typst")`, injectado pelo Cargo em
//! tempo de compilação para integration tests (`tests/`).
//!
//! Reproduz os 5 cenários validados manualmente em 113.D:
//! sucesso com warning, erro de eval, erro de I/O, sem
//! argumentos, compilação limpa.
//!
//! Zero deps externas — `std::process::Command` + `std::fs` apenas
//! (ADR-0046 estabelece a CLI sem deps ergonómicas neste passo).

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Path absoluto do binário `typst` compilado pelo Cargo.
const BIN: &str = env!("CARGO_BIN_EXE_typst");

/// Cria um ficheiro `.typ` temporário único.
///
/// Nome: `typst-passo-114-<name>-<pid>.typ` em `std::env::temp_dir()`.
/// O `pid` evita colisões entre invocações paralelas de `cargo test`;
/// o `name` evita colisões entre testes do mesmo processo.
fn temp_typ(name: &str, content: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!(
        "typst-passo-114-{}-{}.typ",
        name,
        std::process::id()
    ));
    fs::write(&path, content).expect("escrever input temporário");
    path
}

/// Constrói o path de output PDF correspondente — não cria ficheiro.
fn temp_pdf(name: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!(
        "typst-passo-114-{}-{}.pdf",
        name,
        std::process::id()
    ));
    path
}

/// Remove ficheiros temporários, ignorando erros (podem não existir
/// se o teste falhou antes de os criar).
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
        "exit code esperado 0; stderr:\n{}\nstdout:\n{}", stderr, stdout);
    assert!(output.exists(), "PDF deve existir em {}", output.display());
    assert!(stderr.contains("warning:"),
        "stderr deve conter 'warning:'; got:\n{}", stderr);
    assert!(stderr.contains("font"),
        "stderr deve mencionar 'font'; got:\n{}", stderr);

    cleanup(&[&input, &output]);
}

#[test]
fn cli_erro_de_eval() {
    // `#variavel_desconhecida` produz erro de eval (variável não
    // definida no scope). Confirmado em 113.D com binding análogo.
    let input = temp_typ("err", "#variavel_desconhecida");
    let output = temp_pdf("err");

    let result = Command::new(BIN)
        .arg(&input)
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(1),
        "exit code esperado 1 (erro de eval); stderr:\n{}", stderr);
    assert!(stderr.contains("error:"),
        "stderr deve conter 'error:'; got:\n{}", stderr);

    cleanup(&[&input, &output]);
}

#[test]
fn cli_erro_de_io_input_inexistente() {
    // Path que não existe — SystemWorld::new falha.
    let mut input = env::temp_dir();
    input.push(format!(
        "typst-passo-114-inexistente-xyz-{}.typ",
        std::process::id()
    ));
    let output = temp_pdf("io");

    // Garantir que input **não** existe (se algum run anterior
    // deixou ficheiro pelo mesmo pid, remover).
    let _ = fs::remove_file(&input);

    let result = Command::new(BIN)
        .arg(&input)
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(2),
        "exit code esperado 2 (I/O); stderr:\n{}", stderr);
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
        "exit code esperado 2 (argumentos); stderr:\n{}", stderr);
    assert!(stderr.contains("Usage"),
        "stderr deve conter 'Usage'; got:\n{}", stderr);
}

#[test]
fn cli_sucesso_sem_warnings() {
    // Input sem #set text(font: ...) e não vazio — nem o pilot do
    // Passo 106 nem o DEBT-49 disparam.
    let input = temp_typ("clean", "= Título\n\nTexto simples.");
    let output = temp_pdf("clean");

    let result = Command::new(BIN)
        .arg(&input)
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0),
        "exit code esperado 0; stderr:\n{}", stderr);
    assert!(output.exists(), "PDF deve existir");
    assert!(!stderr.contains("warning:"),
        "stderr não deve conter warnings; got:\n{}", stderr);
    assert!(!stderr.contains("error:"),
        "stderr não deve conter errors; got:\n{}", stderr);

    cleanup(&[&input, &output]);
}
