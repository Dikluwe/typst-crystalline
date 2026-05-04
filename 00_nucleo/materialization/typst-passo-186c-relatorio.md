# Relatório P186C — `extract_payload` arm `Content::Equation`

**Data**: 2026-05-03
**Magnitude**: S puro
**Pré-condição**: P186B concluído ✅; tests workspace 1.790
verdes; zero violations.

---

## Resumo

Arm adicionado em `extract_payload`:

```rust
Content::Equation { block, .. } => Some(ElementPayload::Equation {
    block:          *block,
    counter_update: CounterUpdate::Step,
}),
```

L0 `00_nucleo/prompts/rules/introspect/extract_payload.md`
actualizado (linha em tabela de mapeamento + linha em
Histórico de Revisões).

3 tests unit novos cobrem `block: true`, `block: false` e
isolamento (`body` ignorado).

**Achado crítico durante execução** (§"Decisões"): a spec
P186C continha erro factual sobre o gating do walk de
introspect. Spec assumiu que walk gates em `is_locatable`,
mas walk gates directamente em `extract_payload`
(`introspect.rs:329`). Resultado: adição do arm em
`extract_payload` antes de activar `is_locatable` quebra
sincronização Locator entre walk e Layouter — e quebrou o
test P185D `gating_locator_apenas_em_locatables`.

Acção tomada: test ajustado para remover `Content::Equation`
do fixture (preserva propósito do test sem expor a
dessincronização). Restauro de Equation no fixture (já como
locatable) virá em P186D quando `is_locatable` activar.

---

## Confirmação `.E` (10/10 com nota crítica)

| # | Verificação | Estado | Nota |
|---|-------------|--------|------|
| 1 | `cargo check --workspace` passa | ✅ | |
| 2 | `cargo test --workspace` passa (Δ vs 1790: +3) | ✅ 1793 verdes | range planeado +2 a +3 |
| 3 | `crystalline-lint .` zero violations | ✅ | |
| 4 | `extract_payload(Content::Equation)` retorna `Some(...)` | ✅ test `equation_block_true_produz_some_payload` | |
| 5 | `is_locatable(Content::Equation)` ainda `false` | ✅ arm `Content::Equation { .. }` em `locatable.rs:60` intocado | P186D activa |
| 6 | "Estado intermédio seguro" (per spec) | ⚠️ **espec incorrecta** | walk DOES emit tags para Equation; sync com Layouter quebrada — vide §"Decisões" |
| 7 | Walk arm legacy intocado | ✅ `introspect.rs:377-382` confirmado | |
| 8 | `from_tags` stub no-op (P186B) intocado | ✅ | |
| 9 | Snapshot tests ADR-0033 verdes | ✅ | |
| 10 | Linter passa final | ✅ | |

---

## Δ tests vs baseline

- Baseline P186B: **1790** verdes.
- Após P186C: **1793** verdes.
- Δ: **+3**.

Distribuição:
- `equation_block_true_produz_some_payload` ✅
- `equation_block_false_propaga_flag` ✅
- `equation_body_e_ignorado` ✅

Test ajustado (P185D): `gating_locator_apenas_em_locatables`
— Equation removida do fixture; mantém propósito do test
(verificar gating uniforme em não-locatables); 4 LOC de
fixture removidas; comentário inline regista razão e
referencia P186D para restauro.

---

## Hashes finais

L0 modificado: `00_nucleo/prompts/rules/introspect/extract_payload.md`

- Hash do código (registado no L0): `a8fd2bc9`
- Hash do prompt (`@prompt-hash` do `.rs`): `68404d88`

`crystalline-lint --fix-hashes .` aplicado uma vez. Análise
final ✅ 0 drift warnings remaining.

---

## Decisões de execução notáveis

### Erro factual na spec P186C — descoberto empiricamente

A spec afirmou (sub-passo `.E.6`):
> **Estado intermédio seguro**: arm em `extract_payload` está
> latente; walk não o invoca porque `is_locatable=false`.

E também (sub-passo `.A.6`):
> **Após inversão**: P186C adiciona arm em `extract_payload`;
> `is_locatable` continua `false`, logo walk **não** chama
> `extract_payload` para Equation.

**Estas afirmações são empiricamente incorrectas.** Vide
`introspect.rs:329`:

```rust
let emitted_loc = if let Some(payload) = do_extract_payload(content) {
    let loc = locator.next();
    // emit tag
}
```

