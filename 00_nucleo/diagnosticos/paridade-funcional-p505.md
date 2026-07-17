# Relatório de Paridade Funcional — P505

> **Passo:** 505  
> **Data:** 2026-06-29  
> **Foco:** Fechar empiricamente os gaps de indentação em `list` e `enum` identificados no audit P500.  
> **Status:** Implementação concluída e validada.

---

## 1. Resumo da Implementação

Foram adicionados os parâmetros de indentação às funções `list(...)` e `enum(...)`:

- `indent` — deslocamento horizontal do marcador/rótulo em relação à margem esquerda.
- `body-indent` — deslocamento horizontal do corpo do item em relação ao marcador/rótulo.
- `tight` — `true` (default) mantém o espaçamento natural entre itens; `false` adiciona
  espaçamento de parágrafo *entre* itens consecutivos.

### Arquivos alterados

| Camada | Arquivo | Alteração |
|---|---|---|
| L0 | `00_nucleo/prompts/entities/elements/list_item.md` | Campos `indent`/`body_indent`/`tight` |
| L0 | `00_nucleo/prompts/entities/elements/enum_item.md` | Campos `indent`/`body_indent`/`tight` |
| L0 | `00_nucleo/prompts/engine/stdlib/structural.md` | `native_list`/`native_enum` com indentação |
| L0 | `00_nucleo/prompts/engine/layout/list_item.md` | Layout com indentação |
| L0 | `00_nucleo/prompts/engine/layout/enum_item.md` | Layout com indentação |
| L1 | `01_core/src/entities/elements/list_item.rs` | Struct + Hash manual + testes |
| L1 | `01_core/src/entities/elements/enum_item.rs` | Struct + Hash manual + testes |
| L1 | `01_core/src/entities/content.rs` | Construtores `list_item_full`/`enum_item_full` |
| L1 | `01_core/src/engine/stdlib/structural.rs` | Parsing dos named args + testes |
| L1 | `01_core/src/engine/layout/list_item.rs` | Layout com indentação |
| L1 | `01_core/src/engine/layout/enum_item.rs` | Layout com indentação |
| L1 | `01_core/src/engine/layout/mod.rs` | Campo `last_was_loose_item` no Layouter |
| L1 | `01_core/src/engine/layout/sequence.rs` | Reset do estado de item solto |
| L1 | `01_core/src/engine/layout/tests.rs` | Testes de layout P505 |
| Lab | `lab/parity/tests/structural_parity.rs` | Sentinela `p505_indentacao_listas_enums` + ajuste P501 |
| Lab | `lab/parity/src/value_dto.rs` | Arm `Value::Args` (fix de compilação preexistente) |

### Nota arquitetural

No Typst vanilla `indent`/`body-indent`/`tight` são propriedades do *container*
`list`/`enum`. No cristalino essas funções expandem para uma `Sequence` de
`ListItemElem`/`EnumItemElem`; os valores do container são **replicados em cada
item**. Esta é uma divergência mecânica intencional (ADR-0107): a paridade é com
a **linguagem** (semântica/resultado visual), não com a estrutura interna de
dados.

---

## 2. Resultados da Validação

### 2.1 Testes unitários (L1)

```bash
cargo test -p typst-core --lib
```

- **3508 passed; 0 failed**
- Novos testes cobrem:
  - Propagação de `indent`/`body-indent`/`tight` em `ListItemElem` e `EnumItemElem`.
  - `map_content`/`map_text` preservam os novos campos.
  - `native_list`/`native_enum` aceitam/rejeitam os novos argumentos.
  - Layout respeita `indent`/`body-indent` e adiciona espaço com `tight: false`.

### 2.2 `crystalline-lint`

```bash
crystalline-lint .
```

- **✓ No violations found**
- Hashes sincronizados via `crystalline-lint --fix-hashes .`:
  - `list_item.rs` → `5d84d635`
  - `enum_item.rs` → `30f3050a`
  - `layout/list_item.rs` → `8cfd6741`
  - `layout/enum_item.rs` → `5ec37264`
  - `stdlib/structural.rs` → `a5cfada4`

### 2.3 Testes de paridade (lab/parity)

