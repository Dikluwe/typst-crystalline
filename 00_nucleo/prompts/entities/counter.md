# Prompt L0 — `entities/counter` — chave de counter documental
Hash do Código: 9e704044

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/counter.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/entities.md`
**ADRs**: ADR-0107 (paridade linguagem), ADR-0127 (gate de contrato público)

---

## Contexto

`Counter` é o valor de primeira classe devolvido por `counter(selector)`. A
chave tem de distinguir dois espaços de nome independentes:

- **String de utilizador** — `counter("meu-counter")`, manipulado só via
  `.update()`/`.step()`/`.get()`.
- **Selector de elemento locatable** — `counter(heading)`, alimentado
  automaticamente pelo walk de `Heading`/`Figure`/`Table`/`Footnote`.

No vanilla a distinção é explícita via `CounterKey::Str` vs
`CounterKey::Selector` (`typst-library/src/introspection/counter.rs:527`). O
cristalino fundiu os dois numa única `EcoString`, o que causa colisão:
`counter("heading")` devolve o valor do contador automático de headings em
vez de `(0,)`.

## Instrução

### Contrato público

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum CounterKey {
    /// Contador manual identificado por string de utilizador.
    Str(EcoString),
    /// Contador automático de elemento locatable.
    Selector(Selector),
    /// Contador de página (vanilla `CounterKey::Page`).
    Page,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub key: CounterKey,
}
```

- `CounterKey::Page` é incluído agora para paridade vanilla; consumers
  actuais que indexavam por `"page"` passam a usar a variante.
- `Hash` é necessário para usar `CounterKey` como chave de `HashMap` no
  `CounterRegistry`.
- `Selector` já implementa `Clone`, `PartialEq` e `Hash` em
  `entities/selector.rs` (confirmar; se faltar, adicionar).

### Construção

- `counter("foo")` → `Counter { key: CounterKey::Str("foo".into()) }`.
- `counter(heading)` / `counter(<selector>)` → `Counter {
  key: CounterKey::Selector(Selector::Kind(ElementKind::Heading)) }`.
- `counter(page)` (quando suportado) → `Counter { key: CounterKey::Page }`.

### Representação

- `repr(counter)` e mensagens de erro usam a representação vanilla da chave
  (ver `CounterKey::repr` no vanilla: `"heading"` para selector de heading,
  `"foo"` para string com repr de string).

## Gatilhos de reabertura

- Novo tipo de chave de counter no vanilla.
- Alteração ao `Selector` usado como chave.

## Critérios de verificação

```
#let c = counter("heading"); c.step()
#context counter("heading").get()  → (1,)   // counter manual
#context counter(heading).get()     → (0,)   // automático ainda não avançou

= T1
#context counter("heading").get()  → (1,)   // inalterado pelo heading
#context counter(heading).get()     → (1,)   // avançou uma vez
```

Aplicação final: `cargo build && crystalline-lint .` — zero violations.
