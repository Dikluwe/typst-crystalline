//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/tests/cli.md
//! @prompt-hash 4b66a2f5
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
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
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

impl WatchedChild {
    fn assert_alive(&mut self, phase: &str) {
        match self.0.try_wait() {
            Ok(None) => {}
            Ok(Some(status)) => {
                panic!("watch terminou durante {phase} com status {status}")
            }
            Err(error) => {
                panic!("não foi possível consultar watch durante {phase}: {error}")
            }
        }
    }
}

impl Drop for WatchedChild {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn wait_until(
    watch: &mut WatchedChild,
    phase: &str,
    timeout: Duration,
    mut condition: impl FnMut() -> bool,
) {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        watch.assert_alive(phase);
        if condition() {
            return;
        }
        thread::sleep(Duration::from_millis(50));
    }
    watch.assert_alive(phase);
    panic!("timeout durante {phase} após {timeout:?}");
}

struct WatchFixture(PathBuf);

impl Drop for WatchFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
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
    let _ = fs::remove_dir_all(&root);
    let input = root.join("main.typ");
    let asset = root.join("data.txt");
    let irrelevant = root.join("irrelevant.txt");
    let output = root.join("main.pdf");
    fs::create_dir_all(&root).unwrap();
    let _fixture = WatchFixture(root.clone());
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
    let child_pid = child.id();
    let staging = output.with_file_name(format!(
        ".{}.crystalline-watch-{child_pid}.tmp",
        output.file_name().unwrap().to_string_lossy()
    ));
    let mut watch = WatchedChild(child);

    watch.assert_alive("publicação inicial");
    wait_until(&mut watch, "publicação inicial", Duration::from_secs(20), || {
        output.exists()
    });
    let first = fs::read(&output).unwrap();

    fs::write(&asset, "second").unwrap();
    wait_until(&mut watch, "recompilação por asset", Duration::from_secs(20), || {
        fs::read(&output).map(|bytes| bytes != first).unwrap_or(false)
    });
    let second = fs::read(&output).unwrap();

    watch.assert_alive("filtro de ficheiro irrelevante");
    fs::write(&irrelevant, "changed but not observed").unwrap();
    thread::sleep(Duration::from_millis(400));
    watch.assert_alive("filtro de ficheiro irrelevante");
    assert_eq!(fs::read(&output).unwrap(), second);

    watch.assert_alive("erro transitório");
    fs::write(
        &staging,
        format!("p1296-recovery-sentinel-{}-{child_pid}", std::process::id()),
    )
    .unwrap();
    fs::write(&input, "#unknown-watch-name").unwrap();
    wait_until(
        &mut watch,
        "armamento após erro transitório",
        Duration::from_secs(20),
        || !staging.exists(),
    );
    watch.assert_alive("preservação após erro transitório");
    assert_eq!(fs::read(&output).unwrap(), second);

    fs::write(&input, "Recovered").unwrap();
    wait_until(&mut watch, "recuperação", Duration::from_secs(20), || {
        fs::read(&output).map(|bytes| bytes != second).unwrap_or(false)
    });

    drop(watch);
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

#[test]
fn p1163_eval_symbol_json_preserva_grapheme_integral() {
    for (expression, expected) in
        [("emoji.heart", "\"❤️\"\n"), ("symbol(\"👩‍💻\")", "\"👩‍💻\"\n")]
    {
        let result = Command::new(BIN)
            .args(["eval", expression, "--format", "json"])
            .output()
            .expect("executar typst eval");
        assert_eq!(
            result.status.code(),
            Some(0),
            "stderr: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(String::from_utf8(result.stdout).unwrap(), expected);
    }
}

#[test]
fn p1163_eval_symbol_raw_nomeia_tipo_publico() {
    let result = Command::new(BIN)
        .args(["eval", "emoji.heart", "--format", "raw"])
        .output()
        .expect("executar typst eval");
    assert_eq!(result.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("cannot print symbol in raw format"), "{stderr}");
    assert!(stderr.contains("only supports strings and bytes"), "{stderr}");
}

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

#[test]
fn p1166_html_feature_gate_e_elem_vertical() {
    let input =
        temp_typ("p1166-html", "#html.elem(\"article\", attrs: (lang: \"pt\"))[Olá]");
    let output = temp_output_with_ext("p1166-html", "html");
    remove_if_exists(&output);

    let off = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html"])
        .output()
        .unwrap();
    assert_eq!(off.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&off.stderr)
        .contains("html export is only available when `--features html` is passed"));
    assert!(!output.exists());

    let on = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(on.status.code(), Some(0), "{}", String::from_utf8_lossy(&on.stderr));
    let html = fs::read_to_string(&output).unwrap();
    assert!(html.contains("<article lang=\"pt\">Olá</article>"));

    cleanup(&[&input, &output]);
}

#[test]
fn p1166_eval_html_feature_off_on() {
    let off = Command::new(BIN)
        .args(["eval", "repr(type(html))", "--format", "raw"])
        .output()
        .unwrap();
    assert_eq!(off.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&off.stderr).contains(
        "cannot access variable `html` because the `html` feature is not enabled"
    ));

    let on = Command::new(BIN)
        .args([
            "eval",
            "repr(html.elem(\"article\")[Olá])",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(on.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&on.stdout),
        "elem(tag: \"article\", body: [Olá])"
    );
}

