//! P2 — Eval parity (Passo 153).
//!
//! Cristalino-only baseline. Cada ficheiro do corpus
//! `lab/parity/corpus/semantic/` exporta `__resultado__` via
//! `#let __resultado__ = <expr>`. O harness evaluate o source
//! em cristalino, extrai o valor pelo scope do Module, converte
//! para `ValueDTO`. Vanilla side é stub (DEBT-54).
//!
//! Sem `assert!` global — paridade é medição, não verificação
//! (consistente com P150).

#[path = "../src/value_dto.rs"]
mod value_dto;

use std::path::{Path, PathBuf};

use value_dto::ValueDTO;

use typst_core::contracts::world::World;
use typst_infra::pipeline::eval_to_module_with_sink;
use typst_infra::world::SystemWorld;

struct TempDir(PathBuf);

impl TempDir {
    fn path(&self) -> &Path { &self.0 }
}

impl Drop for TempDir {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

fn tempdir() -> TempDir {
    let path = std::env::temp_dir().join(format!(
        "typst-parity-eval-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&path).unwrap();
    TempDir(path)
}

/// Evaluate source em cristalino e devolve `ValueDTO` extraído
/// de `__resultado__`. `Err` se eval falhar ou se `__resultado__`
/// não existir.
fn eval_resultado_cristalino(src: &str) -> Result<ValueDTO, String> {
    let dir = tempdir();
    std::fs::write(dir.path().join("main.typ"), src).map_err(|e| e.to_string())?;
    let world = SystemWorld::new(dir.path(), "main.typ").map_err(|e| format!("{e:?}"))?;
    let source = world.source(world.main()).map_err(|e| format!("{e:?}"))?;
    let (result, _warnings) = eval_to_module_with_sink(&world, &source);
    let module = result.map_err(|e| format!("eval: {} diagnostics", e.len()))?;
    let value = module.scope().get("__resultado__")
        .ok_or_else(|| "__resultado__ não definido no scope".to_string())?
        .clone();
    Ok(ValueDTO::from_cristalino(&value))
}

#[derive(Debug)]
struct CorpusEntry {
    file:   String,
    source: String,
}

fn read_corpus_semantic(base: &Path) -> Vec<CorpusEntry> {
    let mut entries = Vec::new();
    let dir = base.join("semantic");
    if !dir.is_dir() { return entries; }
    let Ok(read) = std::fs::read_dir(&dir) else { return entries };
    for entry in read.flatten() {
        let path = entry.path();
        let Some(ext) = path.extension().and_then(|e| e.to_str()) else { continue };
        if ext != "typ" { continue; }
        let file = path.file_name().and_then(|n| n.to_str()).unwrap_or("?").to_string();
        let Ok(source) = std::fs::read_to_string(&path) else { continue };
        entries.push(CorpusEntry { file, source });
    }
    entries.sort_by(|a, b| a.file.cmp(&b.file));
    entries
}

#[test]
fn corpus_semantic_p2() {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let corpus = read_corpus_semantic(&base.join("corpus"));

    let mut eval_ok    = 0usize;
    let mut eval_fail  = 0usize;
    let mut no_result  = 0usize;
    let total = corpus.len();
    let mut details = Vec::new();

    for entry in &corpus {
        match eval_resultado_cristalino(&entry.source) {
            Ok(dto) => {
                eval_ok += 1;
                details.push(format!(
                    "  {} → {} ({})",
                    entry.file, dto.type_name(),
                    match &dto {
                        ValueDTO::Bool(b) => format!("{b}"),
                        ValueDTO::Int(i)  => format!("{i}"),
                        ValueDTO::Float(bits) => format!("0x{:016x}", bits),
                        ValueDTO::Str(s)  => format!("{:?}", s),
                        ValueDTO::Array(a) => format!("[{}]", a.len()),
                        ValueDTO::Dict(d)  => format!("{{{}}}", d.len()),
                        ValueDTO::Func(name) => format!("fn `{name}`"),
                        ValueDTO::Content(t) => format!("Content({:?})", t.chars().take(20).collect::<String>()),
                        other => format!("{other:?}"),
                    },
                ));
            }
            Err(e) if e.contains("__resultado__ não definido") => {
                no_result += 1;
                eprintln!("[parity P2] {}: sem __resultado__ ({})", entry.file, e);
                details.push(format!("  {} → no_resultado", entry.file));
            }
            Err(e) => {
                eval_fail += 1;
                eprintln!("[parity P2] {}: eval falhou ({})", entry.file, e);
                details.push(format!("  {} → eval_fail", entry.file));
            }
        }
    }

    // Append matriz P2 a `latest.md` mantendo P3 do P150.
    let report_path = base.join("reports").join("latest.md");
    let p3_block = std::fs::read_to_string(&report_path)
        .unwrap_or_else(|_| String::new());

    let mut p2_md = String::new();
    p2_md.push_str("# Paridade — Passo 153 (2026-04-25)\n\n");
    p2_md.push_str("**Matriz multi-nível** (P2 + P3 cristalino-only baseline).\n");
    p2_md.push_str("Vanilla integration depende de **DEBT-54** → fecho **DEBT-53**.\n\n");
    p2_md.push_str("## Matriz P2 (Eval, Passo 153)\n\n");
    p2_md.push_str("| Categoria | Total | Eval ok (crist) | Eval falhou | sem `__resultado__` | value_equality (vs vanilla) |\n");
    p2_md.push_str("|-----------|------:|----------------:|------------:|--------------------:|:---------------------------:|\n");
    p2_md.push_str(&format!(
        "| semantic  |   {:3} |        {:3}/{:3} |       {:3} |                {:3} |                         N/A |\n",
        total, eval_ok, total, eval_fail, no_result,
    ));
    p2_md.push_str(&format!(
        "| **Total** |   **{:3}** |    **{:3}/{:3}** |     **{:3}** |              **{:3}** |                         N/A |\n\n",
        total, eval_ok, total, eval_fail, no_result,
    ));
    p2_md.push_str("### Detalhes (cristalino)\n\n");
    p2_md.push_str("```\n");
    for d in &details {
        p2_md.push_str(d);
        p2_md.push('\n');
    }
    p2_md.push_str("```\n\n");

    // Re-incluir bloco P3 existente (P150) — preserva matriz anterior.
    if !p3_block.is_empty() {
        p2_md.push_str("---\n\n## Matriz P3 (Layout, Passo 150 — preservada)\n\n");
        // Strip duplicate H1 of P150 if present.
        let p3_filtered = p3_block.lines()
            .skip_while(|l| l.starts_with("# Paridade — Passo 150"))
            .collect::<Vec<_>>().join("\n");
        p2_md.push_str(p3_filtered.trim_start());
        p2_md.push('\n');
    }
    p2_md.push_str("\n## Notas P2 (Passo 153)\n\n");
    p2_md.push_str("- **`value_equality`** vs vanilla é `N/A` enquanto **DEBT-54** (setup vanilla workspace) + **DEBT-53** (vanilla World adapter + `from_vanilla` real) não fecharem em sequência.\n");
    p2_md.push_str("- **Diagnósticos canários**: `tipo-inspeccao.typ` (ADR-0058) e `args-rest.typ` (ADR-0059) revelarão divergências quando vanilla integrar.\n");
    p2_md.push_str("- **Eval ok cristalino** mede que o pipeline `eval` cristalino consegue produzir `Value` sem panic; **não** valida correctness vs vanilla — isso virá com DEBT-53.\n");
    p2_md.push_str("- **`Float` é comparado por bits** (`f64::to_bits()`); NaN com payload diferente compara desigual.\n");
    p2_md.push_str("- **`Func` é comparado por nome**; closures sem nome → `\"<closure>\"`.\n");
    p2_md.push_str("- **`Args` não é variant** em `ValueDTO` (ADR-0059); vanilla `Value::Args` mapearia para `Other(\"args\")`.\n");

    let _ = std::fs::write(&report_path, &p2_md);

    // Cópia histórica imutável.
    let history_dir = base.join("reports").join("history");
    let _ = std::fs::create_dir_all(&history_dir);
    let history_path = history_dir.join("2026-04-25-passo-153.md");
    let _ = std::fs::write(&history_path, &p2_md);

    eprintln!("\n=== P2 corrida (Passo 153) ===");
    eprintln!("eval_ok: {}/{}; eval_fail: {}; no_resultado: {}",
        eval_ok, total, eval_fail, no_result);
    eprintln!("Latest:  {}", report_path.display());
    eprintln!("History: {}", history_path.display());
}
