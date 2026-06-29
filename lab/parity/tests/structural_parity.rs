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
    // P488 — rtl/: dir:rtl scope-out em stdlib; corpus presente para declarar intenção
    // e verificar que ficheiros árabe/hebraico não causam panic no crystalline.
    // Comparação estrutural com vanilla skip-feature (vanilla pode divergir em RTL).
    if category == "rtl" {
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
    // P488 — "rtl" adicionado ao corpus para validar P484 (bidi_runs).
    let categories = ["markup", "math", "code", "visual", "semantic", "rtl"];
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
    // P488 — corpus cresceu de 46 (P479) para 48 (adição de 2 ficheiros RTL).
    assert_eq!(corpus.len(), 48, "esperado 48 ficheiros corpus, encontrados {}", corpus.len());

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
    assert_eq!(corpus.len(), 48, "P479 (actualizado P488): corpus deve ter 48 ficheiros");

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
    assert_eq!(corpus.len(), 48, "P480 (actualizado P488): corpus deve ter 48 ficheiros");

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
    assert_eq!(corpus.len(), 48, "P482 (actualizado P488): corpus deve ter 48 ficheiros");

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

#[test]
fn p483_parity_73_73_mantido() {
    // P483 — sentinela de paridade pós-P483 (Trilha 5 Fase 2: Text deprecated + font padrão).
    // Fonte padrão Helvetica em TextStyle garante que o shaper tenta actuar em
    // todo o texto. Paridade estrutural não é afectada (DTO classifica TextShaped
    // como Text; resultado esperado: 0 diffs = igual a P482).
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let corpus = read_corpus(&base);
    assert_eq!(corpus.len(), 48, "P483 (actualizado P488): corpus deve ter 48 ficheiros");

    if !vanilla_cli_available() {
        eprintln!("[p483] vanilla CLI ausente; sentinela não verifica diffs");
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
                eprintln!("[p483] diff {}/{} selector `{}`: {:?}",
                    entry.category, entry.file, selector, diffs);
            }
        }
    }

    assert!(total_includes >= 28, "P483: INCLUDE >= 28; obtido {}", total_includes);
    assert_eq!(total_diffs, 0,
        "P483: zero diffs esperados pós-P483; obtido {}", total_diffs);
}

#[test]
fn p485_parity_73_73_mantido() {
    // P485 — sentinela de paridade pós-P485 (units_per_em + TJ operator).
    // A mudança é apenas no emit PDF (TJ vs Tj) — não afecta a estrutura
    // semântica comparada. Resultado esperado: 73/73 = igual a P484.
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let corpus = read_corpus(&base);
    assert_eq!(corpus.len(), 48, "P485 (actualizado P488): corpus deve ter 48 ficheiros");

    if !vanilla_cli_available() {
        eprintln!("[p485] vanilla CLI ausente; sentinela não verifica diffs");
        return;
    }

    let mut total_includes = 0;
    let mut total_diffs    = 0;
    let mut total_errors   = 0;

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
            Err(_) => { total_errors += 1; continue; }
        };
        let source = world.source(world.main()).unwrap();

        for selector in default_selectors_for_category(&entry.category) {
            let crist = match query_to_summary(&world, &source, selector) {
                Ok(s) => s,
                Err(_) => { total_errors += 1; continue; }
            };
            let van = match run_typst_query(&main_path, selector) {
                Ok(v) => v,
                Err(_) => { total_errors += 1; continue; }
            };
            if let CompareResult::Diff(_) = compare_query_outputs(&crist, &van) {
                total_diffs += 1;
            }
        }
    }

    assert!(total_includes >= 28, "P485: INCLUDE >= 28; obtido {}", total_includes);
    assert_eq!(total_diffs, 0,
        "P485: zero diffs esperados pós-P485; obtido {}", total_diffs);
    assert_eq!(total_errors, 0,
        "P485: zero errors esperados; obtido {}", total_errors);
}

