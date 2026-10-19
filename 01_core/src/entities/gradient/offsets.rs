//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/gradient/offsets.md
//! @prompt-hash 32fb2b46
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn effective_offsets_precise_for(stops: &[GradientStop]) -> Vec<f64> {
    let n = stops.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![stops[0].offset.map(|ratio| ratio.0).unwrap_or(0.0)];
    }

    let mut offsets: Vec<Option<f64>> =
        stops.iter().map(|stop| stop.offset.map(|ratio| ratio.0)).collect();
    if offsets[0].is_none() {
        offsets[0] = Some(0.0);
    }
    if offsets[n - 1].is_none() {
        offsets[n - 1] = Some(1.0);
    }

    let mut resolved = vec![0.0; n];
    let mut i = 0;
    while i < n {
        if let Some(value) = offsets[i] {
            resolved[i] = value;
            i += 1;
            continue;
        }
        let mut j = i;
        while j < n && offsets[j].is_none() {
            j += 1;
        }
        let previous = resolved[i - 1];
        let next = offsets[j].unwrap();
        let gap = j - i + 1;
        for k in 0..gap {
            resolved[i + k] = previous + (next - previous) * (k + 1) as f64 / gap as f64;
        }
        i = j;
    }
    resolved
}
