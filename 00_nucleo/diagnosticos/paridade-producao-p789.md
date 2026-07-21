# P789 — Conflito de célula com header de tabela deve errar, não sobrepor silenciosamente

> **Passo:** 789
> **Data:** 2026-07-20, medições entre ~18:00Z e ~18:30Z
> **Commit:** `0774275fe` (+ working tree não commitado: 53 ficheiros alterados herdados de passos anteriores — `git diff HEAD --stat` = 53 files, +1067/−177; a estes somam-se as alterações deste passo: `00_nucleo/prompts/engine/layout.md`, `01_core/src/engine/layout/grid.rs`, `01_core/src/engine/layout/tests.rs`, headers de hash nos consumidores de `layout.md`)
> **Binários:** vanilla `lab/typst-original/target/release/typst` = typst 0.15.0 (rev `969087ec`, build 2026-06-29); cristalino `./target/release/typst` rebuildado neste passo com a correcção
> **Regras aplicadas:** ADR-0107 (mensagem de erro é observável ao nível da língua), ADR-0108 (medir antes de decidir), regra de proveniência de P569.

---

## Resumo em uma linha

**Item 2 (conflito célula↔header não detectado) corrigido** — o cristalino agora erra com mensagem e hint idênticos ao vanilla; **item 1 (repeat-across-páginas) confirmado como o mesmo débito de P772i e mantido deferido** por decisão registada (§2).

---

## 1. Passo 0 — item 1 é o mesmo débito de P772i? **Sim.**

Leitura de `00_nucleo/diagnosticos/paridade-producao-p772i.md:139-164`:

- #9 `Header` / #12 `Footer` ("só renderiza 1 vez") — scope-out **inalterado**;
- #14 `Repeatable<T>` — "repeat-across-páginas não implementado";
- #6 `expand_row_group`, #8 `find_next_empty_row`, #20/#21 `RowGroupData`/`RowGroupKind` — todos scope-out por dependerem de repeat-across-páginas.

O L0 (`00_nucleo/prompts/engine/layout.md` §"Scope-out explícito: repeat-across-páginas", P772i) regista a decisão: *"se uma futura necessidade exigir repeat-across-páginas, tratar como passo dedicado (implica estruturas novas de `range`/`level` e lógica de re-emissão consciente de paginação no motor de grid)"*.

Re-medição neste passo (evidência de P786, `temp/temp_p786/c_bitset_header.typ`): vanilla **7 páginas** (header repetido), cristalino **5 páginas** (header só na página 1) — números de P786 reproduzem no estado actual.

## 2. Decisão de âmbito (registada, não assumida)

| Item | Decisão | Justificativa |
|---|---|---|
| **1 — Repeat-across-páginas** | **Mantém deferido** | É exactamente o débito de P772i (§1), já scope-out explícito no L0 com estimativa de "passo dedicado" (estruturas `range`/`level` + re-emissão pagination-aware). Nenhum facto novo desde P772i altera essa estimativa; P786 classificou-o como "débito já conhecido" — a prioridade alta do achado é o item 2. O passo foi dimensionado M/L precisamente por excluí-lo. |
| **2 — Detecção de conflito célula↔header** | **Implementado neste passo** | "Aceita em silêncio quando deveria errar" — categoria de maior prioridade. Isolado: verificação de sobreposição de range antes do splice, sem precisar do mecanismo de repetição (confirmado pela sonda, §3). |
| — Conflito célula↔**footer** | **Scope-out documentado no L0** | O vanilla verifica também o footer (`footer.start..footer.end`), mas no modelo splice o range absoluto do footer só existe pós-placement e `PlacedCell` não carrega identidade da célula de origem. Port fiel exige estrutura nova; o achado de P786 é só header e nenhuma divergência real de footer foi observada. |

## 3. Sonda — mecanismo vanilla (medido)

`lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:2112` — `check_for_conflicting_cell_row(header_rows, headers, footer, cell_y, rowspan)`: se algum `row` de `cell_y..cell_y+rowspan` está em `header_rows` → `bail!("cell would conflict with header also spanning row {row}"; hint: "try moving the cell or the header")`. Chamada só nos braços `(Custom x, Custom y)` (resolve.rs:2233) e `(Auto x, Custom y)` (resolve.rs:2272) de `resolve_cell_position`, e só quando `!in_row_group`. É o item #4 de P772f que P772i manteve scope-out — a ligação confirmada antes de reimplementar.

**Medição que corrige o enquadramento do passo** (ADR-0108): o repro sugerido no corpo do passo (`table.cell(rowspan: 2)[X]` **sem** `x`/`y`, posicionamento automático) **não** erra no vanilla — medido: exit 0. Células auto contornam as linhas do header (`find_next_available_position`). A evidência real de P786 (`temp/temp_p786/c_bitset_conflict.typ`) usa `table.cell(x: 0, y: 0, rowspan: 2)` — posição **explícita**:

