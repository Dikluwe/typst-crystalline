//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash 63faaf24
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
    // Passo 132B (ADR-0053): canary migrou de `font` para `hyphenate`
    // porque `font` passou a ser capturado via `FontList`.
    let input = temp_typ("warn", "#set text(hyphenate: true)\n\nOlá");
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
    assert!(stderr.contains("hyphenate"),
        "stderr deve mencionar 'hyphenate'; got:\n{}", stderr);

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

/// Passo 120 (ADR-0051): output positional é opcional; default
/// derivado é `input.with_extension("pdf")`. Teste verifica que
/// `typst input.typ` (sem output) cria `input.pdf`.
#[test]
fn cli_output_omitido_deriva_de_input() {
    let input = temp_typ("default_out", "Texto.");
    let expected_output = input.with_extension("pdf");
    // Garantir que o output derivado não existe antes.
    let _ = fs::remove_file(&expected_output);

    let result = Command::new(BIN)
        .arg(&input)
        // Nenhum output!
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0),
        "exit code esperado 0; stderr:\n{}", stderr);
    assert!(expected_output.exists(),
        "PDF derivado deve existir em {}", expected_output.display());

    cleanup(&[&input, &expected_output]);
}

/// Passo 120 (ADR-0051): `-o` flag funciona. Teste verifica que
/// `typst input.typ -o custom.pdf` cria custom.pdf.
#[test]
fn cli_output_via_flag_o() {
    let input = temp_typ("flag_o", "Texto.");
    let output = temp_pdf("flag_o_out");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0),
        "exit code esperado 0; stderr:\n{}", stderr);
    assert!(output.exists(),
        "PDF deve existir em {}", output.display());

    cleanup(&[&input, &output]);
}

/// Passo 122 (ADR-0051): `--font-path DIR` flag funciona.
///
/// Passa `--font-path` para um directório que existe (temp_dir do
/// sistema) — sem fontes dentro, mas discover_fonts tolera. Binário
/// compila sem erro.
#[test]
fn cli_font_path_explicito() {
    let input = temp_typ("fontpath", "Olá");
    let output = temp_pdf("fontpath_out");
    let fontdir = env::temp_dir();

    let result = Command::new(BIN)
        .arg(&input)
        .arg("--font-path")
        .arg(&fontdir)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0),
        "exit code esperado 0; stderr:\n{}", stderr);
    assert!(output.exists(),
        "PDF deve existir em {}", output.display());

    cleanup(&[&input, &output]);
}

/// Passo 122 (ADR-0051): `--font-path` repetível via `ArgAction::Append`.
#[test]
fn cli_font_path_repetivel() {
    let input = temp_typ("fontpath_multi", "Olá");
    let output = temp_pdf("fontpath_multi_out");
    let dir1 = env::temp_dir();
    let dir2 = env::temp_dir();

    let result = Command::new(BIN)
        .arg(&input)
        .arg("--font-path")
        .arg(&dir1)
        .arg("--font-path")
        .arg(&dir2)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0),
        "exit code esperado 0; stderr:\n{}", stderr);
    assert!(output.exists(),
        "PDF deve existir em {}", output.display());

    cleanup(&[&input, &output]);
}

/// Passo 122 (ADR-0051): path inválido em `--font-path` é silent-skip
/// pela função L3 `discover_fonts` — binário não falha.
#[test]
fn cli_font_path_inexistente_nao_falha() {
    let input = temp_typ("fp_invalid", "Olá");
    let output = temp_pdf("fp_invalid_out");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("--font-path")
        .arg("/path/que/nao/existe/xyz")
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0),
        "exit code esperado 0 (silent skip); stderr:\n{}", stderr);
    assert!(output.exists(),
        "PDF deve existir em {}", output.display());

    cleanup(&[&input, &output]);
}

