//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/text/lorem.md
//! @prompt-hash 5eea2f96
//! @layer L1
//! @updated 2026-08-13
//!
//! `lorem` — texto dummy com byte-parity vanilla (crate `lipsum`).
//!
//! Extraído de `stdlib/text.rs` (2026-08-13) conforme ADR-0109 — atomização
//! forma B: a lógica vive no ficheiro da unidade, o hub é só fronteira.

use crate::compiler::eval::EvalContext;
use crate::compiler::stdlib::expect_no_named;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

// ── Passo 391 + P805 — `lorem(n)` ───────────────────────────────────────

/// `lorem(n)` → `Value::Str` com `n` palavras de Lorem Ipsum.
///
/// `n` deve ser um `Int` ≥ 0. Argumentos nomeados são rejeitados.
///
/// **P805 — byte-parity com o vanilla** (substitui o vocabulário cíclico de
/// Passo 391, cujo scope-out "texto exacto não precisa de coincidir" foi
/// revogado por este passo): usa a mesma crate do vanilla — `lipsum`
/// (whitelist `[l1_allowed_external.lipsum]`) — com a lógica de junção
/// portada do `lorem_impl` do vanilla
/// (`lab/typst-original/crates/typst-library/src/text/lorem.rs`, MIT —
/// baseado na crate lipsum, © 2017 Martin Geisler). A cadeia é construída
/// por chamada (L1 proíbe estado global; o vanilla usa `LazyLock`).
pub fn native_lorem(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let n = match args.items.as_slice() {
        [Value::Int(n)] => *n,
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("lorem() espera um inteiro, recebeu {}", other.type_name()),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "lorem() requer 1 argumento inteiro",
            )])
        }
    };

    if n < 0 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "lorem() não aceita números negativos",
        )]);
    }

    Ok(Value::Str(lorem_impl(n as usize).into()))
}

/// Port do `lorem_impl` do vanilla (`typst-library/src/text/lorem.rs`):
/// gera `n` palavras com a Markov chain da crate `lipsum` (ordem 2,
/// `LOREM_IPSUM` + `LIBER_PRIMUS`, iterada de `("Lorem", "ipsum")` com o RNG
/// determinístico interno da crate — ChaCha20Rng seed 97). `--` vira en-dash
/// (U+2013) sem contar como palavra; capitaliza após `.`/`!`/`?`; garante
/// ponto final.
fn lorem_impl(n: usize) -> String {
    use lipsum::{MarkovChain, LIBER_PRIMUS, LOREM_IPSUM};

    if n == 0 {
        return String::new();
    }

    let mut chain = MarkovChain::new();
    chain.learn(LOREM_IPSUM);
    chain.learn(LIBER_PRIMUS);
    let mut iter = chain.iter_from(("Lorem", "ipsum"));

    // Pontuação que termina uma frase.
    const PUNCTUATION: [char; 3] = ['.', '!', '?'];

    let mut sentence = String::new();
    let mut word_count = 0;
    let mut needs_cap = false;

    while word_count < n {
        let Some(word) = iter.next() else { break };

        if word_count > 0 {
            sentence.push(' ');
        }

        // Saltar `--` sem contar como palavra; anexar en-dash ao output.
        if word == "--" {
            sentence.push('\u{2013}');
            continue;
        }

        if needs_cap {
            if let Some(c) = word.chars().next() {
                sentence.extend(c.to_uppercase());
                sentence.push_str(&word[c.len_utf8()..]);
            }
        } else {
            sentence.push_str(word);
        }

        needs_cap = sentence.ends_with(PUNCTUATION);
        word_count += 1;
    }

    // Garantir que a frase termina com um de ".!?".
    if !sentence.ends_with(PUNCTUATION) {
        // Truncar pontuação final pendente para não adicionar '.' após ','.
        let idx = sentence.trim_end_matches(|c: char| c.is_ascii_punctuation()).len();
        sentence.truncate(idx);
        sentence.push('.');
    }

    sentence
}
