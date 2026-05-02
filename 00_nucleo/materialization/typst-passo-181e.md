# Passo P181E — `from_tags` arm popula `BibStore`

Quarto passo de materialização P181 (após P181B, P181C, P181D).
Magnitude **S**.

Substitui o arm no-op defensivo `ElementPayload::Bibliography {..}
=> {}` (P181C) pela lógica que popula `BibStore` via
`add_bibliography` + `assign_number` em loop. `kind_index` populado
paralelamente para `query_by_kind(Bibliography)` funcionar.

Após P181E:
- Tags de Bibliography são consumidas (`from_tags` arm activo).
- `BibStore` populado em produção.
- `state.bib_*` legacy continua populado paralelamente (walk arm
  inalterado).
- Layouter continua a ler de state legacy (até P181G).

**Pré-condição**: P181D concluído. `Content::Bibliography` é
locatable; walk emite tag automaticamente; `from_tags` arm
Bibliography é no-op; `BibStore` (P181B) acessível via
`intr.bib_store`; `ElementPayload::Bibliography` (P181C) com
`entries: Vec<BibEntry>`.

**Restrições**:
- **Não** modificar walk em `rules/introspect.rs::walk` (P181H).
- **Não** modificar `Introspector` trait (P181F).
- **Não** modificar Layouter (P181G).
- **Não** modificar `Content::Bibliography` walk arm — continua
  a popular `state.bib_*` legacy.
- API pública preservada.
- Output observable não muda — `BibStore` é populado mas ninguém
  o consome ainda.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar arm no-op actual:
   - `01_core/src/rules/introspect/from_tags.rs`.
   - Localizar arm `ElementPayload::Bibliography { .. } => {}`
     (adicionado P181C).
   - Identificar contexto: dentro de match sobre
     `&info.payload`? Variável `intr` ou `introspector`?

2. Confirmar API de `BibStore`:
   - `01_core/src/entities/bib_store.rs`.
   - `pub(crate) fn add_bibliography(&mut self, entries: Vec<BibEntry>)`.
   - `pub(crate) fn assign_number(&mut self, key: String, number: u32)`.
   - `pub fn len(&self) -> usize` (ou similar — P181B
     confirmou 8 métodos; localizar `len`).
   - Se `len()` ausente: adicionar trivialmente.

3. Confirmar acesso a `kind_index`:
   - `from_tags.rs` constrói `TagIntrospector` com
     `kind_index: HashMap<ElementKind, Vec<Location>>`.
   - Padrão estabelecido por outros arms (P165 Heading,
     P169 Metadata, P178 Outline).
   - Replicar mecânica.

4. Confirmar formato de loop:
   - Numeração 1-based.
   - Ordem de iteração: ordem das entries em `entries`
     (ordem de inserção).
   - `assign_number` usa `or_insert` (P181A cláusula 3),
     portanto chamadas duplicadas não sobrescrevem.

5. Confirmar campo `BibEntry.key`:
   - P181B usou `e.key` em `entry_for_key`.
   - Confirmar.

Output: notas internas; sem ficheiro novo.

**Critério de saída**:
- Auditoria limpa.
- `len()` existe ou criado.

### .B Tests primeiro (devem falhar)

Em `01_core/src/rules/introspect/from_tags.rs::tests`:

```rust
#[test]
fn bibliography_arm_popula_bib_store() {
    use crate::entities::{Content, Tag, ElementInfo, ElementPayload, Location, BibEntry};

    let entry_a = make_bib_entry("a");
    let entry_b = make_bib_entry("b");
    let loc = Location::new(0);

    let tags = vec![
        Tag::Start(loc, ElementInfo {
            payload: ElementPayload::Bibliography {
                entries: vec![entry_a.clone(), entry_b.clone()],
            },
            label: None,
        }),
        Tag::End(loc, 0),
    ];

    let intr = from_tags(&tags);

    assert_eq!(intr.bib_store.len(), 2);
    assert!(intr.bib_store.entry_for_key("a").is_some());
    assert!(intr.bib_store.entry_for_key("b").is_some());
}

#[test]
fn bibliography_arm_atribui_numeros_em_ordem() {
    let entries = vec![
        make_bib_entry("primeiro"),
        make_bib_entry("segundo"),
        make_bib_entry("terceiro"),
    ];
    let loc = Location::new(0);

    let tags = vec![
        Tag::Start(loc, ElementInfo {
            payload: ElementPayload::Bibliography { entries },
            label: None,
        }),
        Tag::End(loc, 0),
    ];

    let intr = from_tags(&tags);

    assert_eq!(intr.bib_store.number_for_key("primeiro"), Some(1));
    assert_eq!(intr.bib_store.number_for_key("segundo"), Some(2));
    assert_eq!(intr.bib_store.number_for_key("terceiro"), Some(3));
}

#[test]
fn bibliography_multi_extend_replica_legacy() {
    let loc1 = Location::new(0);
    let loc2 = Location::new(1);

    let tags = vec![
        Tag::Start(loc1, ElementInfo {
            payload: ElementPayload::Bibliography {
                entries: vec![make_bib_entry("a"), make_bib_entry("b")],
            },
            label: None,
        }),
        Tag::End(loc1, 0),
        Tag::Start(loc2, ElementInfo {
            payload: ElementPayload::Bibliography {
                entries: vec![make_bib_entry("c"), make_bib_entry("d")],
            },
            label: None,
        }),
        Tag::End(loc2, 0),
    ];

    let intr = from_tags(&tags);

    assert_eq!(intr.bib_store.len(), 4);
    assert_eq!(intr.bib_store.number_for_key("a"), Some(1));
    assert_eq!(intr.bib_store.number_for_key("d"), Some(4));
}

#[test]
fn bibliography_arm_popula_kind_index() {
    let loc = Location::new(0);
    let tags = vec![
        Tag::Start(loc, ElementInfo {
            payload: ElementPayload::Bibliography {
                entries: vec![make_bib_entry("a")],
            },
            label: None,
        }),
        Tag::End(loc, 0),
    ];

    let intr = from_tags(&tags);

    let bib_locations = intr.kind_index
        .get(&ElementKind::Bibliography)
        .expect("kind_index Bibliography populado");
    assert_eq!(bib_locations.len(), 1);
    assert_eq!(bib_locations[0], loc);
}
```

Confirmar que `cargo test` falha — arm actual é no-op,
`bib_store` permanece vazio.

**Critério de saída**:
- Tests escritos.
- Falham conforme esperado (BibStore vazio).

### .C Update L0 `from_tags.md`

Documentar arm Bibliography:
- Substitui o no-op defensivo (P181C) pela lógica
  efectiva.
- Comportamento: para cada `Tag::Start(loc, info)` com
  `info.payload == Bibliography { entries }`:
  1. `kind_index.entry(Bibliography).or_default().push(loc)`.
  2. Para cada entry em entries: `assign_number(key, len+1)`.
  3. `add_bibliography(entries)`.
- Numeração 1-based.
- `assign_number` usa `or_insert` (cláusula 3) — keys
  duplicadas em multi-Bibliography preservam primeiro
  número.

**Critério de saída**:
- L0 actualizado.
- Hash recalculado em `.D`.

### .D Humano calcula `@prompt-hash`

Marco humano. Após `.C`:
- `crystalline-lint --fix-hashes`.
- L1 linhagem `from_tags.rs` actualizada.

**Critério de saída**:
- L0 hash preenchido.
- L1 `@prompt-hash` correspondente.

### .E Implementar arm

Em `01_core/src/rules/introspect/from_tags.rs`, substituir
o arm no-op:

```rust
// ANTES (P181C):
ElementPayload::Bibliography { .. } => {
    // no-op defensivo até P181E ligar população
}

// DEPOIS (P181E):
ElementPayload::Bibliography { entries } => {
    intr.kind_index
        .entry(ElementKind::Bibliography)
        .or_default()
        .push(*loc);
    let entries_owned = entries.clone();
    for entry in &entries_owned {
        let next_num = intr.bib_store.len() as u32 + 1;
        intr.bib_store.assign_number(entry.key.clone(), next_num);
    }
    intr.bib_store.add_bibliography(entries_owned);
}
```

Adaptar nomes exactos (`intr` vs `introspector`; `loc`
exacto) conforme `.A`.

**Critério de saída**:
- `cargo check` passa.
- Tests `.B` passam.
- Linter passa.

### .F Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` — todos os tests
   passam. Δ vs P181D baseline (1718). Estimativa: +4
   (4 tests `.B`).
3. `crystalline-lint .`: zero violations.
4. L0 `from_tags.md` actualizado com hash.
5. L1 `from_tags.rs` linhagem actualizada.
6. Arm Bibliography em `from_tags` activo (não mais no-op).
7. `BibStore` populado quando `from_tags` processa tags
   Bibliography.
