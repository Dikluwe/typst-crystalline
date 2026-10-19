# ADR-0109 — Atomization: definition and distinction from decoupling

**Status:** ACCEPTED
**Status history:** accepted (owner decision).
**Decision context:** the F arc (P331–P375). Supersedes any earlier use of “atomization” that confused it with the lens metric `content→elements` or with import decoupling.
**Numbering history:** P376 recorded this ADR as `0110` without scanning `00_nucleo/adr/` (an invented number); P377 corrected it to **0109** (the first free slot — the highest in use was 0108).

---

## Decision

**Atomization** means distributing logic and data from **monolithic** files into **smaller, single-responsibility** files, each **independently readable and understandable**.

The success criterion is **readability**: can I understand a unit (an element, a feature) by opening **only its file**, without hunting for its logic across core files?

Atomization is therefore measured by:

- the **size** of monolithic files (which should shrink);
- the **number of responsibilities** per file (which should tend toward one);
- **locality**: a unit’s logic lives in the unit’s file — **in its layer** (render logic in the render layer, not in the data *struct* file; see Form B).

## What atomization is NOT (non-goals recorded to prevent drift)

1. **It is NOT zeroing `content→elements` (the lens metric).** That metric counts **imports** (who imports whom). Atomization concerns **where logic lives**, not who imports whom. The core can continue importing and calling elements; this does **not** harm independent readability (`heading.rs` being imported by `content.rs` does not make `heading.rs` less readable).
2. **It does NOT use dynamic dispatch** (vtable / `Arc<dyn>` / PropMap reification). Static `match` over the enum **remains**, together with the **O(1) jump table** and **compiler exhaustiveness**.
3. **It is NOT decoupling** (replacing elements without recompiling the core, loading external plugins, or creating a core-without-elements compilation boundary). That is a runtime plugin-system/substitutability property — **not a goal of this project**.
4. **It does NOT trade static guarantees for separation.** Separation is achieved by **keeping** the exhaustive `match`, with arm bodies delegating to a feature free function (`Content::Heading(h) => heading::layout(self, h)`). The `match` stays lean and explicit (one line per element); the substantial logic moves to the feature file **in the render layer**. The verbosity of the `match` is an **accepted** cost — it has no runtime cost and is trivial to read (all arms have the same form).

## Canonical form (how to atomize an element) — Option B

> **The form is B (free function in the render layer).** Option A (logic in the *struct* file, `impl XElem { fn layout }`) was **measured and rejected** — see the following section.

Before:
```rust
// engine/layout/mod.rs (monolithic)
match content {
    Content::Heading(h) => { /* 44 lines of heading layout */ }
    Content::Figure(f)  => { /* 34 lines of figure layout */ }
    // … 59 bespoke arms, 1857 lines …
}
```
After:
```rust
// engine/layout/mod.rs (lean — exhaustive match preserved, one-line bodies)
match content {
    Content::Heading(h) => heading::layout(self, h),
    Content::Figure(f)  => figure::layout_figure(self, /* … */),
    // … 59 arms, one line each — exhaustiveness and jump table intact …
}
// engine/layout/heading.rs (atomized render logic, independently readable)
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    h:        &HeadingElem,
) {
    // the 44 lines live here — access to Layouter’s private state is possible
    // because this is a descendant module of rules::layout (no reverse import,
    // no pub(crate)).
}
```

- The `Content` enum **remains closed**, with its variants (ADR-0026/0105-D model).
- The `match` remains **without a wildcard** (exhaustive) — the compiler still catches a forgotten element.
- Dispatch remains **static** (jump table) — no algorithmic gain or loss; **no vtable** (ADR-0026 is satisfied, not amended).
- Each element’s render logic **moves to its file in the render layer** (`engine/layout/<elem>.rs`), as a **free function** that receives `&mut Layouter`. This follows six repository precedents (`figure.rs`, `image.rs`, `grid.rs`, `placement.rs`, `outline.rs`, `references.rs`).

## Option A rejected (recorded so the AI does not reintroduce it)

**Option A** puts render logic in the element’s *struct* file (`entities/elements/heading.rs`, `impl HeadingElem { fn layout(&self, ctx) }`). **P376 measured** that it **requires three things** contrary to the architecture:

1. **Reverse import `entities → rules::layout::Layouter`** — the **data** file would import **render**. This creates a module cycle `entities ↔ rules` and reverses data→render direction (against the separation assumed by ADR-0107).
2. **Widen `Layouter` visibility** (`style`/`regions`/`font_size_pt`/`flush_line`/… from private to `pub(crate)`) — this expands the API surface.
3. **Thread generics** `Layouter<'a, M, S>` through the struct method.

**Option B avoids all three** (the render method lives in the render layer and accesses private `Layouter` state through **module descent**, not a reverse import). Therefore the canonical form is **B**. **Do not reintroduce A** in the name of “opening only the struct file” — independent readability is achieved in the render layer, without data→render coupling.

## Relationship to other ADRs

- **ADR-0026** (enum without vtable): **satisfied.** Atomization introduces no vtable; it uses the “linear enum without vtable” prescribed by 0026, with logic moved to render-layer free functions.
- **ADR-0105** (model D, exhaustiveness clause 3): **satisfied and reinforced.** The exhaustive `match` is **preserved** (dynamic dispatch is not reintroduced with a test/lint).
- **ADR-0107** (parity with the language; data/render separation): Form B **respects** the separation (render remains in the render layer); Form A would **violate** it (data importing render). Atomization is structural and does not change behavior (content-preserving).

## Historical error corrected by this ADR (recorded so the AI does not repeat it)

In P346, the lens metric `content→elements` (target 0) was promoted to “Milestone G” and treated as the goal of atomization. This confused two meanings: the lens measures **imports** (decoupling); the goal is **locality of logic** (atomization). P361/P375 measured that zeroing `content→elements` **relocates** coupling rather than removing it, and requires dynamic dispatch (vtable, rejected by 0026) plus loss of exhaustiveness. **None of this serves atomization** as defined here. Atomization is achieved **without** touching the lens metric, **without** dynamic dispatch, and **without** losing exhaustiveness.

## Excerpt for `claude.md` (to prevent the AI from drifting)

> **Atomization** (ADR-0109) means moving logic from monolithic files to the owning unit’s file, **in its layer** (each element/feature is independently readable). It is **NOT** zeroing `content→elements`, **NOT** import decoupling, does **NOT** use vtable/`dyn`/PropMap, and does **NOT** remove the exhaustive `match`. The **form is B**: the core `match` remains lean (bodies delegate to a free function `<elem>::layout(self, e)` in `engine/layout/<elem>.rs`); render logic moves to the feature file in the render layer (accesses `Layouter` through module descent, **without** reverse import `entities→engine`, **without** `pub(crate)`). **Option A** (logic in the struct file) is **rejected** — it couples data to render. The exhaustive `match`, jump table, and imports **remain**. The lens metric (`content→elements`) is **irrelevant** — do not use it as a gate. If a plan proposes dynamic dispatch, “zeroing `content→elements`,” or Option A in the name of atomization, **that is the drift prohibited by this ADR** — stop and separate the meanings.

## Scope of application (owner decision)

- **First, the elements** handled by the F arc (atomize each element’s layout into its file in the render layer). **P376** materialized the **containers** slice (Block/Boxed/Stack/Pad; monolith −581 lines, Form B, content-preserving). **P377+** continue the remaining slices.
- A **project-wide scan** (all monoliths, not only element monoliths) follows **after the elements are finished**.
- The **crate decision** (creating new crates inside layers for groupings) also follows — start by moving code into **files within current crates**.
