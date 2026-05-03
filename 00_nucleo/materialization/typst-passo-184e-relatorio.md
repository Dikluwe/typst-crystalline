# Relatório P184E — tests E2E paridade C3 (figure auto-number per kind)

**Data**: 2026-05-03
**Passo**: P184E — tests E2E em submódulo `p184e_figure_per_kind`
**Resultado**: 5 tests E2E adicionados (incluindo `.F` opcional);
paridade Introspector vs fallback confirmada; kinds distintos
isolados validados em pipeline real; default kind validado;
Δ +5 (1.764 → 1.769); zero violations linter.

---

## §1 Resumo

Submódulo `p184e_figure_per_kind` em
`01_core/src/rules/layout/tests.rs` adiciona 5 tests E2E:

1. **`pipeline_completo_figure_kind_image_via_introspector`** (`.B`):
   pipeline `walk → from_tags → layout_with_introspector` para 3
   figures `kind: image`; assert directos sobre
   `figure_number_at_index("image", 0|1|2)` + verificação de
   prefixos no `plain_text`.
2. **`pipeline_via_fallback_legacy_dead_code_idx_plus_one`** (`.C`):
   pipeline com `TagIntrospector::empty()` força fallback
   legacy → heurística `idx + 1`; output observable idêntico.
3. **`paridade_layout_legacy_vs_layout_with_introspector_figures`**
   (`.D`): comparação directa `assert_eq!(txt_legacy, txt_new)` —
   confirma inversão (Introspector é o caminho activo, fallback é
   redundante mas não regressivo).
4. **`kinds_distintos_isolados_image_e_table`** (`.E`): documento
   misto `image` + `table` intercalados; cada kind tem numeração
   própria; "Figura 1:" aparece duas vezes (image[0] + table[0]);
   "Figura 2:" duas vezes (image[1] + table[1]).
5. **`kind_none_default_image`** (`.F`, opcional): `kind: None`
   mapeia para chave `figure:image` (default P184B); figures sem
   kind explícito partilham counter com `kind: Some("image")`.

Helper `figure(kind, caption_text)` reutilizado em todos os tests
elimina boilerplate.

---

## §2 Sub-passos executados

| Sub-passo | Estado | Notas |
|-----------|--------|-------|
| `.A` Auditoria | ✅ | Padrão P168/P181I/P182E identificado: submódulos dedicados em `tests.rs`. Helpers existentes: `Content::Sequence(Arc::from(vec![...]))`, `introspect_with_introspector(content, None, None)`, `layout_with_introspector(content, state, intr)`. Layouter format hardcoded "Figura {}: " (`mod.rs:440`) — independente do kind. Cláusula gate trivial: ajustar asserções `.E` a observar captions únicas em vez de "Tabela N:". |
| `.B` Pipeline Introspector | ✅ | 3 figures kind image; `figure_number_at_index` retorna 1, 2, 3; plain_text contém todos os prefixos + captions. |
| `.C` Pipeline fallback | ✅ | `TagIntrospector::empty()` força fallback heurístico; output idêntico. |
| `.D` Paridade legacy vs migrated | ✅ | `assert_eq!(txt_legacy, txt_new)` passa — paridade confirmada. |
| `.E` Kinds isolados | ✅ | Image + table intercalados; numeração própria por kind; "Figura 2:" aparece 2× (image[1] + table[1]). |
| `.F` Default kind None | ✅ | `kind: None` partilha chave `figure:image` com `kind: Some("image")`. |
| `.G` Verificação | ✅ | `cargo check`/`test`/`crystalline-lint` todos passam. Δ +5 vs P184D baseline. |
| `.H` Encerramento | ✅ | Este relatório. |

---

## §3 Confirmação `.G` — 8 verificações