/// Passo 123 (ADR-0051): `TYPST_ROOT` env var preenche `--root`
/// quando flag não é passada.
#[test]
fn cli_env_typst_root() {
    let input = temp_typ("env_root", "Olá");
    let root = input.parent().expect("tempdir tem parent").to_path_buf();
    let file_name = input.file_name().expect("file_name").to_os_string();
    let output = temp_pdf("env_root_out");

    let result = Command::new(BIN)
        .env("TYPST_ROOT", &root)
        .arg(&file_name)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0),
        "exit code esperado 0; stderr:\n{}", stderr);
    assert!(output.exists(),
        "PDF deve existir em {}", output.display());

    cleanup(&[&input, &output]);
}

/// Passo 123 (ADR-0051): precedência clap — flag `--root` vence
/// env `TYPST_ROOT`.
#[test]
fn cli_flag_root_vence_env() {
    let input = temp_typ("root_prec", "Olá");
    let flag_root = input.parent().expect("tempdir").to_path_buf();
    let file_name = input.file_name().expect("file_name").to_os_string();
    let output = temp_pdf("root_prec_out");

    let result = Command::new(BIN)
        // env aponta para path inválido — se vencer, `SystemWorld::new` falha.
        .env("TYPST_ROOT", "/path/inexistente/xyz")
        .arg(&file_name)
        .arg("--root")
        .arg(&flag_root)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0),
        "flag deve vencer env; stderr:\n{}", stderr);
    assert!(output.exists(),
        "PDF deve existir em {}", output.display());

    cleanup(&[&input, &output]);
}

/// Passo 123 (ADR-0051): `TYPST_FONT_PATHS` com delimiter de
/// sistema (`:` Unix / `;` Windows) expande em múltiplos paths.
#[test]
fn cli_env_typst_font_paths_delimiter() {
    let input = temp_typ("env_fonts", "Olá");
    let output = temp_pdf("env_fonts_out");
    let dir = env::temp_dir().display().to_string();
    let sep: char = if cfg!(windows) { ';' } else { ':' };
    let env_value = format!("{}{}{}", &dir, sep, &dir);

    let result = Command::new(BIN)
        .env("TYPST_FONT_PATHS", &env_value)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0),
        "exit code esperado 0; stderr:\n{}", stderr);
    assert!(output.exists(),
        "PDF deve existir em {}", output.display());

    cleanup(&[&input, &output]);
}

/// Passo 121 (ADR-0051): `--root DIR` flag funciona.
///
/// Estratégia: passar `--root <parent(input)>` explicitamente e
/// apenas o file_name como input. SystemWorld resolve main como
/// `root.join(file_name)`. `-o` path absoluto para evitar default
/// derivado cair no directório do file_name.
#[test]
fn cli_root_explicito() {
    let input = temp_typ("root_explicit", "= Root test\n\nOk.");
    let root = input.parent().expect("tempdir tem parent").to_path_buf();
    let file_name = input.file_name().expect("temp_typ cria file_name").to_os_string();
    let output = temp_pdf("root_explicit_out");

    let result = Command::new(BIN)
        .arg(&file_name)
        .arg("--root")
        .arg(&root)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0),
        "exit code esperado 0 com --root explícito; stderr:\n{}", stderr);
    assert!(output.exists(),
        "PDF deve existir em {}", output.display());

    cleanup(&[&input, &output]);
}

// ── Passo 124 — Testes de disciplina CLI ─────────────────────────────
//
// Materializam invariantes estruturais (ADR-0046, ADR-0045) que
// antes só existiam como convenção. Zero código de produção tocado.

/// stdout vazio em compilação bem-sucedida (ADR-0046): tudo em
/// stderr; stdout reservado para bytes (PDF vai para ficheiro).
#[test]
fn disciplina_stdout_vazio_em_sucesso() {
    let input = temp_typ("disc_stdout_ok", "Olá");
    let output = temp_pdf("disc_stdout_ok");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    assert_eq!(result.status.code(), Some(0));
    assert!(
        result.stdout.is_empty(),
        "stdout deve estar vazio; got {:?}",
        String::from_utf8_lossy(&result.stdout)
    );

    cleanup(&[&input, &output]);
}

