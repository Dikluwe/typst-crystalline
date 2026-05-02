# Passo P181B — `entities/bib_store.rs` + field em `TagIntrospector`

Primeiro passo de materialização do plano P181A. Magnitude **S**.

Cria sub-store `BibStore` paralelo a `MetadataStore`/`StateRegistry`/
`LabelRegistry`/`CounterRegistry`. Adiciona field em `TagIntrospector`
+ getter público. Replica decisões cláusula 1, 2, 3 de P181A.

**Pré-condição**: P181A concluído. Diagnóstico
`diagnostico-bib-store-passo-181a.md` disponível com:
- Cláusula 1 fixada: `BibStore = Vec<BibEntry> + HashMap<String, u32>`
  (sem `IndexMap`; replica shape de `CounterStateLegacy`).
- Cláusula 2 fixada: `add_bibliography` faz `extend` (replica
  comportamento legacy).
- Cláusula 3 fixada: `or_insert` em `bib_numbers` (não sobrescreve
  duplicates).

**Restrições**:
- **Não** modificar `Content`, `ElementKind`, `ElementPayload`,
  walk, `is_locatable`, `extract_payload`, `from_tags`,
  `Introspector` trait, ou Layouter (todos sub-passos
  P181C–P181H).
- **Não** popular `BibStore` em produção — campo existe
  vazio em `TagIntrospector::new`. População começa em
  P181E (`from_tags` arm).
- **Não** fechar lacuna #6 (P181I).
- API pública preservada.
- Output observable não muda.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar ausência de prompt prévio:
   - `ls 00_nucleo/prompts/entities/bib_store.md` — não
     deve existir.
   - `grep -rn "bib_store" 00_nucleo/prompts/` — não
     deve haver referências.
   - Se algum existir: gate substancial. Reabrir.

2. Confirmar `BibEntry` em cristalino:
   - `01_core/src/entities/bib_entry.rs:82-100` — 16
     `pub` fields (P181A confirmou).
   - `BibEntry` deriva `Clone`? `Debug`? Confirmar.
   - Se `BibEntry` não deriva `Clone`: gate trivial,
     adaptar (clone via field-by-field se necessário).

3. Confirmar `TagIntrospector` localização:
   - `01_core/src/entities/introspector.rs`.
   - Identificar onde fields existentes são declarados
     (provavelmente: `kind_index`, `labels`, `counters`,
     `metadata`, `state_registry`, `position_of_loc`).

Output: notas internas; sem ficheiro novo.

**Critério de saída**:
- Auditoria L0 limpa.
- `BibEntry` derivações compatíveis (`Clone` mínimo).
- `TagIntrospector` localizado.

### .B Redigir L0 `entities/bib_store.md`

Criar `00_nucleo/prompts/entities/bib_store.md` com
estrutura padrão (cabeçalho + ADRs + restrições + API +
critérios).

Conteúdo essencial:

- **Cabeçalho**:
  - Camada L1, ficheiro alvo
    `01_core/src/entities/bib_store.rs`.
  - Hash do código L0 em branco.
  - ADRs: ADR-0033 (paridade), ADR-0066 (Introspection
    contexto).

- **Origem vanilla**: nenhuma directa. Vanilla usa
  `IndexMap<PicoStr, hayagriva::Entry>`. Cristalino
  divergência: subset minimal sem hayagriva,
  `Vec<BibEntry>` + `HashMap<String, u32>` paralelo.
  Documentado em P181A diagnóstico.

- **Restrições estruturais**:
  - `pub struct BibStore { entries: Vec<BibEntry>, numbers: HashMap<String, u32> }`.
  - Read-only após construção (mutação apenas via
    `pub(crate) fn add_bibliography` e
    `pub(crate) fn assign_number`).
  - Derives: `Clone` mínimo. `Debug` para hash via
    `format!` se necessário.

- **API**:
  - `pub fn empty() -> Self` — vazio.
  - `pub(crate) fn add_bibliography(&mut self, entries: Vec<BibEntry>)`
    — replica `state.bib_entries.extend(...)` (cláusula
    2 P181A).
  - `pub(crate) fn assign_number(&mut self, key: String, number: u32)`
    — usa `or_insert` (cláusula 3).
  - `pub fn entry_for_key(&self, key: &str) -> Option<&BibEntry>`
    — linear scan sobre `entries` por field `key`.
  - `pub fn number_for_key(&self, key: &str) -> Option<u32>`
    — `numbers.get(key).copied()`.
  - `pub fn entries(&self) -> &[BibEntry]` — slice
    completo (preserva ordem inserção).

- **Critérios de verificação** (testes obrigatórios):
  - `BibStore::empty()` produz `entries.is_empty()` +
    `numbers.is_empty()`.
  - `add_bibliography` com 2 entries produz `len() == 2`;
    chamar de novo com mais 2 entries produz `len() == 4`
    (cláusula 2 — `extend`).
  - `assign_number("a", 1)` + `assign_number("a", 2)` →
    `number_for_key("a") == Some(1)` (cláusula 3 —
    `or_insert`).
  - `entry_for_key("inexistente")` → `None`.
  - `number_for_key("inexistente")` → `None`.
  - `entry_for_key` populado retorna referência ao entry
    com `key` matching.

