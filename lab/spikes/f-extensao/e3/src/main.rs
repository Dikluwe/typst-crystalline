//! DISPOSABLE SPIKE — design E3 end-to-end harness + perf.
//!
//! STUBS (declared, as required):
//!  - S1: no real eval/parse pipeline. `#set`/`#show`/`make` are direct Rust
//!        calls, not parsed Typst markup.
//!  - S2: Chain is one flat (kind,key)->value map, not a popped cons-list of
//!        scopes. Enough to demonstrate fallback; no scope nesting.
//!  - S3: `show_default` is identity; layout produces a String, not a real
//!        frame/box tree.
//!  - S4: nested-body rendering uses a throwaway Chain (callout bodies are
//!        plain Text in this toy, so no fallback needed inside the body).
//!  - S5: unknown kind_id renders a placeholder rather than erroring.

mod callout;
mod core;

use core::{Chain, Content, KindId, Registry, Value};
use std::time::Instant;

fn main() {
    // ---- END-TO-END STORY -------------------------------------------------
    let mut reg = Registry::new();

    // 1. Registration: insert the third-party descriptor.
    let callout_kind: KindId = callout::register(&mut reg);
    println!("registered `callout` -> kind {}", callout_kind.0);

    // 2. `#set callout(tone: "warn")` — push a scoped default into the chain.
    let mut chain = Chain::new();
    chain.set_default(callout_kind, callout::TONE, Value::Str("warn".into()));

    // A callout WITHOUT an explicit tone -> should fall back to "warn".
    let c_fallback = {
        let mut c = callout::make(callout_kind, "Heads up", "info", Content::Text("x".into()));
        // strip the explicit tone to exercise fallback:
        if let Content::Dynamic { props, .. } = &mut c {
            *props = {
                let mut p = core::PropMap::new();
                p.set(callout::TITLE, Value::Str("Heads up".into()));
                p.set(callout::BODY, Value::Content(Content::Text("watch out".into())));
                p
            };
        }
        c
    };
    println!("set+fallback : {}", reg.render(&c_fallback, &mut chain));

    // A callout WITH explicit tone -> local prop wins over the scoped default.
    let c_explicit = callout::make(
        callout_kind,
        "Note",
        "success",
        Content::Text("all good".into()),
    );
    println!("explicit tone: {}", reg.render(&c_explicit, &mut chain));

    // 3. `#show callout:` minimal — override the layout fn for the kind.
    reg.set_show_override(callout_kind, |props, _reg, chain| {
        let tone = props
            .resolve(callout::TONE, chain)
            .and_then(Value::as_str)
            .unwrap_or("info");
        format!("<<SHOWN tone={tone}>>")
    });
    println!("show override: {}", reg.render(&c_explicit, &mut chain));

    // 4. query(callout) — collect all nodes of the callout kind.
    let doc = Content::Sequence(vec![
        Content::Text("intro ".into()),
        c_explicit.clone(),
        Content::Text(" mid ".into()),
        c_fallback.clone(),
    ]);
    let mut hits = Vec::new();
    core::query(&doc, callout_kind, &mut hits);
    println!("query(callout): {} hit(s)", hits.len());

    // Remove the show override so perf measures the real layout_fn.
    let mut reg = Registry::new();
    let callout_kind = callout::register(&mut reg);

    // ---- PERF SCENARIO ----------------------------------------------------
    const N: usize = 10_000;
    const RUNS: usize = 12;

    let doc_native = build_doc(N, callout_kind, 0.0); // 0 callouts (all native)
    let doc_mixed = build_doc(N, callout_kind, 0.5); // ~50% callout

    let (mean_n, lo_n, hi_n) = bench(&reg, &doc_native, RUNS);
    let (mean_m, lo_m, hi_m) = bench(&reg, &doc_mixed, RUNS);

    println!("\n--- PERF (N={N}, runs={RUNS}) ---");
    println!(
        "0-callout (all native): mean {:.3} ms  [{:.3}..{:.3}]",
        mean_n, lo_n, hi_n
    );
    println!(
        "mixed (~50% callout)  : mean {:.3} ms  [{:.3}..{:.3}]",
        mean_m, lo_m, hi_m
    );
}

/// Build a doc of `n` elements; `frac` of them are callouts, rest native Text.
fn build_doc(n: usize, callout_kind: KindId, frac: f64) -> Content {
    let mut items = Vec::with_capacity(n);
    let every = if frac <= 0.0 {
        usize::MAX
    } else {
        (1.0 / frac).round() as usize
    };
    for i in 0..n {
        if every != usize::MAX && i % every == 0 {
            items.push(callout::make(
                callout_kind,
                "T",
                "warn",
                Content::Text(format!("body{i}")),
            ));
        } else {
            items.push(Content::Text(format!("native text node {i}")));
        }
    }
    Content::Sequence(items)
}

/// Render `runs` times, return (mean, min, max) in milliseconds.
fn bench(reg: &Registry, doc: &Content, runs: usize) -> (f64, f64, f64) {
    let mut times = Vec::with_capacity(runs);
    let mut sink = 0usize;
    for _ in 0..runs {
        let mut chain = Chain::new();
        let t = Instant::now();
        let s = reg.render(doc, &mut chain);
        let dt = t.elapsed().as_secs_f64() * 1e3;
        sink = sink.wrapping_add(s.len()); // prevent dead-code elimination
        times.push(dt);
    }
    std::hint::black_box(sink);
    let mean = times.iter().sum::<f64>() / runs as f64;
    let lo = times.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (mean, lo, hi)
}