/// stdout vazio também em erro — diagnóstico não escapa para stdout.
#[test]
fn disciplina_stdout_vazio_em_erro() {
    let input = temp_typ("disc_stdout_err", "#variavel_desconhecida");
    let output = temp_pdf("disc_stdout_err");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    assert_eq!(result.status.code(), Some(1));
    assert!(
        result.stdout.is_empty(),
        "stdout deve estar vazio mesmo em erro; got {:?}",
        String::from_utf8_lossy(&result.stdout)
    );

    cleanup(&[&input, &output]);
}

/// PDF começa com o magic header `%PDF-`.
#[test]
fn disciplina_pdf_magic_header() {
    let input = temp_typ("disc_magic", "Olá");
    let output = temp_pdf("disc_magic");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");
    assert_eq!(result.status.code(), Some(0));

    let bytes = fs::read(&output).expect("ler PDF");
    assert!(
        bytes.starts_with(b"%PDF-"),
        "PDF deve começar com '%PDF-'; primeiros bytes: {:?}",
        &bytes[..bytes.len().min(8)]
    );

    cleanup(&[&input, &output]);
}

/// PDF termina com o trailer `%%EOF` (dentro dos últimos 16 bytes,
/// tolerando newline final).
#[test]
fn disciplina_pdf_trailer_eof() {
    let input = temp_typ("disc_eof", "Olá");
    let output = temp_pdf("disc_eof");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");
    assert_eq!(result.status.code(), Some(0));

    let bytes = fs::read(&output).expect("ler PDF");
    let tail_len = 16.min(bytes.len());
    let tail = &bytes[bytes.len() - tail_len..];
    assert!(
        tail.windows(5).any(|w| w == b"%%EOF"),
        "PDF deve conter '%%EOF' perto do fim; tail: {:?}",
        tail
    );

    cleanup(&[&input, &output]);
}

/// Compilação limpa (sem warnings nem errors) não emite nada em
/// stderr. Reforça cli_sucesso_sem_warnings com assertion total.
#[test]
fn disciplina_stderr_vazio_em_compilacao_limpa() {
    let input = temp_typ("disc_clean", "= Título\n\nTexto.");
    let output = temp_pdf("disc_clean");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(0));
    assert!(
        stderr.is_empty(),
        "stderr deve estar vazio em compilação limpa; got {:?}",
        stderr
    );

    cleanup(&[&input, &output]);
}

/// Exit 0 implica PDF escrito e não-vazio.
#[test]
fn disciplina_exit_zero_implica_pdf_nao_vazio() {
    let input = temp_typ("disc_nonempty", "Olá");
    let output = temp_pdf("disc_nonempty");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    assert_eq!(result.status.code(), Some(0));
    assert!(output.exists());

    let size = fs::metadata(&output).expect("metadata").len();
    assert!(size > 0, "PDF deve ter conteúdo; tamanho: {}", size);

    cleanup(&[&input, &output]);
}

/// Ordem: warnings aparecem antes de errors no stderr (ADR-0045).
/// Input misto = `#set text(hyphenate: true)` (warning ADR-0040) +
/// `#variavel_desconhecida` (erro de eval).
///
/// Passo 132B (ADR-0053): canary migrou de `font` para `hyphenate`
/// porque `font` passou a ser capturado via `FontList`.
#[test]
fn disciplina_warnings_antes_de_errors() {
    let input = temp_typ(
        "disc_order",
        "#set text(hyphenate: true)\n#variavel_desconhecida",
    );
    let output = temp_pdf("disc_order");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(result.status.code(), Some(1),
        "esperava exit 1 em erro de eval; stderr:\n{}", stderr);

    let warning_pos = stderr.find("warning:");
    let error_pos   = stderr.find("error:");

    match (warning_pos, error_pos) {
        (Some(w), Some(e)) => assert!(
            w < e,
            "warning: deve aparecer antes de error:; stderr:\n{}",
            stderr
        ),
        (None, _) => panic!(
            "esperava warning no input misto; stderr:\n{}", stderr),
        (_, None) => panic!(
            "esperava error no input misto; stderr:\n{}", stderr),
    }

    cleanup(&[&input, &output]);
}