1. ✅ `cargo check --workspace` passa.
2. ✅ `cargo test --workspace` passa: **1.769 verdes** (1.509 core + 215 infra + 24 shell + 21 integration). Δ vs P184D baseline 1.764: **+5**.
3. ✅ `crystalline-lint .` zero violations (apenas tests adicionados; sem edits L0).
4. ✅ Tests `p184e_figure_per_kind::*` passam isoladamente: `cargo test --workspace --lib p184e` → 5 passed, 0 failed.
5. ✅ Tests existentes não regridem (1.509 core = 1.504 anterior + 5 novos).
6. ✅ Output observable em produção inalterado — P184E não toca produção.
7. ✅ Snapshot tests ADR-0033 verdes (parte do conjunto 1.769).
8. ✅ Linter passa final.

---

## §4 Hashes finais L0 modificados

**Zero edits L0**. P184E adiciona apenas tests; nem L0 prompts nem
ficheiros de produção foram tocados.

---

## §5 Decisões de execução notáveis

1. **`.E` adaptado por cláusula gate trivial**: o passo sugeria
   asserir "Tabela 1:" para figures de kind table. Inspecção
   empírica em `mod.rs:440` revelou que o Layouter formata sempre
   `format!("Figura {}: ", figure_number)` independente do kind.
   Cláusula gate trivial activada: asserções ajustadas para
   observar captions únicos (`im_a`, `im_b`, `tb_a`, `tb_b`) e
   contar ocorrências de "Figura 2:" (esperado 2×: image[1] +
   table[1]).

2. **`.F` incluído** (declarado opcional na spec): default kind
   `None → "image"` é parte fundamental da convenção P184B e
   merece test E2E dedicado para sentinela contra regressão futura.

3. **Helper `figure(kind, caption_text)` partilhado**: extraído
   para reduzir boilerplate em 5 tests. Padrão simétrico aos
   helpers `doc_typico` em P182E_e2e_heading_numbering.

4. **Inversão "fallback legacy → heurística" registada
   honestamente**: tests `.B` e `.C` validam empiricamente o
   achado P184A §3.6 / P184D §1: legacy é dead code, fallback
   `or_else` cai em `unwrap_or(idx + 1)`. Output observable
   coincide com path Introspector populado por construção (counter
   flat com `apply_at(Step)` produz snapshots `[1], [2], [3], …`).

5. **Sem `Content::SetFigureNumbering`**: figures contam por
   defeito (`numbering: Some(_)` activa numeração). Não foi
   necessário adicionar set rule — convenção replica P168
   (figure-ref) que também não usou set rule.

---

## §6 Estado actual

- **P184 série**: A ✅ B ✅ C ✅ D ✅ **E ✅** | F pendente.
- **C3 desbloqueado e validado em pipeline real**: eixos 1 e 2
  atendidos; consumer migrado; 5 tests E2E confirmam paridade
  observable + isolamento por kind + default kind.
- **M5/M4 progresso**: 6 read-sites migrados (P168 + P181G ×2 +
  P182D ×2 + P184D). C1 e C2 esperam P185+ location-aware Layouter.
- **Trait `Introspector`**: 16 métodos.
- **`CounterRegistry`**: 6 métodos públicos.
- **M9**: 11/11 (inalterado).
- **42 passos executados** (P184D = 41 + P184E = 42).

---

## §7 Pendências cumulativas

Inalteradas em relação ao estado pós-P184D:

- Lacuna #3 (TOC entries via Introspector) — bloqueada, separada da
  série P184.
- DEBT M4-residual a abrir/actualizar em P184F (fecho da série):
  cobertura final será **C1 + C2** (não C3 — fechado e validado em
  P184D + E).
- Pendência paralela P182E §5.2 (location-aware Layouter para
  desbloquear C1+C2) — espera M6+.
- Cleanup dead code legacy em M6.

---

## §8 Próximo passo — P184F

Fecho da série P184:

1. Relatório agregado P184 (resumo A–E + impacto cumulativo).
2. Actualização DEBT M4-residual:
   - **Cenário A** (P183F já correu antes): editar DEBT a remover C3.
   - **Cenário B** (P183F não correu): P184F precede; quando P183F
     correr, abre DEBT cobrindo apenas C1+C2.
3. Confirmação final: tests workspace 1.769 verdes; zero
   violations; M5/M4 progresso 6/12 read-sites; C1+C2 esperam
   P185+ location-aware.

Pré-condição P184F: este passo concluído.
