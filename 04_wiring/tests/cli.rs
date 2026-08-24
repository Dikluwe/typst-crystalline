//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash ca7ff38f
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
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// Path absoluto do binário `typst` compilado pelo Cargo.
const BIN: &str = env!("CARGO_BIN_EXE_typst");

/// Cria um ficheiro `.typ` temporário único.
///
/// Nome: `typst-passo-114-<name>-<pid>.typ` em `std::env::temp_dir()`.
/// O `pid` evita colisões entre invocações paralelas de `cargo test`;
/// o `name` evita colisões entre testes do mesmo processo.
fn temp_typ(name: &str, content: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!("typst-passo-114-{}-{}.typ", name, std::process::id()));
    fs::write(&path, content).expect("escrever input temporário");
    path
}

/// Constrói o path de output PDF correspondente — não cria ficheiro.
fn temp_pdf(name: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!("typst-passo-114-{}-{}.pdf", name, std::process::id()));
    path
}

/// Remove ficheiros temporários, ignorando erros (podem não existir
/// se o teste falhou antes de os criar).
fn cleanup(paths: &[&PathBuf]) {
    for p in paths {
        let _ = fs::remove_file(p);
    }
}

struct WatchedChild(Child);

impl Drop for WatchedChild {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn wait_until(timeout: Duration, mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if condition() {
            return;
        }
        thread::sleep(Duration::from_millis(50));
    }
    panic!("condition was not met within {timeout:?}");
}

#[test]
fn p1137_watch_help_alias_e_stdout_rejeitado() {
    let help = Command::new(BIN).arg("--help").output().unwrap();
    let stdout = String::from_utf8_lossy(&help.stdout);
    assert!(stdout.contains("watch"));
    assert!(stdout.contains("[aliases: w]"));

    let input = temp_typ("watch-stdout", "Hello");
    for command in ["watch", "w"] {
        let result =
            Command::new(BIN).arg(command).arg(&input).arg("-").output().unwrap();
        assert_eq!(result.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&result.stderr).contains("file output"));
    }
    cleanup(&[&input]);
}

#[test]
fn p1137_watch_dependencias_recuperacao_e_filtro() {
    let root = env::temp_dir().join(format!("typst-watch-{}", std::process::id()));
    let input = root.join("main.typ");
    let asset = root.join("data.txt");
    let irrelevant = root.join("irrelevant.txt");
    let output = root.join("main.pdf");
    fs::create_dir_all(&root).unwrap();
    fs::write(&asset, "first").unwrap();
    fs::write(&irrelevant, "ignored").unwrap();
    fs::write(&input, "#read(\"data.txt\")").unwrap();

    let child = Command::new(BIN)
        .args(["watch"])
        .arg(&input)
        .arg(&output)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _watch = WatchedChild(child);

    wait_until(Duration::from_secs(20), || output.exists());
    let first = fs::read(&output).unwrap();

    fs::write(&irrelevant, "changed but not observed").unwrap();
    thread::sleep(Duration::from_millis(400));
    assert_eq!(fs::read(&output).unwrap(), first);

    fs::write(&asset, "second").unwrap();
    wait_until(Duration::from_secs(20), || {
        fs::read(&output).map(|bytes| bytes != first).unwrap_or(false)
    });
    let second = fs::read(&output).unwrap();

    fs::write(&input, "#unknown-watch-name").unwrap();
    thread::sleep(Duration::from_millis(500));
    assert_eq!(fs::read(&output).unwrap(), second);

    fs::write(&input, "Recovered").unwrap();
    wait_until(Duration::from_secs(20), || {
        fs::read(&output).map(|bytes| bytes != second).unwrap_or(false)
    });

    drop(_watch);
    let _ = fs::remove_dir_all(root);
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

    assert_eq!(
        result.status.code(),
        Some(0),
        "exit code esperado 0; stderr:\n{}\nstdout:\n{}",
        stderr,
        stdout
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());
    assert!(
        stderr.contains("warning:"),
        "stderr deve conter 'warning:'; got:\n{}",
        stderr
    );
    assert!(
        stderr.contains("hyphenate"),
        "stderr deve mencionar 'hyphenate'; got:\n{}",
        stderr
    );

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

    assert_eq!(
        result.status.code(),
        Some(1),
        "exit code esperado 1 (erro de eval); stderr:\n{}",
        stderr
    );
    assert!(stderr.contains("error:"), "stderr deve conter 'error:'; got:\n{}", stderr);

    cleanup(&[&input, &output]);
}

#[test]
fn cli_erro_de_io_input_inexistente() {
    // Path que não existe — SystemWorld::new falha.
    let mut input = env::temp_dir();
    input.push(format!("typst-passo-114-inexistente-xyz-{}.typ", std::process::id()));
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

    assert_eq!(
        result.status.code(),
        Some(2),
        "exit code esperado 2 (I/O); stderr:\n{}",
        stderr
    );
    assert!(!stderr.is_empty(), "stderr deve ter mensagem de erro");

    cleanup(&[&output]);
}

