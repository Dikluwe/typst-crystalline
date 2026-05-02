# Passo P181F — Trait `Introspector` métodos `bib_entry_for_key` + `bib_number_for_key`

Quinto passo de materialização P181 (após P181B–P181E).
Magnitude **S**.

Adiciona 2 métodos ao trait `Introspector`. Impl em
`TagIntrospector` delega para `BibStore` populado em P181E.

Após P181F:
- Trait `Introspector` expõe API bib (consumers podem programar
  contra abstracção).
- Field `bib_store` continua público (acesso directo também
  disponível — convenção P181B preservada).
- Layouter ainda não usa nenhum dos dois — migração em P181G.

**Pré-condição**: P181E concluído. `BibStore` populado em
produção via `from_tags` arm. `BibStore::entry_for_key` e
`number_for_key` existem (P181B confirmou; cláusula 2/3 P181A
replicada em P181E).

**Restrições**:
- **Não** modificar walk (P181H).
- **Não** modificar Layouter (P181G).
- **Não** alterar field `bib_store` em `TagIntrospector` (mantém
  público; trait métodos são adicionais).
- API pública preservada — apenas estende trait, não muda
  signatures existentes.
- Output observable não muda — métodos novos sem consumers.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar trait `Introspector` actual:
   - `01_core/src/entities/introspector.rs`.
   - Métodos pré-existentes (P165 + extensões posteriores):
     - `kind_index_for(&self, kind: ElementKind) -> &[Location]`.
     - `query_by_kind(&self, kind: ElementKind) -> &[Location]`.
     - `query_by_label(&self, label: &Label) -> Option<Location>`.
     - `formatted_counter(&self, key: &str) -> Option<String>`.
     - `formatted_counter_at(&self, key: &str, location: Location) -> Option<String>`.
     - `state_value(&self, key: &str, location: Location) -> Option<&Value>`.
     - `state_final_value(&self, key: &str) -> Option<&Value>`.
     - `query(&self, selector: &Selector) -> Vec<Location>`.
   - Identificar localização exacta para inserir 2 métodos
     novos.

2. Confirmar API de `BibStore`:
   - `entry_for_key(&self, key: &str) -> Option<&BibEntry>`
     (P181B).
   - `number_for_key(&self, key: &str) -> Option<u32>`
     (P181B).
   - Confirmar nomes exactos.

3. Confirmar `BibEntry` re-export:
   - `entities/mod.rs` deve re-exportar `BibEntry`.
   - Consumers do trait precisam `use BibEntry` para
     trabalhar com `Option<&BibEntry>`.

4. Confirmar L0 actual:
   - `00_nucleo/prompts/entities/introspector.md`.
   - Identificar onde adicionar entradas para 2 métodos
     bib.

Output: notas internas; sem ficheiro novo.

**Critério de saída**:
- Auditoria limpa.
- API `BibStore` confirmada.

### .B Tests primeiro (devem falhar)

Em `01_core/src/entities/introspector.rs::tests`:

```rust
#[test]
fn bib_entry_for_key_em_introspector_vazio_devolve_none_p181f() {
    let intr = TagIntrospector::empty();
    assert_eq!(intr.bib_entry_for_key("any"), None);
}

#[test]
fn bib_number_for_key_em_introspector_vazio_devolve_none_p181f() {
    let intr = TagIntrospector::empty();
    assert_eq!(intr.bib_number_for_key("any"), None);
}

#[test]
fn bib_entry_for_key_em_introspector_populado_resolve_p181f() {
    use crate::entities::{Tag, ElementInfo, ElementPayload, Location};
    let entry = make_bib_entry("intro");
    let loc = Location::new(0);
    let tags = vec![
        Tag::Start(loc, ElementInfo::new(ElementPayload::Bibliography {
            entries: vec![entry.clone()],
        })),
        Tag::End(loc, 0),
    ];
    let intr = from_tags(&tags);
    assert!(intr.bib_entry_for_key("intro").is_some());
    assert_eq!(intr.bib_number_for_key("intro"), Some(1));
}
```

Confirmar que `cargo test` falha — métodos ainda não
existem no trait nem impl.

**Critério de saída**:
- Tests escritos.
- Compilação falha como esperado.

### .C Update L0 `introspector.md`

Adicionar 2 entradas ao trait:

- `fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry>`.
- `fn bib_number_for_key(&self, key: &str) -> Option<u32>`.

Documentar:
- Semântica idêntica a `BibStore::entry_for_key` /
  `number_for_key`.
- Read-only — método de query, não muta.
- Retorno `None` se introspector vazio ou key
  inexistente.
- Order de assignment de números segue ordem de inserção
  (cláusula 3 P181A — `or_insert`).

**Critério de saída**:
- L0 actualizado.
- Hash recalculado em `.D`.

### .D Humano calcula `@prompt-hash`

Marco humano. Após `.C`:
- `crystalline-lint --fix-hashes`.
- L1 linhagem `introspector.rs` actualizada.

