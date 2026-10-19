#![allow(dead_code)]
use std::alloc::{GlobalAlloc,Layout,System};
use std::sync::atomic::{AtomicUsize,Ordering};
use std::hint::black_box;
use std::time::Instant;
use typst_core::entities::{content::Content,value::Value,layout_types::{PagedDocument,Length},element_info::ElementInfo,location::Location,tag::Tag,style::Styles,show::Transformation,func::Func};
use typst_core::compiler::layout::{LayoutPassOutcome,LayoutCallbackRequests};
use typst_core::compiler::math::layout::callbacks::{MathLayoutPassOutcome,MathCancelRequest};
static ALLOCS:AtomicUsize=AtomicUsize::new(0);
static BYTES:AtomicUsize=AtomicUsize::new(0);
struct Counting;
unsafe impl GlobalAlloc for Counting {
 unsafe fn alloc(&self,l:Layout)->*mut u8 { ALLOCS.fetch_add(1,Ordering::Relaxed); BYTES.fetch_add(l.size(),Ordering::Relaxed); unsafe{System.alloc(l)} }
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){unsafe{System.dealloc(p,l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{ ALLOCS.fetch_add(1,Ordering::Relaxed); BYTES.fetch_add(n,Ordering::Relaxed);unsafe{System.realloc(p,l,n)}}
}
#[global_allocator] static ALLOCATOR:Counting=Counting;
struct OutlinePayload {entry:Content,prefix:Value,inner:Value,gap:Length}
enum LayoutCandidate { Pending(LayoutCallbackRequests),Complete(Box<PagedDocument>) }
enum MathCandidate { Pending(Vec<MathCancelRequest>),Complete(Box<PagedDocument>) }
enum TagCandidate { Start(Location,Box<ElementInfo>),End(Location,u128) }
enum TransformationCandidate {Func(Func),Content(Content),Str(ecow::EcoString),Style(Box<Styles>)}
fn size<T>(name:&str){println!("size\t{}\t{}\t{}",name,std::mem::size_of::<T>(),std::mem::align_of::<T>());}
#[inline(never)] fn transport(kind:u8){
 let doc=black_box(PagedDocument::new(Vec::new()));
 match kind {
  0=>{let x=black_box(LayoutPassOutcome::Complete(doc));if let LayoutPassOutcome::Complete(d)=x{black_box(d);}},
  1=>{let x=black_box(LayoutCandidate::Complete(Box::new(doc)));if let LayoutCandidate::Complete(d)=x{black_box(*d);}},
  2=>{let x=black_box(MathLayoutPassOutcome::Complete(doc));if let MathLayoutPassOutcome::Complete(d)=x{black_box(d);}},
  3=>{let x=black_box(MathCandidate::Complete(Box::new(doc)));if let MathCandidate::Complete(d)=x{black_box(*d);}},
  _=>unreachable!()
 }
}
fn main(){
 size::<ContextOriginal>("context_original");size::<ContextCandidate>("context_boxed");size::<OutlinePayload>("outline_payload");
 size::<LayoutPassOutcome>("layout_original");size::<LayoutCandidate>("layout_boxed");size::<MathLayoutPassOutcome>("math_original");size::<MathCandidate>("math_boxed");
 size::<Tag>("tag_original");size::<TagCandidate>("tag_boxed");size::<ElementInfo>("element_info");size::<Transformation>("show_original");size::<TransformationCandidate>("show_boxed");size::<Styles>("styles");size::<PagedDocument>("document");
 for k in 0..4 {transport(k);}
 for sample in 0..8 { for kind in if sample%2==0 {[0,1,2,3]} else {[3,2,1,0]} {
  ALLOCS.store(0,Ordering::Relaxed);BYTES.store(0,Ordering::Relaxed);let t=Instant::now();
  for _ in 0..20000 {transport(kind);}
  let elapsed=t.elapsed().as_nanos();let allocs=ALLOCS.load(Ordering::Relaxed);let bytes=BYTES.load(Ordering::Relaxed);
  println!("transport\t{}\t{}\t{}\t{}\t{}",sample,kind,allocs,bytes,elapsed);
 }}
}

enum ContextOriginal {
    CounterFold {
        key: typst_core::entities::counter::CounterKey,
    },
    CounterAt {
        key: typst_core::entities::counter::CounterKey,
        location: typst_core::entities::location::Location,
    },
    CounterLabelAt {
        key: typst_core::entities::counter::CounterKey,
        label: typst_core::entities::label::Label,
    },
    CounterFinal {
        key: typst_core::entities::counter::CounterKey,
    },
    CounterTotal {
        key: typst_core::entities::counter::CounterKey,
    },
    CounterResolve {
        value: Value,
    },
    CounterLegacyAt {
        key: String,
        label: typst_core::entities::label::Label,
    },
    CounterLegacyFinal {
        key: String,
    },
    StateGet {
        state: typst_core::entities::state::State,
    },
    StateDisplay {
        state: typst_core::entities::state::State,
    },
    StateAt {
        state: typst_core::entities::state::State,
        location: typst_core::entities::location::Location,
    },
    StateFinal {
        state: typst_core::entities::state::State,
    },
    StateResolveLabel {
        label: typst_core::entities::label::Label,
    },
    StateLegacyAt {
        key: String,
        label: typst_core::entities::label::Label,
    },
    StateLegacyFinal {
        key: String,
    },
    Query {
        selector: typst_core::entities::selector::Selector,
    },
    Locate {
        selector: typst_core::entities::selector::Selector,
    },
    Here,
    LocationPage {
        location: typst_core::entities::location::Location,
    },
    LocationPosition {
        location: typst_core::entities::location::Location,
    },
    LocationPageNumbering {
        location: typst_core::entities::location::Location,
    },
    OutlineEntryPage {
        location: typst_core::entities::location::Location,
    },
    OutlineEntryIndented {
        entry: typst_core::entities::content::Content,
        prefix: Value,
        inner: Value,
        gap: typst_core::entities::layout_types::Length,
    },
}
enum ContextCandidate {
    CounterFold {
        key: typst_core::entities::counter::CounterKey,
    },
    CounterAt {
        key: typst_core::entities::counter::CounterKey,
        location: typst_core::entities::location::Location,
    },
    CounterLabelAt {
        key: typst_core::entities::counter::CounterKey,
        label: typst_core::entities::label::Label,
    },
    CounterFinal {
        key: typst_core::entities::counter::CounterKey,
    },
    CounterTotal {
        key: typst_core::entities::counter::CounterKey,
    },
    CounterResolve {
        value: Value,
    },
    CounterLegacyAt {
        key: String,
        label: typst_core::entities::label::Label,
    },
    CounterLegacyFinal {
        key: String,
    },
    StateGet {
        state: typst_core::entities::state::State,
    },
    StateDisplay {
        state: typst_core::entities::state::State,
    },
    StateAt {
        state: typst_core::entities::state::State,
        location: typst_core::entities::location::Location,
    },
    StateFinal {
        state: typst_core::entities::state::State,
    },
    StateResolveLabel {
        label: typst_core::entities::label::Label,
    },
    StateLegacyAt {
        key: String,
        label: typst_core::entities::label::Label,
    },
    StateLegacyFinal {
        key: String,
    },
    Query {
        selector: typst_core::entities::selector::Selector,
    },
    Locate {
        selector: typst_core::entities::selector::Selector,
    },
    Here,
    LocationPage {
        location: typst_core::entities::location::Location,
    },
    LocationPosition {
        location: typst_core::entities::location::Location,
    },
    LocationPageNumbering {
        location: typst_core::entities::location::Location,
    },
    OutlineEntryPage {
        location: typst_core::entities::location::Location,
    },
    OutlineEntryIndented(Box<OutlinePayload>),
}