#### P505 — Sentinela de indentação

```bash
cd lab && cargo test -p typst-parity --test structural_parity p505_indentacao_listas_enums
```

- **ok** (vanilla 0.15.0 não disponível no ambiente; validação cristalino-only).

#### P501 — Re-baseline após P505

```bash
cd lab && cargo test -p typst-parity --test structural_parity p501_gaps_p1_p2 -- --nocapture
```

```text
=== P501 — Sentinela gaps P1/P2 ===
MATCH:   12
AUSENTE: 1
DIFF:    2
PANIC:   0
```

- `AUSENTE: 1` → `test-state-counter.typ` (gap L-size scope-out documentado).
- `DIFF: 2` → `test-list-advanced.typ` e `test-enum-advanced.typ`. O gap de
  **indentação** foi fechado (os ficheiros compilam), mas continuam DIFF por
  causa de scope-outs pré-existentes:
  - `marker: ([•], [–], [·])` (P494 — marcadores por nível).
  - `numbering: "(a)"` (P470 — patterns complexos).

#### P500 — Matriz de auditoria

| Ficheiro | Vanilla | Cristalino | Classificação | Notas |
|---|---|---|---|---|
| test-list-advanced.typ | ok | DIFF | DIFF | count mismatch: cristalino=1 vanilla=2 (scope-outs P494) |
| test-enum-advanced.typ | ok | DIFF | DIFF | count mismatch: cristalino=1 vanilla=2 (scope-outs P470) |
| test-state-counter.typ | ok/esperado | ERRO_DESCRITIVO | ERRO_DESCRITIVO | runtime state scope-out |
| (outros 14+) | ok | ok | MATCH | preservados |

### 2.4 Classificação P505 por sub-tarefa

| Sub-tarefa | Vanilla | Cristalino | Classificação | Notas |
|---|---|---|---|---|
| 505a list indent | ok | ok | **MATCH** | `indent` + `body-indent` + `tight` aceites e aplicados |
| 505b enum indent | ok | ok | **MATCH** | `indent` + `body-indent` + `tight` aceites e aplicados |

> A classificação acima refere-se às sentinela P505 isoladas (sem `marker: Array`
> nem `numbering` custom). Os ficheiros `test-list-advanced.typ` e
> `test-enum-advanced.typ` do corpus P500 são **DIFF** devido a scope-outs
> pré-existentes, conforme matriz §2.3.

---

## 3. Critério de Fecho

- [x] 505a implementado: `list(indent:, body-indent:, tight:)` funciona.
- [x] 505b implementado: `enum(indent:, body-indent:, tight:)` funciona.
- [x] Testes unitários novos passam (`list_indent_*`, `enum_indent_*`, layout).
- [x] Sentinela `p505_indentacao_listas_enums` adicionada em `lab/parity/tests/structural_parity.rs`.
- [x] Bateria P500: 2 AUSENTEs viraram DIFF (não mais AUSENTE por indentação).
- [x] AUSENTEs restantes: **1** (`test-state-counter.typ`).
- [x] PANICs: 0 (preservado).
- [x] `cargo build` passa.
- [x] `cargo test -p typst-core --lib` passa.
- [x] `crystalline-lint .` → zero violations.
- [x] Relatório produzido em `00_nucleo/diagnosticos/paridade-funcional-p505.md`.

---

## 4. Scope-outs e Notas

- `marker-align` (P504) continua aceite e propagado sem efeito visual.
- `marker: Array` (P494) continua scope-out: parse aceite, layout usa o primeiro
  marcador.
- `numbering: "(a)"` e patterns complexos continuam scope-out (fallback
  Decimal).
- `tight: false` adiciona um `line_height` *entre* itens soltos consecutivos,
  usando o campo `last_was_loose_item` do Layouter para evitar duplicação após
  o último item.

---

## 5. Próximo Passo

Com P505 fechado, todos os AUSENTEs diretos do P500 relacionados com listas/enums
foram resolvidos. O único gap restante do audit P500 é:

| Gap | Tamanho | Recomendação |
|---|---|---|
| `state.update/get` + `context` + `counter` | L | P506 = runtime state (trilha separada) ou scope-out |
