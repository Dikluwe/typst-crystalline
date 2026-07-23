# Relatório — typst-passo-847: eliminar arquivos com mais de um header `@prompt`

**Data:** 2026-07-22
**Executor:** Kimi Code (agente principal; inventário por subagente explore, execução por 3 batches paralelos — prompt lido de `00_nucleo/materialization/typst-passo-847.md`).
**Proveniência:** commit HEAD `421241b11` (P846) no arranque, working tree limpa. O trabalho foi feito em paralelo na mesma working tree (batches A e B em ficheiros disjuntos; batch C no repo separado `tekt-linter`).

---

## Passo 1 — Inventário (20 ficheiros)

Comando: `grep -rln '@prompt ' --include='*.rs' 01_core 02_shell 03_infra 04_wiring | while read f; do n=$(grep -c '^//! @prompt ' "$f"); [ "$n" -gt 1 ] && echo "$n $f"; done` → **20 ficheiros** (18 com 2 headers, 2 com 3, 2 com 4 — `bindings.rs` e `structural.rs` com 4). Os quatro conhecidos por erro visível (`rules.rs`, `fallback_fonts.rs`, `pipeline.rs`, `stdlib/mod.rs`) eram só uma fracção.

Causa raiz dominante (9 dos 20): os prompts **de passo** `p792-context-layout-textlang-position.md` e `p793-numbering-enum-hebrew.md` — táticos, multi-ficheiro — tinham sido promovidos a L0, em conflito com a definição de L0 perene por módulo. Outras causas: headers redundantes de padrão (`atomizacao_elementos.md`, `_comum.md`) em ficheiros que já tinham spec dedicada; e headers espúrios apontando para L0s cujo alvo declarado é *outro* ficheiro.

## Passo 2 — Resolução por caso (decisão + justificativa)

**Remoção de header espúrio (6 ficheiros, 8 headers):**
| Ficheiro | Header removido | Justificativa |
|---|---|---|
| `eval/rules.rs` | p792 | O commit P792 só tocou código de P790/P791 aqui; zero símbolos do domínio. |
| `eval/mod.rs` | p792 | 5 linhas mecânicas (registo de `layout` no scope) — absorvida nota em `engine/eval.md`. |
| `layout/mod.rs` | `model/asset.md` | Braço no-op de 3 linhas; o braço `Document` gémeo não declara nada (consistência). |
| `stdlib/mod.rs` | `document.md`, p792, p793 | Só linhas de re-export; fica `_comum.md` (que declara este ficheiro como seu único filho). |
| `fallback_fonts.rs` | `infra/font_metrics.md` | Nasceu com os dois headers em P555 porque o commit tocou o *consumidor*; zero linhas governadas. |
| `layout/curve.rs` | `entities/elements/curve.md`, `engine/stdlib/curve.md` | Os dois L0s declaram alvos que são *outros* ficheiros; fica `atomizacao_elementos.md`. |

**Remoção por redundância de padrão — política única adoptada: spec dedicada substitui o header do padrão** (o padrão só fica nos atomizados sem spec própria — block/boxed/stack/pad; `_comum` só em `stdlib/mod.rs`):
`layout/heading.rs`, `layout/table.rs`, `layout/shape.rs` (perdem `atomizacao_elementos.md`); `stdlib/gradients.rs`, `stdlib/transforms.rs` (perdem `_comum.md`). Bónus: referência morta `engine/layout/shape.md` em `shape_block_behaviour.md` corrigida.

**Fusões de L0 (8 casos):**
| Ficheiro | Fusão | Justificativa |
|---|---|---|
| `eval/bindings.rs` (4→1) | `field-access.md` + `fields.md` + cláusula `text.lang` de p792 → `engine/eval.md` | Os 3 L0s governavam **a mesma função** (`eval_field_access`) — indivisível; grep confirmou zero referências externas antes de apagar. Referências vivas em `eval.md` §P829-D e `eval/table.md` reescritas. |
| `eval/closures.rs` (2→1) | cláusulas de p792 → `engine/eval.md` (§P792 nova) | Intercepção de `layout()` entranhada dentro de `eval_func_call` — dividir partia o dispatcher. |
| `stdlib/structural.rs` (4→1) | `model/document.md` + `model/asset.md` → `structural.md` | 2 nativas entre 30, já reivindicadas no intro do L0; grep confirmou zero referências externas. |
| `stdlib/shapes.rs` (3→1) | `square.md` → `shapes.md` (secção plena) + remoção de `_comum.md` | shapes.md já reservava a "Nota sobre square"; entrada órfã removida de `crystalline.toml`. |
| `stdlib/layout.rs` (3→1) | spec de `native_layout` → `stdlib/layout.md` + remoção de `_comum.md` | `native_layout` é uma nativa de layout como as outras 15. |
| `layout/enum_item.rs` (2→1) | mecanismo `enum_counter` → `layout/enum_item.md` | Cláusula de 10 linhas entranhada na função `layout`. |
| `layout/sequence.rs` (2→1) | idem (reset do enum_counter, absorvido no mesmo L0) | 7 linhas dentro da função. |
| `03_infra/src/pipeline.rs` (2→1) | §4.5 de `footnote_overflow_columns.md` → `infra/pipeline.md` | 7 linhas — responsabilidade natural da pipeline. |
| (extra) `engine/stdlib/curve.md` | absorvido em `stdlib/shapes.md` (namespace `curve`) | Ficou órfão (V7 novo) após a limpeza de `layout/curve.rs` — apagado após absorção. |