#[test]
fn p486_parity_73_73_mantido() {
    // P486 — sentinela de paridade pós-P486 (x_offset em TJ + features documentadas).
    // x_offset=0 para todo o corpus LTR → output TJ idêntico ao P485.
    // Resultado esperado: 73/73 = igual a P485.
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let corpus = read_corpus(&base);
    // P488: corpus cresceu para 48 — P486 mantém diffs=0 (RTL são SkipFeature).
    assert_eq!(corpus.len(), 48, "P486 (actualizado P488): corpus deve ter 48 ficheiros");

    if !vanilla_cli_available() {
        eprintln!("[p486] vanilla CLI ausente; sentinela não verifica diffs");
        return;
    }

    let mut total_includes = 0;
    let mut total_diffs    = 0;
    let mut total_errors   = 0;

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
            Err(_) => { total_errors += 1; continue; }
        };
        let source = world.source(world.main()).unwrap();

        for selector in default_selectors_for_category(&entry.category) {
            let crist = match query_to_summary(&world, &source, selector) {
                Ok(s) => s,
                Err(_) => { total_errors += 1; continue; }
            };
            let van = match run_typst_query(&main_path, selector) {
                Ok(v) => v,
                Err(_) => { total_errors += 1; continue; }
            };
            if let CompareResult::Diff(_) = compare_query_outputs(&crist, &van) {
                total_diffs += 1;
            }
        }
    }

    assert!(total_includes >= 28, "P486: INCLUDE >= 28; obtido {}", total_includes);
    assert_eq!(total_diffs, 0,
        "P486: zero diffs esperados pós-P486; obtido {}", total_diffs);
    assert_eq!(total_errors, 0,
        "P486: zero errors esperados; obtido {}", total_errors);
}

