# Relatório P186B — `ElementPayload::Equation` + `ElementKind::Equation`

**Data**: 2026-05-03
**Magnitude**: S puro
**Pré-condição**: P186A concluído ✅; tests workspace 1.783
verdes; zero violations.

---

## Resumo

2 variants paralelos adicionados:

- **`ElementPayload::Equation { block: bool, counter_update:
  CounterUpdate }`** em `entities/element_payload.rs:117-120`.
  Forma paralela a `Figure` (P184B) per cláusula 1 P186A.
- **`ElementKind::Equation`** em `entities/element_kind.rs:36-40`.
  Indexador para `kind_index`. `as_str()` retorna `"equation"`;
  `from_name("equation")` resolve para `Some(Equation)`.

L0s actualizados (`element_payload.md` + `element_kind.md`)
com entradas novas + linhas no Histórico de Revisões.

7 tests unitários novos cobrem variantes construíveis +
equality + distinção entre variants + hash distinto por
`block` field. Tests existentes não regridem.

**Cláusula gate trivial activada (esperada per spec)**:
adição de variant em `ElementPayload` forçou stub no-op arm
em `from_tags.rs:222-226` (`ElementPayload::Equation { .. }
=> {}`). P186E substitui pelo arm funcional.

Variants ficam **estruturalmente correctos** mas **sem
produtor activo** até:
- P186C activar `is_locatable(Content::Equation) = true`.
- P186D adicionar arm em `extract_payload` produzindo
  `Some(ElementPayload::Equation { ... })`.
- P186E substituir o stub no-op por lógica funcional em
  `from_tags`.

Output observable em produção inalterado.

---

## Confirmação `.G` (11/11)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace` passa (Δ vs 1783: +7) | ✅ 1790 verdes |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | `ElementPayload::Equation { block, counter_update }` construível | ✅ test `equation_constroi_e_compara` |
| 5 | `ElementKind::Equation` construível | ✅ test `equation_existe_e_distinto` |
| 6 | `extract_payload(Content::Equation)` ainda retorna `None` (P186D) | ✅ catch-all `_ => None` intocado |
| 7 | `is_locatable(Content::Equation)` ainda `false` (P186C) | ✅ arm não-locatable intocado |
| 8 | `from_tags` arm Equation é stub no-op (P186E substitui) | ✅ `from_tags.rs:226` |
| 9 | Walk arm legacy intocado | ✅ `introspect.rs:377-382` confirmado por `git diff` |
| 10 | Snapshot tests ADR-0033 verdes | ✅ incluídos em workspace |
| 11 | Linter passa final | ✅ |

---

## Δ tests vs baseline

- Baseline P186A: **1783** verdes.
- Após P186B: **1790** verdes.
- Δ: **+7** (excedeu range planeado +2 a +4 — cobertura
  mais completa que mínimo sugerido).

Distribuição:
- 3 tests para `ElementKind::Equation`:
  `equation_existe_e_distinto`, `equation_as_str`,
  `from_name_equation`.
- 4 tests para `ElementPayload::Equation`:
  `equation_constroi_e_compara`,
  `equation_block_distingue_payloads`,
  `equation_distinto_de_outras_variants`,
  `equation_hash_diferente_para_block_distinto`.

Cobertura: construção, equality, distinção entre variants
(Figure, Outline, Citation), hash distinto por `block`,
round-trip `from_name`.

---

## Hashes finais

L0s modificados (2):

| Ficheiro | Hash código (L0) | Hash prompt (`@prompt-hash`) |
|----------|------------------|------------------------------|
| `entities/element_kind.md` | `4421c65e` | `1c2f3200` (no `.rs`) |
| `entities/element_payload.md` | `35fc2e53` | `2901743a` (no `.rs`) |

`crystalline-lint --fix-hashes .` aplicado uma vez. Análise
final ✅ 0 drift warnings remaining.

---

## Decisões de execução notáveis

### Stub no-op em `from_tags.rs` (cláusula gate trivial)

