# Implementation clarification to frozen Ref proposal

The frozen ref-phase-proposal.md (SHA beginning 1578cbea) remains byte-for-byte
unchanged. This separate clarification resolves the already specified exactly-one
actual child contract without changing a proposed signature.

A canonical produced Styled(Ref) transport always has supplement=Some(Content).
Use Content::Empty when selection yields no content. An otherwise token-matching
carrier with supplement=None is malformed: the borrowed accessor returns None,
the mapping facade returns Ok(None), and transform is never invoked. A recognized
canonical carrier has one actual selected child, including Empty, so its transform
executes exactly once and the facade returns Some(rebuilt carrier).

Fresh raw ContextBlock IDs distinguish actual insertion sites. Their IDs and
occurrence slots must remain stable across retained retries of the same
contribution; blindly consuming a new global ID on every retry is incorrect.
Use the existing producer/frame identity protocol, with reached insertion order
and retained slots. Already Located aliases retain their original IDs/Locations.
Stable repeated-reference and nested-context replay tests are required proof.

## Non-Figure and provisional-missing targets

A resolved non-Figure target passes through with original Ref supplement
None/default/explicit morphology unchanged. It must not recursively admit a new
producer for its returned raw Ref. A separate precisely recognized private
passthrough guard may prevent re-admission; it is not the canonical single-child
Figure token, does not qualify for the public child accessor/map, and requires no
additional public facade. Do not canonicalize a non-Figure default None to Empty
because that would replace its existing type-specific default supplement.

If a provisional target is missing, record the actual Query dependency, then
retain the existing missing-reference error/producer marker for invalidation and
retry. Do not return a raw Ref whose layout could prematurely render rich retained
supplement content before read validation. The retained stable missing target
publishes the existing diagnostic at the Ref source origin; changing target
membership invalidates the contribution through its recorded actual Query. No
hidden callback, fake counter read or early layout execution is allowed.

Acceptance must distinguish resolved non-Figure passthrough, provisional missing
then dynamically produced target, stable missing, and the canonical Figure Empty
selected-child carrier. These are implementation clarifications, not edits to the
frozen proposal or authorization to implement the pending phase.