```text
$ vanilla compile c_bitset_conflict.typ
error: cell would conflict with header also spanning row 0
  ┌─ c_bitset_conflict.typ:4:2
  │
4 │   table.cell(x: 0, y: 0, rowspan: 2)[X],
  │   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  │
  = hint: try moving the cell or the header
EXIT: 1

$ cristalino (antes) c_bitset_conflict.typ
EXIT: 0        ← aceite em silêncio
```

**Achado novo medido neste passo (divergência separada, registada):** o "PDF com sobreposição visual" descrito em P786 **não se reproduz** no binário actual. Medido por `pdftotext -bbox`: `X` cai na linha 2 (yMin 89.8, depois das 2 linhas do header em 61.0/75.4) — ou seja, `table.cell(x:, y:)` (e `colspan:`/`rowspan:`) são **ignorados** pelo placement: `extract_cell_fields` (`01_core/src/engine/layout/grid_placement.rs:224-231`) só faz match de `Content::GridCell`; `Content::TableCell` cai no braço `other` e é auto-posicionada como `1×1`. O sintoma real hoje é "posição explícita de `table.cell` ignorada em silêncio", não sobreposição. Fica registado no L0 como divergência candidata a passo futuro; **fora do âmbito deste passo** (a verificação de conflito cobre `TableCell` com `y` explícito — paridade do **erro** — mas honrar as posições no placement é item separado).

## 4. Implementação

Conforme o Protocolo de Nucleação: L0 actualizado **antes** do código; testes escritos primeiro e confirmados a falhar (3 falhos + 1 controlo positivo já verde); implementação a seguir.

- **L0** — `00_nucleo/prompts/engine/layout.md`, nova sub-secção "Detecção de conflito célula↔header (P789)" na secção de P772i/P772v: mecanismo, mensagem/hint vanilla, scope-out do footer e a divergência `TableCell` x/y registada. Hashes actualizados com `crystalline-lint --fix-hashes .` (novo hash `951cd878` nos consumidores de `layout.md`).
- **Código** — `01_core/src/engine/layout/grid.rs` (`layout_grid`, antes do splice): calcula `header_rows = header_cells.len() / num_cols`; se > 0, percorre as células do **corpo** (`Content::GridCell`/`Content::TableCell` com `y: Some(_)`) e, na primeira cujo range `y..y+rowspan` toca `0..header_rows`, emite `SourceDiagnostic::error` com a mensagem exacta do vanilla + `with_hint("try moving the cell or the header")` via `layout_errors` (mesmo caminho de P647) e retorna. `Span::detached()` — elementos não carregam span (mesmo trade-off de P647). Células com `y: None` não são verificadas (auto contorna o header — paridade dos braços do vanilla); células dentro do corpo do header/footer não são verificadas (equivalente ao `!in_row_group`).
- **Testes** — `01_core/src/engine/layout/tests.rs`, bloco `p789_*` junto aos de P772v: (1) repro exacto de P786 (table, `rowspan: 2`, mensagem **e** hint exactos); (2) variante grid com `x`+`y`; (3) só `y` explícito (braço `(Auto, Custom)` do vanilla); (4) controlo positivo fora do header (sem erros, célula renderiza).

## 5. Validação

```text
$ ./target/release/typst /tmp/p789-conflict-explicit.typ   # = evidência de P786
/tmp/p789-conflict-explicit.typ:<detached>: error: cell would conflict with header also spanning row 0
  hint: try moving the cell or the header
EXIT: 1
```

Erro **idêntico ao vanilla** no observável ao nível da língua (ADR-0107): mesma mensagem, mesmo hint, mesmo exit 1. A moldura de apresentação diverge (cristalino `<detached>` sem caixa de contexto — os elementos não carregam span; mesmo trade-off já aceite em P647 e P772i).

- `cargo test --workspace` — **verde**: 4280 passed / 0 failed no `typst-core` (inclui os 4 `p789_*`; antes 4276), 655 + 33 + 29 + 2 + doc-tests nas restantes crates, 0 falhas.
- `crystalline-lint .` — **0 violações** (ver §6).

## 6. Critério de fecho do passo — checklist

- [x] Confirmado se item 1 é o mesmo débito de P772i — **é** (§1).
- [x] Decisão de âmbito registada para os dois itens separadamente (§2).
- [x] Detecção de conflito de célula/header implementada, erro idêntico ao vanilla (§4, §5).
- [x] Repeat-across-páginas mantido deferido com justificativa actualizada (§2 — reaparecimento em P786 não altera a estimativa de P772i; continua a exigir passo dedicado).
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p789.md`.

## 7. Próximo passo (candidatos)

- Conforme indicado no passo: **show rule por string não aplicada em silêncio** (grupo 4 de P786 §5, módulo `eval::rules`).
- Novos candidatos registados neste passo: (a) honrar `table.cell(x:, y:, colspan:, rowspan:)` no placement (§3 — achado medido); (b) conflito célula↔footer (§2 — scope-out com razão documentada); (c) repeat-across-páginas de header/footer (débito P772i, continua a exigir passo dedicado).
