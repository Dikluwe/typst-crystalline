//! Structural evidence only: no elapsed-time measurement or production API.
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs};
fn ranges(set: &BTreeSet<u32>) -> usize {
    let mut previous = None;
    let mut count = 0;
    for &c in set {
        if previous.is_none_or(|p: u32| c != p.saturating_add(1)) {
            count += 1;
        }
        previous = Some(c);
    }
    count
}
fn u32_at(bytes: &[u8], offset: usize) -> usize {
    u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize
}
fn main() {
    let inputs: serde_json::Value =
        serde_json::from_slice(include_bytes!("../inputs.json")).unwrap();
    let mut rows = Vec::new();
    for input in inputs.as_array().unwrap() {
        let data = fs::read(input["path"].as_str().unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&data)),
            input["sha256"].as_str().unwrap()
        );
        let index = input["face_index"].as_u64().unwrap() as u32;
        let face = ttf_parser::Face::parse(&data, index).unwrap();
        let mut all = Vec::new();
        let mut union = BTreeSet::new();
        let mut subtables = Vec::new();
        if let Some(cmap) = face.tables().cmap {
            for subtable in cmap.subtables {
                if !subtable.is_unicode() {
                    continue;
                }
                let mut points = Vec::new();
                subtable.codepoints(&mut |c| points.push(c));
                let ordered = points.windows(2).all(|p| p[0] < p[1]);
                assert!(ordered, "Pinned streaming assumption refuted for fixture");
                let set: BTreeSet<_> = points.iter().copied().collect();
                let before = union.len();
                union.extend(&set);
                subtables.push(json!({"callback_count":points.len(),"strictly_increasing":ordered,"continuous_ranges":ranges(&set),"new_members_contributed":union.len()-before,"merged_union_ranges":ranges(&union)}));
                all.extend(points);
            }
        }
        let resets = all.windows(2).filter(|p| p[0] > p[1]).count();
        let adjacent_runs = if all.is_empty() {
            0
        } else {
            1 + all.windows(2).filter(|p| p[0].saturating_add(1) != p[1]).count()
        };
        let ttc = ttf_parser::fonts_in_collection(&data);
        let offset =
            if ttc.is_some() { u32_at(&data, 12 + 4 * index as usize) } else { 0 };
        let table_count =
            u16::from_be_bytes(data[offset + 4..offset + 6].try_into().unwrap()) as usize;
        let mut payload = 0usize;
        let mut reconstructed = 12 + 16 * table_count;
        for table in 0..table_count {
            let len = u32_at(&data, offset + 12 + 16 * table + 12);
            payload += len;
            reconstructed += (len + 3) & !3;
        }
        rows.push(json!({"id":input["id"],"font_sha256":input["sha256"],"face_index":index,"input_bytes":data.len(),"collection_faces":ttc,"subtables":subtables,"all_callback_count":all.len(),"unique_members":union.len(),"duplicate_callbacks_across_subtables":all.len()-union.len(),"global_sequence_decreases":resets,"tekt_pre_sort_adjacent_intervals":adjacent_runs,"final_continuous_ranges":ranges(&union),"tekt_collected_vec_payload_bytes":all.len()*4,"sfnt_table_count":table_count,"sfnt_table_payload_bytes":payload,"sfnt_reconstruction_output_bytes_formula":reconstructed,"sfnt_checksum_u32_words_formula":reconstructed/4,"note":"Byte counts are table-directory formulas, not measured allocations, RSS, I/O traffic or timings. Range counts use an independent set; no private CoverageBuilder is exposed or timed."}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"structural_probe_only":true,"fixtures":rows})
        )
        .unwrap()
    );
}
