# Passo 181 — Relatório consolidado (lacuna #6 fechada)

**Datas**: 2026-05-01 a 2026-05-02 (9 sub-passos sequenciais).
**Natureza**: ciclo completo de fecho de **lacuna #6** (`bib_entries`
/ `bib_numbers`) via padrão sub-store + locatable kind. **M9 atinge
10/11 features**.
**Pré-condição**: P180 inventário concluído (`inventario-bib-state.md`)
com magnitude S-M e 6 cláusulas gate identificadas.

---

## §1 — Resumo executivo

P181 fecha **lacuna #6** (`bib_entries` / `bib_numbers` em
`CounterStateLegacy` populados por walk arm directo) via 9
sub-passos cumulativos:

- **P181A** (diagnóstico): 6 cláusulas decididas; plano `.B`–`.J`
  validado; magnitude S-M re-confirmada; sem ADR nova; sem DEBT
  novo.
- **P181B–P181F** (infraestrutura): `BibStore` sub-store
  (`Vec<BibEntry>` + `HashMap<String, u32>`); `ElementKind` +
  `ElementPayload` Bibliography; `is_locatable` + `extract_payload`
  arms; `from_tags` arm popula `BibStore`; trait `Introspector`
  ganha `bib_entry_for_key` + `bib_number_for_key`.
- **P181G** (consumer): Layouter cite-arm consulta via Introspector
  com fallback (padrão substitution-with-fallback P168).
- **P181H** (invariante): walk arm `Content::Bibliography` puro;
  `layout()` legacy re-corre `introspect_with_introspector`
  internamente. **Invariante walk puro P163 restaurada para bib**.
- **P181I** (validação): 5 tests E2E codificam pipeline; lacuna #6
  marcada ✅ Resolvida em `m1-lacunas-captura.md`.

**Janela compat encerrada para bib state**. `BibStore` é fonte
única; `state.bib_*` legacy preservado mas vazio em produção (M6
elimina). `layout()` legacy 100% backward-compat.

**Pipeline final**:

```
Content::Bibliography
  ↓
walk (puro) + extract_payload  →  Tag::Start(loc, ElementInfo {
                                       payload: Bibliography { entries }
                                   })
  ↓
from_tags arm Bibliography  →  bib_store.add_bibliography(entries)
                              + assign_number(key, numbers_len()+1)
  ↓
Layouter cite-arm  →  introspector.bib_entry_for_key(key)  /
                       introspector.bib_number_for_key(key)
  ↓
render: [N] / Author (Year) / Author / Year
```

---

## §2 — Sub-passos materializados

| Sub-passo | Output principal | Δ tests | Magnitude |
|-----------|------------------|---------|-----------|
| **P181A** | Decisões + plano (sem código) | 0 | L0-puro |
| **P181B** | `BibStore` + field `pub bib_store` em `TagIntrospector` | +8 | S |
| **P181C** | `ElementKind::Bibliography` + `ElementPayload::Bibliography` + arm defensivo `from_tags` | +6 | S |
| **P181D** | `is_locatable(Bibliography) = true` + `extract_payload` arm | +4 | S |
| **P181E** | `from_tags` arm popula `BibStore`; +`numbers_len()` | +4 | S |
| **P181F** | Trait `Introspector::bib_entry_for_key` + `bib_number_for_key` | +3 | S |
| **P181G** | Layouter cite-arm via Introspector com fallback F1 | +6 | **M** |
| **P181H** | Walk puro + `layout()` legacy re-walks | +2 | S |
| **P181I** | 5 tests E2E + lacuna #6 fechada | +5 | S |
| **Total** | 9 sub-passos | **+38** | 8 S + 1 M |

**Tests cumulativos**: 1700 (P180 auditoria fresh) → **1738** (P181I).
Δ líquido: **+38**.