#[test]
fn p1168_html_typed_batch_repr_and_dom() {
    let eval = Command::new(BIN)
        .args([
            "eval",
            "repr((html.div(), html.span[x], html.p(id: \"p\", class: (\"a\", \"b\"), hidden: true)[x], html.h1[x], html.h2[x], html.h3[x], html.h4[x], html.h5[x], html.h6[x], html.strong[x], html.em[x], html.ul[x]))",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(eval.status.code(), Some(0), "{}", String::from_utf8_lossy(&eval.stderr));
    let repr = String::from_utf8_lossy(&eval.stdout);
    assert_eq!(
        repr,
        r#"(
  elem(tag: "div", body: none),
  elem(tag: "span", body: [x]),
  elem(
    tag: "p",
    attrs: (id: "p", class: "a b", hidden: ""),
    body: [x],
  ),
  elem(tag: "h1", body: [x]),
  elem(tag: "h2", body: [x]),
  elem(tag: "h3", body: [x]),
  elem(tag: "h4", body: [x]),
  elem(tag: "h5", body: [x]),
  elem(tag: "h6", body: [x]),
  elem(tag: "strong", body: [x]),
  elem(tag: "em", body: [x]),
  elem(tag: "ul", body: [x]),
)"#
    );

    let input = temp_typ(
        "p1168-html-typed",
        "#html.div(hidden: true)[#html.h1[H]#html.p[P #html.strong[S] #html.em[E] #html.span[X]]#html.ul[U]]",
    );
    let output = temp_output_with_ext("p1168-html-typed", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let html = fs::read_to_string(&output).unwrap();
    assert!(html.contains(
        "<div hidden><h1>H</h1><p>P <strong>S</strong> <em>E</em> <span>X</span></p><ul>U</ul></div>"
    ));
    cleanup(&[&input, &output]);
}

#[test]
fn p1169_1_html_ol_li_repr_casts_e_dom() {
    let eval = Command::new(BIN)
        .args([
            "eval",
            "repr(html.ol(id: \"o\", start: -2, reversed: true, type: \"A\")[#html.li(value: 3)[X]])",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(eval.status.code(), Some(0), "{}", String::from_utf8_lossy(&eval.stderr));
    let repr = String::from_utf8_lossy(&eval.stdout);
    assert!(repr.contains("tag: \"ol\""));
    assert!(
        repr.contains("attrs: (id: \"o\", start: \"-2\", reversed: \"\", type: \"A\")")
    );
    assert!(repr.contains("tag: \"li\""));
    assert!(repr.contains("attrs: (value: \"3\")"));

    let input = temp_typ(
        "p1169-1-html-lists",
        "#html.ol(id: \"o\", start: 2, reversed: true, type: \"A\")[#html.li(value: 3)[X]#html.li[Y #html.strong[Z]]]",
    );
    let output = temp_output_with_ext("p1169-1-html-lists", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let html = fs::read_to_string(&output).unwrap();
    assert!(html.contains(
        "<ol id=\"o\" start=\"2\" reversed type=\"A\"><li value=\"3\">X</li><li>Y <strong>Z</strong></li></ol>"
    ));
    cleanup(&[&input, &output]);
}

#[test]
fn p1170_1_html_a_repr_casts_e_dom_aninhado() {
    let eval = Command::new(BIN)
        .args([
            "eval",
            "repr(html.a(id: \"k\", download: \"d\", href: \"/x\", hreflang: \"pt\", ping: (\"/p1\", \"/p2\"), referrerpolicy: none, rel: (\"noopener\", \"noreferrer\"), target: \"named\", type: \"text/plain\")[X])",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(eval.status.code(), Some(0), "{}", String::from_utf8_lossy(&eval.stderr));
    let repr = String::from_utf8_lossy(&eval.stdout);
    assert!(repr.contains("tag: \"a\""));
    assert!(repr.contains("ping: \"/p1 /p2\""));
    assert!(repr.contains("referrerpolicy: \"\""));
    assert!(repr.contains("rel: \"noopener noreferrer\""));
    assert!(repr.contains("target: \"named\""));

    let input = temp_typ(
        "p1170-1-html-a",
        "#html.div[#html.a(id: \"k\", href: \"x&y\", rel: (\"noopener\", \"noreferrer\"), target: \"_blank\")[X #html.strong[Z]]]",
    );
    let output = temp_output_with_ext("p1170-1-html-a", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let html = fs::read_to_string(&output).unwrap();
    assert!(html.contains(
        "<div><a id=\"k\" href=\"x&amp;y\" rel=\"noopener noreferrer\" target=\"_blank\">X <strong>Z</strong></a></div>"
    ));
    cleanup(&[&input, &output]);
}

#[test]
fn p1171_1_html_br_void_e_whitespace_dom() {
    let eval = Command::new(BIN)
        .args([
            "eval",
            "repr(html.br(id: \"b\"))",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(eval.status.code(), Some(0), "{}", String::from_utf8_lossy(&eval.stderr));
    assert_eq!(
        String::from_utf8_lossy(&eval.stdout),
        "elem(tag: \"br\", attrs: (id: \"b\"))"
    );

    let input = temp_typ(
        "p1171-1-html-br",
        "#html.div[\n  A\n  #html.br()\n  B\n  #html.span[C]\n  #html.span[D]\n]",
    );
    let output = temp_output_with_ext("p1171-1-html-br", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let html = fs::read_to_string(&output).unwrap();
    assert!(html.contains("<div>A<br>B <span>C</span> <span>D</span></div>"));
    assert!(!html.contains("</br>"));
    cleanup(&[&input, &output]);
}

#[test]
fn p1172_1_html_agrupa_phrasing_no_topo() {
    let input = temp_typ(
        "p1172-1-html-phrasing",
        "#html.span[A]\n#html.div[B]\n#html.a[C]\n\n#html.elem(\"a\")[D]",
    );
    let output = temp_output_with_ext("p1172-1-html-phrasing", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let html = fs::read_to_string(&output).unwrap();
    assert!(
        html.contains("<p><span>A</span></p><div>B</div><p><a>C</a></p><p><a>D</a></p>")
    );
    cleanup(&[&input, &output]);
}

#[test]
fn p1173_1_html_lote_global_only_2_e_whitespace_block() {
    let eval = Command::new(BIN)
        .args([
            "eval",
            "repr(html.abbr(id: \"sigla\", hidden: true)[A])",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(eval.status.code(), Some(0), "{}", String::from_utf8_lossy(&eval.stderr));
    assert_eq!(
        String::from_utf8_lossy(&eval.stdout),
        r#"elem(
  tag: "abbr",
  attrs: (id: "sigla", hidden: ""),
  body: [A],
)"#
    );

    let input = temp_typ(
        "p1173-1-html-global-only-2",
        "#html.article(id: \"art\")[#html.address[Addr] #html.aside[Side]]\n#html.p[#html.abbr[A]#html.b[B]#html.bdi[C]#html.bdo[D]#html.cite[E]#html.code[F]#html.dfn[G]#html.i[H]#html.kbd[I]]",
    );
    let output = temp_output_with_ext("p1173-1-html-global-only-2", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let html = fs::read_to_string(&output).unwrap();
    assert!(html.contains(
        "<article id=\"art\"><address>Addr</address><aside>Side</aside></article><p><abbr>A</abbr><b>B</b><bdi>C</bdi><bdo>D</bdo><cite>E</cite><code>F</code><dfn>G</dfn><i>H</i><kbd>I</kbd></p>"
    ));
    cleanup(&[&input, &output]);
}

#[test]
fn p1174_1_html_lote_global_only_3() {
    let eval = Command::new(BIN)
        .args([
            "eval",
            "repr(html.mark(id: \"m\", hidden: true)[M])",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(eval.status.code(), Some(0), "{}", String::from_utf8_lossy(&eval.stderr));
    assert_eq!(
        String::from_utf8_lossy(&eval.stdout),
        r#"elem(
  tag: "mark",
  attrs: (id: "m", hidden: ""),
  body: [M],
)"#
    );

    let input = temp_typ(
        "p1174-1-html-global-only-3",
        "#html.dl[#html.dt[Term] #html.dd[Definition]]\n#html.figure[#html.figcaption[Caption]]\n#html.header[Head]\n#html.hgroup[#html.h1[Title]]\n#html.main[#html.legend[Legend] #html.menu[Menu] #html.mark[Marked]]\n#html.footer[Foot]",
    );
    let output = temp_output_with_ext("p1174-1-html-global-only-3", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let html = fs::read_to_string(&output).unwrap();
    assert!(html.contains(
        "<dl><dt>Term</dt><dd>Definition</dd></dl><figure><figcaption>Caption</figcaption></figure><header>Head</header><hgroup><h1>Title</h1></hgroup><main><legend>Legend</legend><menu>Menu</menu><mark>Marked</mark></main><footer>Foot</footer>"
    ));
    cleanup(&[&input, &output]);
}

#[test]
fn p1175_1_html_lote_global_only_4_e_espaco_protegido() {
    let eval = Command::new(BIN)
        .args([
            "eval",
            "repr(html.picture(id: \"pic\", hidden: true)[P])",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(eval.status.code(), Some(0), "{}", String::from_utf8_lossy(&eval.stderr));
    assert_eq!(
        String::from_utf8_lossy(&eval.stdout),
        r#"elem(
  tag: "picture",
  attrs: (id: "pic", hidden: ""),
  body: [P],
)"#
    );

    let input = temp_typ(
        "p1175-1-html-global-only-4",
        "#html.nav[#html.section[Section]]\n#html.picture()\n#html.picture[]\n#html.pre[A B]\n#html.search[Search]\n#html.s[S]#html.samp[Samp]#html.small[Small]#html.sub[Sub]#html.sup[Sup]#html.u[U]#html.var[Var]",
    );
    let output = temp_output_with_ext("p1175-1-html-global-only-4", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let html = fs::read_to_string(&output).unwrap();
    assert!(html.contains(
        "<nav><section>Section</section></nav><p><picture></picture><span style=\"white-space: pre-wrap\">&#x20;</span><picture></picture></p><pre>A B</pre><search>Search</search><p><s>S</s><samp>Samp</samp><small>Small</small><sub>Sub</sub><sup>Sup</sup><u>U</u><var>Var</var></p>"
    ));
    cleanup(&[&input, &output]);
}

#[test]
fn p1176_1_html_lote_residual_normal() {
    let eval = Command::new(BIN)
        .args([
            "eval",
            "repr(html.summary(id: \"s\", hidden: true)[S])",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(eval.status.code(), Some(0), "{}", String::from_utf8_lossy(&eval.stderr));
    assert_eq!(
        String::from_utf8_lossy(&eval.stdout),
        r#"elem(
  tag: "summary",
  attrs: (id: "s", hidden: ""),
  body: [S],
)"#
    );

    let input = temp_typ(
        "p1176-1-html-residual-normal",
        "#html.span[A]#html.datalist[Hidden]#html.noscript[Fallback]#html.span[B]\n#html.summary[Top]\n#html.elem(\"details\")[#html.summary[Inside] Body]",
    );
    let output = temp_output_with_ext("p1176-1-html-residual-normal", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let html = fs::read_to_string(&output).unwrap();
    assert!(html.contains(
        "<p><span>A</span></p><datalist>Hidden</datalist><p><noscript>Fallback</noscript><span>B</span></p><summary>Top</summary><details><summary>Inside</summary>Body</details>"
    ));
    cleanup(&[&input, &output]);
}

#[test]
fn p1177_1_html_familia_ruby() {
    let eval = Command::new(BIN)
        .args([
            "eval",
            "repr(html.ruby(id: \"r\", hidden: true)[R])",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(eval.status.code(), Some(0), "{}", String::from_utf8_lossy(&eval.stderr));
    assert_eq!(
        String::from_utf8_lossy(&eval.stdout),
        r#"elem(
  tag: "ruby",
  attrs: (id: "r", hidden: ""),
  body: [R],
)"#
    );

    let input = temp_typ(
        "p1177-1-html-ruby",
        "#html.ruby[A #html.rt[T]]\n#html.ruby[A #html.span[X] B]\n#html.ruby[#html.rp[(] #html.rt[T] #html.rp[)]]\n#html.rp[P]\n#html.rt[T]",
    );
    let output = temp_output_with_ext("p1177-1-html-ruby", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let html = fs::read_to_string(&output).unwrap();
    assert!(html.contains(
        "<p><ruby>A <rt>T</rt></ruby> <ruby>A <span>X</span> B</ruby> <ruby><rp>(</rp><rt>T</rt><rp>)</rp></ruby></p><rp>P</rp><rt>T</rt>"
    ));
    cleanup(&[&input, &output]);
}

#[test]
fn p1178_1_html_familia_documento() {
    let eval = Command::new(BIN)
        .args([
            "eval",
            "repr(html.title(id: \"t\", hidden: true)[T])",
            "--format",
            "raw",
            "--features",
            "html",
        ])
        .output()
        .unwrap();
    assert_eq!(eval.status.code(), Some(0), "{}", String::from_utf8_lossy(&eval.stderr));
    assert_eq!(
        String::from_utf8_lossy(&eval.stdout),
        r#"elem(
  tag: "title",
  attrs: (id: "t", hidden: ""),
  body: [T],
)"#
    );

    let input = temp_typ(
        "p1178-1-html-documento",
        "#html.html(lang: \"pt\")[#html.head[#html.title[#text(\"A  B\\nC & < > \\\"\")]]#html.body[B]]",
    );
    let output = temp_output_with_ext("p1178-1-html-documento", "html");
    remove_if_exists(&output);
    let compiled = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_eq!(
        compiled.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    assert_eq!(
        fs::read_to_string(&output).unwrap(),
        "<!DOCTYPE html><html lang=\"pt\"><head><title>A  B\nC &amp; &lt; > \"</title></head><body>B</body></html>"
    );
    cleanup(&[&input, &output]);

    let invalid = temp_typ("p1178-1-html-documento-invalido", "A#html.body[B]");
    let invalid_output = temp_output_with_ext("p1178-1-html-documento-invalido", "html");
    remove_if_exists(&invalid_output);
    let failed = Command::new(BIN)
        .args(["compile"])
        .arg(&invalid)
        .arg(&invalid_output)
        .args(["--format", "html", "--features", "html"])
        .output()
        .unwrap();
    assert_ne!(failed.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&failed.stderr)
        .contains("`<body>` element must be the only element in the document"));
    cleanup(&[&invalid, &invalid_output]);
}

#[test]
fn p1285_query_stdin_transporta_yaml_ate_stdout() {
    let mut child = Command::new(BIN)
        .args(["query", "-", "metadata", "--field", "value", "--one", "--format", "yaml"])
        .env("NO_COLOR", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("executar query por stdin");
    child
        .stdin
        .take()
        .expect("stdin do child")
        .write_all(b"#metadata((name: \"x\", n: 2))\n")
        .expect("escrever source Typst em stdin");

    let output = child.wait_with_output().expect("aguardar query");
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "name: x\nn: 2\n");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("subcommand is deprecated"),
        "warning esperado em stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn p1359_compile_pdf(name: &str, flags: &[&str]) -> (std::process::Output, Vec<u8>) {
    let input = temp_typ(name, "$x$");
    let output = temp_pdf(name);
    let result = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(flags)
        .output()
        .expect("executar compile P1359");
    let pdf = fs::read(&output).unwrap_or_default();
    cleanup(&[&input, &output]);
    (result, pdf)
}

fn p1359_has_tagged_pdf_morphology(pdf: &[u8]) -> bool {
    let raw = String::from_utf8_lossy(pdf);
    raw.contains("/StructTreeRoot")
        && raw.contains("/MarkInfo")
        && raw.contains("/Marked true")
}

#[test]
fn p1359_pdf_tagged_requires_equals_and_rejects_invalid_values() {
    let input = temp_typ("p1359-diagnostics", "$x$");
    let output = temp_pdf("p1359-diagnostics");
    let cases: &[(&[&str], &str)] = &[
        (&["--pdf-tagged", "false"], "unexpected argument 'false'"),
        (&["--pdf-tagged=maybe"], "invalid value 'maybe'"),
        (&["--pdf-tagged="], "a value is required"),
    ];

    for (flags, expected) in cases {
        let result = Command::new(BIN)
            .args(["compile"])
            .arg(&input)
            .arg(&output)
            .args(*flags)
            .output()
            .expect("executar diagnóstico P1359");
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert_eq!(result.status.code(), Some(2), "stderr:\n{stderr}");
        assert!(stderr.contains(expected), "esperava {expected:?}; stderr:\n{stderr}");
        assert!(!output.exists(), "CLI inválido não pode produzir PDF");
    }

    cleanup(&[&input, &output]);
}

#[test]
fn p1359_pdf_tagged_default_moderno_legado_e_precedencia() {
    let cases: &[(&str, &[&str], bool)] = &[
        ("p1359-default", &[], true),
        ("p1359-modern-true", &["--pdf-tagged=true"], true),
        ("p1359-modern-false", &["--pdf-tagged=false"], false),
        ("p1359-legacy-false", &["--no-pdf-tags"], false),
        ("p1359-modern-wins", &["--no-pdf-tags", "--pdf-tagged=true"], true),
    ];

    for (name, flags, expected_tagged) in cases {
        let (result, pdf) = p1359_compile_pdf(name, flags);
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert_eq!(result.status.code(), Some(0), "flags={flags:?}; stderr:\n{stderr}");
        assert_eq!(
            p1359_has_tagged_pdf_morphology(&pdf),
            *expected_tagged,
            "morfologia inesperada para flags={flags:?}"
        );
    }
}

#[test]
fn p1359_pdf_tagged_e_compact_sao_ortogonais() {
    let (tagged_result, tagged) =
        p1359_compile_pdf("p1359-compact-tagged", &["--compact", "--pdf-tagged=true"]);
    let (untagged_result, untagged) =
        p1359_compile_pdf("p1359-compact-untagged", &["--compact", "--pdf-tagged=false"]);
    assert_eq!(tagged_result.status.code(), Some(0));
    assert_eq!(untagged_result.status.code(), Some(0));
    assert!(p1359_has_tagged_pdf_morphology(&tagged));
    assert!(!p1359_has_tagged_pdf_morphology(&untagged));
    for pdf in [&tagged, &untagged] {
        let raw = String::from_utf8_lossy(pdf);
        assert!(!raw.contains("/ColorSpace"), "--compact deve permanecer ativo");
        assert!(!raw.contains("/ICCBased"), "--compact deve permanecer ativo");
    }
}

#[test]
fn p1359_pdf_tagged_e_neutro_para_png() {
    let input = temp_typ("p1359-png-neutral", "Texto.");
    let output = temp_output_with_ext("p1359-png-neutral", "png");
    remove_if_exists(&output);
    let result = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .args(["--format", "png", "--pdf-tagged=false"])
        .output()
        .expect("executar PNG P1359");
    let bytes = fs::read(&output).unwrap_or_default();
    cleanup(&[&input, &output]);
    assert_eq!(
        result.status.code(),
        Some(0),
        "stderr:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(bytes.starts_with(&[0x89, 0x50, 0x4e, 0x47]));
}

fn p1485_fixture(name: &str, source: &str) -> (WatchFixture, PathBuf, PathBuf) {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = env::temp_dir().join(format!("typst-p1485-{name}-{}-{stamp}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let input = root.join("main.typ");
    let output = root.join("bundle");
    fs::write(&input, source).unwrap();
    (WatchFixture(root), input, output)
}

fn p1485_compile(input: &PathBuf, output: &PathBuf, features: &str) -> std::process::Output {
    Command::new(BIN)
        .arg("compile")
        .arg(input)
        .arg(output)
        .args(["--format", "bundle", "--features", features])
        .output()
        .unwrap()
}

fn p1485_http_get(port: u16, path: &str) -> Option<Vec<u8>> {
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_millis(200)).ok()?;
    stream.set_read_timeout(Some(Duration::from_millis(500))).ok()?;
    stream.write_all(format!("GET {path} HTTP/1.0\r\nHost: localhost\r\nConnection: close\r\n\r\n").as_bytes()).ok()?;
    let mut response = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(size) => response.extend_from_slice(&chunk[..size]),
            Err(error) if matches!(error.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock) => break,
            Err(_) => return None,
        }
    }
    (!response.is_empty()).then_some(response)
}

fn p1485_http_body(response: &[u8]) -> &[u8] {
    let boundary = response.windows(4).position(|window| window == b"\r\n\r\n").unwrap();
    &response[boundary + 4..]
}

fn p1485_free_port() -> u16 {
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    listener.local_addr().unwrap().port()
}

#[test]
fn p1485_bundle_mistura_documentos_e_preserva_asset_byte_exato() {
    let (_fixture, input, output) = p1485_fixture(
        "mixed",
        "#asset(\"assets/./raw.bin\", bytes((0, 255, 10)))\n\
         #asset(\"assets/../style.css\", \"café\")\n\
         #document(\"pdf/a.pdf\")[Hello]\n\
         #document(\"svg/b.svg\")[Hello]\n\
         #document(\"png/c.png\")[Hello]\n\
         #document(\"web/index.html\")[Hello]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert_eq!(result.status.code(), Some(0), "{stderr}");
    assert!(stderr.contains("bundle export is experimental"), "{stderr}");
    assert_eq!(fs::read(output.join("assets/raw.bin")).unwrap(), [0, 255, 10]);
    assert_eq!(fs::read(output.join("style.css")).unwrap(), "café".as_bytes());
    let pdf = fs::read(output.join("pdf/a.pdf")).unwrap();
    assert!(pdf.starts_with(b"%PDF-"), "PDF ausente ou inválido");
    let pdf_tail = &pdf[pdf.len().saturating_sub(16)..];
    assert!(pdf_tail.windows(5).any(|window| window == b"%%EOF"), "PDF truncado");
    let svg = fs::read_to_string(output.join("svg/b.svg")).unwrap();
    assert!(svg.contains("<svg") && svg.contains("</svg>"), "{svg}");
    let png = fs::read(output.join("png/c.png")).unwrap();
    assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"), "PNG ausente ou inválido");
    let html = fs::read_to_string(output.join("web/index.html")).unwrap();
    assert!(html.contains("Hello"), "{html}");
}

#[test]
fn p1485_bundle_rejeita_entradas_invalidas_sem_publicar_arvore() {
    for (name, source, features, diagnostic) in [
        ("no-feature", "#asset(\"a\", \"x\")", "html", "bundle export is only available"),
        ("duplicate", "#asset(\"a/../x.txt\", \"one\")\n#asset(\"x.txt\", \"two\")", "bundle", "occurs multiple times in the bundle"),
        ("mixed-duplicate", "#asset(\"x.pdf\", \"one\")\n#document(\"./x.pdf\")[Two]", "bundle", "occurs multiple times in the bundle"),
        ("escape", "#asset(\"../outside.txt\", \"x\")", "bundle", "would escape the bundle root"),
        ("top-level", "Loose text", "bundle", "text is not allowed at the top-level in bundle export"),
        ("unknown-format", "#document(\"x.weird\")[X]", "bundle", "unknown document format"),
        ("html-feature", "#document(\"index.html\")[X]", "bundle", "html"),
    ] {
        let (_fixture, input, output) = p1485_fixture(name, source);
        let result = p1485_compile(&input, &output, features);
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert_ne!(result.status.code(), Some(0), "{name}: {stderr}");
        assert!(stderr.contains(diagnostic), "{name}: {stderr}");
        assert!(!output.exists(), "{name}: bundle parcial publicado");
    }

    let (_fixture, input, output) = p1485_fixture("outside", "#asset(\"x.txt\", \"x\")");
    let result = Command::new(BIN)
        .arg("compile").arg(&input).arg(output.with_extension("pdf"))
        .args(["--format", "pdf", "--features", "bundle"])
        .output().unwrap();
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert_ne!(result.status.code(), Some(0), "{stderr}");
    assert!(stderr.contains("assets are only supported in the bundle target"), "{stderr}");
}

#[test]
fn p1485_bundle_default_output_e_stdout() {
    let (_fixture, input, output) = p1485_fixture("default", "#asset(\"a.txt\", \"exact\")");
    let result = Command::new(BIN)
        .arg("compile").arg(&input)
        .args(["--format", "bundle", "--features", "bundle"])
        .output().unwrap();
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    let default_output = input.with_extension("");
    assert_eq!(fs::read(default_output.join("a.txt")).unwrap(), b"exact");
    assert!(!output.exists());

    let stdout = Command::new(BIN)
        .arg("compile").arg(&input).arg("-")
        .args(["--format", "bundle", "--features", "bundle"])
        .output().unwrap();
    assert_ne!(stdout.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&stdout.stderr).contains("cannot write bundle to standard output"));
    assert!(stdout.stdout.is_empty());
}

#[test]
fn p1485_watch_bundle_http_reload_e_preserva_snapshot_apos_erro() {
    let (_fixture, input, output) = p1485_fixture(
        "watch-http",
        "#asset(\"raw.bin\", bytes((0, 255, 10)))\n\
         #document(\"index.html\")[First]\n\
         #document(\"dir/index.html\")[Nested]",
    );
    let port = p1485_free_port();
    let mut child = Command::new(BIN)
        .arg("watch").arg(&input).arg(&output)
        .args(["--format", "bundle", "--features", "bundle,html", "--port"])
        .arg(port.to_string())
        .stdout(Stdio::null()).stderr(Stdio::piped())
        .spawn().unwrap();
    let stderr = child.stderr.take().unwrap();
    let (sender, receiver) = mpsc::channel();
    let log_reader = thread::spawn(move || {
        for line in BufReader::new(stderr).lines().map_while(Result::ok) {
            if sender.send(line).is_err() { break; }
        }
    });
    let mut watch = WatchedChild(child);
    wait_until(&mut watch, "bundle inicial e servidor", Duration::from_secs(20), || {
        output.join("raw.bin").exists()
            && p1485_http_get(port, "/index.html")
                .is_some_and(|response| response.starts_with(b"HTTP/1.1 200") || response.starts_with(b"HTTP/1.0 200"))
    });
    let initial = p1485_http_get(port, "/").unwrap();
    assert!(initial.starts_with(b"HTTP/1.1 200") || initial.starts_with(b"HTTP/1.0 200"));
    assert!(String::from_utf8_lossy(p1485_http_body(&initial)).contains("EventSource"));
    let disk = fs::read(output.join("index.html")).unwrap();
    assert!(!String::from_utf8_lossy(&disk).contains("EventSource"));
    let raw = p1485_http_get(port, "/raw.bin").unwrap();
    assert_eq!(p1485_http_body(&raw), [0, 255, 10]);
    assert!(String::from_utf8_lossy(&raw).to_ascii_lowercase().contains("application/octet-stream"));
    assert!(p1485_http_get(port, "/missing").unwrap().starts_with(b"HTTP/1.1 404")
        || p1485_http_get(port, "/missing").unwrap().starts_with(b"HTTP/1.0 404"));
    let redirect = p1485_http_get(port, "/dir").unwrap();
    assert!(redirect.starts_with(b"HTTP/1.1 301") || redirect.starts_with(b"HTTP/1.0 301"));
    let nested = p1485_http_get(port, "/dir/").unwrap();
    assert!(String::from_utf8_lossy(p1485_http_body(&nested)).contains("Nested"));

    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let mut sse = TcpStream::connect_timeout(&address, Duration::from_secs(2)).unwrap();
    sse.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    sse.write_all(b"GET /__events HTTP/1.1\r\nHost: localhost\r\nAccept: text/event-stream\r\n\r\n").unwrap();
    let mut prelude = [0u8; 1024];
    let size = sse.read(&mut prelude).unwrap();
    assert!(String::from_utf8_lossy(&prelude[..size]).contains("200"));
    fs::write(&input, "#asset(\"raw.bin\", bytes((1, 2, 3)))\n#document(\"index.html\")[Second]").unwrap();
    wait_until(&mut watch, "republicação bundle", Duration::from_secs(20), || {
        fs::read(output.join("raw.bin")).is_ok_and(|bytes| bytes == [1, 2, 3])
    });
    let mut event = Vec::new();
    let mut buffer = [0u8; 1024];
    while !String::from_utf8_lossy(&event).contains("reload") {
        let count = sse.read(&mut buffer).expect("evento SSE após republicação");
        assert_ne!(count, 0, "SSE fechou antes do reload");
        event.extend_from_slice(&buffer[..count]);
    }
    let last_disk = fs::read(output.join("index.html")).unwrap();
    let last_http = p1485_http_get(port, "/index.html").unwrap();
    assert!(String::from_utf8_lossy(p1485_http_body(&last_http)).contains("Second"));

    fs::write(&input, "#unknown-watch-name").unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    let mut observed_error = false;
    while Instant::now() < deadline {
        watch.assert_alive("erro transitório bundle");
        if let Ok(line) = receiver.recv_timeout(Duration::from_millis(200)) {
            if line.contains("unknown-watch-name") || line.contains("unknown variable") {
                observed_error = true;
                break;
            }
        }
    }
    assert!(observed_error, "watch não reportou o erro transitório");
    assert_eq!(fs::read(output.join("index.html")).unwrap(), last_disk);
    assert_eq!(p1485_http_body(&p1485_http_get(port, "/index.html").unwrap()), p1485_http_body(&last_http));
    drop(watch);
    log_reader.join().unwrap();
}

#[test]
fn p1485_watch_flags_no_reload_no_serve_e_porta_ocupada() {
    for (name, flags, expect_http) in [
        ("no-reload", vec!["--no-reload"], true),
        ("no-serve", vec!["--no-serve"], false),
    ] {
        let (_fixture, input, output) = p1485_fixture(name, "#document(\"index.html\")[Hello]");
        let port = p1485_free_port();
        let mut command = Command::new(BIN);
        command.arg("watch").arg(&input).arg(&output)
            .args(["--format", "bundle", "--features", "bundle,html", "--port"])
            .arg(port.to_string()).args(&flags).stdout(Stdio::null()).stderr(Stdio::null());
        let mut watch = WatchedChild(command.spawn().unwrap());
        wait_until(&mut watch, "watch flags", Duration::from_secs(20), || output.join("index.html").exists());
        if expect_http {
            wait_until(&mut watch, "HTTP sem reload", Duration::from_secs(10), || p1485_http_get(port, "/").is_some());
            let response = p1485_http_get(port, "/").unwrap();
            assert!(response.starts_with(b"HTTP/1.1 200") || response.starts_with(b"HTTP/1.0 200"));
            assert!(!String::from_utf8_lossy(p1485_http_body(&response)).contains("EventSource"));
        } else {
            assert!(TcpStream::connect_timeout(
                &SocketAddr::from(([127, 0, 0, 1], port)), Duration::from_millis(300)
            ).is_err(), "--no-serve abriu listener");
        }
    }

    let (_fixture, input, output) = p1485_fixture("occupied", "#document(\"index.html\")[Hello]");
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let port = listener.local_addr().unwrap().port();
    let result = Command::new(BIN)
        .arg("watch").arg(&input).arg(&output)
        .args(["--format", "bundle", "--features", "bundle,html", "--port"])
        .arg(port.to_string())
        .output().unwrap();
    assert!(!result.status.success(), "watch ignorou porta ocupada");
    assert!(!output.exists());
}

#[test]
fn p1485_bundle_target_contextual_por_documento() {
    // Vanilla pinado 586e1bd43: o documento PDF observa "paged" e o HTML observa "html".
    let (_fixture, input, output) = p1485_fixture(
        "targets",
        "#document(\"paged.pdf\")[#context assert.eq(target(), \"paged\") Paged marker]\n\
         #document(\"semantic.html\")[#context assert.eq(target(), \"html\") #context target()]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(
        result.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(fs::read(output.join("paged.pdf")).unwrap().starts_with(b"%PDF-"));
    let html = fs::read_to_string(output.join("semantic.html")).unwrap();
    assert!(html.contains("<body><p>html</p></body>"), "{html}");
}

#[test]
fn p1485_bundle_introspeccao_atravessa_documentos() {
    // Vanilla pinado 586e1bd43: query(heading) vê 3 headings em ambos os arquivos.
    let (_fixture, input, output) = p1485_fixture(
        "cross-introspection",
        "#document(\"one.html\")[= One <one>\n#context query(heading).len()]\n\
         #document(\"two.html\")[= Two <two>\n= Three <three>\n#context query(heading).len()]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(
        result.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let one = fs::read_to_string(output.join("one.html")).unwrap();
    let two = fs::read_to_string(output.join("two.html")).unwrap();
    assert!(one.contains("<h2>One</h2>"), "{one}");
    assert!(two.contains("<h2>Two</h2>") && two.contains("<h2>Three</h2>"), "{two}");
    assert!(one.contains("<p>3</p>"), "consulta cross-document em one.html: {one}");
    assert!(two.contains("<p>3</p>"), "consulta cross-document em two.html: {two}");
}

#[test]
fn p1485_bundle_contexto_posterior_atualiza_consulta_anterior() {
    let (_fixture, input, output) = p1485_fixture(
        "context-cross-generation",
        "#document(\"first.html\")[#context query(heading).len()]\n\
         #document(\"second.html\")[#context [= Generated]]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    let first = fs::read_to_string(output.join("first.html")).unwrap();
    let second = fs::read_to_string(output.join("second.html")).unwrap();
    assert!(first.contains("<p>1</p>"), "{first}");
    assert!(second.contains("<h2>Generated</h2>"), "{second}");
}

#[test]
fn p1485_bundle_counter_global_entre_html_e_pdf() {
    let (_fixture, input, output) = p1485_fixture(
        "counter-cross-target",
        "#set heading(numbering: \"1.\")\n\
         #document(\"first.html\")[#context counter(heading).final().first()]\n\
         #document(\"second.pdf\")[= Beta]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    let first = fs::read_to_string(output.join("first.html")).unwrap();
    assert!(first.contains("<p>1</p>"), "{first}");
    assert!(fs::read(output.join("second.pdf")).unwrap().starts_with(b"%PDF-"));
}

#[test]
fn p1485_bundle_estado_global_e_contexto_aninhado() {
    let (_fixture, input, output) = p1485_fixture(
        "state-nested",
        "#let shared = state(\"bundle-cross\", 0)\n\
         #document(\"first.html\")[#context [#context shared.final()]]\n\
         #document(\"second.pdf\")[#shared.update(7)]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    let first = fs::read_to_string(output.join("first.html")).unwrap();
    assert!(first.contains("<p>7</p>"), "{first}");
    assert!(fs::read(output.join("second.pdf")).unwrap().starts_with(b"%PDF-"));
}

#[test]
fn p1485_bundle_page_counter_e_local_a_cada_pdf() {
    let (_fixture, input, output) = p1485_fixture(
        "page-counter",
        "#document(\"one.pdf\")[#context counter(page).get().first()]\n\
         #document(\"two.pdf\")[#context counter(page).get().first()]",
    );
    let result = p1485_compile(&input, &output, "bundle");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    for name in ["one.pdf", "two.pdf"] {
        let path = output.join(name);
        match Command::new("pdftotext").arg(&path).arg("-").output() {
            Ok(extracted) => {
                assert!(extracted.status.success(), "{name}: pdftotext falhou");
                assert_eq!(String::from_utf8_lossy(&extracted.stdout).trim(), "1", "{name}");
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                assert!(fs::read(path).unwrap().starts_with(b"%PDF-"));
            }
            Err(error) => panic!("{name}: {error}"),
        }
    }
}

#[test]
fn p1485_bundle_page_update_nao_vaza_ao_pdf_seguinte() {
    let (_fixture, input, output) = p1485_fixture(
        "page-update",
        "#document(\"one.pdf\")[#counter(page).update(5) #context counter(page).get().first()]\n\
         #document(\"two.pdf\")[#context counter(page).get().first()]",
    );
    let result = p1485_compile(&input, &output, "bundle");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    for (name, expected) in [("one.pdf", "5"), ("two.pdf", "1")] {
        let path = output.join(name);
        match Command::new("pdftotext").arg(&path).arg("-").output() {
            Ok(extracted) => {
                assert!(extracted.status.success(), "{name}: pdftotext falhou");
                assert_eq!(String::from_utf8_lossy(&extracted.stdout).trim(), expected, "{name}");
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                assert!(fs::read(path).unwrap().starts_with(b"%PDF-"));
            }
            Err(error) => panic!("{name}: {error}"),
        }
    }
}

#[test]
fn p1485_bundle_referencia_rotulada_cruza_pdfs() {
    let (_fixture, input, output) = p1485_fixture(
        "cross-reference",
        "#set heading(numbering: \"1.\")\n\
         #document(\"a.pdf\")[See @beta.]\n\
         #document(\"b.pdf\")[= Beta <beta>]",
    );
    let result = p1485_compile(&input, &output, "bundle");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    match Command::new("pdftotext").arg(output.join("a.pdf")).arg("-").output() {
        Ok(extracted) => {
            assert!(extracted.status.success());
            let text = String::from_utf8_lossy(&extracted.stdout);
            assert!(text.contains("See Section 1."), "{text}");
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("{error}"),
    }
}

#[test]
fn p1485_bundle_html_nao_herda_page_counter_do_pdf() {
    let (_fixture, input, output) = p1485_fixture(
        "page-html-isolation",
        "#document(\"first.pdf\")[#counter(page).update(5)]\n\
         #document(\"second.html\")[#context counter(page).get().first()]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    let html = fs::read_to_string(output.join("second.html")).unwrap();
    assert!(html.contains("<p>1</p>"), "{html}");
}
#[test]
fn p1485_bundle_html_respeita_update_page_proprio() {
    let (_fixture, input, output) = p1485_fixture(
        "page-html-own-update",
        "#document(\"first.pdf\")[#counter(page).update(5)]\n\
         #document(\"second.html\")[#counter(page).update(7) #context counter(page).get().first()]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    let html = fs::read_to_string(output.join("second.html")).unwrap();
    assert!(html.contains("<p>7</p>"), "{html}");
}

#[test]
fn p1485_bundle_page_func_segue_ordem_causal_entre_bodies() {
    let (_fixture, input, output) = p1485_fixture(
        "page-func-boundary",
        "#document(\"first.html\")[#counter(page).update(n => n + 4) #context counter(page).get().first()]\n\
         #document(\"second.html\")[#context counter(page).get().first()]\n\
         #document(\"third.html\")[#counter(page).update(n => n + 6) #counter(page).step() #context counter(page).get().first()]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    for (name, expected) in [("first.html", "5"), ("second.html", "1"), ("third.html", "8")] {
        let html = fs::read_to_string(output.join(name)).unwrap();
        assert!(html.contains(&format!("<p>{expected}</p>")), "{name}: {html}");
    }
}

#[test]
fn p1485_bundle_body_vazio_reinicia_so_page() {
    let (_fixture, input, output) = p1485_fixture(
        "empty-page-boundary",
        "#document(\"first.html\")[#counter(page).update(9) #counter(\"shared\").update(4)]\n\
         #document(\"empty.html\")[]\n\
         #document(\"page.html\")[#context counter(page).get().first()]\n\
         #document(\"shared.html\")[#context counter(\"shared\").get().first()]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    let page = fs::read_to_string(output.join("page.html")).unwrap();
    let shared = fs::read_to_string(output.join("shared.html")).unwrap();
    assert!(page.contains("<p>1</p>"), "page.html: {page}");
    assert!(shared.contains("<p>4</p>"), "shared.html: {shared}");
    assert!(output.join("empty.html").exists(), "body vazio não foi publicado");
}

#[test]
fn p1485_bundle_page_at_label_cruzado_usa_documento_de_origem() {
    let (_fixture, input, output) = p1485_fixture(
        "page-at-cross-document",
        "#document(\"origin.pdf\")[#counter(page).update(5)\n= Origin <origin>]\n\
         #document(\"reader.html\")[#context counter(page).at(<origin>).first()]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    assert!(fs::read(output.join("origin.pdf")).unwrap().starts_with(b"%PDF-"));
    let html = fs::read_to_string(output.join("reader.html")).unwrap();
    assert!(html.contains("<p>5</p>"), "reader.html: {html}");
}

#[test]
fn p1485_bundle_page_func_no_pdf_nao_contamina_html() {
    let (_fixture, input, output) = p1485_fixture(
        "page-func-pdf-html",
        "#document(\"first.pdf\")[#counter(page).update(n => n + 4) #context counter(page).get().first()]\n\
         #document(\"second.html\")[#context counter(page).get().first()]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    let html = fs::read_to_string(output.join("second.html")).unwrap();
    assert!(html.contains("<p>1</p>"), "second.html: {html}");
    match Command::new("pdftotext").arg(output.join("first.pdf")).arg("-").output() {
        Ok(extracted) => {
            assert!(extracted.status.success(), "pdftotext falhou");
            assert_eq!(String::from_utf8_lossy(&extracted.stdout).trim(), "5");
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            assert!(fs::read(output.join("first.pdf")).unwrap().starts_with(b"%PDF-"));
        }
        Err(error) => panic!("{error}"),
    }
}

#[test]
fn p1485_bundle_page_func_erro_impede_publicacao_parcial() {
    let (_fixture, input, output) = p1485_fixture(
        "page-func-error-atomic",
        "#asset(\"ready.txt\", \"not-published\")\n\
         #document(\"first.html\")[#counter(page).update(n => assert.eq(n, 99)) #context counter(page).get().first()]\n\
         #document(\"second.html\")[Would have rendered]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_ne!(result.status.code(), Some(0), "callback inválida passou: {}", String::from_utf8_lossy(&result.stderr));
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("assert"), "erro não veio do callback: {stderr}");
    assert!(!output.exists(), "bundle parcial publicado após erro do callback");
}

#[test]
fn p1485_bundle_html_frame_exporta_svg_inline() {
    let (_fixture, input, output) = p1485_fixture(
        "frame",
        "#document(\"frame.html\")[#html.frame[#rect(width: 20pt, height: 10pt, fill: red)]]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(
        result.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let html = fs::read_to_string(output.join("frame.html")).unwrap();
    assert!(html.contains("<body><svg") && html.contains("</svg></body>"), "{html}");
    assert!(
        (html.contains("<rect") || html.contains("<path"))
            && html.contains("fill=\"#ff4136\""),
        "{html}"
    );
    assert!(!html.contains("<frame"), "frame não deve escapar como tag HTML: {html}");
}

#[test]
fn p1485_bundle_pdf_unicode_e_fonte_embutida() {
    let (_fixture, input, output) = p1485_fixture(
        "unicode-font",
        "#document(\"unicode.pdf\")[#set text(font: \"Libertinus Serif\")\nCafé Ω — Ελληνικά]",
    );
    let result = p1485_compile(&input, &output, "bundle");
    assert_eq!(
        result.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let pdf_path = output.join("unicode.pdf");
    let pdf = fs::read(&pdf_path).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
    let structure = String::from_utf8_lossy(&pdf);
    assert!(structure.contains("/ToUnicode"), "PDF sem mapa Unicode");
    assert!(structure.contains("LibertinusSerif"), "fonte pedida não foi embutida");

    // Poppler é opcional no ambiente de teste; quando existe, verificar também
    // o texto extraível, que é o observável de linguagem medido no vanilla.
    match Command::new("pdftotext").arg(&pdf_path).arg("-").output() {
        Ok(extracted) => {
            assert!(extracted.status.success(), "pdftotext falhou");
            let text = String::from_utf8_lossy(&extracted.stdout);
            assert!(text.contains("Café Ω — Ελληνικά"), "{text}");
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("não foi possível executar pdftotext: {error}"),
    }
}

#[test]
fn p1485_bundle_heading_numbering_continua_entre_pdfs() {
    // Vanilla pinado 586e1bd43: a.pdf extrai "1. Alpha"; b.pdf extrai "2. Beta".
    let (_fixture, input, output) = p1485_fixture(
        "heading-numbering",
        "#set heading(numbering: \"1.\")\n\
         #document(\"a.pdf\")[= Alpha]\n\
         #document(\"b.pdf\")[= Beta]",
    );
    let result = p1485_compile(&input, &output, "bundle");
    assert_eq!(
        result.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let a_path = output.join("a.pdf");
    let b_path = output.join("b.pdf");
    let a = fs::read(&a_path).unwrap();
    let b = fs::read(&b_path).unwrap();
    for (name, pdf) in [("a.pdf", &a), ("b.pdf", &b)] {
        assert!(pdf.starts_with(b"%PDF-"), "{name} não é PDF");
        let body = String::from_utf8_lossy(pdf);
        assert!(body.contains("/ToUnicode"), "{name} sem mapeamento de texto");
    }

    match Command::new("pdftotext").arg(&a_path).arg("-").output() {
        Ok(first) => {
            assert!(first.status.success(), "extração de a.pdf falhou");
            let second = Command::new("pdftotext").arg(&b_path).arg("-").output().unwrap();
            assert!(second.status.success(), "extração de b.pdf falhou");
            let first_text = String::from_utf8_lossy(&first.stdout);
            let second_text = String::from_utf8_lossy(&second.stdout);
            assert!(first_text.contains("1. Alpha"), "a.pdf: {first_text}");
            assert!(second_text.contains("2. Beta"), "b.pdf: {second_text}");
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            // Sem Poppler, o teste ainda exige dois PDFs textuais válidos.
        }
        Err(error) => panic!("não foi possível executar pdftotext: {error}"),
    }
}

#[test]
fn p1485_bundle_html_frame_heading_rotulada_renderiza_svg() {
    // Vanilla pinado 586e1bd43 aceita heading rotulada no body paginado da ilha.
    let (_fixture, input, output) = p1485_fixture(
        "frame-heading",
        "#document(\"frame.html\")[#html.frame[= Framed heading <inside>]]",
    );
    let result = p1485_compile(&input, &output, "bundle,html");
    assert_eq!(
        result.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let html = fs::read_to_string(output.join("frame.html")).unwrap();
    assert!(html.contains("<body><svg") && html.contains("</svg></body>"), "{html}");
    assert!(
        html.contains("<text") || html.contains("<use") || html.contains("<path"),
        "heading deve produzir desenho visível na ilha SVG: {html}"
    );
    assert!(!html.contains("<h2>"), "heading da ilha não deve virar heading HTML: {html}");
    assert!(!html.contains("<frame"), "ilha não deve virar tag HTML frame: {html}");
}
