# Prompt L0 — `entities/locator`
Hash do Código: 6e8c9eb3

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/locator.rs`
**Criado em**: 2026-04-30 (P161 sub-passo .4)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`Locator` é o gerador determinístico de `Location`s usado durante o walk de introspecção. Forma minimalista: contador incremental que produz `Location`s únicas dentro de uma execução e idênticas entre execuções equivalentes.

Determinismo é requisito do desenho (§1.3 do `desenho-introspection-fixpoint.md`): walk chamado duas vezes com o mesmo `Content` deve produzir a mesma sequência de `Location`s. Sem essa propriedade o fixpoint não converge.

P161 materializa a forma simples (contador `u64` incremental). Refinos futuros (combinação com hash de path, suporte a measurement mode, link cross-memoization) ficam para M2/M3 conforme desenho.

---

## Restrições Estruturais

- Camada **L1**: zero I/O; tipo puro.
- Estado interno mutável (counter), exposto só via `&mut self`.
- Sem partilha entre threads — uso restrito a um walk single-thread.
- Determinismo: dois `Locator::new()` independentes, recebendo a mesma sequência de chamadas a `next()`, devem produzir `Location`s iguais.
- A representação interna do counter é opaca; consumidores não inspeccionam.

---

## Interface pública

```rust
pub struct Locator {
    counter: u64,
}

impl Locator {
    /// Cria um novo Locator no estado inicial (counter = 0).
    pub fn new() -> Self;

    /// Produz a próxima Location e avança o counter interno.
    pub fn next(&mut self) -> Location;
}

impl Default for Locator { /* delega para new() */ }
```

`Location` importada de `entities::location`.

---

## Semântica

- `Locator::new()`: produz instância com `counter = 0`.
- `next(&mut self) -> Location`: atribui ao counter actual e devolve `Location::from_raw(self.counter as u128)`. Incrementa `self.counter` (`+= 1`).

A representação `u128` da `Location` é o counter directo (zero-extended de `u64`). Documentar no comentário do ficheiro: "implementação minimalista P161; refinos futuros podem combinar com hash de path para suporte cross-memoization".

Implementação alternativa equivalente (também aceitável): hash de `(parent_path, sibling_index)`. Não exigida em P161 — a forma simples é suficiente desde que walk seja determinístico em ordem.

---

## Invariantes

- Cada `next()` produz `Location` distinta de todas as anteriores na mesma instância.
- Dois `Locator::new()` independentes, com a mesma sequência de chamadas, produzem sequências iguais.
- O counter interno é monotónico crescente.
- `Locator` não é `Clone` por design — clonar romperia o invariante de unicidade entre instâncias do mesmo walk. (Se for útil em futuro para sub-walks, adicionar com cuidado em P162+.)

---

## Tests unitários (obrigatórios em P161 sub-passo .4)

- `next()` chamada duas vezes produz `Location`s diferentes.
- `Locator::new() + 5 chamadas a next()` produz a mesma sequência que outro `Locator::new() + 5 chamadas a next()`.
- `Location::as_u128(loc1) < Location::as_u128(loc2)` para `loc1 = locator.next()`, `loc2 = locator.next()` chamados em sequência (verifica monotonia).

---

## Consumers actuais

Nenhum em P161 — walk em `introspect.rs` não chama `Locator` ainda. Existe como infraestrutura passiva.

## Consumers planeados

- `rules/introspect.rs` walk em P162 — `Locator` é instanciado uma vez por chamada a `introspect()` e passado por `&mut` para gerar `Location`s para cada Heading/Figure/Cite encontrados.
- `Introspector` (M3) consumirá os `Location`s emitidos pelo walk para construir índices.

---

## Sobre paridade

Vanilla `Locator<'a>` em `lab/typst-original/crates/typst-library/src/introspection/locator.rs` linha 153. Forma muito mais rica:

- `local: u128` + ponteiro `outer: Option<&'a LocatorLink<'a>>` para cross-memoization comemo.
- `LocatorMode` enum (Default / Measure(Location, Span)).
- `SplitLocator<'a>` para sub-locators.
- `LocatorLink<'a>` com `OnceLock<Resolved>` para cache hit.

Cristalino P161 não usa comemo neste tipo, não tem measurement mode, não precisa de splitting. A forma minimalista é suficiente para single-pass walk sem cross-memoization. Quando M2/M3 introduzirem fixpoint multi-iteração, este `Locator` será refinado ou substituído conforme o desenho.

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) §3 para classificação de `SplitLocator` e `LocatorLink` como "detalhes de impl vanilla" não aplicáveis directamente.

---

## Resultado Esperado

- `01_core/src/entities/locator.rs` — struct + 2 métodos + Default impl + 3 tests unitários.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P161 sub-passo .4: gerador determinístico de Location — peça M1 da Introspection com fixpoint | `locator.rs`, `locator.md` |