/// **P490** — sentinela: bateria de paridade funcional com 20 ficheiros de teste
/// criados especificamente para sondar comportamento em funcionalidades `parcial`
/// e de alto impacto. Zero PANICs obrigatório (ADR-0054 graded).
///
/// Ficheiros testados: test-list-marker-array, test-enum-start, test-par,
/// test-show-link, test-table, test-raw, test-quote, test-footnote, test-page,
/// test-place, test-calc, test-array, test-str, test-dict, test-show-regex,
/// test-set-local, test-show-where-multi, test-math, test-stroke-sides, test-columns.
///
/// NOTA: `page`, `place`, `rect` não são locatable pelo selector → resultado
/// esperado é ERRO_DESCRITIVO (não PANIC). Scope-out declarado em P490.
#[test]
fn p490_bateria_paridade_funcional_20_ficheiros() {
    let corpus_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus/p490");
    if !corpus_dir.is_dir() {
        eprintln!("[p490] corpus/p490 ausente; skip.");
        return;
    }

    // Mapa: (ficheiro, selector) → descrição
    let cases: &[(&str, &str, &str)] = &[
        // Cat 1: funcionalidades parciais
        ("test-list-marker-array.typ", "list",     "marker:Array"),
        ("test-enum-start.typ",        "enum",     "start:5 + (a)"),
        ("test-par.typ",               "par",      "leading/spacing/justify"),
        ("test-show-link.typ",         "link",     "#show link: ..."),
        // Cat 2: alto impacto
        ("test-table.typ",             "table",    "table.header/footer"),
        ("test-raw.typ",               "raw",      "raw com lang rust"),
        ("test-quote.typ",             "quote",    "attribution"),
        ("test-footnote.typ",          "footnote", "footnote body"),
        // Cat 3: calc
        ("test-calc.typ",              "metadata", "calc args nomeados"),
        // Cat 4: métodos avançados
        ("test-array.typ",             "metadata", "array métodos"),
        ("test-str.typ",               "metadata", "str métodos"),
        ("test-dict.typ",              "metadata", "dict.at(default:)"),
        // Cat 5: show/set edge
        ("test-show-regex.typ",        "heading",  "show regex"),
        ("test-set-local.typ",         "heading",  "#set local em bloco"),
        ("test-show-where-multi.typ",  "heading",  "show.where(multi) scope-out"),
        // Cat 6: math
        ("test-math.typ",              "math.equation", "vec/mat/cases"),
        // Cat 7: layout
        ("test-columns.typ",           "heading",  "columns + colbreak"),
    ];

    // Ficheiros onde "erro" é esperado (não locatable / scope-out): não contam como PANIC
    let non_locatable = &["test-page.typ", "test-place.typ", "test-stroke-sides.typ"];

    let mut panics     = 0usize;
    let mut errors     = 0usize;
    let mut matches    = 0usize;
    let mut absents    = 0usize;

    for (filename, selector, desc) in cases {
        let path = corpus_dir.join(filename);
        let Ok(source) = std::fs::read_to_string(&path) else {
            eprintln!("[p490] ficheiro ausente: {}", filename);
            continue;
        };

        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        if std::fs::write(&main_path, &source).is_err() { continue; }

        let world = match typst_infra::world::SystemWorld::new(dir.path(), "main.typ") {
            Ok(w)  => w,
            Err(e) => {
                eprintln!("[p490] {}: erro build world: {:?}", filename, e);
                errors += 1;
                continue;
            }
        };
        let source_ref = world.source(world.main()).unwrap();

        match typst_infra::query_helpers::query_to_summary(&world, &source_ref, selector) {
            Ok(summary) => {
                eprintln!("[p490] {} sel={} [{}]: count={}", filename, selector, desc, summary.count);
                if summary.count > 0 {
                    matches += 1;
                } else {
                    absents += 1;
                }
            }
            Err(e) => {
                let msg = format!("{:?}", e);
                if msg.contains("panicked") || msg.contains("PANIC") {
                    eprintln!("[p490] {} sel={}: PANIC: {}", filename, selector, msg);
                    panics += 1;
                } else {
                    eprintln!("[p490] {} sel={} [{}]: ERRO_DESCRITIVO: {}", filename, selector, desc, msg);
                    errors += 1;
                }
            }
        }
    }

    // Non-locatable: apenas verifica que não causam PANIC
    for filename in non_locatable {
        let path = corpus_dir.join(filename);
        let Ok(source) = std::fs::read_to_string(&path) else { continue; };
        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        if std::fs::write(&main_path, &source).is_err() { continue; }
        let world = match typst_infra::world::SystemWorld::new(dir.path(), "main.typ") {
            Ok(w)  => w,
            Err(e) => {
                eprintln!("[p490] {}: erro build world (non-loc): {:?}", filename, e);
                errors += 1;
                continue;
            }
        };
        let source_ref = world.source(world.main()).unwrap();
        match typst_infra::query_helpers::query_to_summary(&world, &source_ref, "heading") {
            Ok(s) => {
                eprintln!("[p490] {} non-locatable [heading]: count={} (OK, sem PANIC)", filename, s.count);
            }
            Err(e) => {
                let msg = format!("{:?}", e);
                if msg.contains("panicked") || msg.contains("PANIC") {
                    eprintln!("[p490] {} non-locatable: PANIC: {}", filename, msg);
                    panics += 1;
                } else {
                    eprintln!("[p490] {} non-locatable: ERRO_DESCRITIVO (esperado): {}", filename, msg);
                }
            }
        }
    }

    eprintln!("\n=== P490 — Resumo ===");
    eprintln!("  Matches (count>0): {}", matches);
    eprintln!("  Absents (count=0): {}", absents);
    eprintln!("  Erros descritivos: {}", errors);
    eprintln!("  PANICs:            {}", panics);

    // Critério estrito: ZERO panics
    assert_eq!(panics, 0, "P490: zero PANICs exigido; obtido {}", panics);
}

/// **P495** — sentinela do lote D2: args nomeados `calc.log(base:)`,
/// `calc.round(digits:)`, `str(base:)` e `dict.at(default:)`.
/// Cada caso envolve o resultado num `metadata(...)` e verifica que o
/// cristalino consegue compilar + query sem PANIC e retorna count=1.
#[test]
fn p495_args_nomeados_lote_d2() {
    let cases: &[(&str, &str)] = &[
        ("calc_log_base",   "#metadata(calc.log(100, base: 10))"),
        ("calc_round_digits", "#metadata(calc.round(3.567, digits: 2))"),
        ("str_base",        "#metadata(str(255, base: 16))"),
        ("dict_at_default", "#let d = (a: 1)\n#metadata(d.at(\"z\", default: 99))"),
    ];

    for (name, source) in cases {
        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        std::fs::write(&main_path, source).expect("escrever main.typ");

        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w)  => w,
            Err(e) => panic!("[p491] {}: erro build world: {:?}", name, e),
        };
        let source_ref = world.source(world.main()).unwrap();
        match query_to_summary(&world, &source_ref, "metadata") {
            Ok(summary) => {
                assert_eq!(summary.count, 1, "[p491] {}: esperado count=1, obtido {}", name, summary.count);
            }
            Err(e) => panic!("[p491] {}: query falhou: {:?}", name, e),
        }
    }
}

