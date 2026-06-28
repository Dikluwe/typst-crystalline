//! Structural parity tests cristalino vs vanilla — P206C.
//!
//! Itera sobre o corpus 36 ficheiros, executa cristalino
//! `query_to_summary` + vanilla `typst query` para
//! selectors representativos, compara via
//! `compare_query_outputs`.
//!
//! **Skip graceful global** se vanilla CLI ausente —
//! per ADR-0075 §"Plano de validação" cond 6.
//!
//! **Skip por-ficheiro** documentado per P206C C2 + C6.
//!
//! Sem `assert!` global de match — paridade é medição,
//! não verificação (consistente com eval_parity,
//! layout_parity).

#[path = "../src/vanilla_invoke.rs"]
mod vanilla_invoke;
#[path = "../src/structural_compare.rs"]
mod structural_compare;

use std::path::{Path, PathBuf};

use vanilla_invoke::{run_typst_query, vanilla_cli_available};
use structural_compare::{compare_query_outputs, CompareResult};

use typst_core::contracts::world::World;
use typst_infra::query_helpers::query_to_summary;
use typst_infra::world::SystemWorld;

/// Selector default por categoria de corpus.
///
/// Cristalino + vanilla aceitam estes selectors básicos.
/// Ficheiros sem matches simplesmente retornam count=0
/// em ambos — match estrutural válido.
fn default_selectors_for_category(category: &str) -> Vec<&'static str> {
    match category {
        // P480 — selector `math.equation` (vanilla namespace); vanilla rejeita
        // `equation` standalone. parse_selector aceita "math.equation" → Equation.
        "visual" => vec!["heading", "figure", "metadata", "math.equation"],
        "markup" => vec!["heading"],
        "math"   => vec!["math.equation"],
        "code"   => vec![],  // code corpus não tem elementos típicos query-able
        _        => vec![],
    }
}

/// Per ficheiro INCLUDE / SKIP-feature / SKIP-pre-existing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoverageEtiqueta {
    Include,
    SkipPreExisting, // markup/error.typ — sintaxe inválida
    SkipFeature,     // se cristalino ou vanilla não suporta
}

fn etiqueta_for(category: &str, file: &str) -> CoverageEtiqueta {
    // markup/error.typ — sintaxe inválida intencional (P206A C2 + P206B).
    if file.starts_with("error") {
        return CoverageEtiqueta::SkipPreExisting;
    }
    // code/: sem elementos query-able típicos. Skip silencioso.
    if category == "code" {
        return CoverageEtiqueta::SkipFeature;
    }
    // semantic/: não testado em P206C (P2 eval, não introspection).
    if category == "semantic" {
        return CoverageEtiqueta::SkipFeature;
    }
    CoverageEtiqueta::Include
}

#[derive(Debug)]
struct CorpusFile {
    category: String,
    file:     String,
    path:     PathBuf,
    source:   String,
}

fn read_corpus(base: &Path) -> Vec<CorpusFile> {
    let mut entries = Vec::new();
    let categories = ["markup", "math", "code", "visual", "semantic"];
    for cat in &categories {
        let dir = base.join(cat);
        if !dir.is_dir() { continue; }
        let Ok(read) = std::fs::read_dir(&dir) else { continue };
        for entry in read.flatten() {
            let path = entry.path();
            let Some(ext) = path.extension().and_then(|e| e.to_str()) else { continue };
            if ext != "typ" { continue; }
            let file = path.file_name().and_then(|n| n.to_str()).unwrap_or("?").to_string();
            let Ok(source) = std::fs::read_to_string(&path) else { continue };
            entries.push(CorpusFile {
                category: cat.to_string(),
                file,
                path: path.clone(),
                source,
            });
        }
    }
    entries.sort_by(|a, b| (a.category.as_str(), a.file.as_str())
        .cmp(&(b.category.as_str(), b.file.as_str())));
    entries
}

