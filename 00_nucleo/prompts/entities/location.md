# Prompt L0 — `entities/location`
Hash do Código: a9cb6961

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/location.rs`
**Criado em**: 2026-04-30 (P161 sub-passo .3 — primeiro tipo da série Introspection M1)
**ADRs relevantes**: ADR-0033 (paridade funcional vanilla), ADR-0066 (Introspection runtime — promoção da reserva conceptual a ficheiro PROPOSTO)

---

## Contexto

`Location` é o identificador único e estável de um elemento dentro do documento durante a passagem de introspecção. Forma `u128` em paridade directa com `lab/typst-original/crates/typst-library/src/introspection/location.rs::Location`.

Materializado no P161 como peça inicial da arquitectura Introspection com fixpoint (`desenho-introspection-fixpoint.md`). M1 introduz `Location`, `Locator`, `Tag`, `ElementInfo`, `ElementPayload`, `ElementKind` em paralelo ao motor single-pass actual; walk não emite tags ainda neste passo (P162 fá-lo-á).

`Location` é **pequeno** (16 bytes), `Copy`, `Hash`, `Eq` — usável como chave em qualquer mapa hashado.

Construtor não é público nesta materialização inicial. Só o `Locator` (ver `entities/locator.md`) pode produzir `Location`s. Forçar a passagem pelo gerador garante:

1. Determinismo — duas execuções de walk sobre o mesmo `Content` produzem a mesma sequência de `Location`s.
2. Não-colisão — o gerador é a única fonte; não há risco de duas peças de código produzirem o mesmo `Location` por acidente.

Decisão sobre `Ord`/`PartialOrd` é **adiada** para M2/M3, conforme desenho §8.1. Vanilla expõe `LocationKey(u128)` como wrapper ordenável separado; cristalino postpone até consumer concreto exigir.

---

## Restrições Estruturais

- Camada **L1**: zero I/O; tipo puro.
- Sem `Arc` — é `Copy` directo.
- Hash determinístico (derive `Hash`); a fonte do `u128` interno é responsabilidade do `Locator`.
- Construtor `pub(crate) fn from_raw(u128)` ou similar, restrito ao módulo `entities` para que apenas `Locator` o invoque. **Não** `pub fn new(u128)` — esse seria escapatória.

---

## Interface pública

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location(u128);

impl Location {
    /// Construtor visível apenas dentro do crate L1.
    /// Único call-site legítimo: `Locator::next()`.
    pub(crate) fn from_raw(raw: u128) -> Self;

    /// Retorna o hash interno em forma raw — escape hatch para serialização
    /// e debugging. Não usar para construção.
    pub fn as_u128(&self) -> u128;
}
```

Nenhum trait extra para esta materialização. `Display`, `Ord`, serialização, aritmética: scope-out.

---

## Semântica

- `from_raw(raw)`: empacota o `u128` como `Location`. Visibilidade `pub(crate)` força a entrada via `Locator`.
- `as_u128(&self)`: extrai o hash interno. Uso típico: hash secundário em `Tag::End(location, content_hash)` (paridade vanilla — ver `entities/tag.md`).
- `Hash` derivado: a hash de uma `Location` é o seu próprio `u128` (transparente). Mapas indexados por `Location` são O(1) sem distribuição enviesada.

---

## Invariantes

- Construtor só chamado pelo `Locator`. Violação = bug arquitectural.
- O `u128` interno é opaco para consumidores — nunca interpretado como número, sempre como hash.
- Igualdade é por valor exacto do `u128` (Eq derivado).
- Sem mutação após construção (Copy + sem `&mut self` em métodos).

---

## Consumers actuais

P161 não tem consumers ainda — `Location` existe mas walk não emite. Fica como infraestrutura passiva até P162.

## Consumers planeados

- `entities/tag.rs::Tag::Start(Location, ElementInfo)` (P161 sub-passo .9, ficheiro criado neste mesmo passo).
- `entities/tag.rs::Tag::End(Location, u128)`.
- `rules/introspect.rs` walk em P162 (emite tags com Location gerada por Locator).
- `rules/layout/mod.rs` Layouter recebe stream de tags com Location em P162+.
- `LocationRegistry`/`Introspector` em M3 do desenho (futuro).

---

## Sobre paridade

Vanilla `Location(u128)` em `lab/typst-original/crates/typst-library/src/introspection/location.rs` linha 59. Mesma forma. Cristalino diverge apenas em:

- Construtor não-`pub` (vanilla expõe `pub fn new(u128) -> Self` mas o uso real é também via Locator).
- Sem traits `Locatable`/`Unqueriable`/`Tagged` (vtable markers vanilla, scope-out per `inventario-tipos-introspection-vanilla.md` 2026-04-30).
- Sem `LocationKey` separado (Ord adiada).

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` para mapa completo de tipos vanilla e quais cristalino vai/não vai materializar.

---

## Resultado Esperado

- `01_core/src/entities/location.rs` — definição mínima + 2 métodos + tests unitários básicos (igualdade, hash, copy semantics).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P161 sub-passo .3: criação inicial — peça M1 da arquitectura Introspection com fixpoint | `location.rs`, `location.md` |
