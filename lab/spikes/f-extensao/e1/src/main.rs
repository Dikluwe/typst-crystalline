//! Spike E1 driver — end-to-end story + perf scenario.
//! DISPOSABLE. Run: `cargo run` (demo) / `cargo run --release` (timings).

mod callout;
mod core;

use std::sync::Arc;
use std::time::Instant;

use callout::Callout;
use core::{
    apply_show, query, Content, PropMap, Registry, ShowRule, StyleChain, Value,
};

// ---------------------------------------------------------------------------
// End-to-end story functions
// ---------------------------------------------------------------------------

/// 1) registration: register callout's constructor by name.
fn make_registry() -> Registry {
    let mut reg = Registry::new();
    reg.register(Callout::KIND, Callout::construct);
    reg
}

/// 2) `#set callout(tone: "warn")` — push a scoped layer onto the chain.
fn set_callout_tone(chain: &StyleChain, tone: &str) -> StyleChain {
    let mut props = PropMap::new();
    props.set("tone", Value::Str(tone.to_string()));
    chain.set(Callout::KIND, props)
}

/// 3) `#show callout: it => it.with(tone: "danger")` — a transform fn.
/// STUB: real #show takes a closure with full element access and can return
/// arbitrary content; here it's a fixed `fn(&Content)->Content`. (limit)
fn show_rule_force_danger() -> ShowRule {
    ShowRule {
        kind: Callout::KIND,
        transform: |c| match c {
            Content::Dynamic(e) => Content::dynamic(
                e.with_prop("tone", Value::Str("danger".into())),
            ),
            other => other.clone(),
        },
    }
}

// ---------------------------------------------------------------------------
// Doc builders for the perf scenario
// ---------------------------------------------------------------------------

/// Build a doc of `n` elements; if `with_callouts`, every 2nd item is a callout.
fn build_doc(n: usize, with_callouts: bool, reg: &Registry) -> Content {
    let mut items = Vec::with_capacity(n);
    for i in 0..n {
        if with_callouts && i % 2 == 1 {
            // construct via registry, exercising the dynamic boundary
            let mut props = PropMap::new();
            props.set("title", Value::Str(format!("c{i}")));
            props.set("tone", Value::Str("note".into()));
            props.set("body", Value::Content(Content::text(format!("body {i}"))));
            let el = reg.construct(Callout::KIND, props).unwrap();
            items.push(Content::dynamic(el));
        } else if i % 3 == 0 {
            items.push(Content::heading(2, Content::text(format!("H{i}"))));
        } else {
            items.push(Content::text(format!("para {i}")));
        }
    }
    Content::seq(items)
}

/// Time `render` over the doc `runs` times; return (mean_ms, min_ms, max_ms).
fn time_render(doc: &Content, chain: &StyleChain, runs: usize) -> (f64, f64, f64) {
    let mut samples = Vec::with_capacity(runs);
    let mut sink = 0usize; // prevent dead-code elimination
    for _ in 0..runs {
        let t = Instant::now();
        let s = doc.render(chain);
        sink = sink.wrapping_add(s.len());
        samples.push(t.elapsed().as_secs_f64() * 1000.0);
    }
    std::hint::black_box(sink);
    let mean = samples.iter().sum::<f64>() / runs as f64;
    let min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = samples.iter().cloned().fold(0.0, f64::max);
    (mean, min, max)
}

// ---------------------------------------------------------------------------

fn main() {
    let reg = make_registry();

    // ---- End-to-end demo ----
    println!("=== E1 end-to-end demo ===");

    // a small tree with two callouts + native content
    let c1: Arc<dyn core::Element> = Arc::new(Callout::new(
        "Heads up",
        Content::text("disk almost full"),
        "note",
    ));
    let c2: Arc<dyn core::Element> = Arc::new(Callout::new(
        "FYI",
        Content::text("backup ran"),
        "info",
    ));
    let doc = Content::seq(vec![
        Content::heading(1, Content::text("Report")),
        Content::dynamic(c1),
        Content::text("some prose"),
        Content::dynamic(c2),
    ]);

    // base render (no set/show)
    let base_chain = StyleChain::new();
    println!("render (base):   {}", doc.render(&base_chain));

    // #set callout(tone: "warn")
    let warn_chain = set_callout_tone(&base_chain, "warn");
    println!("render (#set):   {}", doc.render(&warn_chain));

    // query(callout)
    let callouts = query(&doc, Callout::KIND);
    println!("query callouts:  {} found", callouts.len());
    for c in &callouts {
        let tone = match c {
            Content::Dynamic(e) => e.get_prop("tone"),
            _ => None,
        };
        println!("    - plain_text: {:?}  tone={:?}", c.plain_text(), tone);
    }

    // #show callout: force tone=danger
    let shown = apply_show(&doc, &show_rule_force_danger());
    println!("render (#show):  {}", shown.render(&base_chain));

    // ---- Perf scenario ----
    println!("\n=== E1 perf (render, 10 runs each) ===");
    const N: usize = 10_000;
    const RUNS: usize = 10;
    let chain = StyleChain::new();

    let doc_native = build_doc(N, false, &reg);
    let (mean0, min0, max0) = time_render(&doc_native, &chain, RUNS);
    println!(
        "N={N} all-native   mean={mean0:.3} ms  (min={min0:.3} max={max0:.3})"
    );

    let doc_mixed = build_doc(N, true, &reg);
    let (mean1, min1, max1) = time_render(&doc_mixed, &chain, RUNS);
    println!(
        "N={N} mixed callout mean={mean1:.3} ms  (min={min1:.3} max={max1:.3})"
    );
}
