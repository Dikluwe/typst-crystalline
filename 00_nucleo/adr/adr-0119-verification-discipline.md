# ADR-0119 — Verification discipline, not trust

**Status:** ACCEPTED
**Date:** 2026-07-03
**Applies to:** the entire project and every step.
**Numbering history:** no dedicated file/number until P910; reconciled in that step by an actual scan of `00_nucleo/adr/` (slot `0119` was free between `0118` and `0120`).

---

## What was observed

Across the project steps, there were no cases of invented data or false results. There were other kinds of problems:

1. **An explanation accepted without measurement.** A symptom is attributed to a plausible cause, but that cause is not tested. Example: P534/P538e attributed an incorrect line break to “not this step’s objective,” twice, without confirming the assumed cause.
2. **The requested verification was not performed in the report.** A step asks for a specific method (the official benchmark script), but the report uses another one (manual measurement). Example: P544.
3. **No cross-check between nearby steps.** Two steps investigate similar symptoms at the same time without referring to each other. Example: P547 attributed incorrect spacing to the font on the same day P546 was confirming that the same pattern was a real bug.
4. **Scope narrower than requested, without saying so directly.** The report says “fixed” when only part of the problem was addressed, even if its own checklist shows that. Example: P537.

At the same time, when a task asks for a concrete number and a direct comparison, the result is rigorous — P546 and P549 are examples: actual measured positions, not impressions.

## Rule

A broad task (“fix X”) leaves room to choose the easiest path without being false — it may simply be a loose reading of the request. A narrow task with a concrete number to compare against a reference leaves no such room.

From now on, every project step follows these four rules:

### 1. If a step asks for a specific measurement method, the report shows that method’s result

Not an equivalent method, and not a summary of what the method would “probably” say. If a step asks for `benchmark-p507.py`, the report shows that script’s output, not a manual measurement in its place.

### 2. Every explanation of a symptom includes a direct comparison against vanilla

Not “this is probably X.” Instead: “this is X, confirmed as follows: [command, number, comparison].” If there is not enough time or data to confirm, the report says so — “unconfirmed, hypothesis untested” — rather than stating the hypothesis as fact.

### 3. Before closing a step, check whether another recent step touched the same symptom

Search the report directory (for the symptom, not the step name) before writing the conclusion. If another nearby step changed the same area, read its report before deciding.

### 4. “Fixed” applies only to what was actually tested, using the original problem text

If the original problem was `#set page(columns: 2)` and the final test uses `#columns(2)[...]`, the report does not say “fixed”; it says “fixed for X, not tested for Y, which was the original case.”

## Relationship to the previous rule

The rule “no accepted item is permanent without a new decision” (“New decision required”) concerns what gets recorded as accepted. This rule concerns how that decision is reached — deciding again is not enough if the new decision is also made without measurement.

**Reconciliation note (P910):** “New decision required” is cited by name as an existing rule in several batches of this project (`CLAUDE.md`, `handoff-novo-chat-p762.md`, and ADR-0121/0122/0124/0125 below) — including the sentence above, which called it “(previous ADR).” The actual scan of `00_nucleo/adr/` performed in this step (P910) found no file of its own for that rule, and no entry in the curated “Meta-rules in force” list in `README.md`: until this step, it was an informal principle, never materialized as an ADR. ADR-0119 does not formalize it (that is not this ADR’s subject) — it only corrects the above statement, which assumed its existence without checking. Giving it its own number, if the owner decides to do so, is a new and separate decision, not implicit in this reconciliation.

## 5. Typographic output verification: extracted text is not enough

When a step involves kerning, tracking, glyph positioning, or any fine text adjustment, `pdftotext` and the appearance of extracted text are weak indicators. They can look correct when actual positioning is wrong, or incorrect when positioning is right (e.g. differences in subsetting or ToUnicode mapping).

Concrete rules:

1. **Check the PDF `TJ` operator’s internal numbers** with `mutool show <pdf> <obj> | grep TJ` or an equivalent. The sign and order of magnitude of the deltas must be comparable to vanilla for the same document and font.
2. **Check actual glyph positions** with `mutool trace <pdf>` or an equivalent tool. Compare `x`/`y` coordinates and advances (`adv`) character by character against vanilla.
3. **Do not rely only on extracted text** (`pdftotext`, visual `mutool draw`) as proof of correct positioning. Use it as a morphology/language test (ADR-0107), not as mechanical proof.
4. **Watch for error compensation.** If two different problems (e.g. width measured without kerning + `TJ` delta with the wrong sign) cancel out in extracted text, the report must identify the compensation and confirm each error separately with numbers.
5. **Confirm the same font on both sides.** Before comparing `TJ` or positions, check with `mutool info <pdf> | grep Fonts` that crystalline and vanilla use the same family (and preferably the same `.ttf` file).

Example: P549 confirmed that reversing the sign of the P548 `TJ` delta (`nominal - x_advance`) reproduces vanilla’s actual positions, while the earlier P520/P521 formula (`x_advance - nominal`) was wrong from the start — even though P548’s extracted text already looked correct.
