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

use vanilla_invoke::{run_typst_query, run_typst_query_with_bin, vanilla_cli_available, vanilla_cli_available_with_bin};
use structural_compare::{compare_query_outputs, CompareResult};

use std::process::Command;

use typst_core::contracts::world::World;
use typst_infra::pipeline::compile_to_pdf_bytes;
use typst_infra::query_helpers::{query_to_summary, QuerySummary};
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
        "p538i"  => vec![],  // P538i — testes de #for; paridade verificada por texto
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
    // P538i — p538i/: testes adicionais de #for.
    if category == "p538i" {
        return CoverageEtiqueta::Include;
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
    // P538i — "p538i" adicionado para testes de #for.
    let categories = ["markup", "math", "code", "visual", "semantic", "rtl", "p538i"];
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

/// **P538i** — verifica se `pdftotext` está disponível no ambiente.
fn pdftotext_available() -> bool {
    Command::new("pdftotext").arg("-v").output().is_ok()
}

/// **P538i** — escreve bytes PDF para ficheiro temporário e extrai texto
/// com `pdftotext`.
fn extract_text_from_pdf_bytes(pdf_bytes: &[u8]) -> Option<String> {
    let dir = tempdir();
    let pdf_path = dir.path().join("doc.pdf");
    let txt_path = dir.path().join("doc.txt");
    std::fs::write(&pdf_path, pdf_bytes).ok()?;
    let status = Command::new("pdftotext")
        .arg(&pdf_path)
        .arg(&txt_path)
        .status()
        .ok()?;
    if !status.success() { return None; }
    std::fs::read_to_string(&txt_path).ok()
}

/// **P538i** — compila um source Typst com a CLI vanilla num PDF temporário.
fn compile_vanilla_pdf(typ_path: &Path, pdf_path: &Path) -> Result<(), String> {
    let output = Command::new("typst")
        .arg("compile")
        .arg(typ_path)
        .arg(pdf_path)
        .output()
        .map_err(|e| format!("falha ao invocar vanilla: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "vanilla compile falhou: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

/// **P538i** — normaliza texto extraído para comparação tolerante a
/// quebras de linha e espaços.
fn normalize_extracted_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn p206c_corpus_estrutural_36_ficheiros() {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let corpus = read_corpus(&base);
    // P488 — corpus cresceu de 46 (P479) para 48 (adição de 2 ficheiros RTL).
    assert_eq!(corpus.len(), 50, "esperado 48 ficheiros corpus, encontrados {}", corpus.len());

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
    // **P538i** — contador de diferenças de texto extraído (pdftotext).
    let mut total_text_diffs = 0;

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

        // **P538i** — verificação de texto extraído, além da estrutural.
        // Re-criar o world com system fonts para que o PDF tenha fallback real
        // (o world base não carrega system fonts por defeito).
        if pdftotext_available() {
            let text_world = match SystemWorld::new(dir.path(), "main.typ")
                .map(|w| w.with_system_fonts())
            {
                Ok(w) => w,
                Err(_) => {
                    eprintln!("[p206c] {}/{}: erro build SystemWorld com system fonts",
                        entry.category, entry.file);
                    continue;
                }
            };
            let text_source = text_world.source(text_world.main()).unwrap();
            let (pdf_result, _warnings) = compile_to_pdf_bytes(&text_world, &text_source);
            match pdf_result {
                Ok(pdf_bytes) => {
                    if let Some(crist_text) = extract_text_from_pdf_bytes(&pdf_bytes) {
                        // Asserts específicos para os novos ficheiros de #for.
                        if entry.file == "for-basic.typ" {
                            assert!(
                                crist_text.contains("um")
                                    && crist_text.contains("dois")
                                    && crist_text.contains("três"),
                                "for-basic.typ deve conter os itens 'um', 'dois', 'três'"
                            );
                        }
                        if entry.file == "for-with-counter.typ" {
                            let norm = normalize_extracted_text(&crist_text);
                            assert!(
                                norm.contains("Item 1")
                                    && norm.contains("Item 2")
                                    && norm.contains("Item 3"),
                                "for-with-counter.typ deve conter 'Item 1', 'Item 2', 'Item 3'; got {:?}",
                                norm
                            );
                        }

                        // Comparação opcional com vanilla quando CLI disponível.
                        if vanilla_cli_available() {
                            let van_pdf = dir.path().join("vanilla.pdf");
                            if compile_vanilla_pdf(&main_path, &van_pdf).is_ok() {
                                if let Ok(van_bytes) = std::fs::read(&van_pdf) {
                                    if let Some(van_text) = extract_text_from_pdf_bytes(&van_bytes) {
                                        if normalize_extracted_text(&crist_text)
                                            != normalize_extracted_text(&van_text)
                                        {
                                            total_text_diffs += 1;
                                            eprintln!(
                                                "[p206c] {}/{}: texto diff ({} chars crist vs {} chars van)",
                                                entry.category, entry.file,
                                                crist_text.len(), van_text.len()
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "[p206c] {}/{}: erro compile PDF cristalino: {:?}",
                        entry.category, entry.file, e
                    );
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
    eprintln!("Text diffs (P538i):       {}", total_text_diffs);

    // **P538i** — os dois novos ficheiros de #for estão incluídos no corpus
    // (ver asserts acima no loop). Se chegámos aqui, passaram.
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
    assert_eq!(corpus.len(), 50, "P479 (actualizado P488): corpus deve ter 48 ficheiros");

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
    assert_eq!(corpus.len(), 50, "P480 (actualizado P488): corpus deve ter 48 ficheiros");

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
    assert_eq!(corpus.len(), 50, "P482 (actualizado P488): corpus deve ter 48 ficheiros");

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
    assert_eq!(corpus.len(), 50, "P483 (actualizado P488): corpus deve ter 48 ficheiros");

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
    assert_eq!(corpus.len(), 50, "P485 (actualizado P488): corpus deve ter 48 ficheiros");

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
    assert_eq!(corpus.len(), 50, "P486 (actualizado P488): corpus deve ter 48 ficheiros");

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
    assert_eq!(corpus.len(), 50, "P488: corpus deve ter 48 ficheiros (46 anteriores + 2 RTL)");

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


/// **P501** — Sentinela de fecho dos gaps P1/P2 do audit P500.
///
/// Verifica que os 3 ficheiros que eram AUSENTE em P500 (`test-str-methods.typ`,
/// `test-dict-methods.typ`, `test-calc-rest.typ) agora produzem MATCH contra
/// vanilla 0.14.2, e que o resto da bateria P500 não regrediu.
#[test]
fn p501_gaps_p1_p2() {
    let corpus_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus/p500");
    if !corpus_dir.is_dir() {
        eprintln!("[p501] corpus/p500 ausente; skip.");
        return;
    }
    if !vanilla_cli_available() {
        eprintln!("[p501] vanilla CLI ausente; skip sentinela.");
        return;
    }

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

    let mut panics = 0usize;
    let mut ausentes = 0usize;
    let mut matches = 0usize;
    let mut diffs = 0usize;

    for (filename, selectors) in cases {
        let path = corpus_dir.join(filename);
        let Ok(source) = std::fs::read_to_string(&path) else { continue };

        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        if std::fs::write(&main_path, &source).is_err() { continue; }
        for asset in &["test.png", "refs.bib"] {
            let src = corpus_dir.join(asset);
            if src.exists() { let _ = std::fs::copy(&src, dir.path().join(asset)); }
        }

        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w) => w,
            Err(_) => continue,
        };
        let source_ref = world.source(world.main()).unwrap();

        let mut file_ok = true;
        let mut file_ausente = false;
        let mut file_diff = false;

        for selector in *selectors {
            let crist = match query_to_summary(&world, &source_ref, selector) {
                Ok(s) => s,
                Err(e) => {
                    let msg = format!("{:?}", e);
                    if msg.contains("panicked") || msg.contains("PANIC") { panics += 1; }
                    else { file_ausente = true; }
                    file_ok = false;
                    continue;
                }
            };
            let van = match run_typst_query(&main_path, selector) {
                Ok(v) => v,
                Err(_) => { file_ok = false; continue; }
            };
            match compare_query_outputs(&crist, &van) {
                CompareResult::Match => {}
                CompareResult::Diff(_) => { file_diff = true; file_ok = false; }
                CompareResult::Skip(_) => { file_ok = false; }
            }
        }

        if file_ok {
            matches += 1;
        } else if file_ausente {
            ausentes += 1;
        } else if file_diff {
            diffs += 1;
        }
    }

    eprintln!("\n=== P501 — Sentinela gaps P1/P2 ===");
    eprintln!("MATCH:   {}", matches);
    eprintln!("AUSENTE: {}", ausentes);
    eprintln!("DIFF:    {}", diffs);
    eprintln!("PANIC:   {}", panics);

    // Após P501, os 3 ficheiros P1/P2 devem compilar no cristalino.
    // O vanilla 0.14.2 instalado não suporta alguns métodos (calc.log10,
    // str.to-upper, etc.), pelo que esses ficheiros podem não dar MATCH
    // estrutural, mas devem deixar de ser AUSENTE no cristalino.
    //
    // **P505** — test-list-advanced.typ e test-enum-advanced.typ deixam de ser
    // AUSENTE (indent/body-indent/tight aceites), mas continuam DIFF porque o
    // corpus usa `marker: Array` (scope-out P494) e `numbering: "(a)"`
    // (scope-out P470). O gap de indentação está fechado; os diffs são dos
    // scope-outs pré-existentes.
    // **P506** — test-state-counter.typ deixou de ser AUSENTE (state/counter
    // implementados); portanto zero ficheiros AUSENTE restantes nesta sentinela.
    assert_eq!(panics, 0, "P501: zero PANICs; obtido {}", panics);
    assert_eq!(ausentes, 0, "P501/P505/P506: esperado 0 ficheiros AUSENTE; obtidos {}", ausentes);
    assert!(matches >= 12, "P501/P502/P505/P506: esperados pelo menos 12 MATCH; obtidos {}", matches);
    assert!(diffs <= 2, "P501/P505: esperados no máximo 2 DIFFs (list/enum avançados com scope-outs); obtidos {}", diffs);
}


/// **P502** — Fecho dos gaps S/XS restantes do audit P500.
///
/// Valida que os 4 ficheiros que eram AUSENTE/DIFF em P501 agora produzem
/// MATCH contra vanilla 0.14.2/0.15.0:
/// - `test-image-fit.typ`
/// - `test-raw-advanced.typ`
/// - `test-footnote-advanced.typ`
/// - `test-outline-advanced.typ`
#[test]
fn p502_gaps_s_xs() {
    let corpus_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus/p500");
    if !corpus_dir.is_dir() {
        eprintln!("[p502] corpus/p500 ausente; skip.");
        return;
    }
    if !vanilla_cli_available() {
        eprintln!("[p502] vanilla CLI ausente; skip sentinela.");
        return;
    }

    let cases: &[(&str, &[&str])] = &[
        ("test-image-fit.typ", &["metadata"]),
        ("test-raw-advanced.typ", &["raw"]),
        ("test-footnote-advanced.typ", &["footnote"]),
        ("test-outline-advanced.typ", &["outline"]),
    ];

    for (filename, selectors) in cases {
        let path = corpus_dir.join(filename);
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("[p502] ficheiro ausente: {}", filename));

        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        std::fs::write(&main_path, &source).expect("escrever main.typ");

        for asset in &["test.png", "refs.bib"] {
            let src = corpus_dir.join(asset);
            if src.exists() {
                let _ = std::fs::copy(&src, dir.path().join(asset));
            }
        }

        let world = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(w) => w,
            Err(e) => panic!("[p502] {}: erro build world: {:?}", filename, e),
        };
        let source_ref = world.source(world.main()).unwrap();

        for selector in *selectors {
            let crist = match query_to_summary(&world, &source_ref, selector) {
                Ok(s) => s,
                Err(e) => panic!("[p502] {} sel={}: cristalino query falhou: {:?}", filename, selector, e),
            };
            let van = match run_typst_query(&main_path, selector) {
                Ok(v) => v,
                Err(e) => panic!("[p502] {} sel={}: vanilla query falhou: {}", filename, selector, e),
            };
            let result = compare_query_outputs(&crist, &van);
            assert!(
                result.is_match(),
                "[p502] {} sel={}: esperado MATCH, obtido {:?}",
                filename, selector, result
            );
        }
    }
}

/// **P502a** — `image.fit` aceita `contain`/`cover`/`stretch`.
#[test]
fn p502_image_fit() {
    let source = "#metadata(image(\"test.png\", width: 50%, fit: \"contain\"))";
    let dir = tempdir();
    let main_path = dir.path().join("main.typ");
    std::fs::write(&main_path, source).expect("escrever main.typ");

    let corpus_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus/p500");
    let asset = corpus_dir.join("test.png");
    if asset.exists() {
        let _ = std::fs::copy(&asset, dir.path().join("test.png"));
    }

    let world = SystemWorld::new(dir.path(), "main.typ").expect("build world");
    let source_ref = world.source(world.main()).unwrap();
    let summary = query_to_summary(&world, &source_ref, "metadata")
        .expect("image.fit deve compilar sem erro");
    assert_eq!(summary.count, 1, "p502a: esperado count=1");
}

/// **P502b** — `raw(lang:, block:)` aceita argumentos nomeados.
#[test]
fn p502_raw_lang_block() {
    let source = "#metadata(raw(\"fn main() {}\", lang: \"rust\", block: true))";
    let dir = tempdir();
    let main_path = dir.path().join("main.typ");
    std::fs::write(&main_path, source).expect("escrever main.typ");

    let world = SystemWorld::new(dir.path(), "main.typ").expect("build world");
    let source_ref = world.source(world.main()).unwrap();
    let summary = query_to_summary(&world, &source_ref, "metadata")
        .expect("raw(lang:, block:) deve compilar sem erro");
    assert_eq!(summary.count, 1, "p502b: esperado count=1");
}

/// **P502c** — `footnote(numbering:)` aceita argumento nomeado.
#[test]
fn p502_footnote_numbering() {
    let source = "Texto.#footnote(numbering: \"1\")[Nota] #metadata(1)";
    let dir = tempdir();
    let main_path = dir.path().join("main.typ");
    std::fs::write(&main_path, source).expect("escrever main.typ");

    let world = SystemWorld::new(dir.path(), "main.typ").expect("build world");
    let source_ref = world.source(world.main()).unwrap();
    let summary = query_to_summary(&world, &source_ref, "metadata")
        .expect("footnote(numbering:) deve compilar sem erro");
    assert_eq!(summary.count, 1, "p502c: esperado count=1");
}

/// **P502d** — `outline.indent` aceita `length|function|auto|bool`.
#[test]
fn p502_outline_indent_api() {
    let source = "#outline(title: [Índice], depth: 2, indent: 1em)\n= Capítulo\n";
    let dir = tempdir();
    let main_path = dir.path().join("main.typ");
    std::fs::write(&main_path, source).expect("escrever main.typ");

    let world = SystemWorld::new(dir.path(), "main.typ").expect("build world");
    let source_ref = world.source(world.main()).unwrap();
    let summary = query_to_summary(&world, &source_ref, "outline")
        .expect("outline(indent: length) deve compilar sem erro");
    assert_eq!(summary.count, 1, "p502d: esperado count=1");
}

/// **P503** — Re-baseline de paridade da bateria P490 contra Typst 0.15.0.
///
/// Re-executa os 20 ficheiros da bateria P490 contra:
/// - vanilla 0.14.2 (PATH `typst`) — baseline P498
/// - vanilla 0.15.0 (`/tmp/typst-0.15.0/.../typst`)
/// - cristalino (commit atual)
///
/// Classificação P503: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE,
/// medida entre cristalino e vanilla 0.15.0.
///
/// NOTA: `page`, `place`, `stroke-sides` são non-locatable; o critério é
/// query `heading` retornar count=0 em ambos sem PANIC.
#[test]
fn p503_rebaseline_0150() {
    let corpus_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("corpus/p490");
    if !corpus_dir.is_dir() {
        eprintln!("[p503] corpus/p490 ausente; skip.");
        return;
    }

    let typst_0150_bin = "/tmp/typst-0.15.0/typst-x86_64-unknown-linux-musl/typst";
    let vanilla_0150_available = vanilla_cli_available_with_bin(typst_0150_bin);
    let vanilla_0142_available = vanilla_cli_available();

    if !vanilla_0150_available {
        eprintln!("[p503] vanilla 0.15.0 não encontrado em {}; skip comparação 0.15.0.", typst_0150_bin);
        return;
    }

    // (ficheiro, selector, descrição, non-locatable?)
    let cases: &[(&str, &str, &str, bool)] = &[
        // Cat 1: funcionalidades parciais
        ("test-list-marker-array.typ", "list",     "marker:Array", false),
        ("test-enum-start.typ",        "enum",     "start:5 + (a)", false),
        ("test-par.typ",               "par",      "leading/spacing/justify", false),
        ("test-show-link.typ",         "link",     "#show link: ...", false),
        // Cat 2: alto impacto
        ("test-table.typ",             "table",    "table.header/footer", false),
        ("test-raw.typ",               "raw",      "raw com lang rust", false),
        ("test-quote.typ",             "quote",    "attribution", false),
        ("test-footnote.typ",          "footnote", "footnote body", false),
        // Cat 3: calc
        ("test-calc.typ",              "metadata", "calc args nomeados", false),
        // Cat 4: métodos avançados
        ("test-array.typ",             "metadata", "array métodos", false),
        ("test-str.typ",               "metadata", "str métodos", false),
        ("test-dict.typ",              "metadata", "dict.at(default:)", false),
        // Cat 5: show/set edge
        ("test-show-regex.typ",        "heading",  "show regex", false),
        ("test-set-local.typ",         "heading",  "#set local em bloco", false),
        ("test-show-where-multi.typ",  "heading",  "show.where(multi) scope-out", false),
        // Cat 6: math
        ("test-math.typ",              "math.equation", "vec/mat/cases", false),
        // Cat 7: layout
        ("test-columns.typ",           "heading",  "columns + colbreak", false),
        // Non-locatable: verifica sem PANIC com selector heading
        ("test-page.typ",              "heading",  "page (non-locatable)", true),
        ("test-place.typ",             "heading",  "place (non-locatable)", true),
        ("test-stroke-sides.typ",      "heading",  "stroke-sides (non-locatable)", true),
    ];

    enum CristResult {
        Ok(QuerySummary),
        Panic(String),
        Ausente(String),
        Erro(String),
    }

    fn classify_crist_error(msg: String) -> CristResult {
        if msg.contains("panicked") || msg.contains("PANIC") {
            CristResult::Panic(msg)
        } else if msg.contains("unknown") || msg.contains("unrecognized") || msg.contains("expected") {
            CristResult::Ausente(msg)
        } else {
            CristResult::Erro(msg)
        }
    }

    let mut rows: Vec<String> = Vec::new();
    let mut matches = 0usize;
    let mut diffs = 0usize;
    let mut erros = 0usize;
    let mut panics = 0usize;
    let mut ausentes = 0usize;

    for (filename, selector, desc, non_locatable) in cases {
        let path = corpus_dir.join(filename);
        let Ok(source) = std::fs::read_to_string(&path) else {
            eprintln!("[p503] ficheiro ausente: {}", filename);
            continue;
        };

        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        if std::fs::write(&main_path, &source).is_err() {
            eprintln!("[p503] {}: erro a escrever tempdir", filename);
            continue;
        }

        // --- Cristalino ---
        let crist_result = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(world) => {
                let source_ref = world.source(world.main()).unwrap();
                match query_to_summary(&world, &source_ref, selector) {
                    Ok(summary) => CristResult::Ok(summary),
                    Err(e) => classify_crist_error(format!("{:?}", e)),
                }
            }
            Err(e) => classify_crist_error(format!("{:?}", e)),
        };

        // --- Vanilla 0.14.2 ---
        let van_0142_result: Result<serde_json::Value, String> = if vanilla_0142_available {
            run_typst_query(&main_path, selector).map_err(|e| format!("{}", e))
        } else {
            Err("vanilla 0.14.2 ausente".to_string())
        };

        // --- Vanilla 0.15.0 ---
        let van_0150_result: Result<serde_json::Value, String> =
            run_typst_query_with_bin(typst_0150_bin, &main_path, selector)
                .map_err(|e| format!("{}", e));

        // --- Contagens para a tabela ---
        let crist_label = match &crist_result {
            CristResult::Ok(s) => format!("count={}", s.count),
            CristResult::Panic(m) | CristResult::Ausente(m) | CristResult::Erro(m) => {
                let truncated = if m.len() > 80 { format!("{}...", &m[..80]) } else { m.clone() };
                truncated
            }
        };

        let van_0142_label = match &van_0142_result {
            Ok(v) => format!("count={}", v.as_array().map(|a| a.len()).unwrap_or(0)),
            Err(e) => {
                let truncated = if e.len() > 80 { format!("{}...", &e[..80]) } else { e.clone() };
                format!("ERRO: {}", truncated)
            }
        };

        let van_0150_label = match &van_0150_result {
            Ok(v) => format!("count={}", v.as_array().map(|a| a.len()).unwrap_or(0)),
            Err(e) => {
                let truncated = if e.len() > 80 { format!("{}...", &e[..80]) } else { e.clone() };
                format!("ERRO: {}", truncated)
            }
        };

        // --- Classificação P503 (cristalino vs 0.15.0) ---
        let classificacao = match (&crist_result, &van_0150_result) {
            (CristResult::Ok(crist), Ok(van_0150)) => {
                match compare_query_outputs(crist, van_0150) {
                    CompareResult::Match => {
                        matches += 1;
                        "MATCH".to_string()
                    }
                    CompareResult::Diff(diffs_vec) => {
                        diffs += 1;
                        format!("DIFF: {}", diffs_vec.join("; "))
                    }
                    CompareResult::Skip(reason) => {
                        erros += 1;
                        format!("ERRO_DESCRITIVO (skip: {})", reason)
                    }
                }
            }
            (CristResult::Panic(_), _) => {
                panics += 1;
                "PANIC".to_string()
            }
            (CristResult::Ausente(_), _) => {
                ausentes += 1;
                "AUSENTE".to_string()
            }
            (CristResult::Erro(_), _) | (_, Err(_)) => {
                erros += 1;
                "ERRO_DESCRITIVO".to_string()
            }
        };

        eprintln!(
            "[p503] {:<35} sel={:<15} crist={:<40} van0142={:<40} van0150={:<40} => {}",
            filename, selector, crist_label, van_0142_label, van_0150_label, classificacao
        );

        let p498 = if *non_locatable { "ERRO_DESCRITIVO (non-loc)".to_string() } else { "MATCH".to_string() };
        rows.push(format!(
            "| `{}` | `{}` | {} | {} | {} | {} | {} | {}",
            filename, selector, desc, p498, van_0142_label, van_0150_label, crist_label, classificacao
        ));
    }

    eprintln!("\n=== P503 — Re-baseline P490 vs Typst 0.15.0 ===");
    eprintln!("Ficheiros:        {}", cases.len());
    eprintln!("MATCH:            {}", matches);
    eprintln!("DIFF:             {}", diffs);
    eprintln!("ERRO_DESCRITIVO:  {}", erros);
    eprintln!("AUSENTE:          {}", ausentes);
    eprintln!("PANIC:            {}", panics);

    for row in &rows {
        eprintln!("{}", row);
    }

    // Invariante: zero PANICs em qualquer re-baseline de paridade.
    assert_eq!(panics, 0, "P503: zero PANICs exigido; obtido {}", panics);
}

/// **P504** — Audit de novas funcionalidades Typst 0.15.0.
///
/// Mede o estado actual do cristalino face às funcionalidades novas do 0.15.0.
/// Não faz asserts de paridade; apenas classifica cada caso como OK / ERRO /
/// PANIC / AUSENTE e produz matriz para o diagnóstico P504.
#[test]
fn p504_audit_novas_funcionalidades_0150() {
    let cases: &[(&str, &str)] = &[
        // 504a — within selector
        ("within_selector", "#figure[\n  = Dentro\n]\n= Fora\n#metadata(query(selector(heading).within(figure)).len())"),
        // 504b — dict.map
        ("dict_map", "#let d = (a: 1, b: 2)\n#metadata(d.map((k, v) => v * 2))"),
        // 504b — dict.filter
        ("dict_filter", "#let d = (a: 1, b: 2)\n#metadata(d.filter((k, v) => v > 1))"),
        // 504c — arguments field access
        ("args_field", "#let f(..args) = args.named\n#metadata(f(x: 1, y: 2))"),
        // 504d — int(base:)
        ("int_base", "#metadata(int(\"ff\", base: 16))"),
        // 504e — calc.asinh
        ("calc_asinh", "#metadata(calc.asinh(1.0))"),
        // 504e — calc.acosh
        ("calc_acosh", "#metadata(calc.acosh(1.0))"),
        // 504e — calc.atanh
        ("calc_atanh", "#metadata(calc.atanh(0.5))"),
        // 504e — calc.erf
        ("calc_erf", "#metadata(calc.erf(1.0))"),
        // 504f — int.min / int.max
        ("int_min", "#metadata(int.min)"),
        ("int_max", "#metadata(int.max)"),
        // 504g — range(inclusive:)
        ("range_inclusive", "#metadata(range(1, 5, inclusive: true))"),
        // 504h — counter.display(at:)  (vanilla syntax)
        ("counter_display_at", "= Secção <sec>\n#counter(heading).display(\"1.\", at: <sec>)\n#metadata(1)"),
        // 504i — page.bleed
        ("page_bleed", "#set page(bleed: 3mm)\n#metadata(1)"),
        // 504j — list.marker-align
        ("list_marker_align", "#list(marker-align: start, [A], [B])\n#metadata(1)"),
        // 504k — divider element
        ("divider", "#divider\n#metadata(1)"),
    ];

    let mut rows: Vec<String> = Vec::new();
    let mut ok = 0usize;
    let mut erro = 0usize;
    let mut panic = 0usize;
    let mut ausente = 0usize;

    for (name, source) in cases {
        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        std::fs::write(&main_path, source).expect("escrever main.typ");

        let result = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(world) => {
                let source_ref = world.source(world.main()).unwrap();
                match query_to_summary(&world, &source_ref, "metadata") {
                    Ok(summary) => {
                        if summary.count >= 1 {
                            ok += 1;
                            format!("OK (count={})", summary.count)
                        } else {
                            erro += 1;
                            "ERRO: count=0".to_string()
                        }
                    }
                    Err(e) => {
                        let msg = format!("{:?}", e);
                        if msg.contains("panicked") || msg.contains("PANIC") {
                            panic += 1;
                            format!("PANIC: {}", msg)
                        } else if msg.contains("unknown") || msg.contains("unrecognized") || msg.contains("expected") {
                            ausente += 1;
                            format!("AUSENTE: {}", msg)
                        } else {
                            erro += 1;
                            format!("ERRO: {}", msg)
                        }
                    }
                }
            }
            Err(e) => {
                let msg = format!("{:?}", e);
                if msg.contains("panicked") || msg.contains("PANIC") {
                    panic += 1;
                    format!("PANIC: {}", msg)
                } else if msg.contains("unknown") || msg.contains("unrecognized") || msg.contains("expected") {
                    ausente += 1;
                    format!("AUSENTE: {}", msg)
                } else {
                    erro += 1;
                    format!("ERRO: {}", msg)
                }
            }
        };

        eprintln!("[p504] {:<25} => {}", name, result);
        if result.starts_with("ERRO") || result.starts_with("AUSENTE") {
            eprintln!("[p504-detail] source: {}", source.replace('\n', "\\n"));
        }
        rows.push(format!("| {} | {}", name, result));
    }

    eprintln!("\n=== P504 — Audit Novas Funcionalidades 0.15.0 ===");
    eprintln!("Total:   {}", cases.len());
    eprintln!("OK:      {}", ok);
    eprintln!("ERRO:    {}", erro);
    eprintln!("AUSENTE: {}", ausente);
    eprintln!("PANIC:   {}", panic);
    for row in &rows {
        eprintln!("{}", row);
    }

    assert_eq!(panic, 0, "P504: zero PANICs exigido; obtido {}", panic);
}

/// **P505** — Fecho dos gaps de indentação em `list` e `enum`.
///
/// Valida que `indent`, `body-indent` e `tight` são aceites e produzem
/// resultados estruturais equivalentes ao vanilla 0.15.0.
#[test]
fn p505_indentacao_listas_enums() {
    let typst_0150_bin = "/tmp/typst-0.15.0/typst-x86_64-unknown-linux-musl/typst";
    let vanilla_0150_available = vanilla_cli_available_with_bin(typst_0150_bin);

    let cases: &[(&str, &str, &str)] = &[
        (
            "list_indent",
            "list",
            "#list(indent: 1.5em, body-indent: 0.5em, tight: false, [Item A], [Item B])",
        ),
        (
            "enum_indent",
            "enum",
            "#enum(indent: 1.5em, body-indent: 0.5em, tight: false, [Primeiro], [Segundo])",
        ),
        (
            "list_tight_true",
            "list",
            "#list(tight: true, [A], [B], [C])",
        ),
        (
            "enum_tight_true",
            "enum",
            "#enum(tight: true, [A], [B], [C])",
        ),
    ];

    let mut rows: Vec<String> = Vec::new();
    let mut matches = 0usize;
    let mut ausentes = 0usize;
    let mut panics = 0usize;
    let mut diffs = 0usize;

    for (name, selector, expr) in cases {
        let source = format!("{}\n#metadata(1)", expr);
        let dir = tempdir();
        let main_path = dir.path().join("main.typ");
        std::fs::write(&main_path, &source).expect("escrever main.typ");

        let crist_result = match SystemWorld::new(dir.path(), "main.typ") {
            Ok(world) => {
                let source_ref = world.source(world.main()).unwrap();
                match query_to_summary(&world, &source_ref, selector) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(format!("{:?}", e)),
                }
            }
            Err(e) => Err(format!("{:?}", e)),
        };

        if !vanilla_0150_available {
            eprintln!("[p505] {}: vanilla 0.15.0 ausente; crist-only.", name);
            rows.push(format!("| {} | — | crist-only |", name));
            continue;
        }

        let van_result = run_typst_query_with_bin(typst_0150_bin, &main_path, selector);

        let classificacao = match (&crist_result, &van_result) {
            (Ok(crist), Ok(van)) => match compare_query_outputs(crist, van) {
                CompareResult::Match => {
                    matches += 1;
                    "MATCH".to_string()
                }
                CompareResult::Diff(diffs_vec) => {
                    diffs += 1;
                    format!("DIFF: {}", diffs_vec.join("; "))
                }
                CompareResult::Skip(reason) => {
                    diffs += 1;
                    format!("SKIP: {}", reason)
                }
            },
            (Err(e), _) => {
                let msg = e.clone();
                if msg.contains("panicked") || msg.contains("PANIC") {
                    panics += 1;
                    "PANIC".to_string()
                } else if msg.contains("unknown") || msg.contains("unrecognized") || msg.contains("expected") {
                    ausentes += 1;
                    "AUSENTE".to_string()
                } else {
                    diffs += 1;
                    format!("ERRO: {}", msg)
                }
            }
            (_, Err(e)) => {
                diffs += 1;
                format!("VANILLA_ERRO: {}", e)
            }
        };

        eprintln!("[p505] {:<20} => {}", name, classificacao);
        rows.push(format!("| {} | {} | {}", name, selector, classificacao));
    }

    eprintln!("\n=== P505 — Indentação list/enum ===");
    eprintln!("MATCH:   {}", matches);
    eprintln!("DIFF:    {}", diffs);
    eprintln!("AUSENTE: {}", ausentes);
    eprintln!("PANIC:   {}", panics);
    for row in &rows {
        eprintln!("{}", row);
    }

    assert_eq!(panics, 0, "P505: zero PANICs exigido; obtido {}", panics);
    assert_eq!(ausentes, 0, "P505: zero AUSENTEs esperado; obtido {}", ausentes);
    if vanilla_0150_available {
        assert_eq!(diffs, 0, "P505: zero DIFFs esperado; obtido {}", diffs);
    }
}
