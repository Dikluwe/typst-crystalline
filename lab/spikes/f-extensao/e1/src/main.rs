//! Spike E1 driver — end-to-end story + perf scenario.
//! DISPOSABLE. Run: `cargo run` (demo) / `cargo run --release` (timings).

mod callout;
mod core;

use std::sync::Arc;
use std::time::Instant;

use callout::Callout;
use core::{
    query, realize, Content, PropMap, Recipe, Registry, ShowChain, StyleChain,
    Value,
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

/// Helper for show rules that override a callout's `tone` own-prop.
/// `c` may be a guarded node — peel it to reach the real element.
fn force_tone(c: &Content, tone: &str) -> Content {
    match c.peeled() {
        Content::Dynamic(e) => {
            Content::dynamic(e.with_prop("tone", Value::Str(tone.into())))
        }
        other => other.clone(),
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
// The 5 #show cases — each proves one vanilla behavior on the E1 frontier.
// ---------------------------------------------------------------------------

fn callout(title: &str, body: &str, tone: &str) -> Content {
    Content::dynamic(Arc::new(Callout::new(title, Content::text(body), tone)))
}

fn show_cases() {
    let base = StyleChain::new();
    println!("\n=== 5 #show cases ===");

    // --- Case 1: MULTI-RULE — 2 rules on the same kind ---
    // Vanilla: recipes are tried INNERMOST-FIRST; the first matching, un-guarded
    // recipe is THE step for this pass (one rule/pass). The other rule then
    // applies on the NEXT pass. We register both as func rules forcing a tone;
    // the innermost (registered LAST in the scope vec, = closest to the node)
    // must win on pass 1.
    {
        let doc = Content::seq(vec![callout("A", "body", "note")]);
        // scope vec is OUTERMOST-FIRST: [outer=warn, inner=danger]
        let rules = [
            Recipe::func(Callout::KIND, |c| force_tone(c, "warn")),   // outer
            Recipe::func(Callout::KIND, |c| force_tone(c, "danger")), // inner
        ];
        let chain = ShowChain::new().scoped(&rules);
        let (out, passes) = realize(&doc, &chain, 16);
        println!("[1 multi-rule]   {}  [{passes} passes]", out.render(&base));
        println!("                 -> innermost (danger) fires pass 1; outer (warn) pass 2 — final tone = last-applied = WARN");
    }

    // --- Case 2: MULTI-PASS / RECURSION — a rule that emits its OWN kind ---
    // `#show heading: it => [<heading> wrapping the same heading]`. Without a
    // guard this loops forever. Vanilla guards the produced inner heading by the
    // firing recipe's index so it is skipped on the next pass → terminates.
    {
        let doc = Content::seq(vec![Content::heading(1, Content::text("Title"))]);
        let rules = [Recipe::func("heading", |c| {
            // wrap the (already-guarded) heading inside a NEW heading of its kind
            Content::heading(2, Content::seq(vec![Content::text(">> "), c.clone()]))
        })];
        let chain = ShowChain::new().scoped(&rules);
        let (out, passes) = realize(&doc, &chain, 16);
        println!("[2 recursion]    {}  [{passes} passes — TERMINATED via guard]", out.render(&base));
    }

    // --- Case 3: SHOW-SET — `#show callout: set callout(tone: "warn")` ---
    // A recipe that IS a set. It does NOT replace the node nor consume the show
    // step; it pushes a scoped #set layer the node renders under (vanilla
    // `Transformation::Style => map.apply`). Sibling outside scope unaffected.
    {
        let mut props = PropMap::new();
        props.set("tone", Value::Str("warn".into()));
        let doc = Content::seq(vec![callout("S", "body", "note")]);
        let rules = [Recipe::set(Callout::KIND, Callout::KIND, props)];
        let chain = ShowChain::new().scoped(&rules);
        let (out, passes) = realize(&doc, &chain, 16);
        // instance tone is "note", but the show-set layer overrides at render.
        println!("[3 show-set]     {}  [{passes} pass]", out.render(&base));
        println!("                 -> instance tone=note, show-set forces WARN at render");
    }

    // --- Case 4: SCOPE — a rule inside a block must NOT leak to a sibling ---
    // Two sibling callouts. Only the FIRST is realized under a scoped recipe
    // (force danger); the second is realized under the bare chain. Vanilla scopes
    // recipes to the StyledElem subtree → sibling unaffected.
    {
        let scoped_rules = [Recipe::func(Callout::KIND, |c| force_tone(c, "danger"))];
        let inner_chain = ShowChain::new().scoped(&scoped_rules);
        let outer_chain = ShowChain::new();

        let first = callout("scoped", "in block", "note");
        let (first_done, _) = realize(&first, &inner_chain, 16);

        let sibling = callout("sibling", "outside", "note");
        let (sibling_done, _) = realize(&sibling, &outer_chain, 16);

        let doc = Content::seq(vec![first_done, sibling_done]);
        println!("[4 scope]        {}", doc.render(&base));
        println!("                 -> 1st callout DANGER (in scope); sibling stays NOTE (rule didn't leak)");
    }

    // --- Case 5: NATIVE + DYNAMIC — same harness drives both publics ---
    // One recipe on a NATIVE element (heading) and one on the DYNAMIC user
    // element (callout), in the same chain. Both must match & transform via the
    // SAME machinery — proving E1's frontier is uniform across native/dynamic.
    {
        let doc = Content::seq(vec![
            Content::heading(1, Content::text("Native H")),
            callout("Dyn", "user element", "note"),
        ]);
        let rules = [
            // native: uppercase-ish marker by bumping level text
            Recipe::func("heading", |c| {
                Content::heading(3, Content::seq(vec![Content::text("[H] "), inner_of_heading(c)]))
            }),
            // dynamic: force tone
            Recipe::func(Callout::KIND, |c| force_tone(c, "danger")),
        ];
        let chain = ShowChain::new().scoped(&rules);
        let (out, passes) = realize(&doc, &chain, 16);
        println!("[5 native+dyn]   {}  [{passes} passes]", out.render(&base));
        println!("                 -> heading rewritten (native) AND callout tone=danger (dynamic), one chain");
    }
}

/// Extract the body of a (possibly guarded) heading, else the node itself.
fn inner_of_heading(c: &Content) -> Content {
    let kids = c.peeled().children();
    kids.into_iter().next().unwrap_or_else(|| c.clone())
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

    // #show callout: force tone=danger (single rule, single sanity check)
    let danger = [Recipe::func(Callout::KIND, |c| force_tone(c, "danger"))];
    let chain1 = ShowChain::new().scoped(&danger);
    let (shown, passes) = realize(&doc, &chain1, 16);
    println!("render (#show):  {}  [{passes} pass]", shown.render(&base_chain));

    // ---- The 5 #show cases (Passo 333, Parte 2) ----
    show_cases();

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
