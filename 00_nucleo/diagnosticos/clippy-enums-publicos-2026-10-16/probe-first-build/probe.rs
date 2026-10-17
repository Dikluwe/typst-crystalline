#![allow(dead_code)]
use std::{alloc::{GlobalAlloc,Layout,System},sync::{Arc,atomic::{AtomicUsize,Ordering}},hint::black_box};
use typst_core::entities::{tag::Tag,location::Location,element_info::ElementInfo,element_payload::ElementPayload,counter_update::CounterUpdate,show::{ShowRule,Transformation,Selector,NodeKind},style::{Styles,Style},content::Content,value::Value,func::Func};
static ALLOCS:AtomicUsize=AtomicUsize::new(0);static BYTES:AtomicUsize=AtomicUsize::new(0);static LIVE:AtomicUsize=AtomicUsize::new(0);static PEAK:AtomicUsize=AtomicUsize::new(0);
fn added(n:usize){ALLOCS.fetch_add(1,Ordering::Relaxed);BYTES.fetch_add(n,Ordering::Relaxed);let live=LIVE.fetch_add(n,Ordering::Relaxed)+n;PEAK.fetch_max(live,Ordering::Relaxed);}
struct Counting;
unsafe impl GlobalAlloc for Counting {
 unsafe fn alloc(&self,l:Layout)->*mut u8{let p=unsafe{System.alloc(l)};if !p.is_null(){added(l.size());}p}
 unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{let p=unsafe{System.alloc_zeroed(l)};if !p.is_null(){added(l.size());}p}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){LIVE.fetch_sub(l.size(),Ordering::Relaxed);unsafe{System.dealloc(p,l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{let q=unsafe{System.realloc(p,l,n)};if !q.is_null(){LIVE.fetch_sub(l.size(),Ordering::Relaxed);added(n);}q}
}
#[global_allocator]static ALLOCATOR:Counting=Counting;
#[derive(Clone)]enum BoxTag{Start(Location,Box<ElementInfo>),End(Location,u128)}
#[derive(Clone)]enum ArcTag{Start(Location,Arc<ElementInfo>),End(Location,u128)}
#[derive(Clone)]enum BoxTransformation{Func(Func),Content(Content),Str(ecow::EcoString),Style(Box<Styles>)}
#[derive(Clone)]enum ArcTransformation{Func(Func),Content(Content),Str(ecow::EcoString),Style(Arc<Styles>)}
#[derive(Clone)]struct Rule<T>{id:u64,selector:Selector,transform:T}
fn info()->ElementInfo{ElementInfo::new(ElementPayload::Heading{depth:1,body_hash:0,counter_update:CounterUpdate::step(),numbering_active:false})}
fn styles(rich:bool)->Styles{let s=Styles::from_iter([Style::Weight(700)]);if rich{s.push_custom("probe-key",Value::Str("long immutable text used to measure the custom channel".into()))}else{s}}
fn raw_rule(i:usize,style:bool,rich:bool)->ShowRule{ShowRule{id:i as u64,selector:Selector::NodeKind(NodeKind::Heading),transform:if style{Transformation::Style(styles(rich))}else{Transformation::Content(Content::Empty)}}}
fn boxed(r:ShowRule)->Rule<BoxTransformation>{Rule{id:r.id,selector:r.selector,transform:match r.transform{Transformation::Style(s)=>BoxTransformation::Style(Box::new(s)),Transformation::Content(c)=>BoxTransformation::Content(c),Transformation::Func(f)=>BoxTransformation::Func(f),Transformation::Str(s)=>BoxTransformation::Str(s)}}}
fn shared(r:ShowRule)->Rule<ArcTransformation>{Rule{id:r.id,selector:r.selector,transform:match r.transform{Transformation::Style(s)=>ArcTransformation::Style(Arc::new(s)),Transformation::Content(c)=>ArcTransformation::Content(c),Transformation::Func(f)=>ArcTransformation::Func(f),Transformation::Str(s)=>ArcTransformation::Str(s)}}}
fn measure<T>(label:&str,repr:usize,n:usize,pct:usize,rich:bool,keep:bool,f:impl FnOnce()->T){
 let base=LIVE.load(Ordering::Relaxed);ALLOCS.store(0,Ordering::Relaxed);BYTES.store(0,Ordering::Relaxed);PEAK.store(base,Ordering::Relaxed);
 let x=black_box(f());let active=LIVE.load(Ordering::Relaxed)-base;let allocations=ALLOCS.load(Ordering::Relaxed);let bytes=BYTES.load(Ordering::Relaxed);let peak=PEAK.load(Ordering::Relaxed)-base;
 black_box(&x);drop(x);assert_eq!(LIVE.load(Ordering::Relaxed),base);
 println!("sample\t{label}\t{repr}\t{n}\t{pct}\t{rich}\t{keep}\t{allocations}\t{bytes}\t{peak}\t{active}");
}
fn tags<T:Clone>(repr:usize,mut start:impl FnMut(Location,ElementInfo)->T,mut end:impl FnMut(Location,u128)->T){let n=4096;
 let make=||{let mut v=Vec::with_capacity(n*2);for i in 0..n{let l=Location::from_raw(i as u128+1);v.push(start(l,info()));v.push(end(l,i as u128));}v};
 measure("tag-build",repr,n,50,false,false,make);
 let mut v=Vec::with_capacity(n*2);for i in 0..n{let l=Location::from_raw(i as u128+1);v.push(start(l,info()));v.push(end(l,i as u128));}
 measure("tag-clone",repr,n,50,false,false,||v.clone());
}
fn rules<T:Clone>(repr:usize,convert:impl Fn(ShowRule)->T){for rich in [false,true]{for pct in [0,25,50,100]{let n=4096;
 measure("rule-build",repr,n,pct,rich,false,||{let mut v=Vec::with_capacity(n);for i in 0..n{v.push(convert(raw_rule(i,i%100<pct,rich)));}v});
 let v:Vec<T>=(0..n).map(|i|convert(raw_rule(i,i%100<pct,rich))).collect();
 measure("rule-filter-clone",repr,n,pct,rich,false,||v.iter().filter(|_|true).cloned().collect::<Vec<_>>());
 for keep in [false,true]{let n=128;measure("rule-register",repr,n,pct,rich,keep,||{
  let mut rules:Arc<[T]>=Arc::from([]);let mut history=Vec::new();for i in 0..n{let mut v=rules.to_vec();v.push(convert(raw_rule(i,i%100<pct,rich)));rules=Arc::from(v);if keep{history.push(Arc::clone(&rules));}}(rules,history)
 });}
}}}
fn size<T>(name:&str){println!("size\t{name}\t{}\t{}",std::mem::size_of::<T>(),std::mem::align_of::<T>());}
fn main(){size::<Tag>("tag");size::<BoxTag>("tag-box");size::<ArcTag>("tag-arc");size::<ElementInfo>("element-info");size::<Transformation>("transformation");size::<BoxTransformation>("transformation-box");size::<ArcTransformation>("transformation-arc");size::<ShowRule>("rule");size::<Rule<BoxTransformation>>("rule-box");size::<Rule<ArcTransformation>>("rule-arc");size::<Styles>("styles");
 tags(0,Tag::Start,Tag::End);tags(1,|l,i|BoxTag::Start(l,Box::new(i)),BoxTag::End);tags(2,|l,i|ArcTag::Start(l,Arc::new(i)),ArcTag::End);
 rules(0,|r|r);rules(1,boxed);rules(2,shared);
}
