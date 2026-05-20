//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/function_dict.md
//! @prompt-hash 483f3927
//! @layer L3
//! @updated 2026-05-19
//!
//! Function dict — emit_function_dict (RGB) + emit_function_dict_cmyk (CMYK).
//!
//! Extraído de `gradients.rs` em P307b.2 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.



pub(crate) fn emit_function_dict(stops: &[(f32, f32, f32)], function_id: usize, sub_first_id: &mut usize)
    -> (String, Vec<(usize, String)>)
{
    if stops.len() == 2 {
        let (r0, g0, b0) = stops[0];
        let (r1, g1, b1) = stops[1];
        let dict = format!(
            "<< /FunctionType 2 /Domain [0 1] /C0 [{:.4} {:.4} {:.4}] /C1 [{:.4} {:.4} {:.4}] /N 1 >>",
            r0, g0, b0, r1, g1, b1
        );
        let _ = function_id;
        return (dict, Vec::new());
    }
    // Type 3 stitching.
    let n = stops.len();
    let mut sub_objs: Vec<(usize, String)> = Vec::new();
    let mut sub_refs: Vec<String> = Vec::new();
    for i in 0..(n - 1) {
        let (r0, g0, b0) = stops[i];
        let (r1, g1, b1) = stops[i + 1];
        let sub_id = *sub_first_id;
        *sub_first_id += 1;
        let sub_dict = format!(
            "<< /FunctionType 2 /Domain [0 1] /C0 [{:.4} {:.4} {:.4}] /C1 [{:.4} {:.4} {:.4}] /N 1 >>",
            r0, g0, b0, r1, g1, b1
        );
        sub_objs.push((sub_id, sub_dict));
        sub_refs.push(format!("{sub_id} 0 R"));
    }
    let mut bounds = Vec::new();
    for i in 1..(n - 1) {
        let t = i as f64 / (n - 1) as f64;
        bounds.push(format!("{:.4}", t));
    }
    let encode: Vec<String> = (0..(n - 1)).map(|_| "0 1".to_string()).collect();
    let dict = format!(
        "<< /FunctionType 3 /Domain [0 1] /Functions [{}] /Bounds [{}] /Encode [{}] >>",
        sub_refs.join(" "),
        bounds.join(" "),
        encode.join(" "),
    );
    let _ = function_id;
    (dict, sub_objs)
}

/// P270.2 — Emit PDF Function dict CMYK 4-component (Type 2 ou Type 3).
///
/// Análogo `emit_function_dict` (P263; 3-component RGB) mas:
/// - `/Range [0 1 0 1 0 1 0 1]` (8 values; 4 pares c/m/y/k).
/// - `/C0 [c m y k]` `/C1 [c m y k]` 4-component.
///
/// 2 stops → Type 2 (exponential linear `/N 1`).
/// N>2 stops → Type 3 stitching com N-1 sub-funções Type 2.
pub(crate) fn emit_function_dict_cmyk(
    stops: &[(f32, f32, f32, f32)],
    function_id: usize,
    sub_first_id: &mut usize,
) -> (String, Vec<(usize, String)>) {
    if stops.len() == 2 {
        let (c0, m0, y0, k0) = stops[0];
        let (c1, m1, y1, k1) = stops[1];
        let dict = format!(
            "<< /FunctionType 2 /Domain [0 1] /Range [0 1 0 1 0 1 0 1] \
               /C0 [{:.4} {:.4} {:.4} {:.4}] \
               /C1 [{:.4} {:.4} {:.4} {:.4}] /N 1 >>",
            c0, m0, y0, k0, c1, m1, y1, k1
        );
        let _ = function_id;
        return (dict, Vec::new());
    }
    // Type 3 stitching.
    let n = stops.len();
    let mut sub_objs: Vec<(usize, String)> = Vec::new();
    let mut sub_refs: Vec<String> = Vec::new();
    for i in 0..(n - 1) {
        let (c0, m0, y0, k0) = stops[i];
        let (c1, m1, y1, k1) = stops[i + 1];
        let sub_id = *sub_first_id;
        *sub_first_id += 1;
        let sub_dict = format!(
            "<< /FunctionType 2 /Domain [0 1] /Range [0 1 0 1 0 1 0 1] \
               /C0 [{:.4} {:.4} {:.4} {:.4}] \
               /C1 [{:.4} {:.4} {:.4} {:.4}] /N 1 >>",
            c0, m0, y0, k0, c1, m1, y1, k1
        );
        sub_objs.push((sub_id, sub_dict));
        sub_refs.push(format!("{sub_id} 0 R"));
    }
    let mut bounds = Vec::new();
    for i in 1..(n - 1) {
        let t = i as f64 / (n - 1) as f64;
        bounds.push(format!("{:.4}", t));
    }
    let encode: Vec<String> = (0..(n - 1)).map(|_| "0 1".to_string()).collect();
    let dict = format!(
        "<< /FunctionType 3 /Domain [0 1] /Range [0 1 0 1 0 1 0 1] \
           /Functions [{}] /Bounds [{}] /Encode [{}] >>",
        sub_refs.join(" "),
        bounds.join(" "),
        encode.join(" "),
    );
    let _ = function_id;
    (dict, sub_objs)
}
