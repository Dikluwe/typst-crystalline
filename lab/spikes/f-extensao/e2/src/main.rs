//! Spike E2 harness — end-to-end story + perf scenario.
//!
//! Run: `cd lab/spikes/f-extensao/e2 && cargo run --release`

mod callout;
mod core;

use core::{render, query, Chain, Content, RenderTable, Registry, SeqElem};
use std::time::Instant;

// ===========================================================================
// End-to-end story (registration, #set, #show, query, render).
// ===========================================================================
fn end_to_end() {
    let mut reg = Registry::new();
    let mut rt = RenderTable::new();

    // (1) third party registers callout — core untouched.
    let callout_id = callout::register(&mut reg, &mut rt);

    // build a tiny doc: a seq of a native text + a callout(body, title, tone).
    let doc = Content::Seq(SeqElem {
        children: vec![
            Content::text("intro. "),
            callout::callout(
                callout_id,
                Content::text("disk almost full"),
                Some("Storage"),
                None, // no inline tone -> resolved via chain/default.
            ),
        ],
    });

    // (2) #set callout(tone: "warn") -> push Property on the chain.
    let mut chain = Chain::new();
    chain.push(callout::set_tone(callout_id, "warn"));

    println!("== end-to-end ==");
    println!("render (set tone=warn):       {}", render(&doc, &reg, &chain, &rt));

    // (3) #show callout: uppercase -> push a Recipe (innermost).
    chain.push(callout::show_uppercase(callout_id));
    println!("render (set warn + show up):  {}", render(&doc, &reg, &chain, &rt));

    // (4) query(callout) -> collect all callout instances by ElementId.
    let mut found = Vec::new();
    query(&doc, callout_id, &mut found);
    println!("query(callout) count:         {}", found.len());
    if let Some(u) = found.first() {
        let inline_tone: Option<String> = u.field::<String>(callout::PROP_TONE);
        println!("first callout inline tone:    {:?}", inline_tone);
    }
    println!();
}

// ===========================================================================
// Perf scenario. Build a synthetic doc of N elements, render to string, time
// it >=10x. Two variants: 0 callouts (all native) and N/2 callouts mixed.
// The 0-callout variant measures the type-erased-chain overhead on natives:
// the chain still gets walked per resolve attempt — but natives don't resolve
// props here, so the dominant cost is dyn dispatch in render + the chain build.
// To make the downcast cost visible on natives, the chain carries a few set
// entries that natives never match, so every callout read walks past them.
// ===========================================================================
fn build_doc(n: usize, callout_id: core::ElementId, with_callouts: bool) -> Content {
    let mut children = Vec::with_capacity(n);
    for i in 0..n {
        if with_callouts && i % 2 == 0 {
            children.push(callout::callout(
                callout_id,
                Content::text("body"),
                None,
                None,
            ));
        } else {
            children.push(Content::text("x"));
        }
    }
    Content::Seq(SeqElem { children })
}

fn timed(label: &str, doc: &Content, reg: &Registry, chain: &Chain, rt: &RenderTable) {
    const RUNS: usize = 12;
    let mut samples = Vec::with_capacity(RUNS);
    let mut last_len = 0usize;
    for _ in 0..RUNS {
        let t = Instant::now();
        let s = render(doc, reg, chain, rt);
        let dt = t.elapsed().as_secs_f64() * 1e3; // ms
        last_len = s.len();
        samples.push(dt);
    }
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = samples.iter().cloned().fold(0.0, f64::max);
    println!(
        "{label:<28} mean={mean:7.3} ms  min={min:7.3}  max={max:7.3}  (out={last_len} bytes, {RUNS} runs)"
    );
}

fn perf() {
    const N: usize = 10_000;
    let mut reg = Registry::new();
    let mut rt = RenderTable::new();
    let callout_id = callout::register(&mut reg, &mut rt);

    // Chain with several #set entries the readers must walk past (fallback cost).
    let mut chain = Chain::new();
    chain.push(callout::set_tone(callout_id, "info"));
    chain.push(callout::set_tone(callout_id, "warn")); // innermost wins.

    let native_doc = build_doc(N, callout_id, false);
    let mixed_doc = build_doc(N, callout_id, true);

    println!("== perf (N={N}) ==");
    timed("0-callout (all native)", &native_doc, &reg, &chain, &rt);
    timed("N-callout (mixed ~50%)", &mixed_doc, &reg, &chain, &rt);
    println!();
}

fn main() {
    end_to_end();
    perf();
}