#[test]
fn cli_sem_argumentos() {
    let result = Command::new(BIN).output().expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(2),
        "exit code esperado 2 (argumentos); stderr:\n{}",
        stderr
    );
    assert!(stderr.contains("Usage"), "stderr deve conter 'Usage'; got:\n{}", stderr);
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

    assert_eq!(
        result.status.code(),
        Some(0),
        "exit code esperado 0; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir");
    assert!(
        !stderr.contains("warning:"),
        "stderr não deve conter warnings; got:\n{}",
        stderr
    );
    assert!(
        !stderr.contains("error:"),
        "stderr não deve conter errors; got:\n{}",
        stderr
    );

    cleanup(&[&input, &output]);
}

#[test]
fn p1137_compile_e_alias_c_produzem_artefacto() {
    let input = temp_typ("compile-subcommand", "Hello");
    let output_compile = temp_pdf("compile-subcommand");
    let output_alias = temp_pdf("compile-alias");

    for (command, output) in [("compile", &output_compile), ("c", &output_alias)] {
        let result = Command::new(BIN)
            .arg(command)
            .arg(&input)
            .arg(output)
            .output()
            .expect("executar subcomando compile");
        assert_eq!(
            result.status.code(),
            Some(0),
            "stderr: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(output.exists(), "artefacto ausente para {command}");
    }

    cleanup(&[&input, &output_compile, &output_alias]);
}

#[test]
fn p1137_help_expoe_compile_e_oculta_query() {
    let result = Command::new(BIN).arg("--help").output().expect("executar help");
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert_eq!(result.status.code(), Some(0));
    assert!(stdout.contains("compile"), "help sem compile:\n{stdout}");
    assert!(stdout.contains("eval"), "help sem eval:\n{stdout}");
    assert!(!stdout.contains("query"), "query deprecated não deve aparecer:\n{stdout}");
    assert!(
        !stdout.contains("[INPUT] [OUTPUT]"),
        "posicionais legados vazaram no help:\n{stdout}"
    );
}

#[test]
fn p1137_fonts_lista_embutidas_sem_sistema() {
    let result = Command::new(BIN)
        .args(["fonts", "--ignore-system-fonts"])
        .output()
        .expect("executar fonts");
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert_eq!(
        result.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(stdout.contains("Libertinus Serif"), "família embutida ausente:\n{stdout}");
    assert!(!stdout.contains("query"));
}

#[test]
fn p1137_fonts_variants_expoe_campos() {
    let result = Command::new(BIN)
        .args(["fonts", "--ignore-system-fonts", "--variants"])
        .output()
        .expect("executar fonts variants");
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert_eq!(
        result.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(stdout.contains("(Embedded)"));
    assert!(stdout.contains("Style:"));
    assert!(stdout.contains("Weight:"));
    assert!(stdout.contains("Stretch:"));
}

#[test]
fn p1137_completions_bash_usa_arvore_cli_real() {
    let result = Command::new(BIN)
        .args(["completions", "bash"])
        .output()
        .expect("executar completions");
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert_eq!(
        result.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(stdout.contains("compile"));
    assert!(stdout.contains("fonts"));
    assert!(stdout.contains("completions"));
    // O vanilla também mantém `query` oculto nas tabelas de completion.
    assert!(stdout.contains("query"));
}

#[test]
fn p1137_completions_shell_invalido_e_exit_2() {
    let result = Command::new(BIN)
        .args(["completions", "invalid-shell"])
        .output()
        .expect("executar completions inválida");
    assert_eq!(result.status.code(), Some(2));
}

#[test]
fn p1137_info_json_expoe_schema_e_commit_integral() {
    let result = Command::new(BIN)
        .args(["info", "--format", "json"])
        .env("XDG_DATA_HOME", "/tmp/crystalline-info-data")
        .env("XDG_CACHE_HOME", "/tmp/crystalline-info-cache")
        .output()
        .expect("executar info JSON");
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert_eq!(
        result.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_slice(&result.stdout).expect("JSON válido");
    assert_eq!(value["version"], "0.15.1");
    assert!(value["build"]["commit"].as_str().is_some());
    assert!(value["build"]["platform"]["os"].is_string());
    assert_eq!(
        value["packages"]["data-path"],
        "/tmp/crystalline-info-data/typst/packages"
    );
    assert_eq!(
        value["packages"]["cache-path"],
        "/tmp/crystalline-info-cache/typst/packages"
    );
    assert!(stdout.ends_with('\n'));
}

#[test]
fn p1137_info_humano_contem_categorias() {
    let result = Command::new(BIN).arg("info").output().expect("executar info humano");
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert_eq!(
        result.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    for category in
        ["Version: 0.15.1", "Commit:", "Platform:", "Features:", "Fonts:", "Packages:"]
    {
        assert!(stdout.contains(category), "categoria ausente {category:?}:\n{stdout}");
    }
}

#[test]
fn p1137_info_nao_expoe_segredos() {
    let result = Command::new(BIN)
        .args(["info", "--format", "json"])
        .env("TYPST_CERT", "SEGREDO-CERTIFICADO")
        .env("HTTPS_PROXY", "https://usuario:SEGREDO-PROXY@example.invalid")
        .output()
        .expect("executar info com segredos");
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert_eq!(result.status.code(), Some(0));
    assert!(!stdout.contains("SEGREDO-CERTIFICADO"));
    assert!(!stdout.contains("SEGREDO-PROXY"));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&result.stdout).unwrap()["packages"]
            ["custom-ca-configured"],
        true
    );
}

#[test]
fn p1137_info_pretty_sem_formato_e_exit_2() {
    let result = Command::new(BIN)
        .args(["info", "--pretty"])
        .output()
        .expect("executar info inválido");
    assert_eq!(result.status.code(), Some(2));
}

#[test]
fn p1137_cert_global_e_env_aparecem_apenas_como_presenca_no_info() {
    for configure in ["flag", "env"] {
        let mut command = Command::new(BIN);
        if configure == "flag" {
            command.args(["--cert", "/tmp/SEGREDO-FLAG.pem", "info", "--format", "json"]);
        } else {
            command
                .args(["info", "--format", "json"])
                .env("TYPST_CERT", "/tmp/SEGREDO-ENV.pem");
        }
        let result = command.output().expect("executar info com CA configurada");
        let stdout = String::from_utf8_lossy(&result.stdout);
        assert_eq!(result.status.code(), Some(0));
        assert!(stdout.contains("\"custom-ca-configured\":true"));
        assert!(!stdout.contains("SEGREDO"));
    }
}

fn init_fixture(version: &str, template_section: &str) -> PathBuf {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let serial = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "typst-init-fixture-{}-{}-{}",
        std::process::id(),
        serial,
        version.replace('.', "-")
    ));
    let package = root.join("typst/packages/local/starter").join(version);
    fs::create_dir_all(package.join("template/src")).unwrap();
    fs::write(
        package.join("typst.toml"),
        format!(
            "[package]\nname = \"starter\"\nversion = \"{version}\"\nentrypoint = \"lib.typ\"\n\n{template_section}\n"
        ),
    )
    .unwrap();
    fs::write(package.join("template/src/main.typ"), "Hello from template").unwrap();
    root
}

#[test]
fn p1137_init_explicito_materializa_somente_template() {
    let data = init_fixture(
        "1.2.3",
        "[template]\npath = \"template\"\nentrypoint = \"src/main.typ\"",
    );
    let destination =
        std::env::temp_dir().join(format!("typst-init-dest-{}", std::process::id()));
    let result = Command::new(BIN)
        .args(["init", "@local/starter:1.2.3"])
        .arg(&destination)
        .env("XDG_DATA_HOME", &data)
        .output()
        .expect("executar init");
    assert_eq!(
        result.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        fs::read_to_string(destination.join("src/main.typ")).unwrap(),
        "Hello from template"
    );
    assert!(!destination.join("typst.toml").exists());
    let _ = fs::remove_dir_all(data);
    let _ = fs::remove_dir_all(destination);
}

#[test]
fn p1137_init_sem_versao_escolhe_maior_local() {
    let data = init_fixture(
        "1.2.3",
        "[template]\npath = \"template\"\nentrypoint = \"src/main.typ\"",
    );
    let package_new = data.join("typst/packages/local/starter/2.0.0");
    fs::create_dir_all(package_new.join("template")).unwrap();
    fs::write(package_new.join("template/main.typ"), "newest").unwrap();
    fs::write(package_new.join("typst.toml"), "[package]\nname=\"starter\"\nversion=\"2.0.0\"\nentrypoint=\"lib.typ\"\n[template]\npath=\"template\"\nentrypoint=\"main.typ\"\n").unwrap();
    let work =
        std::env::temp_dir().join(format!("typst-init-work-{}", std::process::id()));
    fs::create_dir_all(&work).unwrap();
    let result = Command::new(BIN)
        .current_dir(&work)
        .args(["init", "@local/starter"])
        .env("XDG_DATA_HOME", &data)
        .output()
        .expect("executar init versionless");
    assert_eq!(
        result.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(fs::read_to_string(work.join("starter/main.typ")).unwrap(), "newest");
    let _ = fs::remove_dir_all(data);
    let _ = fs::remove_dir_all(work);
}

#[test]
fn p1137_init_destino_existente_nao_escreve() {
    let data = init_fixture(
        "1.2.3",
        "[template]\npath = \"template\"\nentrypoint = \"src/main.typ\"",
    );
    let destination =
        std::env::temp_dir().join(format!("typst-init-existing-{}", std::process::id()));
    fs::create_dir_all(&destination).unwrap();
    fs::write(destination.join("sentinel"), "keep").unwrap();
    let result = Command::new(BIN)
        .args(["init", "@local/starter:1.2.3"])
        .arg(&destination)
        .env("XDG_DATA_HOME", &data)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(fs::read_to_string(destination.join("sentinel")).unwrap(), "keep");
    let _ = fs::remove_dir_all(data);
    let _ = fs::remove_dir_all(destination);
}

#[test]
fn p1137_init_rejeita_manifesto_sem_template_e_path_escapando() {
    for (section, expected) in [
        ("", "does not contain a [template] section"),
        (
            "[template]\npath = \"../outside\"\nentrypoint = \"main.typ\"",
            "must be a relative path inside the package",
        ),
    ] {
        let data = init_fixture("1.2.3", section);
        let destination = data.join("must-not-exist");
        let result = Command::new(BIN)
            .args(["init", "@local/starter:1.2.3"])
            .arg(&destination)
            .env("XDG_DATA_HOME", &data)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&result.stderr).contains(expected));
        assert!(!destination.exists());
        let _ = fs::remove_dir_all(data);
    }
}

#[cfg(unix)]
#[test]
fn p1137_init_falha_no_meio_sem_arvore_parcial() {
    use std::os::unix::fs::symlink;

    let data = init_fixture(
        "1.2.3",
        "[template]\npath = \"template\"\nentrypoint = \"src/main.typ\"",
    );
    let template = data.join("typst/packages/local/starter/1.2.3/template");
    symlink("/tmp", template.join("unsafe-link")).unwrap();
    let destination = data.join("must-not-exist");
    let result = Command::new(BIN)
        .args(["init", "@local/starter:1.2.3"])
        .arg(&destination)
        .env("XDG_DATA_HOME", &data)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
    assert!(!destination.exists());
    let leftovers = fs::read_dir(&data)
        .unwrap()
        .flatten()
        .any(|entry| entry.file_name().to_string_lossy().contains(".typst-init-"));
    assert!(!leftovers, "staging deve ser removido após falha");
    let _ = fs::remove_dir_all(data);
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

    assert_eq!(
        result.status.code(),
        Some(0),
        "exit code esperado 0; stderr:\n{}",
        stderr
    );
    assert!(
        expected_output.exists(),
        "PDF derivado deve existir em {}",
        expected_output.display()
    );

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

    assert_eq!(
        result.status.code(),
        Some(0),
        "exit code esperado 0; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());

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

    assert_eq!(
        result.status.code(),
        Some(0),
        "exit code esperado 0; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());

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

    assert_eq!(
        result.status.code(),
        Some(0),
        "exit code esperado 0; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());

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

    assert_eq!(
        result.status.code(),
        Some(0),
        "exit code esperado 0 (silent skip); stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());

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

    assert_eq!(
        result.status.code(),
        Some(0),
        "exit code esperado 0; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());

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

    assert_eq!(
        result.status.code(),
        Some(0),
        "flag deve vencer env; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());

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

    assert_eq!(
        result.status.code(),
        Some(0),
        "exit code esperado 0; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());

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

    assert_eq!(
        result.status.code(),
        Some(0),
        "exit code esperado 0 com --root explícito; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());

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

/// P616: `#set text(dir: ttb)` é rejeitado com a mesma mensagem do vanilla.
#[test]
fn p616_text_dir_ttb_rejeitado() {
    let input = temp_typ("p616_ttb", "#set text(dir: ttb)\nTexto.");
    let output = temp_pdf("p616_ttb");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(1),
        "esperava exit 1 para dir: ttb; stderr:\n{}",
        stderr
    );
    assert!(
        stderr.contains("text direction must be horizontal"),
        "stderr deve conter a mensagem do vanilla; got:\n{}",
        stderr
    );
    assert!(!output.exists(), "não deve criar PDF quando dir: ttb é inválido");

    cleanup(&[&input, &output]);
}

/// P616: `#set text(dir: btt)` também é rejeitado.
#[test]
fn p616_text_dir_btt_rejeitado() {
    let input = temp_typ("p616_btt", "#set text(dir: btt)\nTexto.");
    let output = temp_pdf("p616_btt");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(1),
        "esperava exit 1 para dir: btt; stderr:\n{}",
        stderr
    );
    assert!(
        stderr.contains("text direction must be horizontal"),
        "stderr deve conter a mensagem do vanilla; got:\n{}",
        stderr
    );

    cleanup(&[&input, &output]);
}

/// P616: `#set text(dir: rtl)` continua a funcionar sem regressão.
#[test]
fn p616_text_dir_rtl_continua_funcionar() {
    let input =
        temp_typ("p616_rtl", "#set text(dir: rtl, lang: \"ar\", size: 20pt)\nمرحبا");
    let output = temp_pdf("p616_rtl");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(0),
        "esperava exit 0 para dir: rtl; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir para dir: rtl");

    cleanup(&[&input, &output]);
}

/// Extrai um valor do pacote XMP embutido no PDF (texto plano).
fn extract_xmp_id(pdf: &[u8], tag: &str) -> Option<String> {
    let text = String::from_utf8_lossy(pdf);
    let start = text.find(&format!("<xmpMM:{tag}>"))? + tag.len() + 10;
    let end = text[start..].find(&format!("</xmpMM:{tag}>"))?;
    Some(text[start..start + end].to_string())
}

/// P617: `--document-id` fixa DocumentID; InstanceID continua aleatório.
#[test]
fn p617_document_id_fixo_instance_id_aleatorio() {
    let id = "f81d4fae-7dec-11d0-a765-00a0c91e6bf6";
    let input = temp_typ("p617_docid", "Texto.");
    let output1 = temp_pdf("p617_docid_1");
    let output2 = temp_pdf("p617_docid_2");

    for out in [&output1, &output2] {
        let result = Command::new(BIN)
            .arg(&input)
            .arg("--document-id")
            .arg(id)
            .arg("-o")
            .arg(out)
            .output()
            .expect("executar binário");

        let stderr = String::from_utf8_lossy(&result.stderr);
        assert_eq!(
            result.status.code(),
            Some(0),
            "esperava exit 0 com --document-id; stderr:\n{}",
            stderr
        );
        assert!(out.exists(), "PDF deve existir");
    }

    let pdf1 = fs::read(&output1).expect("ler PDF 1");
    let pdf2 = fs::read(&output2).expect("ler PDF 2");

    let doc1 = extract_xmp_id(&pdf1, "DocumentID").expect("DocumentID em PDF 1");
    let doc2 = extract_xmp_id(&pdf2, "DocumentID").expect("DocumentID em PDF 2");
    let inst1 = extract_xmp_id(&pdf1, "InstanceID").expect("InstanceID em PDF 1");
    let inst2 = extract_xmp_id(&pdf2, "InstanceID").expect("InstanceID em PDF 2");

    assert_eq!(doc1, doc2, "DocumentID deve ser igual quando --document-id é o mesmo");
    assert_ne!(inst1, inst2, "InstanceID deve ser diferente em cada compilação");

    cleanup(&[&input, &output1, &output2]);
}

/// P617: sem `--document-id`, DocumentID continua aleatório (P615).
#[test]
fn p617_sem_document_id_document_id_aleatorio() {
    let input = temp_typ("p617_sem_docid", "Texto.");
    let output1 = temp_pdf("p617_sem_docid_1");
    let output2 = temp_pdf("p617_sem_docid_2");

    for out in [&output1, &output2] {
        let result = Command::new(BIN)
            .arg(&input)
            .arg("-o")
            .arg(out)
            .output()
            .expect("executar binário");

        let stderr = String::from_utf8_lossy(&result.stderr);
        assert_eq!(
            result.status.code(),
            Some(0),
            "esperava exit 0 sem --document-id; stderr:\n{}",
            stderr
        );
        assert!(out.exists(), "PDF deve existir");
    }

    let pdf1 = fs::read(&output1).expect("ler PDF 1");
    let pdf2 = fs::read(&output2).expect("ler PDF 2");

    let doc1 = extract_xmp_id(&pdf1, "DocumentID").expect("DocumentID em PDF 1");
    let doc2 = extract_xmp_id(&pdf2, "DocumentID").expect("DocumentID em PDF 2");

    assert_ne!(doc1, doc2, "DocumentID deve ser diferente sem --document-id");

    cleanup(&[&input, &output1, &output2]);
}

/// P617: `--document-id` inválido produz erro claro.
#[test]
fn p617_document_id_invalido_erro() {
    let input = temp_typ("p617_invalid", "Texto.");
    let output = temp_pdf("p617_invalid_out");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("--document-id")
        .arg("nao-e-uuid")
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(2),
        "esperava exit 2 para document-id inválido; stderr:\n{}",
        stderr
    );
    assert!(
        stderr.contains("invalid document ID"),
        "stderr deve mencionar document ID inválido; got:\n{}",
        stderr
    );
    assert!(!output.exists(), "não deve criar PDF com document-id inválido");

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
    let input =
        temp_typ("disc_order", "#set text(hyphenate: true)\n#variavel_desconhecida");
    let output = temp_pdf("disc_order");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(1),
        "esperava exit 1 em erro de eval; stderr:\n{}",
        stderr
    );

    let warning_pos = stderr.find("warning:");
    let error_pos = stderr.find("error:");

    match (warning_pos, error_pos) {
        (Some(w), Some(e)) => {
            assert!(w < e, "warning: deve aparecer antes de error:; stderr:\n{}", stderr)
        }
        (None, _) => panic!("esperava warning no input misto; stderr:\n{}", stderr),
        (_, None) => panic!("esperava error no input misto; stderr:\n{}", stderr),
    }

    cleanup(&[&input, &output]);
}

/// P772b: erro dentro de ficheiro importado mostra o path/linha/coluna
/// do ficheiro alvo, não `<detached>` do documento principal.
#[test]
fn p772b_span_cross_file_aponta_para_ficheiro_importado() {
    let root = env::temp_dir().join(format!("typst-p772b-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("criar root temp");

    let subdir = root.join("subdir");
    fs::create_dir(&subdir).expect("criar subdir");

    let lib_path = subdir.join("lib.typ");
    fs::write(&lib_path, "#let broken(x) = x + y\n").expect("escrever lib.typ");

    let main_path = root.join("main.typ");
    fs::write(&main_path, "#import \"subdir/lib.typ\": broken\n#broken(1)\n")
        .expect("escrever main.typ");

    let output = root.join("main.pdf");

    let result = Command::new(BIN)
        .arg(&main_path)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(1),
        "esperava exit 1 para erro em ficheiro importado; stderr:\n{}",
        stderr
    );
    assert!(stderr.contains("error:"), "stderr deve conter 'error:'; got:\n{}", stderr);
    assert!(
        stderr.contains("unknown variable `y`"),
        "stderr deve mencionar a variável desconhecida; got:\n{}",
        stderr
    );
    assert!(
        stderr.contains("lib.typ"),
        "stderr deve apontar para lib.typ, não para main.typ; got:\n{}",
        stderr
    );
    assert!(
        !stderr.contains("<detached>"),
        "stderr não deve conter '<detached>' para span resolvível; got:\n{}",
        stderr
    );
    assert!(
        stderr.contains(":1:"),
        "stderr deve conter linha:coluna (esperada linha 1); got:\n{}",
        stderr
    );

    let _ = fs::remove_dir_all(&root);
}

/// P772d: erro de I/O em `#import` de path relativo inexistente mostra
/// o span do path no documento principal, não `<detached>`.
#[test]
fn p772d_io_import_path_inexistente_nao_detached() {
    let input = temp_typ("p772d", "#import \"preview/nome:1.0.0\"\n");
    let output = temp_pdf("p772d_out");

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(1),
        "esperava exit 1 para path inexistente; stderr:\n{}",
        stderr
    );
    assert!(stderr.contains("error:"), "stderr deve conter 'error:'; got:\n{}", stderr);
    assert!(
        stderr.contains("ficheiro não encontrado"),
        "stderr deve mencionar ficheiro não encontrado; got:\n{}",
        stderr
    );
    assert!(
        !stderr.contains("<detached>"),
        "stderr não deve conter '<detached>'; got:\n{}",
        stderr
    );
    assert!(
        stderr.contains(":1:"),
        "stderr deve conter linha:coluna do path no doc principal; got:\n{}",
        stderr
    );

    cleanup(&[&input, &output]);
}

// ── P819 — plugin.transition + mensagens/spans de plugin (e2e, host wasmi
// real). Fixture: módulo mutável de 274 B (fonte WAT em temp/p819/mut-ascii.wat
// — protocolo typst_env): exporta `add` (1 arg; incrementa o byte 0 da
// memória) e `get` (devolve o byte 0, inicial "a" via data segment). ─────────

/// `temp/p819/mut-ascii.wasm` (274 B), embebido para o teste ser auto-contido.
const P819_MUT_WASM: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x13, 0x04, 0x60, 0x01, 0x7f,
    0x00, 0x60, 0x02, 0x7f, 0x7f, 0x00, 0x60, 0x01, 0x7f, 0x01, 0x7f, 0x60, 0x00, 0x01,
    0x7f, 0x02, 0x6e, 0x02, 0x09, 0x74, 0x79, 0x70, 0x73, 0x74, 0x5f, 0x65, 0x6e, 0x76,
    0x2a, 0x77, 0x61, 0x73, 0x6d, 0x5f, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x5f,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x5f, 0x77, 0x72, 0x69, 0x74, 0x65,
    0x5f, 0x61, 0x72, 0x67, 0x73, 0x5f, 0x74, 0x6f, 0x5f, 0x62, 0x75, 0x66, 0x66, 0x65,
    0x72, 0x00, 0x00, 0x09, 0x74, 0x79, 0x70, 0x73, 0x74, 0x5f, 0x65, 0x6e, 0x76, 0x29,
    0x77, 0x61, 0x73, 0x6d, 0x5f, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x5f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x5f, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x68, 0x6f, 0x73, 0x74, 0x00,
    0x01, 0x03, 0x03, 0x02, 0x02, 0x03, 0x05, 0x03, 0x01, 0x00, 0x01, 0x07, 0x16, 0x03,
    0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x02, 0x00, 0x03, 0x61, 0x64, 0x64, 0x00,
    0x02, 0x03, 0x67, 0x65, 0x74, 0x00, 0x03, 0x0a, 0x2b, 0x02, 0x1e, 0x00, 0x41, 0x00,
    0x41, 0x00, 0x2d, 0x00, 0x00, 0x41, 0x01, 0x6a, 0x3a, 0x00, 0x00, 0x41, 0x80, 0x20,
    0x10, 0x00, 0x41, 0x80, 0xc0, 0x00, 0x41, 0x00, 0x10, 0x01, 0x41, 0x00, 0x0b, 0x0a,
    0x00, 0x41, 0x00, 0x41, 0x01, 0x10, 0x01, 0x41, 0x00, 0x0b, 0x0b, 0x07, 0x01, 0x00,
    0x41, 0x00, 0x0b, 0x01, 0x61, 0x00, 0x2b, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x01, 0x1a,
    0x02, 0x00, 0x0a, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x61, 0x72, 0x67, 0x73, 0x01,
    0x0b, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x02, 0x08,
    0x01, 0x02, 0x01, 0x00, 0x03, 0x6c, 0x65, 0x6e,
];

/// Escreve um ficheiro auxiliar (wasm) no temp_dir com nome único por pid.
fn temp_file(name: &str, bytes: &[u8]) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!("typst-passo-819-{}-{}", name, std::process::id()));
    fs::write(&path, bytes).expect("escrever ficheiro auxiliar temporário");
    path
}

/// Corre o binário sobre `content` e devolve (status, stderr).
fn run_doc(name: &str, content: &str) -> (std::process::ExitStatus, String) {
    let input = temp_typ(name, content);
    let output = temp_pdf(name);
    let result = Command::new(BIN)
        .arg(&input)
        .arg(&output)
        .output()
        .expect("executar binário");
    let stderr = String::from_utf8_lossy(&result.stderr).into_owned();
    cleanup(&[&input, &output]);
    (result.status, stderr)
}

#[test]
fn cli_plugin_transition_caminho_feliz_p819() {
    let wasm = temp_file("mut.wasm", P819_MUT_WASM);
    let wasm_name = wasm.file_name().unwrap().to_string_lossy().into_owned();
    // base fica inalterado ("a"); cada transition observa a mutação
    // acumulada ("b", "c"). O arg de `add` é `bytes` vindo de `get()`
    // (o construtor `bytes()` ainda não existe — achado transversal P810).
    let doc = format!(
        "#let base = plugin(\"{wasm_name}\")\n\
         #let m1 = plugin.transition(base.add, base.get())\n\
         #let m2 = plugin.transition(m1.add, base.get())\n\
         #str(base.get())#str(m1.get())#str(m2.get())"
    );
    let (status, stderr) = run_doc("transition-ok", &doc);
    assert_eq!(status.code(), Some(0), "exit 0 esperado; stderr:\n{stderr}");
    cleanup(&[&wasm]);
}

#[test]
fn cli_plugin_mensagens_e_spans_verbatim_p819() {
    let wasm = temp_file("mut2.wasm", P819_MUT_WASM);
    let wasm_name = wasm.file_name().unwrap().to_string_lossy().into_owned();
    let garbage = temp_file("garbage.wasm", b"isto nao e um modulo wasm de certeza");
    let garbage_name = garbage.file_name().unwrap().to_string_lossy().into_owned();

    let cases: Vec<(String, &str)> = vec![
        // t5 — tipo do argumento de plugin()
        ("#plugin(42)".into(), "error: expected path, string, or bytes, found integer"),
        // t2 — ficheiro inexistente (verbatim de L3, formato FileError vanilla)
        (
            "#plugin(\"nao-existe-p819.wasm\")".into(),
            "error: file not found (searched at",
        ),
        // t3 — parse WASM: texto wasmi sem a feature `wat` (alinhamento P819)
        (
            format!("#plugin(\"{garbage_name}\")"),
            "error: failed to load WebAssembly module (magic header not detected",
        ),
        // t6 — argumento não-bytes na chamada
        (
            format!("#let p = plugin(\"{wasm_name}\")\n#p.get(1)"),
            "error: expected bytes, found integer",
        ),
        // tm3 — transition sem args
        ("#plugin.transition()".into(), "error: missing argument: func"),
        // tm4 — transition com não-função
        ("#plugin.transition(42)".into(), "error: expected function, found integer"),
        // tm5 — transition com função nativa (não-plugin)
        ("#plugin.transition(plugin)".into(), "error: expected plugin function"),
        // tm6 — transition com arg não-bytes
        (
            format!("#let p = plugin(\"{wasm_name}\")\n#plugin.transition(p.get, 1)"),
            "error: expected bytes, found integer",
        ),
    ];

    for (i, (doc, expected)) in cases.iter().enumerate() {
        let (status, stderr) = run_doc(&format!("p819case{i}"), doc);
        assert_eq!(
            status.code(),
            Some(1),
            "caso {i}: exit 1 esperado; stderr:\n{stderr}",
        );
        assert!(
            stderr.contains(expected),
            "caso {i}: stderr deve conter `{expected}`; got:\n{stderr}",
        );
        assert!(
            !stderr.contains("<detached>"),
            "caso {i}: stderr não deve conter '<detached>' (span do callsite); got:\n{stderr}",
        );
    }

    cleanup(&[&wasm, &garbage]);
}

// ── P866 — detecção de formato de saída pela extensão ───────────────────────

/// Cria um path de output com a extensão pedida (não cria ficheiro).
fn temp_output_with_ext(name: &str, ext: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!("typst-passo-866-{}-{}.{}", name, std::process::id(), ext));
    path
}

/// Remove ficheiro se existir, ignorando erros.
fn remove_if_exists(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

#[test]
fn p870_output_png_gera_png_valido() {
    let input = temp_typ("png_ok", "Texto.");
    let output = temp_output_with_ext("png_ok", "png");
    remove_if_exists(&output);

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(0),
        "esperava exit 0 para output .png; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "deve criar ficheiro .png em {}", output.display());
    let bytes = fs::read(&output).expect("ler png");
    assert!(
        bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]),
        "ficheiro deve começar com magic bytes PNG"
    );

    cleanup(&[&input, &output]);
}

