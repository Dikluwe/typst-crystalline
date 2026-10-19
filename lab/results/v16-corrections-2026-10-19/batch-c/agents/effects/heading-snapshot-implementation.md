# Prepared implementation (not productive code)

After root confirms the revised L0/Nucleus seal and source RED:

1. `stdlib/structural/heading.rs`: immediately after validation and the existing
   `styles.push_custom("heading.supplement", value.clone())`, append the private
   `heading.constructor.supplement` with another immutable `value.clone()`.
   No argument reevaluation, no callbacks, no public field/accessor changes.
2. `expression_dispatch.rs` selected exhaustive Content match: replace the
   Styled false complement with an arm using `node.constructor_heading()`;
   from its borrowed Styles inspect the last exact private snapshot key and pass
   that borrowed Value to the existing exhaustive value_has_update classifier.
   Do not read public heading.supplement or arbitrary style entries. Keep normal
   visitor descent through Styled body, even if the snapshot is absent/negative.
3. The original source constructor update and explicit override are positive;
   inherited numbering-only and explicit-none controls are negative. Existing
   public identity/presence/field-order/external-styled tests must remain GREEN.