**Files L0 produzidos/modificados**: 8.
- Novo: `bib_store.md`.
- Modificados: `introspector.md` (P181B + P181F), `element_kind.md`
  (P181C), `element_payload.md` (P181C), `locatable.md` (P181D),
  `extract_payload.md` (P181D), `from_tags.md` (P181E),
  `rules/layout.md` (P181G + P181H), `rules/introspect.md` (P181H).

**Files L1 produzidos/modificados**: 9.
- Novo: `bib_store.rs`.
- Modificados: `introspector.rs`, `mod.rs` (entities), `element_kind.rs`,
  `element_payload.rs`, `from_tags.rs`, `locatable.rs`,
  `extract_payload.rs`, `introspect.rs`, `layout/mod.rs`,
  `layout/tests.rs`.

**Diagnóstico**: `m1-lacunas-captura.md` (P181A snapshot decisões;
P181I lacuna fechada).

---

## §3 — Decisões arquitecturais (6 cláusulas P181A §3)

| # | Cláusula | Decisão fixada |
|---|----------|----------------|
| 1 | Forma de `BibStore` | `Vec<BibEntry>` + `HashMap<String, u32>` (sem `IndexMap`; replica shape de `CounterStateLegacy`) |
| 2 | Multi-Bibliography concat | `add_bibliography` faz `extend` (replica `state.bib_entries.extend`) |
| 3 | `bib_numbers` order preservation | `or_insert` mantido (não sobrescreve duplicates) |
| 4 | Walk arm modificação | **Opção β (walk puro)** — Tag emitida; mutação directa removida; locatable kind adicionado |
| 5 | Layouter cite-arm migração | Trait methods `bib_entry_for_key`/`bib_number_for_key`; cite-arm consulta via Introspector (caminho P168) |
| 6 | Critério de fecho lacuna #6 | **Opção 3** — fecha em "infraestrutura pronta + consumer migrado"; fields legacy preservados até M6 |

**Sem ADR nova** (decisões replicam invariantes/padrões P162–P178).
**Sem DEBT novo** (trabalho residual coberto por F1/M6/DEBT-55).

---

## §4 — Achados não-triviais durante execução

### §4.1 — Bug semântico capturado em P181E

Snippet sugerido na instrução usava `bib_store.len() as u32 + 1`
para `next_num`. Como `add_bibliography` é chamado **depois** do
loop, `len()` (= `entries.len()`) permanece 0 durante toda a
iteração — todas as entries recebiam número 1.

**Solução**: novo método `BibStore::numbers_len()` paralelo a
`state.bib_numbers.len()` do walk arm legacy. Cresce **só em keys
novas** via `or_insert`. L0 `bib_store.md` actualizado.

**Lição**: snippets de instrução são sketches arquitecturais, não
código pronto. Verificar semântica observável antes de copiar.

### §4.2 — Discrepância signature `layout()` em P181H

Instrução assumiu `layout(content)` (1 arg) mas signature real é
`layout(content, initial_state)`. Adaptação: `layout()` mantém
signature; passa a re-correr `introspect_with_introspector(content)`
internamente, descarta o state novo, e usa `initial_state` recebido.

**Custo**: walk extra (caller já fez 1 walk via `introspect()`).
Aceitável — bib feature é raramente usada; documentado em
`rules/layout.md`.

**Lição**: instrução pode ter desactualizações sobre signatures
reais. Auditoria `.A` deve confirmar API actual antes de aplicar
template.

### §4.3 — Test diferencial em P181G

Os 5 tests originais sugeridos pela instrução para P181G passariam
mesmo sem migração efectiva — path legacy via `state.bib_*` daria
mesmo resultado. Adicionei 6º teste
`cite_consulta_introspector_quando_state_legacy_vazio` que constrói
cenário contrived (state vazio + introspector populado) e prova
**explicitamente** que cite-arm consulta o Introspector.

**Lição**: testes de paridade legacy↔novo path são confortáveis mas
podem mascarar falhas de migração. Test diferencial força o caminho
novo a ser exercitado.

### §4.4 — Padrão "trait estendido" replicado pela 4ª vez (P181F)