#[test]
fn p870_output_svg_gera_svg_valido() {
    let input = temp_typ("svg_ok", "Texto.");
    let output = temp_output_with_ext("svg_ok", "svg");
    remove_if_exists(&output);

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(0),
        "esperava exit 0 para output .svg; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "deve criar ficheiro .svg em {}", output.display());
    let text = fs::read_to_string(&output).expect("ler svg");
    assert!(text.contains("<svg"), "SVG deve conter elemento <svg");
    assert!(text.contains("</svg>"), "SVG deve conter </svg>");

    cleanup(&[&input, &output]);
}

#[test]
fn p870_format_flag_png_vence_extensao_pdf() {
    let input = temp_typ("format_flag_png", "Texto.");
    let output = temp_output_with_ext("format_flag_png", "pdf");
    remove_if_exists(&output);

    let result = Command::new(BIN)
        .arg(&input)
        .arg("--format")
        .arg("png")
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(0),
        "esperava exit 0 quando --format png é usado; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "deve criar ficheiro de output quando --format png é usado");
    let bytes = fs::read(&output).expect("ler output");
    assert!(
        bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]),
        "conteúdo deve ser PNG independentemente da extensão .pdf"
    );

    cleanup(&[&input, &output]);
}

#[test]
fn p870_format_flag_svg_vence_extensao_pdf() {
    let input = temp_typ("format_flag_svg", "Texto.");
    let output = temp_output_with_ext("format_flag_svg", "pdf");
    remove_if_exists(&output);

    let result = Command::new(BIN)
        .arg(&input)
        .arg("--format")
        .arg("svg")
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(0),
        "esperava exit 0 quando --format svg é usado; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "deve criar ficheiro de output quando --format svg é usado");
    let text = fs::read_to_string(&output).expect("ler output");
    assert!(
        text.contains("<svg"),
        "conteúdo deve ser SVG independentemente da extensão .pdf"
    );

    cleanup(&[&input, &output]);
}