`from_tags.rs:50` tem `match &info.payload` exaustivo (sem
catch-all). Adição de `ElementPayload::Equation` forçou
arm explícito. Per spec §"O que pode sair errado" item 5
("match não-exhaustivo: cláusula gate trivial — adicionar
arm `_ => ...`"), adicionei stub no-op:

```rust
ElementPayload::Equation { .. } => {}
```

P186E substituirá por lógica funcional:
```rust
ElementPayload::Equation { block, counter_update } => {
    intr.kind_index.entry(ElementKind::Equation).or_default().push(*loc);
    if *block && state-active-gate {
        intr.counters.apply_at("equation".to_string(), counter_update.clone(), *loc);
    }
}
```

Sem funcionalidade activada — o caminho extract_payload →
from_tags ainda não está ligado para Equation (P186C/D faltam).

### Convenção de inserção: ordem cronológica de adição

`ElementPayload` e `ElementKind` seguem ordem cronológica
de adição (Heading→Figure→Citation→Metadata→State→
StateUpdate→Outline→Bibliography). `Equation` vai após
`Bibliography` em ambos. Convenção confirmada
empiricamente em `.A.1` e `.A.2`.

### `as_str` + `from_name` adicionados (não mencionados explicitamente em spec)

P186B foi sobre adicionar variants. `ElementKind` tem
métodos `as_str()` e `from_name()` para round-trip
textual (per `element_kind.rs:38-68`). Adicionei
`Equation => "equation"` e `"equation" => Equation` para
preservar invariantes do enum (todos variants têm round-trip).
Sem isto, `as_str()` panic em runtime se chamado para
Equation (Rust exhaustiveness check força). Mesma
abordagem replicada em P181C para Bibliography.

### Sem cláusula gate substancial disparada

- Derives existentes cobrem `Equation` automaticamente
  (`Debug, Clone, PartialEq` em `ElementPayload`;
  `Debug, Clone, Copy, PartialEq, Eq, Hash` em
  `ElementKind`).
- `Hash` manual em `ElementPayload` via `format!("{:?}", self)`
  cobre Equation sem alteração — test
  `equation_hash_diferente_para_block_distinto` confirma.
- `CounterUpdate::Step` aceite directamente.

---

## Estado actual

- **P186 série**: A ✅ B ✅ | C-F pendentes.
- **`ElementPayload`**: 9 → **10 variants**.
- **`ElementKind`**: 8 → **9 variants**.
- **Tests workspace**: 1.783 → **1.790** (+7).
- **Trait `Introspector`**: 18 métodos (inalterado).
- **49 passos executados**.
- **DEBT M4-residual**: cobre C1 + C2 (inalterado;
  fecha após P187 + P188).
- **Padrão diagnóstico-primeiro**: 11ª aplicação confirmada
  (P186A); P186B é primeira materialização da série P186.

---

## Pendências cumulativas

Inalteradas em P186B:

- P183B (C1 heading prefix) — depende P187.
- P183C (C2 equation counter) — depende P186F + P188.
- 4 sites M4-fora-de-escopo — fora de escopo P186.

P186 ainda pendente:
- P186C: activar `is_locatable(Content::Equation) = true`.
- P186D: arm em `extract_payload`.
- P186E: substituir stub no-op em `from_tags` por lógica
  funcional.
- P186F: tests E2E + relatório consolidado.

---

## Próximo passo

**P186C** — modificar `is_locatable` arm `Content::Equation`
de `false` para `true`. Magnitude **trivial** (~3 LOC):
remover `Content::Equation { .. }` da lista de
não-locatables; adicionar arm explícito retornando `true`.
Test pontual + actualizar L0 `rules/introspect/locatable.md`.

Após P186C, `is_locatable(Content::Equation { .. })` retorna
`true`. Layouter (P185C) começa a avançar `Locator` em
equations — mas como `extract_payload` ainda retorna `None`,
walk de introspect **não** emite tag para Equations.
Sincronização-por-construção quebra temporariamente até
P186D ligar o caminho extract_payload.

P185D test `gating_locator_apenas_em_locatables` precisa de
revisão em P186C: actualmente assume Equation não-locatable;
após P186C, terá que passar a contar Equation entre os
locatables.