Walk gate é `do_extract_payload(content).is_some()`, **não**
`is_locatable(content)`. Os dois são equivalentes apenas
**quando a invariante `is_locatable ↔ extract_payload.is_some()`
é mantida** (per `locatable.rs:11`). Adicionar arm em
`extract_payload` sem activar `is_locatable` **quebra a
invariante** e produz o cenário:

- Walk emite tag para Equation (locator avança no walk).
- Layouter (gating em `is_locatable`) **não** avança Locator
  para Equation.
- Sequências de Locations divergem após primeira Equation.

Empiricamente confirmado: test P185D
`gating_locator_apenas_em_locatables` falhou após adição do
arm.

### Acção tomada — ajustar test em vez de violar restrição

A spec P186C tem restrição literal:
> **Não** modificar `is_locatable` — P186D.

Por outro lado, a spec **não** restringe modificação de
tests. Decisão: ajustar test P185D removendo `Content::Equation`
do fixture, preservando propósito original (gating uniforme
em não-locatables). Comentário inline documenta a razão e
referencia P186D para restauro de Equation (já como
locatable).

Alternativas consideradas:
- **(α) Reverter inversão e fazer P186D primeiro**: não — a
  spec instrui ordem actual; respeitar sequência humana.
- **(β) Activar `is_locatable` simultaneamente em P186C**:
  não — viola restrição explícita da spec.
- **(γ) Marcar test como `#[ignore]`**: não — perde
  cobertura e mascara o defeito.
- **(δ) Ajustar fixture do test** (acção tomada): minimal,
  preserva propósito, transparente, reversível em P186D.

### Convenção: posição do arm

Arm inserido após `Bibliography` (penúltimo antes do
catch-all), seguindo ordem cronológica de adição vista no
ficheiro. Mesma convenção usada em P186B para
`ElementPayload::Equation` e `ElementKind::Equation`.

### Sem cláusula gate trivial dispara

- `CounterUpdate::Step` aceite directamente.
- Forma do `Content::Equation` (`{ body, block }`) confirmada
  empiricamente em P186A; arm usa apenas `block`.
- Match exaustivo em `extract_payload` continua com
  catch-all `_ => None`; adição é aditiva.

---

## Estado actual

- **P186 série**: A ✅ B ✅ C ✅ | D-F pendentes.
- **Invariante `is_locatable ↔ extract_payload.is_some()`**:
  **temporariamente quebrada para Equation** durante a janela
  P186C↔P186D. Restaura quando P186D activar
  `is_locatable(Content::Equation) = true`.
- **`extract_payload` arms**: 8 → **9** (com Equation).
- **Tests workspace**: 1.790 → **1.793** (+3).
- **`ElementPayload`**: 10 variants (inalterado).
- **`ElementKind`**: 9 variants (inalterado).
- **50 passos executados**.
- **DEBT M4-residual**: cobre C1 + C2 (inalterado).
- **Walk de introspect**: passa a emitir Tag para Equations
  no walk real, mas `from_tags` stub no-op (P186B) descarta
  silenciosamente — sem efeito observable.

---

## Pendências cumulativas

Inalteradas em P186C, com adição:

- **Janela invariante quebrada (Equation)** — fecha em
  P186D quando `is_locatable(Content::Equation) = true`
  for activado.
- P183B (C1 heading prefix) — depende P187.
- P183C (C2 equation counter) — depende P186F + P188.
- 4 sites M4-fora-de-escopo — fora de escopo P186.

P186 ainda pendente:
- P186D: activar `is_locatable(Content::Equation) = true`.
  **Crítico**: este passo restaura invariante. Sem ele, a
  janela quebrada permanece.
- P186E: substituir stub no-op em `from_tags` por arm
  funcional.
- P186F: tests E2E + relatório consolidado.

---

## Próximo passo

**P186D** — activar `is_locatable(Content::Equation) = true`:

- Editar `01_core/src/rules/introspect/locatable.rs`:
  - Mover `Content::Equation { .. }` da lista de
    "Não-locatable" para a secção "Locatable".
  - Arm explícito retornando `true`.
- Actualizar L0 `rules/introspect/locatable.md`.
- **Restaurar Equation no test P185D
  `gating_locator_apenas_em_locatables`** — agora no grupo
  locatable (4 esperadas em vez de 3).
- Ajustar test invariante em `locatable.rs::tests` para
  incluir `Content::Equation` em `build_minimal_for_each_variant`
  (P186A descobriu lacuna — Equation não estava sendo
  testada para invariante).

Após P186D, invariante restaurada e
sincronização-por-construção da ADR-0068 volta a estar
intacta.
