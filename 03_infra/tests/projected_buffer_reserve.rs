//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/projected_buffer_reserve.md
//! @prompt-hash dc590eb6
//! @layer L3
//! @updated 2026-11-10

#[path = "../src/fonts/coverage.rs"]
mod coverage;
#[path = "support/projected_buffer_alloc.rs"]
mod reserve_alloc;
#[path = "../src/fontdb/selective.rs"]
mod selective;

mod fonts {
    pub fn font_info_from_bytes(
        bytes: &[u8],
        index: u32,
    ) -> Option<typst_core::entities::font_book::FontInfo> {
        typst_infra::fonts::font_info_from_bytes(bytes, index)
    }
    pub fn extract_coverage(
        face: &ttf_parser::Face,
    ) -> typst_core::entities::font_book::Coverage {
        crate::coverage::extract(face)
    }
}
mod fontdb {
    pub use typst_infra::fontdb::load_fonts_from_dir;
}

#[global_allocator]
static ALLOCATOR: reserve_alloc::TraceAlloc = reserve_alloc::TraceAlloc;

use selective::project_for_test as project;
use std::io::Cursor;
const MAX_BUFFER: usize = 8 * 1024 * 1024;

fn sfnt(tables: &[([u8; 4], Vec<u8>)]) -> Vec<u8> {
    let mut b = vec![0; 12 + 16 * tables.len()];
    b[..4].copy_from_slice(b"\0\x01\0\0");
    b[4..6].copy_from_slice(&(tables.len() as u16).to_be_bytes());
    for (i, (tag, body)) in tables.iter().enumerate() {
        let at = 12 + 16 * i;
        let offset = b.len() as u32;
        b[at..at + 4].copy_from_slice(tag);
        b[at + 4..at + 8].copy_from_slice(&(0x12340000 + i as u32).to_be_bytes());
        b[at + 8..at + 12].copy_from_slice(&offset.to_be_bytes());
        b[at + 12..at + 16].copy_from_slice(&(body.len() as u32).to_be_bytes());
        b.extend_from_slice(body);
    }
    b
}
fn post(version: u32, len: usize, count: u16) -> Vec<u8> {
    let mut p = vec![0x59; len];
    if len >= 4 {
        p[..4].copy_from_slice(&version.to_be_bytes());
    }
    if len >= 34 {
        p[32..34].copy_from_slice(&count.to_be_bytes());
    }
    p
}
#[test]
fn one_output_reserve_has_at_most_one_reallocation_from_original_header() {
    let b = sfnt(&[
        (*b"cmap", vec![1; 200]),
        (*b"head", vec![2; 54]),
        (*b"hhea", vec![3; 36]),
        (*b"maxp", vec![4; 32]),
        (*b"name", vec![5; 80]),
        (*b"OS/2", vec![6; 96]),
        (*b"post", post(0x20000, 4096, 17)),
    ]);
    let _serial = reserve_alloc::serial();
    let window = reserve_alloc::start(0);
    let out = project(&mut Cursor::new(&b), b.len() as u64, 0);
    let trace = reserve_alloc::stop(window);
    assert!(!trace.overflow);
    let (out, _) = out.expect("known stable projection");
    let (growths, origin) = trace.lineage(out.as_ptr());
    assert_eq!(origin, 12, "reuse the initial header as the output buffer");
    assert!(growths <= 1, "single reserve restriction: output grew {growths} times");
    assert!(out.capacity() >= out.len());
    assert!(out.capacity() <= MAX_BUFFER);
}
#[test]
fn each_injected_reservation_failure_declines_without_a_partial_success() {
    let b = sfnt(&[
        (*b"cmap", vec![5; 400]),
        (*b"name", vec![6; 80]),
        (*b"post", post(0x20000, 512, 12)),
        (*b"head", vec![8; 54]),
    ]);
    let _serial = reserve_alloc::serial();
    for nth in 1.. {
        let window = reserve_alloc::start(nth);
        let out = project(&mut Cursor::new(&b), b.len() as u64, 0);
        let trace = reserve_alloc::stop(window);
        assert!(!trace.overflow);
        if trace.failed {
            assert!(out.is_none(), "failure {nth} cannot yield partial projection");
        } else {
            assert!(out.is_some());
            break; // The first uninjected run has exhausted every reservation.
        }
    }
}

#[test]
fn oversized_complete_output_declines_before_large_reservation() {
    let bytes = sfnt(&[(*b"cmap", vec![0x63; MAX_BUFFER - 27])]);
    let _serial = reserve_alloc::serial();
    let window = reserve_alloc::start(0);
    let out = project(&mut Cursor::new(&bytes), bytes.len() as u64, 0);
    let trace = reserve_alloc::stop(window);
    assert!(out.is_none());
    assert!(!trace.overflow);
    assert!(
        trace.max_request() < MAX_BUFFER,
        "reject oversized total before output or table reservation"
    );
}
