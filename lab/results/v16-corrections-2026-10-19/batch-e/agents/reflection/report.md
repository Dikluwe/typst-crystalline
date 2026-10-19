# Reflection audit and pending materialization

The baseline source and owner are unchanged from the ledger's recorded hashes at HEAD `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`. The reverse seal independently computes `4b43fd2d`; the declared forward effective seal is `a9e109c5` (forward graph validation remains parent-controlled). `audit-state.json` records time, hashes and working-tree provenance. Root and L1 instructions, ADR-0107/0108/0127/0129/0131, the full owning L0, its ten nuclei, the Content enum, field getters and native catalogue were read. No Cargo, oracle execution, linter or sealing was performed by this agent. Only the authorized owning L0 and colocated tests have changed; no productive implementation has changed.

## Measured boundaries

`family-matrix.json` inventories every current nominal Content variant. It records baseline Set/Unset/Undeclared behavior, candidate fields, native callback status and Equation-recognition disposition. These are implementation observations, not assertions of full language coverage. Direct field availability is not enough to establish lexical presence.

- `content_set_fields` uses body as an open complement. Figure.caption, PdfArtifact.kind and Metadata.value are getter-known examples omitted from fields(). OutlineEntry is dangerous to promote blindly: its getter returns fill even when fill_set is false. Additional per-family source/oracle evidence and strict-presence decisions are required.
- `equation_descendant` crosses Styled only. The existing entity getter recognizer crosses Source/ShowRecipe/Located.body and Realized.visual under an already-ratified contract. Thus nested carrier fields() loses keys even when has/at/get_field retain the values. A helper-output difference alone was not used as semantic proof: the local RED exercises the fields method and its ordered dictionary keys.
- `content_elem_func` has established native mappings for Sequence, Matrix, Cell, Table, Text, Strong, Emph, Heading/marked Heading, Title, Raw, typed transforms, Figure, Caption, Link, SmallCaps, Linebreak and OutlineEntry. The rest currently use the noncallable fallback. Catalogue admission does not prove retained constructor identity. Shape and generic Matrix transform remain non-reconstructible; names and geometry are not identity.
- `content_method_with_fields` projects only causal carriers and preserves supplied Location over a Located location, plus authoritative optional snapshots. Ordinary receivers must remain receivers. The continuation is a legitimate explicit nominal complement.
- Neighboring structural holes are the Content wildcard in content_field and the ContentField wildcard in the candidate Set filter. Closing the four ledger sites while moving the same open classification into either helper would not satisfy the obligation. String field names remain an open domain.

## Refuted convenient conclusion: Styled is not uniformly transparent

The language agent's current-pin receipt identifies a further semantic defect: `text(fill: red, math.equation(numbering: "(1)", $x$))` leaks Equation fields in the baseline, while vanilla treats the outer Styled as its own semantic receiver. This refutes interpreting every Styled on a path to Equation as constructor transport. The initial carrier-only proposal is incomplete without this distinction.

The Equation native at `stdlib/structural/math/equation.rs:164–183` wraps present options in plain Styled; it supplies no causal marker. The text native at `stdlib/text/constructor.rs:275` also wraps its body in Styled. Content::styled at `entities/content/construction.rs:27–31` retains the wrappers separately. Existing Heading construction explicitly supplies heading.constructor and its getter recognizes that marker. No equivalent Equation marker exists. Inferring transport from equation.* names, empty styles, geometry or the presence of an Equation child is not valid provenance.

The parent was asked to authorize or assign the Equation producer, shared fields owner and identity owner before adding an internal causal marker/projection. No additional owner was edited. A dedicated local ordinary-Styled RED was added; its expected baseline failure is explicit in tests.json. The previous unmarked carrier fixture remains only a baseline refuter, not proof that arbitrary Styled should become transparent.

## Preservation and pending work

The local suite currently contains the nested-carrier fields RED, has/at explicit-none and precedence controls, non-Equation/Sequence/Strong opacity controls, Realized visual-not-original control, authoritative empty snapshot and Location precedence controls, prescribed-native carrier controls, and the external Styled Equation RED. They have not been run; the parent owns baseline execution and sealing.

Preserve Box body omission, Raw lexical omission versus false/none, Cell auto/none and prepared snapshots, Matrix explicit fields/rows, marked Heading, typed transform operation identity, Sequence immediate children, and same-name unrelated native functions. Existing suites are part of the preservation corpus; additional focused tests should follow the parent's selected measured semantic scope. Native Equation current-pin identity/call failure is measured by the language agent, but safe constructor-transport identity projection requires the missing provenance distinction above.

The language supplement additionally confirms the known-field omissions and OutlineEntry.fill presence mismatch. Figure explicit none remains a representation limitation; it must not be silently certified as omission parity. Full ordinary Styled child/styles reflection, generic rich-field reflection and geometry-derived native recovery are not authorized by this audit.

All conclusions use language values, presence and diagnostics. Exhaustive matches and future-variant compile failures establish domain review protection only, not language parity or performance.

## Phase-one extension after parent authorization

The parent authorized shared fields.rs ownership and an internal causal marker, with no public helper or field. Both owning L0 files now consume the reviewed equation-constructor-origin nucleus. Valid marked wrappers and causal carriers remain transparent; unmarked, false-marker, wrong-type and marked non-Equation wrappers are barriers. Existing synthetic getter fixtures now explicitly model constructor provenance without removing their carrier/precedence checks. New tests cover invalid provenance, the real Equation native callback, present Figure caption, Metadata.value and OutlineEntry.fill lexical presence. Implementation remains unchanged and no tests have been executed by this agent. `phase1-freeze.json` supersedes the initial file hashes for the prepared tests/specification.

PdfArtifact kind promotion is excluded: the language agent's artifact-marker-receipt.json confirms omitted kind and explicit other collapse in the entity but differ in language fields/has/at. Adding kind universally would introduce false presence. Figure explicit none remains similarly uncertified. Existing PdfArtifact behavior is retained pending a separately gated representation repair.

Ordinary Styled controls were narrowed to reject Equation keys, without requiring an empty fields dictionary. Missing strict child/styles reflection remains an explicit intended-contract debt; the tests do not obstruct that future correction.

## Implemented bounded correction

After the parent confirmed six baseline semantic REDs and the getter preservation controls, the two source owners were updated. Reflection now classifies each Content nominally at all four ledger boundaries and the adjacent getter fallback; the Set filter explicitly names Unset and Undeclared. Equation constructor recognition checks the outer wrapper as well as every recursive Styled marker, resolves the last direct marker only, and crosses the four causal carriers without reading Realized.original. Shared get_field uses the same causal boundary. Bare/marked Equation maps to the existing native callback; other newly unmeasured native families retain their legacy disposition.

Figure known caption and Metadata value are enumerated. OutlineEntry enumerates level/element/fill and uses its retained fill_set bit, keeping omitted fill Unset and explicit none known. Candidate ordering is recorded in implementation-freeze.json. PdfArtifact kind, explicit Figure caption:none and ordinary Styled strict child/styles remain visible debts. No existing tests were weakened or altered during implementation. The parent controls build, RED/GREEN, formatting and resealing; independent language-agent review was requested.