**Critério de saída**:
- L0 hash preenchido.
- L1 `@prompt-hash` correspondente.

### .E Implementar trait + impl

Em `01_core/src/entities/introspector.rs`:

1. Adicionar ao trait `Introspector`:
   ```rust
   pub trait Introspector {
       // ... métodos existentes
       fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry>;
       fn bib_number_for_key(&self, key: &str) -> Option<u32>;
   }
   ```

2. Adicionar impl em `TagIntrospector`:
   ```rust
   impl Introspector for TagIntrospector {
       // ... impls existentes

       fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry> {
           self.bib_store.entry_for_key(key)
       }

       fn bib_number_for_key(&self, key: &str) -> Option<u32> {
           self.bib_store.number_for_key(key)
       }
   }
   ```

3. Confirmar `use crate::entities::BibEntry` no topo
   (ou path qualificado).

**Critério de saída**:
- `cargo check` passa.
- Tests `.B` passam.
- Linter passa.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` — todos os tests
   passam. Δ vs P181E baseline (1722). Estimativa: +3
   (3 tests `.B`).
3. `crystalline-lint .`: zero violations.
4. L0 `introspector.md` actualizado com hash.
5. L1 `introspector.rs` linhagem actualizada.
6. Trait `Introspector` tem 2 métodos novos.
7. `TagIntrospector` impl delega para `bib_store`.
8. `bib_entry_for_key` em vazio → `None`.
9. `bib_number_for_key` em vazio → `None`.
10. `bib_entry_for_key` em populado resolve.
11. Walk **NÃO modificado**.
12. `from_tags` **NÃO modificado** estruturalmente.
13. Layouter **NÃO modificado** (P181G).
14. Walk arm `Content::Bibliography` em `walk()`
    **inalterado** — continua a popular state legacy.
15. Field `bib_store` em `TagIntrospector` continua
    público (acesso directo paralelo).
16. Snapshot tests ADR-0033 verdes.
17. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-181f-relatorio.md`
com:

- Resumo: trait `Introspector` estendido com 2 métodos
  bib; impl em `TagIntrospector` delega para `BibStore`.
  Layouter ainda não consome (P181G).
- Confirmação de cada verificação `.F`.
- Hash final de `introspector.md`.
- Decisões registadas em `.A`:
  - Localização exacta no trait/impl.
  - Re-export de `BibEntry` confirmado.
- Δ tests vs baseline P181E (esperado +3).
- **Estado de P181**: A, B, C, D, E, F concluídos; G-J
  pendentes.
- Pendências cumulativas inalteradas.
- Estado pós-passo: P181F concluído. P181G desbloqueado
  (Layouter cite-arm migra para Introspector).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate.
2. Tests escritos primeiro (`.B`) e falharam.
3. L0 `introspector.md` actualizado com 2 métodos.
4. Hash calculado (`.D`).
5. Trait + impl materializados.
6. Verificações `.F` 1-17 passam.
7. Relatório `.G` escrito.
8. Output observable não muda.
9. API pública estendida sem regressão.
10. Layouter inalterado.

---

## O que pode sair errado

- **`BibEntry` não está re-exportado em `entities/mod.rs`**:
  trait method `Option<&BibEntry>` requer tipo acessível.
  Adicionar `pub use bib_entry::BibEntry`. Cláusula gate
  trivial.
- **Trait `Introspector` tem default impls**: alguns métodos
  podem ter default (e.g. `query_first` baseado em
  `query_by_kind`). Métodos bib são puros delegates —
  sem default.
- **Naming convention divergente**: P165/P175 usaram
  `query_by_kind`, `query_by_label`. P181F adopta
  `bib_entry_for_key`, `bib_number_for_key`. Convenção
  consistente com `BibStore::entry_for_key` (P181B).
- **Tests precisam helper `make_bib_entry`**: criado em
  P181D ou P181E. Reusar.
- **Linter divergência**: ajustar.

---

## Notas operacionais

- **Tamanho**: S. 2 métodos no trait + 2 impls + 3 tests.
  Sem cascade.
- **Pré-condição P181G**: trait expõe API bib; Layouter
  pode consumir via trait OU acesso directo a
  `intr.bib_store`. Decisão local em P181G.
- **Cláusula gate trivial**: aplicável a re-export
  `BibEntry`.
- **Convenção naming**: `bib_entry_for_key` /
  `bib_number_for_key` segue padrão do `BibStore` API
  (P181B). Consistente.
- **Acesso paralelo preservado**: field `bib_store`
  público + getter via trait. Layouter pode escolher
  qual usar. Convenção cristalina sub-store-pub
  (P181B) garante flexibilidade.
- **Output observable inalterado**: trait métodos novos
  sem consumers. P181G começa a usar.
- **Padrão "trait estendido para feature stdlib"**:
  replicado de P175 (query), P176 (formatted_counter),
  P177 (formatted_counter_at), P181F (bib).