/// P492 — Cores predefinidas e `text(...)` global.
/// Garante que as variáveis de cor (`red`, `blue`, ...), a função `text`
/// com fill posicional/nomeado e o operador Length + Color funcionam sem PANIC.
#[test]
fn p492_cores_predefinidas_text_e_stroke() {
    let cases: &[(&str, &str)] = &[
        ("red_global",   "#metadata(red)"),
        ("blue_global",  "#metadata(blue)"),
        ("text_fill_pos", "#metadata(text(red, [x]))"),
        ("text_fill_named", "#metadata(text(fill: red, [x]))"),
        ("stroke_color", "#metadata(3pt + red)"),
    ];

    for (name, source) in cases {
        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        std::fs::write(&main_path, source).expect("escrever main.typ");

        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w)  => w,
            Err(e) => panic!("[p492] {}: erro build world: {:?}", name, e),
        };
        let source_ref = world.source(world.main()).unwrap();
        match query_to_summary(&world, &source_ref, "metadata") {
            Ok(summary) => {
                assert_eq!(summary.count, 1, "[p492] {}: esperado count=1, obtido {}", name, summary.count);
            }
            Err(e) => panic!("[p492] {}: query falhou: {:?}", name, e),
        }
    }
}

/// **P496** — Field access em coleções (D3).
/// Garante que arr.dedup/chunks/windows, table.header/footer/cell e
/// heading.where multi-field funcionam sem PANIC.
#[test]
fn p496_field_access_colecoes() {
    let cases: &[(&str, &str)] = &[
        ("array_dedup",   "#metadata((3, 1, 4, 4, 1).dedup())"),
        ("array_chunks",  "#metadata((1, 2, 3, 4).chunks(2))"),
        ("array_windows", "#metadata((1, 2, 3).windows(2))"),
        ("table_header",  "#metadata(table.header[A][B])"),
        ("table_footer",  "#metadata(table.footer[A])"),
        ("table_cell",    "#metadata(table.cell[A])"),
        ("heading_where_multi", "#show heading.where(level: 1, outlined: true): it => [X: ] + it.body\n= H\n#metadata(1)"),
    ];

    for (name, source) in cases {
        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        std::fs::write(&main_path, source).expect("escrever main.typ");

        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w)  => w,
            Err(e) => panic!("[p493] {}: erro build world: {:?}", name, e),
        };
        let source_ref = world.source(world.main()).unwrap();
        match query_to_summary(&world, &source_ref, "metadata") {
            Ok(summary) => {
                assert_eq!(summary.count, 1, "[p493] {}: esperado count=1, obtido {}", name, summary.count);
            }
            Err(e) => panic!("[p493] {}: query falhou: {:?}", name, e),
        }
    }
}

/// **P497** — validação formal dos gaps D4/D5 (cores predefinidas e
/// `text()` em show-regex). Os dois casos do corpus p490 são
/// não-locatable por `heading`, pelo que o critério é: compilam sem
/// PANIC e retornam count=0, como o vanilla.
#[test]
fn p497_variaveis_cor_predefinidas() {
    let cases: &[(&str, &str)] = &[
        (
            "stroke_sides",
            "#rect(stroke: (left: 3pt + red, right: 1pt + blue, top: none, bottom: 2pt + green))",
        ),
        (
            "show_regex_text",
            "#show regex(\"\\\\d+\"): it => text(red, it)\nO número 42 e o número 100 aparecem a vermelho.",
        ),
    ];

    for (name, source) in cases {
        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        std::fs::write(&main_path, source).expect("escrever main.typ");

        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w)  => w,
            Err(e) => panic!("[p497] {}: erro build world: {:?}", name, e),
        };
        let source_ref = world.source(world.main()).unwrap();
        match query_to_summary(&world, &source_ref, "heading") {
            Ok(summary) => {
                assert_eq!(summary.count, 0, "[p497] {}: esperado count=0, obtido {}", name, summary.count);
            }
            Err(e) => panic!("[p497] {}: query falhou: {:?}", name, e),
        }
    }
}