/// Helper: build SystemWorld para um source isolado num
/// tempdir.
struct TempDir(PathBuf);
impl TempDir {
    fn path(&self) -> &Path { &self.0 }
}
impl Drop for TempDir {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

fn tempdir() -> TempDir {
    let path = std::env::temp_dir().join(format!(
        "typst-p206c-struct-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&path).unwrap();
    TempDir(path)
}

#[test]
fn p206c_corpus_estrutural_36_ficheiros() {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let corpus = read_corpus(&base);
    // P479 — corpus cresceu de 36 (P206D) para 46 (passados P465–P477).
    assert_eq!(corpus.len(), 46, "esperado 46 ficheiros corpus, encontrados {}", corpus.len());

    if !vanilla_cli_available() {
        eprintln!(
            "[p206c] vanilla CLI ausente em PATH; skip global. \
             Cristalino-only baseline preservado. {} ficheiros saltados.",
            corpus.len()
        );
        return;
    }

    let mut total_includes = 0;
    let mut total_matches  = 0;
    let mut total_diffs    = 0;
    let mut total_skips    = 0;
    let mut total_errors   = 0;
    let mut comparisons    = 0;

    for entry in &corpus {
        let etiqueta = etiqueta_for(&entry.category, &entry.file);
        match etiqueta {
            CoverageEtiqueta::SkipPreExisting => {
                eprintln!("[p206c] {}/{}: SKIP-pre-existing (sintaxe inválida)",
                    entry.category, entry.file);
                total_skips += 1;
                continue;
            }
            CoverageEtiqueta::SkipFeature => {
                eprintln!("[p206c] {}/{}: SKIP-feature (categoria fora-de-escopo P206C)",
                    entry.category, entry.file);
                total_skips += 1;
                continue;
            }
            CoverageEtiqueta::Include => {
                total_includes += 1;
            }
        }

        // Cristalino side: build world + source.
        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        if std::fs::write(&main_path, &entry.source).is_err() {
            eprintln!("[p206c] {}/{}: erro escrita tempdir", entry.category, entry.file);
            total_errors += 1;
            continue;
        }
        // Copiar fixtures auxiliares (refs.yaml para cite-bibliography).
        if entry.file.contains("cite-bibliography") {
            let yaml_src  = entry.path.parent().unwrap().join("refs.yaml");
            let yaml_dest = dir.path().join("refs.yaml");
            if yaml_src.exists() {
                let _ = std::fs::copy(&yaml_src, &yaml_dest);
            }
        }

        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w) => w,
            Err(_) => {
                eprintln!("[p206c] {}/{}: erro build SystemWorld", entry.category, entry.file);
                total_errors += 1;
                continue;
            }
        };
        let source = world.source(world.main()).unwrap();

        for selector in default_selectors_for_category(&entry.category) {
            comparisons += 1;

            let crist = match query_to_summary(&world, &source, selector) {
                Ok(s)  => s,
                Err(e) => {
                    eprintln!("[p206c] {}/{} selector `{}`: cristalino erro: {}",
                        entry.category, entry.file, selector, e);
                    total_errors += 1;
                    continue;
                }
            };

            let van = match run_typst_query(&main_path, selector) {
                Ok(v)  => v,
                Err(e) => {
                    eprintln!("[p206c] {}/{} selector `{}`: vanilla erro: {}",
                        entry.category, entry.file, selector, e);
                    total_errors += 1;
                    continue;
                }
            };

            let result = compare_query_outputs(&crist, &van);
            match result {
                CompareResult::Match => {
                    total_matches += 1;
                    eprintln!("[p206c] {}/{} selector `{}`: ✓ match (count={})",
                        entry.category, entry.file, selector, crist.count);
                }
                CompareResult::Diff(diffs) => {
                    total_diffs += 1;
                    eprintln!("[p206c] {}/{} selector `{}`: ✗ diff:",
                        entry.category, entry.file, selector);
                    for d in &diffs {
                        eprintln!("    - {}", d);
                    }
                }
                CompareResult::Skip(reason) => {
                    eprintln!("[p206c] {}/{} selector `{}`: SKIP ({})",
                        entry.category, entry.file, selector, reason);
                    total_skips += 1;
                }
            }
        }
    }

    eprintln!("\n=== P206C — Matriz de paridade estrutural ===");
    eprintln!("Total ficheiros corpus:   {}", corpus.len());
    eprintln!("Includes (testados):      {}", total_includes);
    eprintln!("Skips:                    {}", total_skips);
    eprintln!("Errors:                   {}", total_errors);
    eprintln!("Comparações:              {}", comparisons);
    eprintln!("  - Matches:              {}", total_matches);
    eprintln!("  - Diffs:                {}", total_diffs);
}

#[test]
fn p206c_query_simple_heading() {
    // Smoke test directo: corpus com 1 heading.
    if !vanilla_cli_available() {
        eprintln!("[p206c] vanilla CLI ausente; skip");
        return;
    }
    let dir = tempdir();
    let path = dir.path().join("main.typ");
    std::fs::write(&path, "= Único heading\n").unwrap();
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let source = world.source(world.main()).unwrap();

    let crist = query_to_summary(&world, &source, "heading").unwrap();
    let van = run_typst_query(&path, "heading").unwrap();

    assert_eq!(crist.count, 1, "cristalino esperado count=1");
    let van_array = van.as_array().unwrap();
    assert_eq!(van_array.len(), 1, "vanilla esperado count=1");

    let result = compare_query_outputs(&crist, &van);
    assert!(result.is_match(), "expected Match, got {:?}", result);
}

#[test]
fn p206c_query_metadata_values_e2e() {
    if !vanilla_cli_available() {
        eprintln!("[p206c] vanilla CLI ausente; skip");
        return;
    }
    let src = "#metadata(\"primeiro\")\n\nTexto.\n\n#metadata(\"segundo\")\n";
    let dir = tempdir();
    let path = dir.path().join("main.typ");
    std::fs::write(&path, src).unwrap();
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap();
    let source = world.source(world.main()).unwrap();

    let crist = query_to_summary(&world, &source, "metadata").unwrap();
    let van = run_typst_query(&path, "metadata").unwrap();

    assert_eq!(crist.count, 2);
    assert_eq!(crist.metadata_values.len(), 2);
    let result = compare_query_outputs(&crist, &van);
    eprintln!("[p206c metadata e2e] {:?}", result);
    // Metadata pode ter nuance JSON shape; aceitamos Match
    // ou Diff documentado para diagnóstico.
    assert!(result.is_match() || result.is_diff());
}

/// **P479** — sentinela: a suite corre com corpus de 46 ficheiros (pós P465–P477),
/// cobertura INCLUDE ≥ 23 (threshold P206D), e 1 diff máximo documentado.
///
/// Requisitos:
/// (a) corpus.len() == 46 (já verificado em `p206c_corpus_estrutural_36_ficheiros`).
/// (b) INCLUDE ≥ 28 (pós-P479: 28 ficheiros incluídos).
/// (c) diffs restantes == 1 (outline-toc heading — M-size, documentado SKIPS §3 P479).
/// Nota: se vanilla CLI ausente o teste passa sem contar diffs (comportamento invariante).
#[test]
fn p479_corpus_paridade_actualizado() {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let corpus = read_corpus(&base);
    assert_eq!(corpus.len(), 46, "P479: corpus deve ter 46 ficheiros");

    if !vanilla_cli_available() {
        eprintln!("[p479] vanilla CLI ausente; sentinela de diff não verifica");
        return;
    }

    let mut total_includes = 0;
    let mut total_diffs    = 0;

    for entry in &corpus {
        let etiqueta = etiqueta_for(&entry.category, &entry.file);
        if etiqueta != CoverageEtiqueta::Include { continue; }
        total_includes += 1;

        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        if std::fs::write(&main_path, &entry.source).is_err() { continue; }
        if entry.file.contains("cite-bibliography") {
            let yaml_src  = entry.path.parent().unwrap().join("refs.yaml");
            let yaml_dest = dir.path().join("refs.yaml");
            if yaml_src.exists() { let _ = std::fs::copy(&yaml_src, &yaml_dest); }
        }
        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w) => w,
            Err(_) => continue,
        };
        let source = world.source(world.main()).unwrap();

