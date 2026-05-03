# Passo P184E — Tests E2E paridade C3 (figure auto-number per kind)

Quarto passo de implementação P184 (após P184A diagnóstico,
P184B refinamento arm, P184C trait method + helper, P184D
migração consumer).
Magnitude **S**.

Adiciona tests E2E em submódulo `p184e_figure_per_kind` que
cobrem o pipeline completo `walk → from_tags →
layout_with_introspector` para documentos com figures
numeradas. Confirma paridade observable entre path
Introspector (activo após P184B+C+D) e path legacy
(fallback dead code → heurística `idx + 1`). Cobre kinds
distintos isolados.

Após P184E:
- Tests E2E confirmam que migração P184B–D não quebra
  output observable.
- Path Introspector activo validado em pipeline real (não
  só em test unitário do P184C).
- Caminho de fallback validado contra heurística `idx + 1`.
- Sentinela contra regressão de cleanup futuro M6.

**Pré-condição**: P184D concluído. Tests workspace 1.764
verdes; zero violations. Consumer C3 migrado com
substitution-with-fallback. Trait `Introspector` 16
métodos; `CounterRegistry` 6 métodos públicos.

**Restrições**:
- **Não** modificar código de produção em
  `01_core/src/rules/`, `01_core/src/entities/`,
  `02_shell/`, `03_infra/`, `04_wiring/`.
- **Não** modificar walk arm, write-sites, copy-sites
  legacy.
- **Não** modificar trait `Introspector`,
  `extract_payload`, `is_locatable`, `from_tags`,
  Layouter consumer.
- **Não** remover fallback (M6).
- API pública preservada.
- Output observable em produção inalterado — P184E é
  apenas tests novos.

---

## Sub-passos

### .A Auditoria de tests existentes

1. Inventariar tests existentes que cobrem o caminho:
   - `grep -rn "layout_with_introspector\|figure_number_at_index"
     01_core/src/`.
   - Localizar tests P181I (bibliografia) e P182E (heading
     numbering) que correm pipeline E2E. Padrão de helper
     a replicar.
   - Identificar submódulos existentes em `tests.rs`:
     `p182d_heading_numbering`, `p182e_e2e_heading_numbering`,
     `p183b_heading_prefix` (se existir após reverter
     P183B). Submódulo novo `p184e_figure_per_kind` segue
     padrão.

2. Inventariar helpers disponíveis:
   - Construção manual de `Content::Sequence` (padrão
     P181I/P182E que evitou `eval`).
   - `Content::Figure { body, kind, caption, ... }` —
     forma exacta do construtor (verificar
     empiricamente).
   - `Content::SetFigureNumbering` ou similar — existe
     em cristalino? Se não, figures contam por defeito.
   - Helpers para correr pipeline completo (`walk + from_tags`
     + `layout_with_introspector`).

3. Confirmar tests pré-existentes que cobrem figure:
   - `from_tags::tests::figuras_numeradas_recebem_numeros_sequenciais`
     ou similar (per P184B).
   - `figure_number_at_index_*` tests unitários (P184C).
   - Identificar lacuna que P184E preenche: pipeline
     **completo** Layouter + Introspector + paridade
     observable.

Output: tabela com tests existentes + cobertura + lacuna
identificada.

**Critério de saída e gate de decisão**:
- Se helpers de pipeline E2E ausentes (improvável dado
  P181I/P182E os usaram): cláusula gate substancial —
  recuar e investigar.
- Se cobertura existente já inclui pipeline completo
  (improvável): cláusula gate trivial — P184E reduz a
  1-2 tests adicionais cobrindo kinds isolados.
- Senão prosseguir com 3-4 tests novos.

### .B Test E2E — pipeline completo via Introspector

1. Adicionar test em `01_core/src/rules/layout/tests.rs`
   submódulo novo `p184e_figure_per_kind` (irmão de
   `p182e_e2e_heading_numbering`):

2. Construir documento com 3 figures `kind: "image"`:
   - 3 instâncias de `Content::Figure { body, kind:
     Some("image"), caption: Some(...), .. }` em
     sequência.
   - Cada figure tem caption distinto para identificação
     no `plain_text` do output.