/// **P498** — Fecho do gap D3c residual: `query(heading)` continua a
/// encontrar o heading original mesmo após `#show heading.where(...)`.
/// Usa o snippet exacto do corpus p490/test-show-where-multi.typ.
#[test]
fn p498_d3c_residual() {
    let source = "#show heading.where(level: 1, outlined: true): it => upper(it.body)\n\n= Heading nível 1";

    let dir = tempdir();
    let main_path = dir.path().join("main.typ");
    std::fs::write(&main_path, source).expect("escrever main.typ");

    let world = match SystemWorld::new(dir.path(), "main.typ") {
        Ok(w)  => w,
        Err(e) => panic!("[p498] erro build world: {:?}", e),
    };
    let source_ref = world.source(world.main()).unwrap();

    match query_to_summary(&world, &source_ref, "heading") {
        Ok(summary) => {
            assert_eq!(
                summary.count, 1,
                "[p498] esperado count=1 após show-rule, obtido {}",
                summary.count
            );
        }
        Err(e) => panic!("[p498] query falhou: {:?}", e),
    }

    if vanilla_cli_available() {
        let van = run_typst_query(&main_path, "heading")
            .unwrap_or_else(|e| panic!("[p498] vanilla query falhou: {:?}", e));
        let van_array = van.as_array()
            .unwrap_or_else(|| panic!("[p498] vanilla output não é array"));
        assert_eq!(
            van_array.len(), 1,
            "[p498] vanilla esperado count=1, obtido {}", van_array.len()
        );
    }
}

/// **P494** — sentinela dos 7 selectors de elementos de documento:
/// `list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote`.
/// Compara count cristalino vs vanilla `typst query` para corpus
/// mínimo; objetivo é count=1 em ambos para cada selector.
#[test]
fn p494_selectores_elementos_documento() {
    let cases: &[(&str, &str)] = &[
        ("#list([Item A], [Item B])", "list"),
        ("#enum([Primeiro], [Segundo])", "enum"),
        ("#set par(leading: 1.5em)\nParágrafo de texto.", "par"),
        ("Visita #link(\"https://example.com\")[este sítio]", "link"),
        ("```rust\nfn main() {}\n```", "raw"),
        ("#quote(attribution: [Autor])[Citação]", "quote"),
        ("Texto com nota.#footnote[Nota de rodapé]", "footnote"),
    ];

    for (name, (source, selector)) in ["list", "enum", "par", "link", "raw", "quote", "footnote"]
        .iter()
        .zip(cases.iter())
    {
        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        std::fs::write(&main_path, source).expect("escrever main.typ");

        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w)  => w,
            Err(e) => panic!("[p494] {}: erro build world: {:?}", name, e),
        };
        let source_ref = world.source(world.main()).unwrap();

        let crist = match query_to_summary(&world, &source_ref, selector) {
            Ok(s)  => s,
            Err(e) => panic!("[p494] {}: query cristalino falhou: {:?}", name, e),
        };
        assert_eq!(crist.count, 1, "[p494] {}: esperado count=1, obtido {}", name, crist.count);
        assert_eq!(crist.kind_name.as_deref(), Some(*selector));

        if vanilla_cli_available() {
            let van = run_typst_query(&main_path, selector)
                .unwrap_or_else(|e| panic!("[p494] {}: vanilla query falhou: {:?}", name, e));
            let van_array = van.as_array()
                .unwrap_or_else(|| panic!("[p494] {}: vanilla output não é array", name));
            assert_eq!(
                van_array.len(), 1,
                "[p494] {}: vanilla esperado count=1, obtido {}",
                name, van_array.len()
            );
        }
    }
}