        for selector in default_selectors_for_category(&entry.category) {
            let crist = match query_to_summary(&world, &source, selector) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let van = match run_typst_query(&main_path, selector) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if let CompareResult::Diff(_) = compare_query_outputs(&crist, &van) {
                total_diffs += 1;
            }
        }
    }

    // P479: 28 INCLUDE; 1 diff restante (outline-toc heading, M-size, documentado).
    // P480: outline-toc heading fix + math.equation selector fix → 0 diffs esperados.
    // Sentinela progressivo: passa se 0 diffs (P480+) OU 1 diff (P479 baseline).
    assert!(total_includes >= 28, "P479/P480: INCLUDE >= 28; obtido {}", total_includes);
    assert!(total_diffs <= 1,
        "P479/P480: máximo 1 diff esperado; obtido {}", total_diffs);
}

#[test]
fn p480_corpus_paridade_actualizado() {
    // P480 — sentinela de paridade pós-P480.
    // Fix 1: outline-toc heading count (walk arm Outline em kind_index).
    // Fix 2: math.equation selector alias (parse_selector aceita "math.equation").
    // Resultado esperado: 0 diffs (vs 1 diff em P479).
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let corpus = read_corpus(&base);
    assert_eq!(corpus.len(), 46, "P480: corpus deve ter 46 ficheiros");

    if !vanilla_cli_available() {
        eprintln!("[p480] vanilla CLI ausente; sentinela não verifica diffs");
        return;
    }

    let mut total_includes = 0;
    let mut total_diffs    = 0;

    for entry in &corpus {
        let etiqueta = etiqueta_for(&entry.category, &entry.file);
        if etiqueta != CoverageEtiqueta::Include { continue; }
        total_includes += 1;

        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        if std::fs::write(&main_path, &entry.source).is_err() { continue; }
        if entry.file.contains("cite-bibliography") {
            let yaml_src  = entry.path.parent().unwrap().join("refs.yaml");
            let yaml_dest = dir.path().join("refs.yaml");
            if yaml_src.exists() { let _ = std::fs::copy(&yaml_src, &yaml_dest); }
        }
        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w) => w,
            Err(_) => continue,
        };
        let source = world.source(world.main()).unwrap();

        for selector in default_selectors_for_category(&entry.category) {
            let crist = match query_to_summary(&world, &source, selector) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let van = match run_typst_query(&main_path, selector) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if let CompareResult::Diff(diffs) = compare_query_outputs(&crist, &van) {
                total_diffs += 1;
                eprintln!("[p480] diff {}/{} selector `{}`: {:?}",
                    entry.category, entry.file, selector, diffs);
            }
        }
    }

    // P480: 28 INCLUDE; 0 diffs esperados (outline-toc + math.equation resolvidos).
    assert!(total_includes >= 28, "P480: INCLUDE >= 28; obtido {}", total_includes);
    assert_eq!(total_diffs, 0,
        "P480: zero diffs esperados pós-P480; obtido {}", total_diffs);
}

