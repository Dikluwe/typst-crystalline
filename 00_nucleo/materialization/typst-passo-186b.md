# Passo P186B — `ElementPayload::Equation` + `ElementKind::Equation`

Primeiro passo de implementação P186 (após P186A diagnóstico).
Magnitude **S**.

Adiciona dois variants paralelos:
- `ElementPayload::Equation { block: bool, counter_update:
  CounterUpdate }` — payload com forma paralela a `Figure`
  (per cláusula 1 P186A: Opção B).
- `ElementKind::Equation` — kind paralelo aos 8 existentes
  (descoberta P186A §11.3 — ambos exigidos).

L0s `entities/element_payload.md` e `entities/element_kind.md`
actualizados. 2-4 tests unit dos enums (variantes
construíveis, equality, debug).

Após P186B:
- `ElementPayload` e `ElementKind` ganham variant Equation.
- `extract_payload` ainda retorna `None` para
  `Content::Equation` (catch-all `_`) — P186D adiciona arm.
- `is_locatable(Content::Equation)` continua `false` —
  P186C activa.
- `from_tags` ainda não tem arm para
  `ElementPayload::Equation` — P186E adiciona.
- Sub-store não recebe entries — P186E.

**Pré-condição**: P186A concluído. Tests workspace 1.783
verdes; zero violations. 6 cláusulas P186A fixadas.

**Restrições**:
- **Não** modificar `is_locatable` — P186C.
- **Não** modificar `extract_payload` — P186D.
- **Não** modificar `from_tags` — P186E.
- **Não** migrar consumer C2 — P188.
- **Não** modificar walk arm legacy
  (`introspect.rs:377-382`).
- API pública preservada (adição de variant é
  retrocompatível).
- Output observable em produção inalterado — variants novos
  sem produtor ainda.

---

## Sub-passos

### .A Auditoria L0

1. Confirmar `ElementPayload` actual:
   - `01_core/src/entities/element_payload.rs:33-105` (per
     P186A §2).
   - 9 variants existentes. Identificar onde inserir
     `Equation` (ordem alfabética? agrupar com `Figure`?
     verificar convenção).

2. Confirmar `ElementKind` actual:
   - `01_core/src/entities/element_kind.rs:16-37` (per
     P186A §2).
   - 8 variants existentes. Identificar onde inserir
     `Equation`.

3. Confirmar `CounterUpdate` tipo:
   - `01_core/src/entities/counter_update.rs` (ou
     similar).
   - Confirmar variants disponíveis (`Step`, `Set(...)`,
     etc. per P171/P184B).
   - Para Equation usar `Step` (counter incrementa por
     equation processada).

4. Confirmar L0s actuais:
   - `00_nucleo/prompts/entities/element_payload.md`.
   - `00_nucleo/prompts/entities/element_kind.md`.
   - Localizar listas de variants.

5. Confirmar tests existentes em `mod tests`:
   - `element_payload.rs` — padrão de tests dos variants
     existentes (provavelmente equality, debug).
   - `element_kind.rs` — idem.
   - Replicar padrão para variants novos.

6. Confirmar derives existentes:
   - `ElementPayload`: `#[derive(...)]` — `Debug`,
     `Clone`, `PartialEq`, `Eq`?
   - `ElementKind`: idem.
   - Variants novos herdam automaticamente — sem ajuste.

Output: tabela com item + estado + linha actual.

**Critério de saída**:
- Ambos enums localizados.
- `CounterUpdate` confirmado.
- L0s + tests padrão identificados.

### .B Actualizar L0 `entities/element_payload.md`

1. Adicionar entrada para variant novo:
   - Nome: `Equation`.
   - Campos: `block: bool`, `counter_update:
     CounterUpdate`.
   - Propósito: payload emitido quando
     `Content::Equation` é processada como locatable.
     Permite walk/from_tags popular `CounterRegistry`
     com chave `"equation"`.
   - Cross-reference: P184B `ElementPayload::Figure`
     (forma paralela).

2. Hash em branco aguarda recálculo.

**Critério de saída**:
- L0 contém entrada nova.
- Coerente com convenção dos variants existentes.

### .C Actualizar L0 `entities/element_kind.md`

1. Adicionar entrada para variant novo:
   - Nome: `Equation`.
   - Propósito: identificador de kind para indexação em
     `kind_index` quando `Content::Equation` é emitido
     como Tag.
   - Cross-reference: variants existentes (Heading,
     Figure, etc.) — replica padrão.

2. Hash em branco aguarda recálculo.

**Critério de saída**:
- L0 contém entrada nova.

### .D Adicionar variants ao `ElementPayload`

1. Em `01_core/src/entities/element_payload.rs`:
   - Adicionar variant
     `Equation { block: bool, counter_update:
     CounterUpdate }` após `Figure` (ou conforme
     convenção `.A.1`).

2. Confirmar `@prompt-hash` actualiza após edit.