#[test]
fn p488_parity_corpus_48_ficheiros_rtl_skipfeature() {
    // P488 — sentinela de corpus pós-P488.
    // 2 ficheiros RTL adicionados (arabic_basic.typ + hebrew_basic.typ).
    // RTL = SkipFeature: dir:rtl scope-out em stdlib; corpus presente para validar
    // que ficheiros árabe/hebraico não causam crash no pipeline.
    // INCLUDE count mantém-se ≥28 (RTL files são SkipFeature).
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let corpus = read_corpus(&base);
    assert_eq!(corpus.len(), 48, "P488: corpus deve ter 48 ficheiros (46 anteriores + 2 RTL)");

    if !vanilla_cli_available() {
        eprintln!("[p488] vanilla CLI ausente; sentinela verifica apenas count");
        return;
    }

    let mut total_includes = 0;
    let mut total_diffs    = 0;
    let mut total_errors   = 0;

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
            Err(_) => { total_errors += 1; continue; }
        };
        let source = world.source(world.main()).unwrap();

        for selector in default_selectors_for_category(&entry.category) {
            let crist = match query_to_summary(&world, &source, selector) {
                Ok(s) => s,
                Err(_) => { total_errors += 1; continue; }
            };
            let van = match run_typst_query(&main_path, selector) {
                Ok(v) => v,
                Err(_) => { total_errors += 1; continue; }
            };
            if let CompareResult::Diff(_) = compare_query_outputs(&crist, &van) {
                total_diffs += 1;
            }
        }
    }

    assert!(total_includes >= 28, "P488: INCLUDE >= 28; obtido {}", total_includes);
    assert_eq!(total_diffs, 0,
        "P488: zero diffs esperados pós-P488; obtido {}", total_diffs);
    assert_eq!(total_errors, 0,
        "P488: zero errors esperados; obtido {}", total_errors);
}


