//! P3 — Layout parity (Passo 150).
//!
//! Materializa a primeira matriz agregada de paridade no
//! cristalino. **Vanilla integration é DEBT-53 (candidato)**:
//! nesta iteração, o teste corre **cristalino-only baseline**
//! (compilação + extracção de `FrameDTO`), sem comparação contra
//! vanilla. As colunas `text_content` e `structural` da matriz
//! ficam `N/A` até DEBT-53 ser resolvido.
//!
//! `geometric` é **experimental** por construção (cristalino usa
//! `FixedMetrics` enquanto vanilla usa `FontBookMetrics`;
//! divergência estrutural coberta por ADR-0054 perfil graded).
//!
//! O harness **não tem `assert!` global** — paridade é medição,
//! não verificação. Falhas individuais são informação para a
//! matriz, não causam `cargo test` a falhar.

#[path = "../src/frame_dto.rs"]
mod frame_dto;
#[path = "../src/report.rs"]
mod report;

use std::path::{Path, PathBuf};

use frame_dto::FrameDTO;
use report::{CategoryRow, ParityMatrix};

use typst_core::contracts::world::World;
use typst_core::rules::introspect::introspect;
use typst_core::rules::layout::layout;
use typst_infra::pipeline::eval_to_module_with_sink;
use typst_infra::world::SystemWorld;

/// Helper: TempDir local sem dependência externa.
struct TempDir(PathBuf);

impl TempDir {
    fn path(&self) -> &Path { &self.0 }
}

impl Drop for TempDir {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

fn tempdir() -> TempDir {
    let path = std::env::temp_dir().join(format!(
        "typst-parity-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&path).unwrap();
    TempDir(path)
}

/// Compila um source `.typ` em cristalino e devolve o
/// `PagedDocument`. Devolve `None` se a compilação produzir
/// erros (sem panic).
fn compile_cristalino(src: &str) -> Option<typst_core::entities::layout_types::PagedDocument> {
    let dir = tempdir();
    std::fs::write(dir.path().join("main.typ"), src).ok()?;
    let world = SystemWorld::new(dir.path(), "main.typ").ok()?;
    let source = world.source(world.main()).ok()?;
    let (result, _warnings) = eval_to_module_with_sink(&world, &source);
    let module = result.ok()?;
    let content = module.content()?;
    let state = introspect(content);
    let doc = layout(content, state);
    Some(doc)
}

#[derive(Debug)]
struct CorpusEntry {
    category: String,
    file:     String,
    source:   String,
}

fn read_corpus(base: &Path) -> Vec<CorpusEntry> {
    let mut entries = Vec::new();
    let categories = ["markup", "math", "code", "visual"];
    for cat in &categories {
        let dir = base.join(cat);
        if !dir.is_dir() { continue; }
        let Ok(read) = std::fs::read_dir(&dir) else { continue };
        for entry in read.flatten() {
            let path = entry.path();
            let Some(ext) = path.extension().and_then(|e| e.to_str()) else { continue };
            if ext != "typ" { continue; }
            let file = path.file_name().and_then(|n| n.to_str()).unwrap_or("?").to_string();
            // Skip parse-only error fixtures (intencionalmente
            // sintaxe inválida — não pertencem ao corpus de
            // layout). Convenção: nome começa com "error".
            if file.starts_with("error") { continue; }
            let Ok(source) = std::fs::read_to_string(&path) else { continue };
            entries.push(CorpusEntry {
                category: cat.to_string(),
                file,
                source,
            });
        }
    }
    entries.sort_by(|a, b| (a.category.as_str(), a.file.as_str()).cmp(&(b.category.as_str(), b.file.as_str())));
    entries
}

#[test]
fn corpus_completo_p3() {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let corpus = read_corpus(&base.join("corpus"));

    // Agrupar por categoria.
    let mut by_cat: std::collections::BTreeMap<String, Vec<&CorpusEntry>> = Default::default();
    for entry in &corpus {
        by_cat.entry(entry.category.clone()).or_default().push(entry);
    }

    let mut categories = Vec::new();
    for (name, entries) in &by_cat {
        let mut row = CategoryRow {
            name: name.clone(),
            total_files: entries.len(),
            ..Default::default()
        };
        for entry in entries {
            match compile_cristalino(&entry.source) {
                Some(doc) => {
                    row.compiled_ok += 1;
                    // Construir FrameDTO cristalino — baseline disponível
                    // para futuras comparações vanilla.
                    let _crist_dto = FrameDTO::from_cristalino(&doc);
                    // Placeholder: vanilla DTO ainda não disponível.
                    // text_content / structural / geometric ficam N/A
                    // (None nas colunas da CategoryRow).
                }
                None => {
                    eprintln!("[parity] {}/{}: compilação cristalino falhou", name, entry.file);
                }
            }
        }
        categories.push(row);
    }

    let matrix = ParityMatrix {
        categories,
        date:    "2026-04-25".to_string(),
        passo:   "150".to_string(),
        summary: "**Primeira matriz agregada (Passo 150)**. \
                  Esta iteração entrega **cristalino-only baseline**: \
                  cada ficheiro do corpus é compilado em cristalino e \
                  contado como sucesso/falha. **Comparação contra vanilla** \
                  está pendente em **DEBT-53** (candidato): integração do \
                  pipeline vanilla `lab/typst-original/crates/typst::compile` \
                  exige World adapter (vanilla `World` ≠ cristalino `World`) \
                  e materializar `from_vanilla` em `frame_dto.rs`. \
                  As colunas `text_content`, `structural` e `geometric` \
                  ficam `N/A` até DEBT-53 ser resolvido — a infraestrutura \
                  (DTO + matriz + render) está pronta e validada.".to_string(),
    };

    let latest_path = matrix.write_latest(&base).expect("escrita latest.md");
    let history_path = matrix.write_history(&base).expect("escrita history");

    eprintln!("\n=== Matriz Passo 150 ===");
    eprintln!("{}", matrix.render_markdown());
    eprintln!("Latest:  {}", latest_path.display());
    eprintln!("History: {}", history_path.display());
}
