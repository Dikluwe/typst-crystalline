# Independent structural baseline review

Reviewed the raw second-round cargo JSON logs and immutable copied sources against clean baseline `4382891f0d5f0839669597cb5c90a98f24cfb8bd`. All997 archived Rust inputs match their committed blobs. The copied stage is exactly the original sources plus three single sentinel-variant insertions and the recorded compiler-suggested copied-core bridges. No copied infra source was edited.

The first real core check reports207 E0004 errors and no other error code; none has a primary span at any of the5 selected original core matches. The bridged copied core subsequently compiles. The real infra check reports9 E0004 errors and no other error code; none has a primary span at any of the13 selected original infra matches. Thus all18 selected original catchalls absorb enum growth. Raw primary spans and exact selected byte ranges are retained in the JSON review.

To compile the dependent real infra crate,100 copied-core files receive207 exact compiler-suggested probe arms (with only fully qualified crate-path correction). Together with the third enum-only file,101 archived copied-core files differ from baseline. Every bridge is independently reconstructed from the raw suggestions; none is a productive edit or runtime execution. Without the bridge, the207 existing core errors prevent Cargo from checking infra. Core witnesses were recorded before bridging.

Round1 remains failed infrastructure evidence: its raw initial core check also had207 E0004 diagnostics, but the copied bridge introduced4 E0433 path-qualification errors. That round cannot discharge the infra gate; its logs and failure classification are preserved.

Three active L0 files changed during round2. Consequently this review does not call the active working tree unchanged: the baseline evidence is bound instead to the immutable997-file clean source copy and exact reconstructed disposable mutations. This qualification does not certify productive GREEN, tuple-axis independence or all-alternative runtime behavior; those remain later obligations.
