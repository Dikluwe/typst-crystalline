# Passo 1019 — Relatório final

**Data**: 2026-08-13  
**Commit de base**: `6fc51d954` (P1018-P1020: CounterKey enum, heading counter gate e footnote superscript)  
**Ficheiros alterados**:

- `01_core/src/compiler/introspect.rs:733` — gate `if *numbering_active` antes de `apply_hierarchical_at` do counter de heading.
- `01_core/src/compiler/layout/heading.rs` — usa `chain.custom("heading.numbering")` para decidir prefixo visual, consistente com o gate.
- `01_core/src/compiler/eval/rules.rs` — `#set heading(numbering: ...)` empurra `heading.numbering = Bool(active)` e `heading.numbering.pattern = Str(pattern)` na chain.
- `01_core/src/compiler/introspect.rs:1101-1103` — de-bake do `numbering_active` a partir da chain no momento de emissão do payload.
- `01_core/src/compiler/layout/tests.rs` — testes de layout ajustados à nova semântica.
- `03_infra/src/integration_tests.rs` — ajustados `p506_counter_at_label_via_context` e `p844_a6_display_sem_pattern_usa_numbering_do_set`.
- `00_nucleo/prompts/compiler/introspect.md` — L0 actualizado.

---

## Resumo

O cristalino avançava o counter de `heading` incondicionalmente quando processava um heading. O vanilla só avança quando existe um `numbering` activo (`#set heading(numbering: ...)`). Aplicámos o mesmo gate.

### Estado antes

```typst
= A
= B
#context counter(heading).get()
```

Cristalino devolvia `(2,)`; vanilla devolve `(0,)`.

### Estado depois

O avanço do counter está condicionado ao campo assado `numbering_active` no `ElementPayload::Heading`:

```rust
// 01_core/src/compiler/introspect.rs:733
ElementPayload::Heading { depth, numbering_active, .. } => {
    intr.kind_index.entry(ElementKind::Heading).or_default().push(loc);
    if *numbering_active {
        intr.counters.apply_hierarchical_at(
            CounterKey::Selector(Selector::Kind(ElementKind::Heading)),
            *depth as usize,
            loc,
        );
    }
    ...
}
```

A flag `numbering_active` é assada a partir da chain (`chain.custom("heading.numbering") == Some(Value::Bool(true))`) no momento de emissão do payload (`introspect.rs:1101-1103`). O heading continua locatable e indexado mesmo sem numeração; só o counter fica dormente.

### Decisão do gate

Aprovado pelo dono. A condição era reaproveitar `numbering_active` já computado em `ElementPayload::Heading` em vez de introduzir estado novo. Foi exactamente isso que foi feito.

---

## Validação

```
cargo test --workspace
```

Resultado:

- typst-core: 4971 passed
- typst-infra: 787 passed
- typst-shell: 41 passed
- benches: 2 passed
- wiring: 37 passed
- crystalline_lint: 2 passed

Total: **5840 tests passed**, 0 failed.

```
crystalline-lint .
```

Zero erros. Três warnings V7 pré-existentes (`auditar-fatiamento.md`, `auditar-spec.md`, `package_version_resolution.md`), nada relacionado com esta mudança.

```
crystalline-lint --fix-hashes .
```

Nenhum drift restante.

---

## Ajustes de testes

Dois testes de integração assumiam o comportamento antigo (heading avança sempre) e foram actualizados para a semântica P1019:

- `p506_counter_at_label_via_context`: sem `#set heading(numbering:)`, o heading não avança; o `.step()` inicial deixa o counter em 1, logo `counter(heading).at(<lbl>)` devolve `(1,)`.
- `p844_a6_display_sem_pattern_usa_numbering_do_set`: sem `numbering:`, `counter(heading).display()` devolve `0` em vez de `1`; com `#set heading(numbering: "1.")` continua a devolver `1.`.

---

## Notas para passos futuros

- O gate partilha o mesmo `numbering_active` usado pelo layout para decidir se desenha o prefixo do heading, garantindo consistência entre introspecção e render.
- A regra vanilla confirmada é: avanço condicional à presença de `numbering`, não a `outlined` ou ao nível do heading.
