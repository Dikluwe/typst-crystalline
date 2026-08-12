//! Harness Criterion para benchmark do scanner/lexer (P441).
//!
//! Mede o tempo de tokenização de 5 inputs representativos,
//! reportando ns/byte por input.

use std::fs;
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use typst_core::compiler::lexer::Lexer;
use typst_core::entities::syntax_mode::SyntaxMode;

/// Carrega um ficheiro do corpus relativo ao manifesto desta crate.
fn load_corpus(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("corpus");
    path.push(name);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("falha ao ler {}: {}", path.display(), e))
}

/// Tokeniza `src` completamente no modo dado.
fn lex_all(src: &str, mode: SyntaxMode) {
    let mut lexer = Lexer::new(src, mode);
    while lexer.next().0 != typst_core::entities::syntax_kind::SyntaxKind::End {
        // consumir tokens
    }
}

fn bench_scanner(c: &mut Criterion) {
    let inputs = [
        ("b1_hello", "b1_hello.typ", SyntaxMode::Markup),
        ("b2_text", "b2_text.typ", SyntaxMode::Markup),
        ("b3_math", "b3_math.typ", SyntaxMode::Markup),
        ("b4_code", "b4_code.typ", SyntaxMode::Markup),
        ("b5_utf8", "b5_utf8.typ", SyntaxMode::Markup),
    ];

    for (id, file, mode) in inputs {
        let src = load_corpus(file);
        let len = src.len() as u64;
        let mut group = c.benchmark_group("scanner");
        group.throughput(Throughput::Bytes(len));
        group.bench_function(id, |b| {
            b.iter(|| lex_all(&src, mode));
        });
        group.finish();
    }
}

criterion_group!(benches, bench_scanner);
criterion_main!(benches);