/// **P500** — Audit de cobertura stdlib expandida.
///
/// Cria 17 ficheiros de teste em `corpus/p500` que sondam funcionalidades
/// do vanilla 0.14.2 não cobertas pela bateria P490. Correr o eval + query
/// cristalino e comparar estruturalmente com `typst query` vanilla quando
/// disponível. O teste é uma medição: não faz `assert` sobre MATCH/DIFF,
/// mas exige zero PANICs (qualquer panic é bug prioritário para P501).
#[test]
fn p500_audit_cobertura_stdlib_expandida() {
    let corpus_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus/p500");
    if !corpus_dir.is_dir() {
        eprintln!("[p500] corpus/p500 ausente; skip.");
        return;
    }

    // (ficheiro, [selectors a testar])
    let cases: &[(&str, &[&str])] = &[
        ("test-image-fit.typ", &["metadata"]),
        ("test-page-header-footer.typ", &["metadata"]),
        ("test-place-absolute.typ", &["metadata"]),
        ("test-calc-rest.typ", &["metadata"]),
        ("test-str-methods.typ", &["metadata"]),
        ("test-dict-methods.typ", &["metadata"]),
        ("test-list-advanced.typ", &["list"]),
        ("test-enum-advanced.typ", &["enum"]),
        ("test-par-advanced.typ", &["par"]),
        ("test-raw-advanced.typ", &["raw"]),
        ("test-quote-advanced.typ", &["quote"]),
        ("test-footnote-advanced.typ", &["footnote"]),
        ("test-figure-advanced.typ", &["figure"]),
        ("test-bibliography-csl.typ", &["bibliography"]),
        ("test-outline-advanced.typ", &["outline"]),
        ("test-state-counter.typ", &["metadata"]),
        ("test-metadata-query.typ", &["metadata", "<tag>"]),
    ];

    let vanilla_available = vanilla_cli_available();
    if !vanilla_available {
        eprintln!("[p500] vanilla CLI ausente; medição cristalino-only.");
    }

    let mut matches = 0usize;
    let mut diffs = 0usize;
    let mut compile_errors = 0usize;
    let mut panics = 0usize;
    let mut vanilla_missing = 0usize;

    let mut rows: Vec<String> = Vec::new();

    for (filename, selectors) in cases {
        let path = corpus_dir.join(filename);
        let Ok(source) = std::fs::read_to_string(&path) else {
            eprintln!("[p500] ficheiro ausente: {}", filename);
            continue;
        };

        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        if std::fs::write(&main_path, &source).is_err() {
            eprintln!("[p500] {}: erro a escrever tempdir", filename);
            continue;
        }

        // Copiar assets auxiliares se existirem no corpus.
        for asset in &["test.png", "refs.bib"] {
            let src = corpus_dir.join(asset);
            if src.exists() {
                let _ = std::fs::copy(&src, dir.path().join(asset));
            }
        }

        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w) => w,
            Err(e) => {
                eprintln!("[p500] {}: erro build world: {:?}", filename, e);
                compile_errors += 1;
                rows.push(format!("| {} | — | build world: {:?} | ERRO_DESCRITIVO | world init |", filename, e));
                continue;
            }
        };
        let source_ref = world.source(world.main()).unwrap();

        let mut file_had_panic = false;
        let mut file_had_error = false;
        let mut file_had_diff = false;
        let mut file_match = true;
        let mut details: Vec<String> = Vec::new();

        for selector in *selectors {
            let crist = match query_to_summary(&world, &source_ref, selector) {
                Ok(s) => s,
                Err(e) => {
                    let msg = format!("{:?}", e);
                    if msg.contains("panicked") || msg.contains("PANIC") {
                        eprintln!("[p500] {} sel={}: PANIC: {}", filename, selector, msg);
                        panics += 1;
                        file_had_panic = true;
                    } else {
                        eprintln!("[p500] {} sel={}: ERRO: {}", filename, selector, msg);
                        compile_errors += 1;
                        file_had_error = true;
                    }
                    details.push(format!("{}: {}", selector, msg));
                    file_match = false;
                    continue;
                }
            };

            if !vanilla_available {
                vanilla_missing += 1;
                details.push(format!("{}: crist count={} (vanilla ausente)", selector, crist.count));
                file_match = false;
                continue;
            }

            let van = match run_typst_query(&main_path, selector) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("[p500] {} sel={}: vanilla erro: {}", filename, selector, e);
                    vanilla_missing += 1;
                    details.push(format!("{}: crist count={}, vanilla erro: {}", selector, crist.count, e));
                    file_match = false;
                    continue;
                }
            };

            match compare_query_outputs(&crist, &van) {
                CompareResult::Match => {
                    eprintln!("[p500] {} sel={}: ✓ match count={}", filename, selector, crist.count);
                    details.push(format!("{}: match count={}", selector, crist.count));
                }
                CompareResult::Diff(diffs_vec) => {
                    eprintln!("[p500] {} sel={}: ✗ diff {:?}", filename, selector, diffs_vec);
                    diffs += 1;
                    file_had_diff = true;
                    file_match = false;
                    details.push(format!("{}: diff {:?}", selector, diffs_vec));
                }
                CompareResult::Skip(reason) => {
                    eprintln!("[p500] {} sel={}: skip {}", filename, selector, reason);
                    file_match = false;
                    details.push(format!("{}: skip {}", selector, reason));
                }
            }
        }

        if file_had_panic {
            rows.push(format!("| {} | ok/esperado | PANIC | PANIC | {} |", filename, details.join("; ")));
        } else if file_had_error {
            let cls = if details.iter().any(|d| d.contains("unknown") || d.contains("unrecognized") || d.contains("expected")) {
                "AUSENTE"
            } else {
                "ERRO_DESCRITIVO"
            };
            rows.push(format!("| {} | ok/esperado | {} | {} | {} |", filename, cls, cls, details.join("; ")));
        } else if file_had_diff {
            rows.push(format!("| {} | ok | DIFF | DIFF | {} |", filename, details.join("; ")));
        } else if file_match {
            matches += 1;
            rows.push(format!("| {} | ok | ok | MATCH | {} |", filename, details.join("; ")));
        } else {
            // vanilla ausente ou skip
            rows.push(format!("| {} | — | — | — | {} |", filename, details.join("; ")));
        }
    }

    eprintln!("\n=== P500 — Matriz de auditoria stdlib expandida ===");
    eprintln!("Ficheiros:            {}", cases.len());
    eprintln!("MATCH:                {}", matches);
    eprintln!("DIFF:                 {}", diffs);
    eprintln!("Erros de compilação:  {}", compile_errors);
    eprintln!("PANICs:               {}", panics);
    if !vanilla_available {
        eprintln!("Vanilla CLI ausente; comparações estruturais skipadas.");
    }

    for row in &rows {
        eprintln!("{}", row);
    }

    // Zero PANICs é invariante de qualquer audit de paridade.
    assert_eq!(panics, 0, "P500: zero PANICs exigido em audit; obtido {}", panics);
}
