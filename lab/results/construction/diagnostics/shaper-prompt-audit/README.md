# Shaper prompt audit protocol

Behavioral audit using public APIs, not a performance benchmark. No production or L0 changes. Build offline release then --locked using run.py; retain exact incoming patch and before/after fingerprints. Fixtures/hashes are in inputs.json. run.py refuses to replace existing results.

Compare isolated fresh shaping with AB/BA in the same document for absolute tracking at 12/24pt and tracking values 0.00599/0.00601pt at 12pt. Hold other fields fixed. Probe normal versus forced math script with six texts across the four font fixtures; coincidence is non-discriminating, not proof of complete keys.

Fallback control uses synthetic catalog names/flags/variants with real font bytes: primary slot 0, candidate slots 5/6 have the same CJK face; slots 1/2 contain decoy metadata. Compare actual emitted family with public FontBook::select_fallback over global [5,6]. This tests internal consistency, not a vanilla output or system catalog's probability of failure.

Same-font script control: shape abαβ with Libertinus Serif. Verify Latin and Greek font coverage via successful shaped glyphs and inspect emitted sub-run text ranges against the explicit L0 script-boundary contract. Preliminary probe/source archived separately before adding this control.