3. Pipeline:
   - `walk(content) → tags`.
   - `from_tags(tags) → TagIntrospector`.
   - `layout_with_introspector(content, &introspector) → PagedDocument`.

4. Asserções:
   - `intr.figure_number_at_index("image", 0) == Some(1)`.
   - `intr.figure_number_at_index("image", 1) == Some(2)`.
   - `intr.figure_number_at_index("image", 2) == Some(3)`.
   - `plain_text` do output contém prefixos "Figura 1:",
     "Figura 2:", "Figura 3:" (ou padrão equivalente
     conforme convenção da Layouter).

5. Nome sugerido: `pipeline_completo_figure_kind_image_via_introspector`.

**Critério de saída**:
- Test passa.
- Helpers replicados correctamente (padrão P181I/P182E).

### .C Test E2E — pipeline via fallback (heurística)

1. Adicionar test que valida o caminho fallback:
   - Construir documento idêntico ao `.B`.
   - Pipeline com `TagIntrospector::empty()` em vez do
     populado.
   - `layout_with_introspector(content, &empty)` cai em
     fallback legacy → `unwrap_or(idx + 1)` activa.

2. Asserções:
   - Output observable é idêntico ao `.B`: prefixos
     "Figura 1:", "Figura 2:", "Figura 3:" em ordem.
   - Path activo é `unwrap_or(idx + 1)` mas observable
     é o mesmo.

3. Nome sugerido: `pipeline_via_fallback_legacy_dead_code_idx_plus_one`.

**Critério de saída**:
- Test passa.
- Confirma que fallback heurístico produz mesmo output
  que Introspector populado para casos típicos.

### .D Test E2E — paridade legacy vs migrated

1. Adicionar test que compara paths directos:
   - Documento típico: 3 figures kind "image".
   - Path A: `layout(content)` legacy (ainda existe em
     produção; P184D não removeu — usa o novo consumer
     migrado mas com fallback `unwrap_or(idx + 1)`
     activo se Introspector vazio).
   - Path B: `layout_with_introspector(content,
     introspector_populado)`.
   - Confirmar `plain_text` (ou outro accessor
     observable) idêntico entre A e B.

2. Nome sugerido: `paridade_layout_legacy_vs_layout_with_introspector_figures`.

**Critério de saída**:
- Test passa.
- Paridade observable confirmada.

### .E Test E2E — kinds distintos isolados

1. Adicionar test que cobre o caso "kinds isolados"
   (eixo 2 do bloqueio P183D ratificado):
   - Documento com mistura: 2 figures `kind: "image"`
     + 2 figures `kind: "table"` intercaladas.
   - Pipeline completo.

2. Asserções:
   - `intr.figure_number_at_index("image", 0) == Some(1)`.
   - `intr.figure_number_at_index("image", 1) == Some(2)`.
   - `intr.figure_number_at_index("table", 0) == Some(1)`.
   - `intr.figure_number_at_index("table", 1) == Some(2)`.
   - `plain_text` contém "Figura 1:" para image,
     "Tabela 1:" para table (ou padrão equivalente).
   - Numeração de "image" não interfere com numeração
     de "table" (key isolation).

3. Nome sugerido: `kinds_distintos_isolados_image_e_table`.

**Critério de saída**:
- Test passa.
- Eixo 2 do bloqueio P183D validado em pipeline real
  (não só unitário).

### .F (Opcional) Test E2E — kind None default image

1. Adicionar test cobrindo o caso default `kind: None`:
   - Documento com 1 figure `kind: None` + 1 figure
     `kind: Some("image")`.
   - Pipeline completo.
   - Confirmar que `kind: None` mapeia para chave
     `figure:image` (per P184B convenção).

2. Asserções:
   - Ambas figures aparecem em
     `figure_number_at_index("image", *)`.
   - `intr.figure_number_at_index("image", 0) == Some(1)`.
   - `intr.figure_number_at_index("image", 1) == Some(2)`.

3. Nome sugerido: `kind_none_default_image`.

**Critério de saída**:
- Test passa.
- Default kind validado em pipeline real.

### .G Verificação estrutural

1. `cargo check --workspace` passa.
2. `cargo test --workspace --lib` passa. Δ vs P184D
   baseline (1.764): +3 a +5 dependendo de cobertura
   (`.F` opcional).