**Critério de saída**:
- L0 escrito conforme estrutura padrão.
- Sem hash calculado ainda (humano calcula em `.D`).

### .C Tests primeiro (devem falhar)

Criar `01_core/src/entities/bib_store.rs` com **apenas
o módulo `tests`** populado, **sem implementação de
struct**. Tests devem falhar a compilar.

Conteúdo inicial:

```rust
// @prompt 00_nucleo/prompts/entities/bib_store.md
// @prompt-hash <em branco até .D>

// (sem struct ainda — apenas tests)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::BibEntry;

    fn make_entry(key: &str) -> BibEntry {
        // construir BibEntry mínimo conforme campos confirmados em .A
        // (forma exacta depende do que .A revelou)
        todo!()
    }

    #[test]
    fn empty_produz_estado_vazio() {
        let store = BibStore::empty();
        assert!(store.entries().is_empty());
        assert_eq!(store.number_for_key("any"), None);
    }

    #[test]
    fn add_bibliography_extend_replica_legacy() {
        let mut store = BibStore::empty();
        store.add_bibliography(vec![make_entry("a"), make_entry("b")]);
        assert_eq!(store.entries().len(), 2);
        store.add_bibliography(vec![make_entry("c"), make_entry("d")]);
        assert_eq!(store.entries().len(), 4);
    }

    #[test]
    fn assign_number_or_insert_nao_sobrescreve() {
        let mut store = BibStore::empty();
        store.assign_number("a".to_string(), 1);
        store.assign_number("a".to_string(), 2);
        assert_eq!(store.number_for_key("a"), Some(1));
    }

    #[test]
    fn entry_for_key_inexistente_devolve_none() {
        let store = BibStore::empty();
        assert_eq!(store.entry_for_key("nao_existe"), None);
    }

    #[test]
    fn entry_for_key_populado_devolve_referencia() {
        let mut store = BibStore::empty();
        store.add_bibliography(vec![make_entry("intro")]);
        assert!(store.entry_for_key("intro").is_some());
    }
}
```

Confirmar que `cargo test --workspace --lib` falha
(struct não existe).

**Critério de saída**:
- Tests escritos.
- Compilação falha como esperado (`BibStore` undefined).

### .D Humano calcula `@prompt-hash`

Marco humano. Após `.B` redigido, calcular hash de
`bib_store.md` e preencher cabeçalho L0 + linhagem em
`bib_store.rs` (`@prompt-hash <hash>`).

`crystalline-lint --fix-hashes` pode automatizar este
sub-passo.

**Critério de saída**:
- L0 cabeçalho com hash preenchido.
- L1 linhagem `@prompt-hash` correspondente.

### .E Implementar `BibStore`

Em `01_core/src/entities/bib_store.rs`, antes do módulo
`tests`:

```rust
use std::collections::HashMap;

use crate::entities::BibEntry;

#[derive(Clone, Debug)]
pub struct BibStore {
    entries: Vec<BibEntry>,
    numbers: HashMap<String, u32>,
}

impl BibStore {
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
            numbers: HashMap::new(),
        }
    }

    pub(crate) fn add_bibliography(&mut self, entries: Vec<BibEntry>) {
        self.entries.extend(entries);
    }

    pub(crate) fn assign_number(&mut self, key: String, number: u32) {
        self.numbers.entry(key).or_insert(number);
    }

    pub fn entry_for_key(&self, key: &str) -> Option<&BibEntry> {
        self.entries.iter().find(|e| e.key == key)
    }

    pub fn number_for_key(&self, key: &str) -> Option<u32> {
        self.numbers.get(key).copied()
    }

    pub fn entries(&self) -> &[BibEntry] {
        &self.entries
    }
}
```

Adaptar `e.key == key` ao field exacto de `BibEntry` (P181A
.A confirmou nome do field; default `key` ou `citation_key`
ou similar).

Update helper `make_entry` em tests com forma real.

**Critério de saída**:
- `cargo check --workspace` passa.
- `cargo test --workspace --lib` passa todos os 5 tests.

### .F Re-export em `entities/mod.rs`

Adicionar:

```rust
pub use self::bib_store::BibStore;
```

(ou padrão equivalente conforme convenção cristalina —
verificar `mod.rs` actual).

**Critério de saída**:
- `cargo check` passa.
- `BibStore` acessível externamente.

### .G Field em `TagIntrospector` + getter

1. Em `01_core/src/entities/introspector.rs`:
   - Adicionar field privado `bib_store: BibStore` ao
     `struct TagIntrospector`.
   - Inicializar com `BibStore::empty()` no construtor
     (`new()` ou `default()`).
   - Adicionar getter público:
     ```rust
     impl TagIntrospector {
         pub fn bib_store(&self) -> &BibStore {
             &self.bib_store
         }
     }
     ```