**Critério de saída**:
- `cargo check --workspace` passa.
- Linter passa.

### .E Adicionar variants ao `ElementKind`

1. Em `01_core/src/entities/element_kind.rs`:
   - Adicionar variant `Equation` após `Figure` (ou
     conforme convenção `.A.2`).

2. Confirmar `@prompt-hash` actualiza após edit.

**Critério de saída**:
- `cargo check --workspace` passa.
- Linter passa.

### .F Tests unit dos variants

2-4 tests obrigatórios. Padrão dos variants existentes.

#### Tests para `ElementPayload::Equation`

1. **Variant é construível** — `let _ =
   ElementPayload::Equation { block: true,
   counter_update: CounterUpdate::Step };` compila.

2. **Equality** — duas instâncias com mesmos campos são
   iguais.

3. (Opcional) **Debug não panica** — `format!("{:?}",
   instance)` não panica.

#### Tests para `ElementKind::Equation`

1. **Variant é construível** — `let _ =
   ElementKind::Equation;` compila.

2. **Equality** — `ElementKind::Equation ==
   ElementKind::Equation`; diferente de outros kinds.

Tests co-localizados em `mod tests` dos respectivos
ficheiros. Replicar padrão dos tests existentes.

**Critério de saída**:
- 2-4 tests novos passam.
- Tests existentes não regridem.

### .G Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P186A
   baseline (1.783): +2 a +4 dependendo de cobertura.
3. `crystalline-lint .` zero violations.
4. `ElementPayload::Equation { block, counter_update }`
   construível.
5. `ElementKind::Equation` construível.
6. `extract_payload` ainda retorna `None` para
   `Content::Equation` (catch-all intocado — P186D
   adiciona arm).
7. `is_locatable(Content::Equation)` ainda `false`
   (P186C activa).
8. `from_tags` ainda não tem arm Equation (P186E).
9. Walk arm legacy intocado.
10. Snapshot tests ADR-0033 verdes.
11. Linter passa final.

### .H Encerramento

Escrever
`00_nucleo/materialization/typst-passo-186b-relatorio.md`
com:

- Resumo: 2 variants adicionados (`ElementPayload::Equation`
  + `ElementKind::Equation`); enums fechados continuam
  fechados; sem produtor ainda.
- Confirmação `.G` (11 verificações).
- Δ tests vs baseline P186A (esperado +2 a +4).
- Hashes finais L0 modificados (2 ficheiros).
- Decisões de execução notáveis (se houver).
- Estado actual:
  - P186 série: A ✅ B ✅ | C-F pendentes.
  - `ElementPayload`: 9 → 10 variants.
  - `ElementKind`: 8 → 9 variants.
  - 49 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P186C (modificar `is_locatable` arm
  Equation de `false` para `true`).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. L0 `entities/element_payload.md` actualizado.
3. L0 `entities/element_kind.md` actualizado.
4. `ElementPayload::Equation` declarado.
5. `ElementKind::Equation` declarado.
6. 2-4 tests unitários passam.
7. Tests existentes não regridem.
8. Verificações `.G` passam (11/11).
9. Relatório `.H` escrito.
10. Output observable em produção inalterado.

---

## O que pode sair errado

- **`CounterUpdate` exige variant não-`Step`**: cláusula
  gate trivial — adaptar.
- **Derives faltam em `ElementPayload`** (`PartialEq`,
  `Eq`): cláusula gate trivial — adicionar ou ajustar
  tests.
- **Convenção de ordem de variants difere** do esperado
  (não alfabético, não agrupado por similaridade):
  cláusula gate trivial — seguir convenção empírica.
- **`ElementKind` é não-exhaustive** (`#[non_exhaustive]`)
  e adição requer flag adicional: cláusula gate trivial.
- **Tests existentes regridem por adição de variant**
  (ex.: match não-exhaustivo): cláusula gate trivial —
  adicionar arm `_ => ...` ou cobrir explicitamente.
- **Linter divergência V13/V14**: cláusula gate trivial —
  `--fix-hashes`.

---

## Notas operacionais

- **Tamanho**: S puro. ~10 LOC produção (2 variants +
  edits L0). ~30 LOC tests.
- **Sem dependências externas novas**.
- **Pré-condição P186C**: este passo concluído.
- **Padrão replicado**: P181 (BibStore variants), P178
  (Outline locatable kind).
- **Cláusula gate trivial**: aplicável a derives, ordem
  de variants, tests não-exhaustive.
- **Sem cláusula gate substancial esperada**.
- **Achado P186A §11.3 confirmado**: `ElementKind` exige
  adição (não só `ElementPayload`). P186B trata os dois
  num passo só por economia.
- **Gate dormente registado**: variants ficam
  estruturalmente correctos mas sem produtor activo até
  P186D + P186E. P186B é fundação; activação vem nos
  passos seguintes.