#[test]
fn p866_output_pdf_continua_funcionar() {
    let input = temp_typ("pdf_ok", "Texto.");
    let output = temp_output_with_ext("pdf_ok", "pdf");
    remove_if_exists(&output);

    let result = Command::new(BIN)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(0),
        "esperava exit 0 para output .pdf; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());

    let bytes = fs::read(&output).expect("ler PDF");
    assert!(
        bytes.starts_with(b"%PDF-"),
        "PDF deve começar com '%PDF-'; primeiros bytes: {:?}",
        &bytes[..bytes.len().min(8)]
    );

    cleanup(&[&input, &output]);
}

#[test]
fn p866_format_flag_pdf_continua_funcionar() {
    let input = temp_typ("format_flag_pdf", "Texto.");
    let output = temp_output_with_ext("format_flag_pdf", "pdf");
    remove_if_exists(&output);

    let result = Command::new(BIN)
        .arg(&input)
        .arg("--format")
        .arg("pdf")
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar binário");

    let stderr = String::from_utf8_lossy(&result.stderr);

    assert_eq!(
        result.status.code(),
        Some(0),
        "esperava exit 0 para --format pdf; stderr:\n{}",
        stderr
    );
    assert!(output.exists(), "PDF deve existir em {}", output.display());

    cleanup(&[&input, &output]);
}