2. Update L0 `entities/introspector.md`:
   - Documentar field novo.
   - Documentar getter.

3. Tests:
   - `TagIntrospector::default()` → `bib_store().entries().is_empty()`.

**Critério de saída**:
- `cargo check` passa.
- Tests novos passam.
- Linter passa (sincronização L0↔L1 OK).

### .H Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` — todos os tests
   passam. Δ vs P180 baseline (1700). Estimativa: +6
   (5 BibStore + 1 TagIntrospector).
3. `crystalline-lint .`: zero violations.
4. L0 `entities/bib_store.md` existe com hash preenchido.
5. L1 `entities/bib_store.rs` existe com linhagem.
6. `entities/mod.rs` re-exporta `BibStore`.
7. `TagIntrospector.bib_store: BibStore` field privado.
8. `TagIntrospector::bib_store(&self) -> &BibStore`
   getter público.
9. Walk **NÃO modificado**.
10. `is_locatable`, `extract_payload`, `from_tags`
    **NÃO modificados**.
11. Layouter **NÃO modificado**.
12. `Content::Bibliography` walk arm **inalterado**
    (continua a popular `state.bib_*` legacy — mutação
    apenas removida em P181H).
13. Snapshot tests ADR-0033 verdes.
14. Linter passa final.

### .I Encerramento

Escrever
`00_nucleo/materialization/typst-passo-181b-relatorio.md`
com:

- Resumo: `BibStore` materializado; field em
  `TagIntrospector` + getter público; sem população em
  produção (começa em P181E).
- Confirmação de cada verificação `.H`.
- Hash final de `entities/bib_store.md` (preenchido em
  `.D`).
- Decisões registadas em `.A`:
  - Field exacto de `BibEntry` para lookup (`key` ou
    similar).
  - `BibEntry` derivações confirmadas.
- Δ tests vs baseline P180 (esperado +6).
- **Estado de M9**: 9/11 features (sem alteração — P181B
  é infra; feature bib não conta até P181I fechar
  lacuna #6).
- **Estado de P181**: A concluído (decisões); B concluído
  (BibStore criado); C-J pendentes.
- Pendências cumulativas (sem alteração — P181B é puro
  infra).
- Estado pós-passo: P181B concluído. P181C
  desbloqueado (ElementKind::Bibliography +
  ElementPayload::Bibliography).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria L0 sem disparar gate.
2. L0 `bib_store.md` redigido conforme estrutura padrão.
3. Tests escritos primeiro e confirmaram falhar (`.C`).
4. Hash calculado e preenchido (`.D`).
5. `BibStore` implementado com 5 métodos + 5 tests.
6. Re-exportado em `entities/mod.rs`.
7. Field + getter em `TagIntrospector`.
8. Verificações `.H` 1-14 passam.
9. Relatório `.I` escrito.
10. Output observable não muda.
11. Walk, is_locatable, extract_payload, from_tags,
    Layouter — nenhum modificado.

---

## O que pode sair errado

- **`BibEntry` não deriva `Clone`**: gate trivial.
  `add_bibliography` recebe `Vec<BibEntry>` por valor
  (move semantic), pode evitar clone se Vec não for
  reusado pelo caller.
- **`BibEntry` field para key tem nome diferente do
  esperado**: P181A diagnóstico tem nome exacto.
  Confirmar e adaptar.
- **`BibEntry` Hash/Eq problemáticos**: P181B não
  precisa Hash/Eq de `BibEntry` (apenas Clone +
  iteração + comparação por key). Workaround P169
  desnecessário neste passo.
- **`TagIntrospector` construtor exige Engine ou
  outro contexto**: improvável (P165 estabeleceu
  construção pura). Verificar.
- **Linter divergência no L0 novo**: ajustar conforme
  erro reportado.

---

## Notas operacionais

- **Tamanho**: S. Trabalho mecânico — sub-store
  paralelo aos 4 existentes. Padrão estabelecido.
- **Pré-condição P181C**: `BibStore` existe e é
  acessível via `TagIntrospector.bib_store()`. P181C
  adiciona variants Element* para Bibliography.
- **Cláusula gate trivial**: aplicável a derivações
  de `BibEntry`, nome exacto de field key.
- **Sub-store sem população em produção**: campo
  existe vazio até P181E (`from_tags` arm) começar a
  popular. Aceitável; tests cobrem só comportamento
  da estrutura.
- **L0 padrão sub-store**: seguir convenção
  estabelecida por `metadata_store.md` (P169) e
  `state_registry.md` (P171). Forma de cabeçalho,
  secções, critérios.
- **Protocolo de Nucleação respeitado**: `.A`
  auditoria → `.B` L0 → `.C` tests falham → `.D`
  humano hash → `.E` impl → `.F` re-export → `.G`
  field+getter → `.H` verif → `.I` encerramento.
