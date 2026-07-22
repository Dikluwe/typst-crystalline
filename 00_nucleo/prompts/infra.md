# Infra — typst-infra
Hash do Código: bd52f49a

Em migração.

---

## P844 — testes de integração dos achados #47–#54 de P831

- `03_infra/src/integration_tests.rs` ganhou os testes `p844_a1_...` a `p844_a8_...` (helpers `p844_expand_plain_text`/`p844_expand_errors`, padrão P506/P821): query→content, seletor por função de elemento, `state.at`/`state.final`/`counter.final`, `counter.at(Location)`, repr de array em `#context`, `counter.display` com numbering do `#set` e com pattern real, e a sonda #54 (`#context` entre headings) via `expand_context_blocks_and_reintrospect` + `layout_with_introspector`.
