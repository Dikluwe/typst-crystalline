//! `report.rs` — agregação de resultados em matriz markdown.
//!
//! Materializado no Passo 150. Output:
//! - `lab/parity/reports/latest.md` (sempre o mais recente).
//! - `lab/parity/reports/history/YYYY-MM-DD-passo-NNN.md`
//!   (cópia imutável por passo).

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct ParityMatrix {
    pub categories: Vec<CategoryRow>,
    pub date:       String,
    pub passo:      String,
    /// Sumário descritivo no topo do markdown (ex: "primeira
    /// matriz; cristalino-only baseline; vanilla integração
    /// é DEBT-53").
    pub summary:    String,
}

#[derive(Debug, Default)]
pub struct CategoryRow {
    pub name: String,
    pub total_files:        usize,
    /// Files que compilam sem panic em cristalino.
    pub compiled_ok:        usize,
    /// Files que passam `text_content` (vs vanilla; cristalino-only baseline = N/A).
    pub text_content_passed: Option<usize>,
    /// Files que passam `structural` (vs vanilla).
    pub structural_passed:   Option<usize>,
    /// Modo geometric — números brutos para calibração.
    pub geometric_max_dx:    Option<f64>,
    pub geometric_max_dy:    Option<f64>,
    pub geometric_mean_dx:   Option<f64>,
    pub geometric_mean_dy:   Option<f64>,
}

impl ParityMatrix {
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# Paridade — Passo {} ({})\n\n", self.passo, self.date));
        if !self.summary.is_empty() {
            out.push_str(&format!("{}\n\n", self.summary));
        }

        out.push_str("## Matriz\n\n");
        out.push_str("| Categoria | Total | Compila (cristalino) | text_content | structural | geometric (experimental) |\n");
        out.push_str("|-----------|------:|---------------------:|-------------:|-----------:|:------------------------:|\n");

        let mut total_files = 0usize;
        let mut total_compiled = 0usize;
        let mut total_text = 0usize;
        let mut total_struct = 0usize;
        let mut text_known = false;
        let mut struct_known = false;

        for row in &self.categories {
            total_files    += row.total_files;
            total_compiled += row.compiled_ok;

            let text_cell = match row.text_content_passed {
                Some(n) => { total_text += n; text_known = true; format!("{}/{}", n, row.total_files) }
                None    => "N/A".to_string(),
            };
            let struct_cell = match row.structural_passed {
                Some(n) => { total_struct += n; struct_known = true; format!("{}/{}", n, row.total_files) }
                None    => "N/A".to_string(),
            };
            let geom_cell = match (row.geometric_max_dx, row.geometric_max_dy) {
                (Some(mx), Some(my)) => {
                    let mean_x = row.geometric_mean_dx.unwrap_or(0.0);
                    let mean_y = row.geometric_mean_dy.unwrap_or(0.0);
                    format!("max=({:.1}, {:.1})pt; mean=({:.1}, {:.1})pt", mx, my, mean_x, mean_y)
                }
                _ => "N/A".to_string(),
            };

            out.push_str(&format!(
                "| {} | {} | {}/{} | {} | {} | {} |\n",
                row.name, row.total_files, row.compiled_ok, row.total_files,
                text_cell, struct_cell, geom_cell,
            ));
        }

        let text_total_cell = if text_known {
            format!("{}/{}", total_text, total_files)
        } else { "N/A".to_string() };
        let struct_total_cell = if struct_known {
            format!("{}/{}", total_struct, total_files)
        } else { "N/A".to_string() };

        out.push_str(&format!(
            "| **Total** | **{}** | **{}/{}** | **{}** | **{}** | — |\n\n",
            total_files, total_compiled, total_files,
            text_total_cell, struct_total_cell,
        ));

        out.push_str("## Notas\n\n");
        out.push_str("- **`geometric` é experimental** (per `typst-paridade-definicoes.md` §P3, classe introduzida no Passo 150). Os números brutos são registados para calibração futura mas **não contam para a % agregada**: cristalino usa `FixedMetrics` (~0.6×size por char, monoespaçado) enquanto vanilla usa `FontBookMetrics` (proporcional via `ttf-parser`). Divergência geométrica é **estrutural**, não defeito (ADR-0054 perfil observacional graded cobre).\n");
        out.push_str("- **Cobertura declarada** (per inventário 148, pós-Passo 149): user-facing 54%, arquitectural 72%.\n");
        out.push_str("- **Esta matriz mede paridade observacional** contra vanilla para o subconjunto declarado como suportado pelo cristalino.\n");
        out.push_str("- **Coluna `Compila (cristalino)`** é baseline inicial enquanto a integração vanilla está pendente. Quando vanilla integration estiver em produção (DEBT-53 candidato), `text_content` e `structural` substituem `N/A` por contagens reais.\n");

        out
    }

    /// Escreve `lab/parity/reports/latest.md` (sobre-escrita).
    pub fn write_latest(&self, base: &Path) -> std::io::Result<PathBuf> {
        let dir = base.join("reports");
        fs::create_dir_all(&dir)?;
        let path = dir.join("latest.md");
        fs::write(&path, self.render_markdown())?;
        Ok(path)
    }

    /// Escreve `lab/parity/reports/history/YYYY-MM-DD-passo-NNN.md`.
    pub fn write_history(&self, base: &Path) -> std::io::Result<PathBuf> {
        let dir = base.join("reports").join("history");
        fs::create_dir_all(&dir)?;
        let path = dir.join(format!("{}-passo-{}.md", self.date, self.passo));
        fs::write(&path, self.render_markdown())?;
        Ok(path)
    }
}