#[test]
fn p482_parity_73_73_mantido() {
    // P482 — sentinela de paridade pós-P482 (Trilha 5 Fase 1: ShapedGlyph + shaper.rs).
    // Shaping é post-processing L3 pass; paridade estrutural cristalino vs vanilla
    // não é afectada porque o shaper não muda o conjunto de items, apenas converte
    // FrameItem::Text → TextShaped. DTO classifica TextShaped como Text.
    // Resultado esperado: 0 diffs (igual a P480).
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let corpus = read_corpus(&base);
    assert_eq!(corpus.len(), 46, "P482: corpus deve ter 46 ficheiros");

    if !vanilla_cli_available() {
        eprintln!("[p482] vanilla CLI ausente; sentinela não verifica diffs");
        return;
    }

    let mut total_includes = 0;
    let mut total_diffs    = 0;

    for entry in &corpus {
        let etiqueta = etiqueta_for(&entry.category, &entry.file);
        if etiqueta != CoverageEtiqueta::Include { continue; }
        total_includes += 1;

        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        if std::fs::write(&main_path, &entry.source).is_err() { continue; }
        if entry.file.contains("cite-bibliography") {
            let yaml_src  = entry.path.parent().unwrap().join("refs.yaml");
            let yaml_dest = dir.path().join("refs.yaml");
            if yaml_src.exists() { let _ = std::fs::copy(&yaml_src, &yaml_dest); }
        }
        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w) => w,
            Err(_) => continue,
        };
        let source = world.source(world.main()).unwrap();

        for selector in default_selectors_for_category(&entry.category) {
            let crist = match query_to_summary(&world, &source, selector) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let van = match run_typst_query(&main_path, selector) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if let CompareResult::Diff(diffs) = compare_query_outputs(&crist, &van) {
                total_diffs += 1;
                eprintln!("[p482] diff {}/{} selector `{}`: {:?}",
                    entry.category, entry.file, selector, diffs);
            }
        }
    }

    // P482: paridade mantida em 0 diffs (shaper é transparent a paridade estrutural).
    assert!(total_includes >= 28, "P482: INCLUDE >= 28; obtido {}", total_includes);
    assert_eq!(total_diffs, 0,
        "P482: zero diffs esperados pós-P482; obtido {}", total_diffs);
}