**Divisões de ficheiro (2 casos):**
| De → Para | O quê | Porque divisão e não fusão |
|---|---|---|
| `stdlib/structural.rs` → `stdlib/numbering.rs` **(novo)** | `native_numbering`, `format_pattern`, `format_numeral`, `to_hebrew_numeral`, `to_circled_number`, `nth_alpha_char`, `to_roman_numeral` (313 linhas verbatim) | Funções independentes e coesas; promove p793 §1 a L0 perene novo `engine/stdlib/numbering.md`. `counter.rs` repointado para `super::numbering::format_pattern`. |
| `layout/cursor.rs` → `layout/footnote_flush.rs` **(novo)** | `flush_pending_footnote_bodies` (289 linhas verbatim, impl block) | Método coeso; o L0 `footnote_overflow_columns.md` (perene, ~270 linhas governadas) merecia ficheiro próprio em vez de ser absorvido num `layout.md` já grande. |

**L0s apagados (7):** `p792-context-layout-textlang-position.md`, `p793-numbering-enum-hebrew.md`, `engine/eval/field-access.md`, `engine/eval/fields.md`, `engine/model/document.md`, `engine/model/asset.md`, `engine/stdlib/square.md`, `engine/stdlib/curve.md` — todos após absorção integral + grep de referências zero em código e `crystalline.toml`.
**L0 novo (1):** `engine/stdlib/numbering.md`.

## Passo 3 — Hashes sem correção manual

`crystalline-lint --fix-hashes .` → 16 ficheiros re-hasheados, **"0 drift warnings remaining"** — sem nenhuma intervenção manual (a condição que causava o comportamento indefinido deixou de existir). Verificação independente: `grep -c '^//! @prompt '` em todo o código → **zero ficheiros com mais de 1**.

## Passo 4 — Lint endurecido (repo separado `tekt-linter`)

Nova violação **V15 `MultiPromptHeader`** (erro bloqueante): ficheiro L1–L4 com 2+ linhas `@prompt` → erro com a lista dos prompts e a regra. Implementado em `tekt-linter` (`01_core/rules/multi_prompt_header.rs`), com campo `prompt_refs` no parser, wiring/CLI/SARIF, prompt L0 novo no próprio repo (Protocolo de Nucleação respeitado: L0 antes do código), 516+61 testes do linter verdes. Instalado via `cargo install --path .` (substitui `~/.cargo/bin/crystalline-lint`). **Não commitado no tekt-linter** (working tree dele tem a alteração — fica para o dono commitar lá, repositório separado).

## Passo 5 — Validação

- **Sintético V15**: ficheiro temporário com 2 headers criado em `01_core/src/` → `error: Arquivo com 2 headers @prompt (...) [V15]` — removido de seguida (verificado por mim e, independentemente, pelo batch C).
- `crystalline-lint .` → **exit 0, zero errors**; único V7 restante é o pré-existente `infra/package_version_resolution.md` (os outros órfãos foram resolvidos por esta limpeza).
- `cargo test --workspace` → todo verde: typst-core **4632 passed; 0 failed** (4631 + 1 smoke test do módulo novo `footnote_flush.rs`), typst-infra **714 passed; 0 failed**, shell 36, wiring 31, demais 4. Nenhuma lógica alterada — testes movidos de ficheiro, não de comportamento.

## Notas

- O `@prompt-hash` duplicado/errado que o inventário encontrou em ficheiros com 2+ headers (artefacto de copy-paste de `f0db5cd20`) desapareceu com os headers — confirmado pelo `0 drift`.
- `tekt-linter`: a nova V15 está na working tree dele sem commit; `00_nucleo/prompts/linter-core.md` dele já estava desatualizado para V13/V14 (sem V15) — registado como follow-up lá, fora do âmbito deste passo.
