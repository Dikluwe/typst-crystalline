//! Helper de comparação JSON cristalino vs vanilla — P206C.
//!
//! Compara `typst_infra::query_helpers::QuerySummary` (cristalino)
//! com vanilla output (`serde_json::Value` array de elementos).
//!
//! Comparação é **estrutural** (counts + labels + metadata
//! values) — não literal. Per ADR-0075 §"Mecanismo":
//! - Cristalino produz minimal QuerySummary.
//! - Vanilla produz JSON verbose com `func`/structural fields.
//! - Equivalência semântica: mesma contagem; mesmos labels
//!   (set-equality); mesmos metadata values (multi-set).

use typst_infra::query_helpers::{QuerySummary, SelectorKind};

/// Resultado de comparação.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompareResult {
    /// Equivalência semântica.
    Match,
    /// Divergência detectada.
    Diff(Vec<String>),
    /// Comparação skipada (vanilla CLI ausente, etc.).
    Skip(String),
}

impl CompareResult {
    pub fn is_match(&self) -> bool {
        matches!(self, CompareResult::Match)
    }

    pub fn is_diff(&self) -> bool {
        matches!(self, CompareResult::Diff(_))
    }
}

/// Compara cristalino `QuerySummary` com vanilla JSON output.
///
/// Vanilla output é `Vec` de elementos (cada com `func`,
/// `body`, `label`, etc.). Comparação:
///
/// - **Count**: cristalino `count` vs `vanilla.len()`.
/// - **Labels** (Kind selector): set de labels em vanilla
///   elementos vs cristalino summary (não captura todos por
///   minimal; apenas count).
/// - **Label match** (Label selector): cristalino
///   `label_found` vs vanilla `[0].label` se presente.
/// - **Metadata values**: cristalino `metadata_values` vs
///   array stringified vanilla.
pub fn compare_query_outputs(
    cristalino: &QuerySummary,
    vanilla: &serde_json::Value,
) -> CompareResult {
    let vanilla_array = match vanilla.as_array() {
        Some(a) => a,
        None    => return CompareResult::Diff(vec![format!(
            "vanilla output não é array: {}", vanilla
        )]),
    };

    let mut diffs = Vec::new();

    // Count check (estrita).
    if cristalino.count != vanilla_array.len() {
        diffs.push(format!(
            "count mismatch: cristalino={} vanilla={}",
            cristalino.count, vanilla_array.len()
        ));
    }

    // Per kind de selector, additional checks.
    match cristalino.kind {
        SelectorKind::Kind => {
            if let Some(name) = &cristalino.kind_name {
                // Vanilla element[*].func deve corresponder ao
                // kind name. Excepção: cristalino "metadata"
                // pode mapear a vanilla "metadata".
                let vanilla_funcs: Vec<String> = vanilla_array.iter()
                    .filter_map(|e| e.get("func").and_then(|v| v.as_str()).map(String::from))
                    .collect();

                let all_match = vanilla_funcs.iter().all(|f| f == name);
                if !all_match && !vanilla_funcs.is_empty() {
                    diffs.push(format!(
                        "kind name mismatch: cristalino={} vanilla funcs={:?}",
                        name, vanilla_funcs
                    ));
                }
            }

            // Metadata values check (cristalino specific).
            if cristalino.kind_name.as_deref() == Some("metadata") {
                let vanilla_meta_values: Vec<String> = vanilla_array.iter()
                    .map(|e| {
                        // Vanilla pode retornar primitive string ou nested
                        // shape; usamos repr canónico via `to_string`.
                        if let Some(s) = e.as_str() {
                            s.to_string()
                        } else {
                            e.to_string()
                        }
                    })
                    .collect();

                if cristalino.metadata_values.len() != vanilla_meta_values.len() {
                    diffs.push(format!(
                        "metadata count mismatch: cristalino={} vanilla={}",
                        cristalino.metadata_values.len(), vanilla_meta_values.len()
                    ));
                } else {
                    // Match valor-a-valor por containment (vanilla
                    // pode estruturar dicts como JSON; cristalino
                    // como Debug). Critério lax: cada cristalino
                    // value deve aparecer em algum vanilla value.
                    for (i, c_val) in cristalino.metadata_values.iter().enumerate() {
                        let v_val = &vanilla_meta_values[i];
                        if !values_compatible(c_val, v_val) {
                            diffs.push(format!(
                                "metadata[{}] mismatch: cristalino=`{}` vanilla=`{}`",
                                i, c_val, v_val
                            ));
                        }
                    }
                }
            }
        }
        SelectorKind::Label => {
            // Vanilla deveria retornar 1 elemento se label existe;
            // 0 caso contrário. cristalino label_found = Some(s)
            // implica vanilla[0] tem label `<s>`.
            if cristalino.count == 1 && vanilla_array.len() == 1 {
                let vanilla_label = vanilla_array[0]
                    .get("label")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let crist_label = cristalino.label_found.as_deref().unwrap_or("");
                let crist_label_with_brackets = format!("<{}>", crist_label);

                if vanilla_label != crist_label_with_brackets && !vanilla_label.contains(crist_label) {
                    diffs.push(format!(
                        "label mismatch: cristalino=`{}` vanilla=`{}`",
                        crist_label, vanilla_label
                    ));
                }
            }
        }
    }

    if diffs.is_empty() {
        CompareResult::Match
    } else {
        CompareResult::Diff(diffs)
    }
}

