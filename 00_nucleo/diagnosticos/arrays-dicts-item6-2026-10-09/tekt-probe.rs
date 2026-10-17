use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use typst_core::entities::value::Value;

struct Counted;
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static REALLOCS: AtomicUsize = AtomicUsize::new(0);
static FREES: AtomicUsize = AtomicUsize::new(0);
static REQUESTED: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
#[global_allocator] static ALLOC: Counted = Counted;
fn add_live(bytes: usize) {
    let current = LIVE.fetch_add(bytes, Relaxed) + bytes;
    PEAK.fetch_max(current, Relaxed);
}
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = System.alloc(layout);
        if !p.is_null() { ALLOCS.fetch_add(1,Relaxed); REQUESTED.fetch_add(layout.size(),Relaxed); add_live(layout.size()); }
        p
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let p = System.alloc_zeroed(layout);
        if !p.is_null() { ALLOCS.fetch_add(1,Relaxed); REQUESTED.fetch_add(layout.size(),Relaxed); add_live(layout.size()); }
        p
    }
    unsafe fn realloc(&self, ptr: *mut u8, old: Layout, new: usize) -> *mut u8 {
        let p = System.realloc(ptr, old, new);
        if !p.is_null() {
            REALLOCS.fetch_add(1,Relaxed); REQUESTED.fetch_add(new,Relaxed);
            if new >= old.size() { add_live(new-old.size()); }
            else { LIVE.fetch_sub(old.size()-new,Relaxed); }
        }
        p
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr,layout); FREES.fetch_add(1,Relaxed); LIVE.fetch_sub(layout.size(),Relaxed);
    }
}
#[derive(Clone,Copy)] struct Snap { a:usize,r:usize,f:usize,q:usize,l:usize,p:usize }
fn snap() -> Snap { Snap { a:ALLOCS.load(Relaxed),r:REALLOCS.load(Relaxed),f:FREES.load(Relaxed),q:REQUESTED.load(Relaxed),l:LIVE.load(Relaxed),p:PEAK.load(Relaxed) } }
fn begin() -> Snap { let s=snap(); PEAK.store(s.l,Relaxed); s }
fn result(label:&str,n:usize,b:Snap) {
    let e=snap(); println!("{}\t{}\tallocs={}\treallocs={}\tfrees={}\trequested={}\tlive_delta={}\tpeak_delta={}",label,n,e.a-b.a,e.r-b.r,e.f-b.f,e.q-b.q,e.l as isize-b.l as isize,e.p.saturating_sub(b.l));
}
fn main() {
    println!("layout\tValue={}\tEcoString={}\tPair={}\tIndexMap={}",size_of::<Value>(),size_of::<EcoString>(),size_of::<(EcoString,Value)>(),size_of::<IndexMap<EcoString,Value,FxBuildHasher>>());
    for n in [1000usize,5000,20000,40000] {
        let mut values=Vec::new();
        let b=begin();
        for i in 0..n { values.push(Value::Int(i as i64)); }
        black_box(&values); result("array-build",n,b);
        println!("array-capacity\t{}\tlen={}\tcapacity={}\texcess_bytes={}",n,values.len(),values.capacity(),(values.capacity()-values.len())*size_of::<Value>());
        let value=Value::Array(values);
        let b=begin(); let clone=black_box(value.clone()); result("array-clone-one",n,b);
        black_box(&clone); drop(clone); drop(value);
    }
    for n in [200usize,1000,2000,4000] {
        let mut entries=IndexMap::with_hasher(FxBuildHasher);
        let b=begin();
        for i in 0..n { entries.insert(EcoString::from(i.to_string()),Value::Int(i as i64)); }
        black_box(&entries); result("dict-build",n,b);
        println!("dict-capacity\t{}\tlen={}\tcapacity={}\texcess_pair_bytes={}",n,entries.len(),entries.capacity(),(entries.capacity()-entries.len())*size_of::<(EcoString,Value)>());
        let value=Value::Dict(entries);
        let b=begin(); let clone=black_box(value.clone()); result("dict-clone-one",n,b);
        black_box(&clone); drop(clone); drop(value);
    }
}