3. `crystalline-lint .` zero violations.
4. Tests `p184e_figure_per_kind::*` passam isoladamente
   (`cargo test --workspace --lib p184e`).
5. Tests existentes não regridem.
6. Output observable em produção inalterado (P184E não
   toca produção).
7. Snapshot tests ADR-0033 verdes.
8. Linter passa final.

### .H Encerramento

Escrever
`00_nucleo/materialization/typst-passo-184e-relatorio.md`
com:

- Resumo: 3-5 tests E2E adicionados; paridade Introspector
  vs fallback confirmada; kinds isolados validados em
  pipeline real; default kind validado.
- Confirmação `.G` (8 verificações).
- Δ tests vs baseline P184D.
- Hashes finais de L0s modificados (esperado zero edits
  L0 — apenas tests).
- Decisões de execução notáveis.
- Estado actual:
  - P184 série: A ✅ B ✅ C ✅ D ✅ **E ✅** | F pendente.
  - **C3 desbloqueado e validado** em pipeline real.
  - M9: 11/11 (inalterado).
  - 42 passos executados.
- Pendências cumulativas: inalteradas.
- Próximo passo: P184F (fecho da série + actualização
  DEBT M4-residual).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` produziu auditoria sem disparar gate substancial.
2. Test pipeline completo via Introspector passa (`.B`).
3. Test pipeline via fallback passa (`.C`).
4. Test paridade legacy vs migrated passa (`.D`).
5. Test kinds distintos isolados passa (`.E`).
6. (Opcional) Test default kind passa (`.F`).
7. Tests existentes não regridem.
8. Verificações `.G` passam (8/8).
9. Relatório `.H` escrito.
10. Output observable em produção inalterado.

---

## O que pode sair errado

- **Helpers de pipeline E2E ausentes**: cláusula gate
  substancial. Improvável (P181I/P182E usaram). Se sim,
  recuar.
- **`Content::Figure` tem campos diferentes do esperado**:
  cláusula gate trivial — adaptar construção.
- **Layouter não produz prefixo "Figura N:"** mas outra
  forma (ex.: "Fig 1:", "1.", número solto): cláusula
  gate trivial — adaptar asserções para o que Layouter
  realmente produz. Verificar empiricamente em `.A`.
- **`SetHeadingNumbering` ou `SetFigureNumbering` exigido
  para activar numeração**: se figures não são numeradas
  por defeito, test precisa de set rule. Cláusula gate
  trivial — adaptar.
- **`plain_text` do PagedDocument não inclui texto de
  caption**: cláusula gate trivial — usar accessor
  alternativo (`paged_document.text()`,
  `get_pages()[0].content()`, etc.).
- **Snapshot tests divergem entre paths A e B no `.D`**:
  cláusula gate substancial atrasada. Indica divergência
  observable real entre Introspector e legacy. Investigar
  antes de aceitar P184E como sucesso. Causa provável:
  P184B/C podem ter offset ou semântica diferente que
  passou os tests unitários mas falha no pipeline real.
- **Linter divergência V13/V14**: cláusula gate trivial —
  `--fix-hashes`.

---

## Notas operacionais

- **Tamanho**: S puro. ~80-150 LOC (3-5 tests + helpers
  partilhados se necessário).
- **Sem código de produção tocado** — tests apenas.
- **Sem dependências externas novas**.
- **Pré-condição P184F**: este passo concluído.
- **Padrão replicado**: P181I + P182E (tests E2E em
  submódulo dedicado, paridade legacy vs Introspector via
  pipeline completo).
- **Cláusula gate trivial**: aplicável a forma de
  `Content::Figure`, helpers, accessors do output, nomes
  de tests.
- **Cláusula gate substancial**: aplicável apenas se
  helpers ausentes ou se snapshot tests divergirem
  inesperadamente.
- **`.F` é opcional** — só accionado se `.A` identificar
  que default kind merece teste dedicado. Default é
  incluir.
- **Diferença face a P182E**: P182E §5.2 descobriu que
  fallback legacy era o caminho funcional, não Introspector.
  P184E **invertido**: Introspector é o caminho funcional,
  fallback legacy é dead code → heurística. Confirmação
  desta inversão em pipeline real é o objectivo central
  de P184E `.D`.