8. `kind_index[Bibliography]` populado paralelamente.
9. Walk **NÃO modificado**.
10. Walk arm `Content::Bibliography` em `walk()` (linha
    567-573) **inalterado** — continua a popular state
    legacy.
11. `Introspector` trait **NÃO modificado** (P181F).
12. Layouter **NÃO modificado** (P181G).
13. Snapshot tests ADR-0033 verdes.
14. Linter passa final.

### .G Encerramento

Escrever
`00_nucleo/materialization/typst-passo-181e-relatorio.md`
com:

- Resumo: arm Bibliography activo; `BibStore` populado em
  produção; `state.bib_*` continua paralelamente.
- Confirmação de cada verificação `.F`.
- Hash final de `from_tags.md`.
- Decisões registadas em `.A`:
  - `BibStore::len()` existente ou criado.
  - Padrão de acesso a `intr.kind_index`.
  - Nome exacto da variável local em from_tags.
- Δ tests vs baseline P181D (esperado +4).
- **Estado de P181**: A, B, C, D, E concluídos; F-J
  pendentes.
- **Paridade verificável**: após P181E, `BibStore` deve
  conter mesmas entries que `state.bib_entries` para
  qualquer Content. Test E2E em P181I valida.
- Pendências cumulativas inalteradas.
- Estado pós-passo: P181E concluído. P181F desbloqueado
  (trait Introspector métodos `bib_entry_for_key` +
  `bib_number_for_key`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate.
2. Tests escritos primeiro (`.B`) e falharam.
3. L0 `from_tags.md` actualizado.
4. Hash calculado (`.D`).
5. Arm Bibliography substitui no-op com lógica completa.
6. Verificações `.F` 1-14 passam.
7. Relatório `.G` escrito.
8. `BibStore` populado em produção.
9. `kind_index[Bibliography]` populado.
10. Output observable não muda — Layouter ainda usa
    state legacy.
11. Walk e walk arm intactos.

---

## O que pode sair errado

- **`BibStore::len()` ausente**: criar trivialmente
  (`pub fn len(&self) -> usize { self.entries.len() }`).
  Talvez já exista de P181B (8 métodos reportados).
- **`intr` é nome diferente** (e.g. `result`, `builder`):
  adaptar.
- **`*loc` deref incorrecto**: `Location` deriva `Copy`;
  `*loc` ou `loc.clone()` ou `*loc` dependendo de como
  `Tag::Start(loc, ...)` patterns. Adaptar.
- **`entries.clone()` caro**: `BibEntry` deriva `Clone`
  (P181B confirmou); custo aceitável. Refino futuro pode
  evitar dupla iteração se performance preocupar.
- **`BibStore.len()` cresce em multi-Bibliography**:
  primeira chamada produz números 1..N; segunda chamada
  produz N+1..M. Order preservada. Test
  `bibliography_multi_extend_replica_legacy` confirma.
- **`assign_number` com `or_insert` em multi-Bibliography**:
  keys duplicadas em Bibliographies separadas preservam
  primeiro número (cláusula 3 P181A). Comportamento
  correcto vs vanilla? Confirmar paridade em P181I.
- **Tests E2E precisam construir Tags**: helper
  `make_bib_entry` reutilizável (criado em P181D).
- **Linter divergência**: ajustar.

---

## Notas operacionais

- **Tamanho**: S. Substituição directa de arm + 4 tests.
  Sem cascade.
- **Pré-condição P181F**: `BibStore` populado em
  produção; `kind_index[Bibliography]` indexável.
  P181F adiciona métodos ao trait `Introspector` que
  expõem isto a consumers.
- **Cláusula gate trivial**: aplicável a ausência de
  `len()`, nome de variável local, deref de Location.
- **Padrão multi-Bibliography**: `add_bibliography`
  faz `extend` (cláusula 2 P181A); `assign_number` usa
  `or_insert` (cláusula 3). Replica comportamento de
  `state.bib_entries.extend` + `state.bib_numbers.entry().or_insert`.
- **Paridade `BibStore` vs `state.bib_*`**: garantida
  por construção (mesma lógica replicada). Validação E2E
  em P181I quando ambos podem ser comparados directamente.
- **Output observable inalterado**: Layouter consome
  state legacy; `BibStore` é shadow data até P181G.
  Snapshot tests verdes.
