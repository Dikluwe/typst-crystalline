//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/tests/p1291_callback_runtime.md
//! @prompt-hash ed83a91d
//! @layer L4
//! @updated 2026-08-31
//!
//! Testes black-box independentes para P1291.cancel-angle-runtime.
//!
//! Derivados exclusivamente dos prompts protegidos pelo selo
//! `p1291-callback-runtime-seal.json`. A primeira passagem provisoria nunca e
//! um resultado exportavel; estes casos observam somente compilacoes finais.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_typst");

struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let path = env::temp_dir().join(format!(
            "typst-p1291-callback-{name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id(),
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create P1291 scratch directory");
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn diagnostic(output: &Output) -> String {
    format!(
        "exit={:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
}

fn compile(name: &str, source: &str, extension: &str) -> (Scratch, PathBuf, Output) {
    let scratch = Scratch::new(name);
    let input = scratch.0.join("main.typ");
    let output = scratch.0.join(format!("main.{extension}"));
    fs::write(&input, source).expect("write P1291 fixture");
    let result = Command::new(BIN)
        .args(["compile"])
        .arg(&input)
        .arg(&output)
        .output()
        .expect("run crystalline candidate");
    (scratch, output, result)
}

fn compile_svg(name: &str, source: &str) -> String {
    let (_scratch, output, result) = compile(name, source, "svg");
    assert!(result.status.success(), "{}", diagnostic(&result));
    fs::read_to_string(output).expect("read candidate SVG")
}

fn svg_lines(svg: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut rest = svg;
    while let Some(start) = rest.find("<line ") {
        rest = &rest[start..];
        let end = rest.find("/>").expect("complete SVG line element") + 2;
        lines.push(&rest[..end]);
        rest = &rest[end..];
    }
    lines.sort_unstable();
    lines
}

#[test]
fn p1291_callback_runtime_altera_angulo_final() {
    let callback = compile_svg(
        "callback-zero",
        "#set page(width: 100pt, height: 100pt, margin: 10pt)\n$cancel(x, angle: a => 0deg)$",
    );
    let explicit_zero = compile_svg(
        "explicit-zero",
        "#set page(width: 100pt, height: 100pt, margin: 10pt)\n$cancel(x, angle: 0deg)$",
    );
    let explicit_ninety = compile_svg(
        "explicit-ninety",
        "#set page(width: 100pt, height: 100pt, margin: 10pt)\n$cancel(x, angle: 90deg)$",
    );

    assert_eq!(
        callback, explicit_zero,
        "a callback realizada com retorno 0deg deve produzir a geometria de 0deg",
    );
    assert_ne!(
        callback, explicit_ninety,
        "a callback nao pode ser ignorada nem rebaixada a outro angulo",
    );
}

#[test]
fn p1291_callback_runtime_observa_style_efetivo() {
    let contextual = compile_svg(
        "style-effective",
        "#set page(width: 100pt, height: 100pt, margin: 10pt)\n#set text(size: 19pt)\n$cancel(x, angle: a => context if text.size == 19pt { 0deg } else { 90deg })$",
    );
    let explicit = compile_svg(
        "style-explicit-control",
        "#set page(width: 100pt, height: 100pt, margin: 10pt)\n#set text(size: 19pt)\n$cancel(x, angle: 0deg)$",
    );
    assert_eq!(
        contextual, explicit,
        "a callback deve receber o snapshot com o tamanho efetivo da posicao",
    );
}

#[test]
fn p1291_callback_runtime_erro_preserva_span_da_chamada() {
    let source = "#set page(width: 100pt, height: 100pt, margin: 10pt)\n\n\n$cancel(x, angle: a => 123)$";
    let (_scratch, output, result) = compile("invalid-return-span", source, "pdf");
    assert!(!result.status.success(), "invalid callback return unexpectedly compiled");
    assert!(!output.exists(), "callback error must abort before export");

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("main.typ"), "diagnostic lost source path:\n{stderr}");
    assert!(
        stderr.contains("4:"),
        "diagnostic must be anchored at the cancel call on line 4:\n{stderr}",
    );
    assert!(
        !stderr.contains("panicked at") && !stderr.contains("thread '"),
        "Pending/erro da callback deve virar Err, não panic:\n{stderr}",
    );
}

#[test]
fn p1291_cross_sobrepoe_inverted_na_geometria_final() {
    let cross = compile_svg(
        "cross-not-inverted",
        "#set page(width: 100pt, height: 100pt, margin: 10pt)\n$cancel(x, cross: #true, inverted: #false, angle: a => a)$",
    );
    let cross_inverted = compile_svg(
        "cross-inverted",
        "#set page(width: 100pt, height: 100pt, margin: 10pt)\n$cancel(x, cross: #true, inverted: #true, angle: a => a)$",
    );
    assert_eq!(svg_lines(&cross).len(), 2, "cross deve emitir duas linhas");
    assert_eq!(
        svg_lines(&cross),
        svg_lines(&cross_inverted),
        "cross desenha o mesmo multiconjunto de direções e deve prevalecer sobre inverted",
    );
}

#[test]
fn p1291_callback_aninhada_em_script_observa_size_efetivo_reduzido() {
    let contextual = compile_svg(
        "script-effective-size",
        "#set page(width: 120pt, height: 100pt, margin: 10pt)\n#set text(size: 20pt)\n$ x_(cancel(y, angle: a => context if text.size == 14pt { 0deg } else if text.size == 20pt { 90deg } else { 45deg })) $",
    );
    let explicit_effective = compile_svg(
        "script-explicit-effective",
        "#set page(width: 120pt, height: 100pt, margin: 10pt)\n#set text(size: 20pt)\n$ x_(cancel(y, angle: 0deg)) $",
    );
    let explicit_lexical = compile_svg(
        "script-explicit-lexical",
        "#set page(width: 120pt, height: 100pt, margin: 10pt)\n#set text(size: 20pt)\n$ x_(cancel(y, angle: 90deg)) $",
    );
    let explicit_default = compile_svg(
        "script-explicit-default",
        "#set page(width: 120pt, height: 100pt, margin: 10pt)\n#set text(size: 20pt)\n$ x_(cancel(y, angle: 45deg)) $",
    );

    assert_eq!(
        contextual, explicit_effective,
        "script reduz 20pt para 14pt (0.7×) e esse TextStyle efetivo deve chegar à callback",
    );
    assert_ne!(contextual, explicit_lexical, "não pode observar o size léxico de 20pt");
    assert_ne!(contextual, explicit_default, "não pode cair no size/default isolado");
}
