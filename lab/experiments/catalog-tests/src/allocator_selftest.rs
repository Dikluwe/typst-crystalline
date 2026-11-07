//! Explicit allocator operations; no output or allocating setup inside snapshots.
use std::alloc::{GlobalAlloc, Layout};
use super::{report, snapshot, ALLOCATOR};

pub(super) fn run() {
    let layout128 = Layout::from_size_align(128, 8).expect("valid allocator control layout");
    let layout192 = Layout::from_size_align(192, 8).expect("valid allocator control layout");
    let layout64 = Layout::from_size_align(64, 8).expect("valid allocator control layout");
    let s0 = snapshot();
    // This window calls the actual global adapter explicitly. Successful-path
    // pointer inspection, byte writes, and snapshots perform no allocations.
    let (zero_valid, grow_valid, shrink_valid, s1, s2, s3, s4) = unsafe {
        let original = GlobalAlloc::alloc_zeroed(&ALLOCATOR, layout128);
        if original.is_null() { std::alloc::handle_alloc_error(layout128); }
        let zero_valid = std::slice::from_raw_parts(original, 128).iter().all(|byte| *byte == 0);
        for index in 0..128 {
            original.add(index).write((index as u8).wrapping_mul(17).wrapping_add(3));
        }
        let s1 = snapshot();
        let grown = GlobalAlloc::realloc(&ALLOCATOR, original, layout128, 192);
        if grown.is_null() {
            GlobalAlloc::dealloc(&ALLOCATOR, original, layout128);
            std::alloc::handle_alloc_error(layout192);
        }
        let grow_valid = std::slice::from_raw_parts(grown, 128).iter().enumerate()
            .all(|(index, byte)| *byte == (index as u8).wrapping_mul(17).wrapping_add(3));
        let s2 = snapshot();
        let shrunk = GlobalAlloc::realloc(&ALLOCATOR, grown, layout192, 64);
        if shrunk.is_null() {
            GlobalAlloc::dealloc(&ALLOCATOR, grown, layout192);
            std::alloc::handle_alloc_error(layout64);
        }
        let shrink_valid = std::slice::from_raw_parts(shrunk, 64).iter().enumerate()
            .all(|(index, byte)| *byte == (index as u8).wrapping_mul(17).wrapping_add(3));
        let s3 = snapshot();
        GlobalAlloc::dealloc(&ALLOCATOR, shrunk, layout64);
        let s4 = snapshot();
        (zero_valid, grow_valid, shrink_valid, s1, s2, s3, s4)
    };
    assert!(zero_valid, "alloc_zeroed must initialize all requested bytes");
    assert!(grow_valid && shrink_valid, "realloc must retain the requested prefix");
    report("allocator.zeroed", s0, s1);
    report("allocator.grow", s1, s2);
    report("allocator.shrink", s2, s3);
    report("allocator.free", s3, s4);
    println!("allocator_control zero_valid=true prefix_valid=true");
}