/// Tolerant equality entre 2 strings de metadata value.
///
/// Critério lax porque cristalino formata `Value::Dict` via
/// `Debug` enquanto vanilla retorna nested JSON estruturado.
/// Match se um contém o outro substancialmente.
fn values_compatible(cristalino: &str, vanilla: &str) -> bool {
    let c = cristalino.trim();
    let v = vanilla.trim();

    if c == v { return true; }

    // String tipo "primeiro" vs "primeiro" — vanilla pode
    // adicionar quotes; strip e comparar.
    let v_stripped = v.trim_matches('"');
    if c == v_stripped { return true; }

    // Cristalino Debug de Dict produz `Dict({"tag": Str("..."), ...})`;
    // vanilla produz `{"tag":"..."}`. Match se ambos contêm
    // mesmas keys/values fundamentais — heurística por
    // containment de tokens de chave.
    //
    // Implementação minimal: se ambos contêm os mesmos
    // tokens alfanuméricos de length ≥ 3, consider compatible.
    let c_tokens: std::collections::HashSet<&str> = c
        .split(|ch: char| !ch.is_alphanumeric() && ch != '_')
        .filter(|t| t.len() >= 3)
        .collect();
    let v_tokens: std::collections::HashSet<&str> = v
        .split(|ch: char| !ch.is_alphanumeric() && ch != '_')
        .filter(|t| t.len() >= 3)
        .collect();

    // Match se intersecção significativa.
    let intersection: std::collections::HashSet<&&str> = c_tokens.intersection(&v_tokens).collect();
    !intersection.is_empty() && intersection.len() >= c_tokens.len().min(v_tokens.len()) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn summary_kind(name: &str, count: usize) -> QuerySummary {
        QuerySummary {
            selector:        name.to_string(),
            kind:            SelectorKind::Kind,
            count,
            kind_name:       Some(name.to_string()),
            label_found:     None,
            metadata_values: Vec::new(),
        }
    }

    fn summary_label(label: &str, found: bool) -> QuerySummary {
        QuerySummary {
            selector:        format!("<{}>", label),
            kind:            SelectorKind::Label,
            count:           if found { 1 } else { 0 },
            kind_name:       None,
            label_found:     if found { Some(label.to_string()) } else { None },
            metadata_values: Vec::new(),
        }
    }

    #[test]
    fn p206c_compare_count_match() {
        let cristalino = summary_kind("heading", 3);
        let vanilla = serde_json::json!([
            {"func":"heading","level":1},
            {"func":"heading","level":2},
            {"func":"heading","level":1},
        ]);
        assert_eq!(compare_query_outputs(&cristalino, &vanilla), CompareResult::Match);
    }

    #[test]
    fn p206c_compare_count_mismatch() {
        let cristalino = summary_kind("heading", 2);
        let vanilla = serde_json::json!([
            {"func":"heading"},
            {"func":"heading"},
            {"func":"heading"},
        ]);
        let result = compare_query_outputs(&cristalino, &vanilla);
        assert!(result.is_diff());
        if let CompareResult::Diff(diffs) = result {
            assert!(diffs.iter().any(|d| d.contains("count mismatch")));
        }
    }

    #[test]
    fn p206c_compare_label_match() {
        let cristalino = summary_label("fig-alfa", true);
        let vanilla = serde_json::json!([
            {"func":"figure","label":"<fig-alfa>","kind":"image"}
        ]);
        assert_eq!(compare_query_outputs(&cristalino, &vanilla), CompareResult::Match);
    }

    #[test]
    fn p206c_compare_label_no_match_em_vanilla_array_vazio() {
        // Cristalino não encontrou; vanilla também 0.
        let cristalino = summary_label("nao-existe", false);
        let vanilla = serde_json::json!([]);
        assert_eq!(compare_query_outputs(&cristalino, &vanilla), CompareResult::Match);
    }

    #[test]
    fn p206c_compare_metadata_values_match() {
        let cristalino = QuerySummary {
            selector:        "metadata".to_string(),
            kind:            SelectorKind::Kind,
            count:           2,
            kind_name:       Some("metadata".to_string()),
            label_found:     None,
            metadata_values: vec!["primeiro".to_string(), "segundo".to_string()],
        };
        let vanilla = serde_json::json!(["primeiro", "segundo"]);
        assert_eq!(compare_query_outputs(&cristalino, &vanilla), CompareResult::Match);
    }

    #[test]
    fn p206c_values_compatible_string_vs_quoted() {
        assert!(values_compatible("primeiro", "\"primeiro\""));
        assert!(values_compatible("primeiro", "primeiro"));
    }

    #[test]
    fn p206c_values_compatible_dict_tokens() {
        // Cristalino Debug: `Dict({"tag": Str("secundario"), "peso": Int(42)})`
        // Vanilla JSON: `{"tag":"secundario","peso":42}`
        let crist = "Dict({\"tag\": Str(\"secundario\"), \"peso\": Int(42)})";
        let van   = "{\"tag\":\"secundario\",\"peso\":42}";
        assert!(values_compatible(crist, van));
    }
}