Sequência: P175 (`query`) → P176 (`formatted_counter`) → P177
(`formatted_counter_at`) → **P181F (`bib_entry_for_key` +
`bib_number_for_key`)**. Mecânica idêntica:

1. Adicionar 1+ métodos ao trait `Introspector` com semântica
   declarativa.
2. Impl em `TagIntrospector` delega para sub-store correspondente.
3. Tests verificam empty + populated cases.

Confirma reusabilidade — futuras features Introspection seguem o
mesmo padrão.

### §4.5 — Padrão substitution-with-fallback P168 replicado (P181G)

Padrão estabelecido em P168 (figure-ref) replicado pela 2ª vez em
P181G (cite-arm):

```rust
let X = self.introspector.X_for_key(key)
    .or_else(|| self.counter.X_legacy.<lookup>(key));
```

`layout()` legacy continua a funcionar (Introspector vazio →
fallback a state); `layout_with_introspector()` usa Introspector
populado. Janela compat com fallback defensivo.

Confirma reusabilidade para outros consumers M5 futuros (4
restantes: `layout_outline`, `counter_helpers`, section-arm,
`layout_equation`).

---

## §5 — Estado final M9 e M5

### §5.1 — M9: 10/11 features

| # | Feature | Sub-passo |
|---|---------|-----------|
| 1 | `metadata(value)` | P169 |
| 2 | `CounterKey` hierarquia | P170 (resolve lacuna #5) |
| 3 | `state(key, init)` + `state_update(key, value)` | P171 |
| 4 | `state_update_with(key, fn)` | P172 + P173 |
| 5 | `query(selector)` minimal | P175 |
| 6 | `counter.final(key)` minimal | P176 |
| 7 | `counter.at(label)` minimal | P177 |
| 8 | Outline cascade — lacuna #7 | P178 |
| 9 | `query` upgrade (`Vec<Location>`) | P179 |
| 10 | **Bib state — lacuna #6** | **P181** |
| 11 | (pendente) `numbering_active` — lacuna #4 | — |

**Restante**: lacuna #4 (`numbering_active`). Infraestrutura pronta
P171; consumer aguarda M5 retomar.

### §5.2 — M5: 2/6 consumers migrados

| Consumer | Estado | Sub-passo |
|----------|--------|-----------|
| `references.rs::layout_ref` (figure-arm) | ✅ | P168 |
| Layouter cite-arm | ✅ | **P181G** |
| Layouter (cite/layout demais) | ⏳ | — |
| `layout_outline` | ⏳ | bloqueado lacuna #3 |
| `counter_helpers` | ⏳ | — |
| `references.rs::layout_ref` (section-arm) | ⏳ | — |
| `layout_equation` | ⏳ | — |

---

## §6 — Estado final lacunas

| # | Lacuna | Estado pós-P181 |
|---|--------|-----------------|
| 1 | `figure.kind` None vs "image" | Parcial (P168 adressa em parte) |
| 2 | Auto-labels | Adiar (intencional) |
| 3 | Body frozen | Manter (intencional) |
| 4 | `numbering_active` | Infraestrutura P171; consumer aguarda |
| 5 | `format_hierarchical` | ✅ **Resolvida em P170** |
| 6 | `bib_entries` / `bib_numbers` | ✅ **Resolvida em P181** |
| 7 | `has_outline` | ✅ **Resolvida em P178** |

**3 resolvidas (#5, #6, #7)**. **1 com infraestrutura pronta (#4)**.
**3 adiadas/intencionais (#1, #2, #3)**.

---

## §7 — Pendências cumulativas

### §7.1 — Janela compat encerrada para bib state

P181H encerrou a janela compat — `BibStore` é fonte única em
produção; state legacy preservado mas vazio. **M6 elimina**:

- `CounterStateLegacy.bib_entries` e `bib_numbers` (vazios em
  produção pós-P181H).
- Copy-sites em `pub fn layout` (linhas 1397, 1399) e
  `pub fn layout_with_introspector` (linhas 1425, 1427).
- Cite-arm fallback a state legacy em `layout/mod.rs:593, 601`.
- Re-walk em `layout()` legacy quando callers adoptarem
  `introspect_with_introspector + layout_with_introspector`
  directamente.

### §7.2 — Pendências pré-existentes inalteradas

- **F1** — `CounterStateLegacy` 18 fields (M6).
- **F2** — `Content` 59 variants em 3 560 linhas (M6/M9).
- **F3** — `Layouter` 19 fields (M6).
- **F10** — `format!("{:?}", x)` como hash determinístico — agora
  cobre payloads `Bibliography` também.
- **DEBT-55** — Bibliography + Cite XL (ADR-0062 PROPOSTO).

### §7.3 — ADR-0062 (`hayagriva` PROPOSTO) independente

P181 trabalha sobre **subset minimal cristalino** (`Vec<BibEntry>`
literal, sem hayagriva). Promoção de ADR-0062 para `IMPLEMENTADO`
continua a depender da decisão futura de adoptar `hayagriva` para
CSL parsing — independente de P181.

Quando ADR-0062 transitar, `BibStore` pode ser reformulado sobre
`IndexMap<EcoString, hayagriva::Entry>` mas o seu papel
arquitectural (sub-store de `TagIntrospector` populado em
`from_tags`) permanece.

---

## §8 — Próximos passos sugeridos

### §8.1 — `numbering_active` (lacuna #4)

Próxima candidata para fechar **M9 11/11**. Infraestrutura pronta:
`StateRegistry` (P171) já cobre o conceito de state booleano por
chave. Falta materializar:
- Padrão sub-store paralelo (`NumberingActiveStore` ou usar
  `StateRegistry` directamente com chave `"numbering:{kind}"`).
- `Content::SetHeadingNumbering` arm em walk + extract_payload.
- Trait method `Introspector::is_numbering_active(kind)`.
- Layouter consulta via Introspector.

Magnitude estimada: **S-M** (similar a P181). Replica padrão.

### §8.2 — M5 retomar

4 consumers restantes:
- **`layout_outline`** — bloqueado lacuna #3 (body frozen). Decisão
  arquitectural: aceitar divergência walk/Introspector ou refactor.
- **`counter_helpers`** — possível refactor sem decisão arquitectural
  pendente.
- **section-arm** em `references.rs::layout_ref` — replicar P168.
- **`layout_equation`** — investigar dependências.

### §8.3 — M6 cleanup

Eliminar fields legacy quando M5 saturar:
- `bib_entries` / `bib_numbers` (P181H deixou vazios).
- Outros fields conforme M5 progredir.

### §8.4 — Features M9 adicionais

- **`here()`** — primitiva que retorna Location actual.
- **`locate(callback)`** — callback baseado em location.

Pré-requisitos arquitecturais a investigar (P175 desbloqueou query
genérica; estes podem precisar de eval no fixpoint).

---

## §9 — Conclusão

P181 é caso paradigmático de fecho de lacuna em cristalino:

- **Diagnóstico-primeiro** (P181A) com 6 decisões fixadas e plano
  validado em 9 sub-passos.
- **Infraestrutura modular** (P181B–F): cada sub-passo S, padrão
  sub-store + locatable kind + trait estendido.
- **Migração de consumer** (P181G): única magnitude M; padrão
  substitution-with-fallback P168 replicado.
- **Restauração de invariante** (P181H): walk puro P163 restaurado;
  janela compat encerrada via `layout()` re-walk interno.
- **Validação E2E** (P181I): 5 tests codificam invariantes;
  lacuna fechada formalmente.

**Métricas finais**: +38 tests; 8 L0; 9 L1; 1 diagnóstico
actualizado; 0 ADR nova; 0 DEBT novo.

**M9: 10/11**. **Lacuna #6: ✅ Resolvida**. **Janela compat
encerrada para bib state**.

P181J (este relatório consolidado) encerra formalmente a série.
